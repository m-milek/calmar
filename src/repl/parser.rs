mod help;
use crate::event::Event;
use crate::repl::get_input;
use crate::validator::*;
use chrono::{Duration, Local, TimeZone};
use std::fs;
use std::fs::File;
use std::path::Path;

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
    let mut start_date = get_input();
    while !validate_date(&start_date) {
        println!("Entered date is not valid.");
        print!("Start date: ");
        start_date = get_input();
    }
    let split_start_date: Vec<&str> = start_date.split('/').collect();
    println!("{:?}", split_start_date);

    print!("Start time: ");
    let mut start_time = get_input();
    while !validate_time(&start_time) {
        println!("Entered time is not valid.");
        print!("Start time: ");
        start_time = get_input();
    }
    let split_start_time: Vec<&str> = start_time.split(':').collect();
    println!("{:?}", split_start_time);

    print!("Duration: ");
    let mut duration = get_input();
    while !validate_duration(&duration) {
        println!("Entered duration is not valid.");
        print!("Duration: ");
        duration = get_input();
    }

    print!("End date: ");
    let mut end_date = get_input();
    while !validate_date(&end_date) {
        println!("Entered date is not valid.");
        print!("End date: ");
        end_date = get_input();
    }
    let split_end_date: Vec<&str> = end_date.split('/').collect();
    println!("{:?}", split_end_date);

    print!("End time: ");
    let mut end_time = get_input();
    while !validate_time(&end_time) {
        println!("Entered time is not valid.");
        print!("End time: ");
        end_time = get_input();
    }
    let split_end_time: Vec<&str> = end_time.split(':').collect();
    println!("{:?}", split_end_time);

    print!("Difficulty: ");
    let mut difficulty = get_input();
    while !validate_difficulty(&difficulty) {
        println!("Entered difficulty is not valid.");
        print!("Difficulty: ");
        difficulty = get_input();
    }

    print!("Priority: ");
    let mut priority = get_input();
    while !validate_priority(&priority) {
        println!("Entered priority is not valid.");
        print!("Priority: ");
        priority = get_input();
    }

    println!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        name, start_date, start_time, duration, end_date, end_time, priority, difficulty
    );

    Event {
        name: name,
        start: Local
            .ymd(
                split_start_date[2].trim().parse().expect("Wanted a number"),
                split_start_date[1].trim().parse().expect("Wanted a number"),
                split_start_date[0].trim().parse().expect("Wanted a number"),
            )
            .and_hms(
                split_start_time[0].trim().parse().expect("Wanted a number"),
                split_start_time[1].trim().parse().expect("Wanted a number"),
                0,
            ),
        duration: Duration::hours(1),
        end: Local
            .ymd(
                split_end_date[2]
                    .trim()
                    .parse()
                    .expect("Wanted a number as year"),
                split_end_date[1]
                    .trim()
                    .parse()
                    .expect("Wanted a number as month"),
                split_end_date[0]
                    .trim()
                    .parse()
                    .expect("Wanted a number as day"),
            )
            .and_hms(
                split_end_time[0]
                    .trim()
                    .parse()
                    .expect("Wanted a number as hour"),
                split_end_time[1]
                    .trim()
                    .parse()
                    .expect("Wanted a number as minute"),
                0,
            ),
        priority: priority
            .trim()
            .parse()
            .expect("Wanted a number as priority"),
        difficulty: difficulty
            .trim()
            .parse()
            .expect("Wanted a number as difficulty"),
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
        "help" | "h" => help::print_help(split_input[1]),
        "remove" | "rm" | "r" => remove(&split_input),
        "removecal" | "rmcal" | "rc" => removecal(&split_input),
        "show" | "s" => show(&split_input),
        "quit" | "q" => std::process::exit(0),
        _ => println!("Unknown command: {}", split_input[0].trim()),
    }
}
