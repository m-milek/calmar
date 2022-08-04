mod getdata;
mod help;
mod savedata;
use self::savedata::{save_calendar_index, save_new_calendar};
use crate::CONFIG;
use crate::calendar::{get_active_calendar_reference, get_calendar_index, CalendarReference, get_active_calendar};
use crate::event::{Event, save_calendar};
use crate::repl::get_input;
use crate::validator::get_home_dir;
use chrono::{Date, Duration, Local, NaiveTime, TimeZone, Timelike};
use colored::Colorize;
use getdata::*;
use savedata::save_event;
use std::fs::{self};
use std::path::{Path, PathBuf};

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
    let duration = parse_into_duration(&get_duration());

    let end_date;
    let end_time;
    if duration.is_zero() {
        print!("End date: ");
        end_date = parse_into_date(&get_end_date(&start_date));
        print!("End time: ");
        end_time = parse_into_time(&get_end_time(&start_date, &start_time, &end_date));
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
                    "{}", format!("Failed to create directory {}\n{}",
                    calmar_dir.display(),
                    err).red().bold()
                );
            }
        },
    }
}

pub fn default_or_custom(input: String) -> String {
    if input.trim().is_empty() {
	return CONFIG.default_path.clone();
    }
    input
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
    let path = default_or_custom(get_dir_path());
    
    let mut path_to_calendar = PathBuf::from(path).join(&name);
    path_to_calendar.set_extension("json");
    let path_to_calendar_string = match path_to_calendar.to_str() {
        Some(string) => string,
        None => {
            println!(
                "{}", format!("Failed to convert {} to string.",
                path_to_calendar.display()).red().bold()
            );
            std::process::exit(1);
        }
    };
    CalendarReference {
        name,
        path: path_to_calendar_string.to_owned(),
        active: false,
    }
}

pub fn yesno(prompt: &str) -> bool {
    print!("{}", prompt);
    matches!(get_input().trim().to_lowercase().as_str(), "yes" | "y")
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
                "{}", format!("add: Too many arguments provided. Expected: 0 or 1, Got: {}",
                split_input.len() - 1).yellow().bold()
            ); // do not count "add" as an argument
            return;
        }
    };
    match save_event(new_event, get_active_calendar_reference()) {
        true => {
            println!("{}", "Successfully saved new event.".green().bold());
        }
        false => {
            println!("{}", "Failed to save new event.".red().bold());
        }
    }
}

/*
Call calendar creation with name given optionally
*/
pub fn cal(split_input: &Vec<&str>) {
    let mut new_reference = match split_input.len() {
        1 => get_new_calendar_reference(None),
        2 => get_new_calendar_reference(Some(split_input[1].to_owned())),
        _ => {
            println!(
                "{}", format!("cal: Too many arguments provided. Expected: 0 or 1, Got: {}",
                split_input.len() - 1).yellow().bold()
            ); // do not count "cal" as an argument
            return;
        }
    };

    let mut calendar_index = get_calendar_index();
    if calendar_index.calendars.is_empty() {
        new_reference.active = true;
    }

    match calendar_index.add_entry(&new_reference) {
        Ok(_) => println!("{}", "Added entry to calendar index.".green().bold()),
        Err(_) => {
            println!("{}", "Failed to add new calendar reference to calendar index.".red().bold());
            return;
        }
    }
    save_calendar_index(calendar_index);
    println!("{}", "Saved calendar index".green().bold());
    save_new_calendar(new_reference);
}

pub fn get_valid_event_name() -> String {
    let mut input = get_input();
    while input.is_empty() {
	println!("{}", "Event name cannot be an empty string".yellow().bold());
	print!("Name: ");
	input = get_input();
    }
    input
}

/*
Delete an event from the currently set calendar
*/
pub fn remove(split_input: &Vec<&str>) {
    let name = match split_input.len() {
	1 => get_valid_event_name(),
	2 => split_input[1].to_owned(),
	_ => {
	    println!("remove: Too many arguments provided. Expected: 1 or 2. Got: {}", split_input.len()-1);
	    return ()
	}
    };
    let mut active_calendar = get_active_calendar();
    active_calendar.events.retain(|event| event.name != name);
    save_calendar(active_calendar, get_active_calendar_reference().path);
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

    let mut index = get_calendar_index();
    let name = match split_input.len() {
	1 => get_valid_calendar_name(),
	2 => split_input[1].to_string(),
	_ => {
	    println!("{}", format!("removecal: Too many arguments provided. Expected: 0 or 1. Got: {}", split_input.len()).yellow().bold());
	    return
	}
    };

    match index.delete_entry(name) {
	Ok(_) => (),
	Err(_) => return
    }

    save_calendar_index(index);
    println!("{}", "Successfully removed calendar".green().bold());
}

/*
Display events in the currently set calendar
*/
pub fn show(split_input: &Vec<&str>) {
    let active_calendar = get_active_calendar();
    for event in active_calendar.events {
	println!("{:#?}\n\n", event);
    }
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
        _ => println!("{}", format!("Unknown command: {}", split_input[0].trim()).yellow().bold()),
    }
}
