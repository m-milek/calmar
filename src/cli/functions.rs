use crate::{cal::{event::EventJSON, calendar_ref::CalendarReference}, cli::{repl::get_input, getdata::get_dir_path, messages::{error, print_err_msg}}};
use chrono::{DateTime, Local};
use std::str::FromStr;
use std::collections::HashMap;
use crate::{
    cal::{calendar_index::CalendarIndex, event::Event},
    cli::messages::warning,
    cli::util::{select_in_range, uppercase_first_letter},
};
use crate::cli::getdata::{
            get_difficulty, get_duration, get_end_date, get_end_time, get_priority, get_start_date,
    get_start_time, get_valid_event_name};
use crate::cli::commands::default_or_custom_save_path;
use std::path::PathBuf;


/// Create a new event and return it.
pub fn get_new_event(name: Option<String>) -> Event {
    let name = match name {
        Some(name) => name,
        None => {
            print!("Name: ");
            get_valid_event_name()
        }
    };

    print!("Start date: ");
    let start_date = get_start_date();

    print!("Start time: ");
    let start_time = get_start_time();

    print!("Duration: ");
    let duration = get_duration();

    let end_date;
    let end_time;
    if duration.is_zero() {
        print!("End date: ");
        end_date = get_end_date(&start_date);
        print!("End time: ");
        end_time = get_end_time(&start_date, &start_time, &end_date);
    } else {
        let end_timedate = start_date.and_time(start_time).unwrap() + duration;
        end_date = end_timedate.date();
        end_time = end_timedate.time();
    }

    print!("Difficulty: ");
    let difficulty = get_difficulty();

    print!("Priority: ");
    let priority = get_priority();

    Event {
        name,
        start: start_date.and_time(start_time).unwrap(),
        end: end_date.and_time(end_time).unwrap(),
        priority,
        difficulty,
    }
}

pub fn edit_event(event_name: &str) {
    let index = CalendarIndex::get();
    let mut active_calendar = index.active_calendar();

    let mut index_map = HashMap::<usize, usize>::with_capacity(active_calendar.events.len());

    let mut i = 0;
    for (num, event) in active_calendar.events.iter().enumerate() {
        if event.name == event_name {
            index_map.insert(i, num);
            i += 1;
        }
    }

    // Choose an event to be edited
    let events_named_like_arg: Vec<EventJSON> = active_calendar
        .events
        .clone()
        .into_iter()
        .filter(|event| event.name == event_name)
        .collect();
    if events_named_like_arg.is_empty() {
        warning(format!("No event named {} found.", event_name));
        return;
    }
    println!("{:#?}", events_named_like_arg);
    let index_to_select = match events_named_like_arg.len() {
        1 => 0,
        _ => select_in_range("Select an event to edit", events_named_like_arg.len()) - 1,
    };

    // Choose a property to be edited
    let fields = EventJSON::FIELD_NAMES_AS_ARRAY.to_vec();
    let mut fields_list: Vec<String> = fields.into_iter().map(uppercase_first_letter).collect();
    fields_list.insert(2, "Duration".to_string());

    fields_list
        .iter()
        .enumerate()
        .for_each(|(i, field)| println!("{}. {field}", i + 1));

    let edited_event = &mut active_calendar.events[index_map[&index_to_select]];
    let num: usize = select_in_range("Select what to edit", fields_list.len());

    match num {
        // Edit name
        1 => {
            print!("Name: ");
            edited_event.name = get_valid_event_name();
        }
        // Edit start timedate
        2 => {
            println!("1. Start date\n2. Start time\n3. Start datetime");
            let num = select_in_range("Select what to edit", 3);
            let current_end = edited_event.parsed_end().unwrap();
            let current_start = edited_event.parsed_start().unwrap();

            if num == 1 || num == 3 {
                print!("Start date: ");
                let mut new_start_date = get_start_date();
                while new_start_date.and_time(current_start.time()).unwrap() > current_end {
                    println!("Start timedate cannot be after end timedate");
                    print!("Start date: ");
                    new_start_date = get_start_date();
                }
                edited_event.start = new_start_date
                    .and_time(current_start.time())
                    .unwrap()
                    .to_string();
            }
            if num == 2 || num == 3 {
                print!("Start time: ");
                let mut new_start_time = get_start_time();
                while current_start.date().and_time(new_start_time).unwrap() > current_end {
                    println!("Start timedate cannot be after end timedate");
                    print!("Start date: ");
                    new_start_time = get_start_time();
                }
                edited_event.start = current_start
                    .date()
                    .and_time(new_start_time)
                    .unwrap()
                    .to_string();
            }
        }
        // Edit duration
        3 => {
            print!("Duration: ");
            let new_duration = get_duration();
            let start =
                DateTime::<Local>::from_str(&edited_event.start).expect("Valid start timedate");
            let end = start + new_duration;
            edited_event.end = end.to_string();
        }
        // Edit end datetime
        4 => {
            println!("1. End date\n2. End time\n3. End datetime");
            let num: usize = select_in_range("Select what to edit", 3);
            let mut current_end = edited_event.parsed_end().unwrap();
            let current_start = edited_event.parsed_start().unwrap();

            if num == 1 || num == 3 {
                print!("End date: ");
                let mut new_end_date = get_end_date(&current_start.date());
                while new_end_date.and_time(current_end.time()).unwrap() < current_start {
                    println!("End timedate cannot be before start timedate");
                    print!("End date: ");
                    new_end_date = get_end_date(&current_start.date());
                }
                edited_event.end = new_end_date
                    .and_time(current_end.time())
                    .unwrap()
                    .to_string();
            }
            if num == 2 || num == 3 {
                current_end = edited_event.parsed_end().unwrap();
                print!("End time: ");
                let mut new_end_time = get_end_time(
                    &current_start.date(),
                    &current_start.time(),
                    &current_end.date(),
                );
                while current_end.date().and_time(new_end_time).unwrap() < current_start {
                    println!("End timedate cannot be before start timedate");
                    print!("End date: ");
                    new_end_time = get_end_time(
                        &current_start.date(),
                        &current_start.time(),
                        &edited_event.parsed_end().unwrap().date(),
                    );
                }
                edited_event.end = current_end
                    .date()
                    .and_time(new_end_time)
                    .unwrap()
                    .to_string();
            }
        }
        // Edit priority
        5 => {
            print!("Priority: ");
            edited_event.priority = get_priority()
        }
        // Edit difficulty
        6 => {
            print!("Difficulty: ");
            edited_event.difficulty = get_difficulty()
        }
        _ => panic!("Impossible"),
    }
    
    let path = index.active_calendar_reference().path;
    if let Err(e) = active_calendar.save(&path) {
	print_err_msg(e, &path)
    }
}

/// Create a calendar reference and return it.
pub fn get_new_calendar_reference(name: Option<String>) -> CalendarReference {
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
            error(format!(
                "Failed to convert {} to string.",
                path_to_calendar.display()
            ));
            std::process::exit(1);
        }
    };
    CalendarReference::new(name, path_to_calendar_string.to_owned(), false)
}
