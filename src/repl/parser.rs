mod getdata;
mod help;
mod savedata;
use crate::calendar::{get_calendar_index, CalendarReference};
use crate::event::Event;
use crate::repl::get_input;
use crate::validator::{get_home_dir, validate_dir_path};
use chrono::{Date, Duration, Local, NaiveTime, TimeZone, Timelike};
use getdata::*;
use savedata::save_event;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::str::FromStr;

use self::savedata::{save_calendar_index, save_new_calendar};

pub fn parse_into_date(input: &str) -> Date<Local> {
    if input.trim().is_empty() {
        return Local::now().date();
    }

    let split_string: Vec<&str> = input.split('/').collect();

    Local.ymd(
        split_string[2].parse().expect("A number was given as year"),
        split_string[1]
            .parse()
            .expect("A number was given as month"),
        split_string[0].parse().expect("A number was given as day"),
    )
}

pub fn parse_into_time(input: &str) -> NaiveTime {
    if input.trim().is_empty() {
        return Local::now().time().with_second(0).unwrap();
    }

    let split_string: Vec<&str> = input.split(':').collect();
    NaiveTime::from_hms(
        split_string[0].parse().expect("A number was given as hour"),
        split_string[1]
            .parse()
            .expect("A number was given as minute"),
        0,
    )
}
/*
As of now, this only accepts input such as '3d', '40min' or '3h'
Eventually, support for a format like '1:20h' should be added.
*/
pub fn parse_into_duration(input: &str) -> Duration {
    if input.trim().is_empty() {
        return Duration::zero();
    }

    let input_lower = &input.to_lowercase();

    match (
        input_lower.contains('d'),
        input_lower.contains('h'),
        input_lower.contains('m'),
    ) {
        (true, false, false) => {
            // Duration has to be 'days'
            Duration::days(
                input_lower.split('d').collect::<Vec<&str>>()[0]
                    .parse()
                    .expect("Valid duration was given"),
            )
        }
        (false, true, false) => {
            // Duration has to be 'hours'
            Duration::hours(
                input_lower.split('h').collect::<Vec<&str>>()[0]
                    .parse()
                    .expect("Valid duration was given"),
            )
        }
        (false, false, true) => Duration::minutes(
            input_lower.split('m').collect::<Vec<&str>>()[0]
                .parse()
                .expect("Valid duration was given"),
        ),
        (_, _, _) => panic!("Error parsing duration. This error should be unreachable"),
    }
}

/*
Get all data necessary to construct a valid Event object and return an Event.
Validate all input and ask for it until it's valid
 */
pub fn get_new_event(name: Option<String>) -> Event {
    println!("Getting new event data...");

    let name = match name {
        Some(name) => name,
        None => {
            print!("Name: ");
            get_name()
        }
    };

    print!("Start date: ");
    let start_date = parse_into_date(&get_start_date());

    print!("Start time: ");
    let start_time = parse_into_time(&get_start_time());

    print!("Duration: ");
    let mut duration = parse_into_duration(&get_duration());

    let end_date;
    let end_time;
    if duration.is_zero() {
        print!("End date: ");
        end_date = parse_into_date(&get_end_date(&start_date));
        print!("End time: ");
        end_time = parse_into_time(&get_end_time(&start_date, &start_time, &end_date));
	duration = end_date.and_time(end_time).unwrap() - start_date.and_time(start_time).unwrap();
    } else {
        let end_timedate = start_date.and_time(start_time).unwrap() + duration;
        end_date = end_timedate.date();
        end_time = end_timedate.time();
    }

    print!("Difficulty: ");
    let difficulty = get_difficulty().parse().unwrap();

    print!("Priority: ");
    let priority = get_priority().parse().unwrap();

    Event {
        name,
        start: start_date.and_time(start_time).unwrap(),
        duration,
        end: end_date.and_time(end_time).unwrap(),
        priority,
        difficulty,
    }
}

pub fn check_calmar_dir() {
    let mut calmar_dir = get_home_dir();
    calmar_dir.push(".calmar");

    match Path::new(&calmar_dir).is_dir() {
        true => (),
        false => match fs::create_dir(&calmar_dir) {
            Ok(_) => (),
            Err(err) => {
                println!(
                    "Failed to create directory {}\n{}",
                    calmar_dir.display(),
                    err
                );
                return;
            }
        },
    }
}

/*
Given 'name' of a new calendar, the function gets the home directory,
verifies the existence of a $HOME/.calmar directory,
creates a JSON file with the given 'name' under $HOME/.calmar.
If file named 'name' already exists, it asks the user for confirmation.
*/
pub fn get_new_calendar_reference(name: Option<String>) -> CalendarReference {
    let name = match name {
        Some(name) => name,
        None => {
            print!("Name: ");
            get_input()
        }
    };

    print!("Path: ");
    let mut path = get_input();
    while !validate_dir_path(&path){
	println!("Invalid input.");
	print!("Path: ");
	path = get_input();
    }
    let mut path_to_calendar = PathBuf::from(path).join(&name);
    path_to_calendar.set_extension("json");
    let path_to_calendar_string = match path_to_calendar.to_str() {
	Some(string) => string,
	None => {
	    println!("Failed to convert {} to string.", path_to_calendar.display());
	    std::process::exit(1);
	}
    };
    CalendarReference { name, path: path_to_calendar_string.to_owned() }
}

pub fn yesno(text: &str) -> bool {
     match text.to_lowercase().as_str() {
         "yes" | "y" => true,
         _ => false,
     }
}

/*
Call event creation with name given optionally
*/
pub fn add(split_input: &Vec<&str>) {
    let new_event: Event = match split_input.len() {
        1 => get_new_event(None),
        2 => get_new_event(Some(split_input[1].to_owned())),
        _ => {
            println!(
                "add: Too many arguments provided. Expected: 0 or 1, Got: {}",
                split_input.len() - 1
            ); // do not count "add" as an argument
            return;
        }
    };
    match save_event(new_event, get_calendar_index().current_calendar) {
	true => {
	    println!("Successfully saved new event.");
	},
	false => {
	    println!("Failed to save new event.");
	}
    }
}

/*
Call calendar creation with name given optionally
*/
pub fn cal(split_input: &Vec<&str>) {
    let new_reference = match split_input.len() {
        1 => get_new_calendar_reference(None),
        2 => get_new_calendar_reference(Some(split_input[1].to_owned())),
        _ => {
            println!(
                "add: Too many arguments provided. Expected: 0 or 1, Got: {}",
                split_input.len() - 1
            ); // do not count "add" as an argument
	    return;
        }
    };
    let mut calendar_index = get_calendar_index();
    match calendar_index.add_entry(&new_reference) {
	Ok(_) => println!("Added entry to calendar index."),
	Err(_) => println!("Failed to get calendar reference")
    }
    save_calendar_index(calendar_index);
    println!("Saved calendar index");
    save_new_calendar(new_reference);
}

/*
Delete an event from the currently set calendar
*/
pub fn remove(split_input: &Vec<&str>) {
    println!("{:?}", split_input);
    todo!();
}

/*
Edit attributes of a given event and save it
*/
pub fn edit(split_input: &Vec<&str>) {
    println!("{:?}", split_input);
    todo!();
}

/*
Delete a given calendar
*/
pub fn removecal(split_input: &Vec<&str>) {
    println!("{:?}", split_input);
    todo!();
}

/*
Display events in the currently set calendar
*/
pub fn show(split_input: &Vec<&str>) {
    println!("{:?}", split_input);
    todo!();
}

pub fn parse(input: String) {
    let split_input: Vec<&str> = input.split_whitespace().collect();
    match split_input[0].trim().to_lowercase().as_str() {
        "add" | "a" => add(&split_input),
        "cal" | "c" => cal(&split_input),
        "edit" | "e" => edit(&split_input),
        "help" | "h" => help::print_help(split_input[0]),
        "remove" | "rm" | "r" => remove(&split_input),
        "removecal" | "rmcal" | "rc" => removecal(&split_input),
        "show" | "s" => show(&split_input),
        "quit" | "q" => std::process::exit(0),
        _ => println!("Unknown command: {}", split_input[0].trim()),
    }
}
