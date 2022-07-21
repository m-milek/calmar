mod help;
use crate::event::Event;
use crate::repl::get_input;
use crate::validator::*;
use chrono::{Date, DateTime, Duration, Local, NaiveTime, TimeZone, Timelike};
use std::fs;
use std::fs::File;
use std::path::Path;

pub fn parse_into_date(input: &String) -> Date<Local> {
    println!("{:?}", input);

    if input.trim().is_empty() {
        return Local::now().date();
    }

    let split_string: Vec<&str> = input.split('/').collect();

    let date = Local.ymd(
        split_string[2].parse().expect("A number was given as year"),
        split_string[1]
            .parse()
            .expect("A number was given as month"),
        split_string[0].parse().expect("A number was given as day"),
    );
    return date;
}

pub fn parse_into_time(input: &String) -> NaiveTime {
    println!("{:?}", input);

    if input.trim().is_empty() {
        return Local::now().time();
    }

    let split_string: Vec<&str> = input.split(':').collect();
    return NaiveTime::from_hms(
        split_string[0].parse().expect("A number was given as hour"),
        split_string[1]
            .parse()
            .expect("A number was given as minute"),
        0,
    );
}
/*
As of now, this only accepts input such as '3d', '40min' or '3h'
Eventually, support for a format like '1:20h' should be added.
*/
pub fn parse_into_duration(input: &String) -> Duration {
    println!("{:?}", input);
    let input_lower = &input.to_lowercase();
    
    match (input_lower.contains("d"), input_lower.contains("h"), input_lower.contains("m")){
	(true, false, false) => {
	    // Duration has to be 'days'
	    Duration::days(input_lower.split('d').collect::<Vec<&str>>()[0].parse().expect("Valid duration was given"))
	}
	(false, true, false) => {
	    // Duration has to be 'hours'
	    Duration::hours(input_lower.split('h').collect::<Vec<&str>>()[0].parse().expect("Valid duration was given"))
	}
	(false, false, true) => {
	    Duration::minutes(input_lower.split('m').collect::<Vec<&str>>()[0].parse().expect("Valid duration was given"))
	},
	(_,_,_) => panic!("Error parsing duration. This error should be unreachable")
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
            get_input()
        }
    };

    print!("Start date: ");
    let mut input = get_input();
    while !validate_date(&input) {
        println!("{} is not a valid date.", input);
        print!("Start date: ");
        input = get_input();
    }
    let start_date = parse_into_date(&input);

    print!("Start time: ");
    input = get_input();
    while !validate_time(&input) {
        println!("Entered time is not valid.");
        print!("Start time: ");
        input = get_input();
    }
    let start_time = parse_into_time(&input);

    print!("Duration: ");
    input = get_input();
    while !validate_duration(&input) {
        println!("Entered duration is not valid.");
        print!("Duration: ");
        input = get_input();
    }
    let duration = parse_into_duration(&input);

    print!("End date: ");
    input = get_input();
    while !validate_date(&input) {
        println!("Entered date is not valid.");
        print!("End date: ");
        input = get_input();
    }
    let end_date = parse_into_date(&input);
    

    print!("End time: ");
    input = get_input();
    while !validate_time(&input) {
        println!("Entered time is not valid.");
        print!("End time: ");
        input = get_input();
    }
    let end_time = parse_into_time(&input);

    print!("Difficulty: ");
    input = get_input();
    while !validate_difficulty(&input) {
        println!("Entered difficulty is not valid.");
        print!("Difficulty: ");
        input = get_input();
    }
    let difficulty = input.parse().unwrap();

    print!("Priority: ");
    input = get_input();
    while !validate_priority(&input) {
        println!("Entered priority is not valid.");
        print!("Priority: ");
        input = get_input();
    }
    let priority = input.parse().unwrap();

    println!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}",
        name, start_time, duration, end_date, end_time, priority, difficulty
    );

    Event {
        name,
        start: start_date.and_time(start_time).unwrap(),
        duration,
        end: end_date.and_time(end_time).unwrap(),
        priority,
        difficulty,
    }
}

/*
Given 'name' of a new calendar, the function gets the home directory,
verifies the existence of a $HOME/.calmar directory,
creates a JSON file with the given 'name' under $HOME/.calmar.
If file named 'name' already exists, it asks the user for confirmation.
*/
pub fn create_new_calendar(name: Option<String>) {
    let name = match name {
        Some(name) => name,
        None => {
            print!("Name: ");
            get_input()
        }
    };

    let mut calmar_dir = match home::home_dir() {
        Some(dir) => dir,
        None => {
            println!("Failed to acquire home dir");
            return ();
        }
    };
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
                return ();
            }
        },
    }

    match Path::new(&calmar_dir.join(&name).with_extension("json")).exists() {
        true => {
            print!("A calendar by that name already exists. Overwrite it with an empty file?\nThis will cause complete loss of data. [y/N]: ");
            match get_input().trim().to_lowercase().as_str() {
                "y" | "yes" => (),
                "n" | "no" => {
                    println!("Aborting...");
                    return ();
                }
                _ => {
                    println!("Invalid option, aborting...");
                    return ();
                }
            }
        }
        false => (),
    }

    match File::create(calmar_dir.join(&name).with_extension("json")) {
        Ok(_) => (),
        Err(err) => {
            println!("Failed to create file\n{}", err);
            return ();
        }
    }

    println!(
        "Successfully created a new calendar named {} in {}",
        name,
        calmar_dir.display()
    );
}

// fn yesno(text: &str) -> bool {
//     match text.to_lowercase().as_str() {
//         "yes" | "y" => true,
//         _ => false,
//     }
// }

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
            return ();
        }
    };
    println!("{:?}", new_event);
}

/*
Call calendar creation with name given optionally
*/
pub fn cal(split_input: &Vec<&str>) {
    match split_input.len() {
        1 => create_new_calendar(None),
        2 => create_new_calendar(Some(split_input[1].to_owned())),
        _ => {
            println!(
                "add: Too many arguments provided. Expected: 0 or 1, Got: {}",
                split_input.len() - 1
            ); // do not count "add" as an argument
            return ();
        }
    };
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
