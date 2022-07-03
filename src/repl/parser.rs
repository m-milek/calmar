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
pub fn get_new_event() -> Event {
    println!("Getting new event data...");

    print!("Name: ");
    let name = get_input();

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
    while !validate_date(&start_time) {
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
    while !validate_date(&end_time) {
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
pub fn new_calendar(name: String) {
    //TODO: check if the calendar already exists and ask user for a confirmation or refusal

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

    match File::create(calmar_dir.join(name).with_extension("json")) {
        Ok(_) => (),
        Err(err) => {
            println!("Failed to create file\n{}", err);
            return ();
        }
    }

    println!(
        "Successfully created a new calendar at {}",
        calmar_dir.display()
    );
}

/*
Accepts and splits input into an array of string slices.
Then the function matches words in the input in order to call appropriate functions.
*/
fn yesno(text: &str) -> bool {
    match text.to_lowercase().as_str() {
        "yes"|"y" => true,
	_ => false
    }
}
fn new(args: &Vec<&str>) {
    let questions = ["Type [event/calendar]: ", "Name: "];
    let mut entry_type: String;
    let mut name: String;
    let mut i = 0; // Determines the currently processed property
    for arg in args.iter(){
        match i {
	0 => (),
	1 => {
        match arg.to_lowercase().as_str() {
	    "" => {
	        i-=1;
		break;
	    },
            "event"|"calendar" => {
	        entry_type = arg.to_string();
            }
            _ => {
                println!("Incorrect type of new entry: {}", arg);
                println!("Do you want to try again in interactive mode? [y/n]");
		let mut input;
		input = get_input();
		if !yesno(&input) {
                    return ()
		}
                i -=1;
		break;
            }
        }},
	2 => {
            match arg.to_lowercase().as_str() {
	    "" => {
	        i-=1;
		break;
	    },
            _ => name = arg.to_string()
	    }
	},
	_ => println!("Received too many arguments!")
	}
        i += 1;
    }
    let mut input;
    while i < questions.len() {
        print!("{}", questions[i]);
	input = get_input();
	println!("Now we should process the input");
        i += 1;
    }
    // To use: new_calendar(split_input[2].to_owned());
}
pub fn parse(input: String) {
    let mut split_input: Vec<&str> = input.split_whitespace().collect();
    split_input.push("");
    match split_input[0].to_lowercase().as_str() {
        "new"|"ne"|"n" => new(&split_input),
        "remove"|"rem"|"re"|"r" => todo!(),
        "show"|"sho"|"sh"|"s" => todo!(),
        "exit"|"ex"|"e" => std::process::exit(0),
        "help"|"hel"|"he"|"h" => help::print_help(split_input[1]),
        _ => println!("Unknown command"),
    }
}
