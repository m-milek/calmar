mod getdata;
mod help;
mod savedata;
use self::savedata::{save_calendar_index, save_new_calendar};
use crate::calendar::{
    check_if_calendar_exists, get_active_calendar, get_active_calendar_reference,
    get_calendar_index, CalendarReference,
};
use crate::event::{self, save_calendar, Event, EventJSON};
use crate::repl::get_input;
use crate::validator::is_numeric;
use crate::CONFIG;
use chrono::{Date, Duration, Local, NaiveTime, TimeZone, Timelike};
use colored::Colorize;
use getdata::*;
use savedata::save_event;
use std::collections::HashMap;
use std::path::PathBuf;

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

/// Create a new event and return it.
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

pub fn default_or_custom_save_path(input: String) -> String {
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
/// Create a calendar reference and return it.
fn get_new_calendar_reference(name: Option<String>) -> CalendarReference {
    let name = match name {
        Some(name) => name,
        None => {
            print!("Name: ");
            get_input()
        }
    };

    print!("Path: ");
    let path = default_or_custom_save_path(get_dir_path());

    let mut path_to_calendar = PathBuf::from(path).join(&name);
    path_to_calendar.set_extension("json");
    let path_to_calendar_string = match path_to_calendar.to_str() {
        Some(string) => string,
        None => {
            println!(
                "{}",
                format!(
                    "Failed to convert {} to string.",
                    path_to_calendar.display()
                )
                .red()
                .bold()
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
/// Create a new event and save it to the active calednar.
fn add(split_input: &Vec<&str>) {
    let new_event: Event = match split_input.len() {
        1 => get_new_event(None),
        2 => get_new_event(Some(split_input[1].to_owned())),
        _ => {
            println!(
                "{}",
                format!(
                    "add: Too many arguments provided. Expected: 0 or 1, Got: {}",
                    split_input.len() - 1
                )
                .yellow()
                .bold()
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

/// Create a new calendar and save it to the calendar index.
fn cal(split_input: &Vec<&str>) {
    let mut new_reference = match split_input.len() {
        1 => get_new_calendar_reference(None),
        2 => get_new_calendar_reference(Some(split_input[1].to_owned())),
        _ => {
            println!(
                "{}",
                format!(
                    "cal: Too many arguments provided. Expected: 0 or 1, Got: {}",
                    split_input.len() - 1
                )
                .yellow()
                .bold()
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
            println!(
                "{}",
                "Failed to add new calendar reference to calendar index."
                    .red()
                    .bold()
            );
            return;
        }
    }
    save_calendar_index(calendar_index);
    println!("{}", "Saved calendar index".green().bold());
    save_new_calendar(new_reference);
}

fn get_valid_event_name() -> String {
    let mut input = get_input();
    while input.is_empty() {
        println!("{}", "Event name cannot be an empty string".yellow().bold());
        print!("Name: ");
        input = get_input();
    }
    input
}

/// Delete an event from the active calendar
fn remove(split_input: &Vec<&str>) {
    let name = match split_input.len() {
        1 => get_valid_event_name(),
        2 => split_input[1].to_owned(),
        _ => {
            println!(
                "remove: Too many arguments provided. Expected: 1 or 2. Got: {}",
                split_input.len() - 1
            );
            return;
        }
    };
    let mut active_calendar = get_active_calendar();
    active_calendar.events.retain(|event| event.name != name);
    save_calendar(active_calendar, get_active_calendar_reference().path);
}

fn get_event_index(events: &Vec<EventJSON>) -> usize {
    let displayed_range = if events.len() == 1 {
        1.to_string()
    } else {
        1.to_string() + "-" + &events.len().to_string()
    };
    println!("{:#?}", events);
    let num: usize = {
        print!("Which event do you want to edit? [{}]: ", displayed_range);
        let mut num_str = get_input();
        while !is_numeric(&num_str) {
            println!(
                "{}",
                "Invalid input. Enter a non-negative number".yellow().bold()
            );
            print!("Which event do you want to edit?: ");
            num_str = get_input();
        }

        let mut number = num_str.parse::<usize>().unwrap();
        while !(1..=events.len()).contains(&number) {
            println!(
                "{}",
                format!("Number not in range. Allowed values: {}", displayed_range)
                    .yellow()
                    .bold()
            );
            print!("Which event do you want to edit?: ");
            num_str = get_input();
            number = num_str.parse::<usize>().unwrap();
        }
        number
    };
    num
}

fn uppercase_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

fn select_in_range(prompt: &str, max: usize) -> usize {

    let displayed_range = match max {
	1 => 1.to_string(),
	_ => 1.to_string() + "-" + max.to_string().as_str()
    };

    loop {
	print!("{} [{}]: ", prompt, displayed_range);
	match get_input().parse::<usize>() {
	    Ok(num) => {
		match (1..=max).contains(&num) {
		    true => {
			return num;
		    },
		    false => println!("{}", "Number not in range".yellow().bold())
		}
	    },
	    Err(_) => {
		println!("{}", "Invalid input. Enter a non-negative number".yellow().bold());
	    }
	}
    }
}

fn edit_event(event_name: &str) {
    let mut active_calendar = get_active_calendar();

    let mut index_map = HashMap::<usize, usize>::with_capacity(active_calendar.events.len());
    let mut i=0;
    for (num, event) in active_calendar.events.iter().enumerate(){
	if event.name == event_name {
	    index_map.insert(i, num);
	    i+=1;
	}
    }
    
    // Choose an event to be edited
    let events_named_like_arg: Vec<event::EventJSON> = active_calendar
        .events
        .clone()
        .into_iter()
        .filter(|event| event.name == event_name)
        .collect();
    if events_named_like_arg.is_empty() {
        println!(
            "{}",
            format!("No event named {} found.", event_name)
                .yellow()
                .bold()
        );
        return;
    }
    let index_to_select = match events_named_like_arg.len() {
        1 => 0,
        _ => get_event_index(&events_named_like_arg) - 1,
    };

    // Choose a property to be edited
    let fields = EventJSON::FIELD_NAMES_AS_ARRAY.to_vec();
    let mut fields_list: Vec<String> = fields.into_iter().map(|s| uppercase_first_letter(s)).collect();
    fields_list.insert(2, "Duration".to_string());
    
    let mut i: u8 = 1;
    for field in &fields_list {
	println!("{i}. {field}");
	i+=1;
    }
    
    let edited_event = &mut active_calendar.events[index_map[&index_to_select]];
    let num: usize = select_in_range("Select what to edit", fields_list.len());

    match num {
	// Edit name
	1 => {
	    print!("Name: ");
	    edited_event.name = get_valid_event_name();
	},
	// Edit start timedate
	2 => {
	    println!("1. Start date\n2. Start time\n3. Start datetime");
	    let num = select_in_range("Select what to edit", 3);
	    match num {
		1 => println!("Edit Start date"),
		2 => println!("Edit Start time"),
		3 => println!("Edit Start datetime"),
		_ => panic!("Impossible")
	    }
	    
	},
	// Edit duration
	3 => todo!("Edit duration"),
	// Edit end datetime
	4 => {
	    println!("1. End date\n2. End time\n3. End datetime");
	    let num: usize = select_in_range("Select what to edit", 3);
	    println!("{num}");
	},
	// Edit priority 
	5 => {
	    print!("Priority: ");
	    edited_event.priority = get_priority().parse().unwrap();
	},
	// Edit difficulty 
	6 => {
	    print!("Difficulty: ");
	    edited_event.difficulty = get_difficulty().parse().unwrap();
	},
	_ => panic!("Impossible")
    }
    save_calendar(active_calendar, get_active_calendar_reference().path);
}

/*
Edit attributes of a given event and save it
*/
fn edit(split_input: &Vec<&str>) {
    for event_name in split_input[1..].iter() {
        edit_event(event_name);
    }
}

/// Delete a calendar
fn removecal(split_input: &Vec<&str>) {
    let mut index = get_calendar_index();
    let name = match split_input.len() {
        1 => get_valid_calendar_name(),
        2 => split_input[1].to_string(),
        _ => {
            println!(
                "{}",
                format!(
                    "removecal: Too many arguments provided. Expected: 0 or 1. Got: {}",
                    split_input.len() - 1
                )
                .yellow()
                .bold()
            );
            return;
        }
    };

    match index.delete_entry(name) {
        Ok(_) => (),
        Err(_) => return,
    }

    save_calendar_index(index);
    println!("{}", "Successfully removed calendar".green().bold());
}

/// Display events in the active calendar
fn list(split_input: &Vec<&str>) {
    let active_calendar = get_active_calendar();
    for event in active_calendar.events {
        println!("{:#?}\n", event.to_standard_event());
    }
}

/// Change the active calednar
fn set(split_input: &Vec<&str>) {
    let name = match split_input.len() {
        1 => get_valid_event_name(),
        2 => split_input[1].to_string(),
        _ => {
            println!(
                "{}",
                format!(
                    "set: Too many arguments provided. Expected: 1 or 2. Got: {}",
                    split_input.len()
                )
                .yellow()
                .bold()
            );
            return;
        }
    };

    if !check_if_calendar_exists(&name) {
        return;
    }

    match get_number_of_active_calendars() {
        0 => {
            println!(
                "{}",
                "No calendars are set as active. Please correct this and retry."
                    .yellow()
                    .bold()
            );
            return;
        }
        1 => (),
        _ => {
            println!(
                "{}",
                "More than one calendar is set as active. Please correct this and retry."
                    .yellow()
                    .bold()
            );
            return;
        }
    }

    let mut index = get_calendar_index();
    // Set the currently active calendar as not active
    // Set the desired calendar as active
    for calendar in &mut index.calendars {
        if calendar.active {
            calendar.active = false;
        }
        if calendar.name == name {
            calendar.active = true;
        }
    }
    save_calendar_index(index)
}

fn clear(split_input: &Vec<&str>) {
    match split_input.len() {
        1 => {
            println!("\u{001b}c");
        }
        _ => {
            println!(
                "{}",
                format!(
                    "clear: Invalid number of arguments. Expected: 0. Got: {}",
                    split_input.len() - 1
                )
                .yellow()
                .bold()
            );
        }
    }
}

/// Handle input and call appropriate functions.
pub fn parse(input: String) {
    let split_input: Vec<&str> = input.split_whitespace().collect();
    match split_input[0].trim() {
        "add" | "a" => add(&split_input),
        "cal" | "c" => cal(&split_input),
        "clear" => clear(&split_input),
        "edit" | "e" => edit(&split_input),
        "help" | "h" => help::print_help(split_input[0]),
        "remove" | "rm" | "r" => remove(&split_input),
        "removecal" | "rmcal" | "rc" => removecal(&split_input),
        "set" | "s" => set(&split_input),
        "list" | "l" | "ls" => list(&split_input),
        "quit" | "q" => std::process::exit(0),
        _ => println!(
            "{}",
            format!("Unknown command: {}", split_input[0].trim())
                .yellow()
                .bold()
        ),
    }
}
