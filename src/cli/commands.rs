use super::{functions::generate_until, getdata::parse_into_duration, util::round_to_full_day};
use crate::{
    active_calendar, active_calendar_reference,
    cal::event::Event,
    calendar_index,
    cli::{
        functions::{duration_fmt, edit_event, get_new_calendar_reference, get_new_event},
        getdata::{get_valid_calendar_name, get_valid_event_name},
        messages::{error, print_err_msg, success, warning},
    },
    CONFIG,
};
use chrono::{Duration, Local};
use std::{io::Write, ops::Neg};

/*
Given 'name' of a new calendar, the function gets the home directory,
verifies the existence of a $HOME/.calmar directory,
creates a JSON file with the given 'name' under $HOME/.calmar.
If file named 'name' already exists, it asks the user for confirmation.
 */
/// Create a new calendar and save it to the calendar index.
pub fn cal(split_input: &Vec<&str>) {
    let mut new_reference = match split_input.len() {
        1 => get_new_calendar_reference(None),
        2 => get_new_calendar_reference(Some(split_input[1].to_owned())),
        _ => {
            warning(format!(
                "cal: Too many arguments provided. Expected: 0 or 1, Got: {}",
                split_input.len() - 1
            )); // do not count "cal" as an argument
            return;
        }
    };

    let mut index = calendar_index!();

    if index.calendars().is_empty() {
        new_reference.set_active()
    }

    match index.add_entry(&new_reference) {
        Ok(_) => success("Added entry to calendar index.".to_string()),
        Err(_) => {
            error("Failed to add new calendar reference to calendar index.".to_string());
            return;
        }
    }
    if let Err(e) = index.save() {
        print_err_msg(e, &CONFIG.index_path);
        return;
    };
    success("Saved calendar index".to_string());
    if let Err(e) = new_reference.create_file() {
        print_err_msg(e, new_reference.path())
    }
}

/// Delete a calendar
pub fn removecal(split_input: &Vec<&str>) {
    let mut index = calendar_index!();

    let name = match split_input.len() {
        1 => get_valid_calendar_name(),
        2 => split_input[1].to_string(),
        _ => {
            warning(format!(
                "removecal: Too many arguments provided. Expected: 0 or 1. Got: {}",
                split_input.len() - 1
            ));
            return;
        }
    };

    match index.delete_entry(name) {
        Ok(_) => (),
        Err(_) => return,
    }

    if let Err(e) = index.save() {
        print_err_msg(e, &CONFIG.index_path);
        return;
    };
    success("Successfully removed calendar".to_string());
}

/// Delete an event from the active calendar
pub fn remove(split_input: &Vec<&str>) {
    let name = match split_input.len() {
        1 => get_valid_event_name(),
        2 => split_input[1].to_owned(),
        _ => {
            warning(format!(
                "remove: Too many arguments provided. Expected: 1 or 2. Got: {}",
                split_input.len() - 1
            ));
            return;
        }
    };

    let mut active_calendar = active_calendar!();
    let path = active_calendar_reference!().path().clone();

    active_calendar
        .events_mut()
        .retain(|event| event.name().ne(&name));

    if let Err(e) = active_calendar.save(&path) {
        print_err_msg(e, &path)
    }
}

/// Change the active calednar
pub fn set(split_input: &Vec<&str>) {
    let mut index = calendar_index!();
    let name = match split_input.len() {
        1 => {
            print!("Name: ");
            get_valid_event_name()
        }
        2 => split_input[1].to_string(),
        _ => {
            warning(format!(
                "set: Too many arguments provided. Expected: 1 or 2. Got: {}",
                split_input.len() - 1
            ));
            return;
        }
    };

    if !index.contains_one_named(&name) {
        return;
    }

    match index.number_of_active_calendars() {
        0 | 1 => {
            index.set_active(name);
            if let Err(e) = index.save() {
                print_err_msg(e, &CONFIG.index_path);
            };
        }
        _ => {
            warning(
                "More than one calendar is set as active. Please correct this and retry."
                    .to_string(),
            );
        }
    }
}

/*
Call event creation with name given optionally
 */
/// Create a new event and save it to the active calednar.
pub fn add(split_input: &Vec<&str>) {
    let new_event: Event = match split_input.len() {
        1 => get_new_event(None),
        2 => get_new_event(Some(split_input[1].to_owned())),
        _ => {
            warning(format!(
                "add: Too many arguments provided. Expected: 0 or 1, Got: {}",
                split_input.len() - 1
            ));
            // do not count "add" as an argument
            return;
        }
    };

    let path = active_calendar_reference!().path().clone();
    let mut active_calendar = active_calendar!();
    active_calendar.add_event(new_event);
    if let Err(e) = active_calendar.save(&path) {
        print_err_msg(e, &path)
    }
}

/*
Edit attributes of a given event and save it
*/
pub fn edit(split_input: &[&str]) {
    split_input[1..].iter().for_each(|e| edit_event(e))
}

/// Display events in the active calendar
pub fn raw(split_input: &[&str]) {
    active_calendar!()
        .events()
        .iter()
        .filter(|e| {
            if split_input.len().ne(&1) {
                split_input[1..].contains(&e.name().as_str())
            } else {
                true
            }
        })
        .for_each(|e| println!("{:#?}", e))
}

/// Clear the screen
pub fn clear(split_input: &Vec<&str>) {
    match split_input.len() {
        1 => {
            println!("\u{001b}c");
        }
        _ => {
            warning(format!(
                "clear: Invalid number of arguments. Expected: 0. Got: {}",
                split_input.len() - 1
            ));
        }
    }
}

// List calendars and their properties
pub fn listcal(split_input: &Vec<&str>) {
    calendar_index!()
        .calendars()
        .iter()
        .filter(|r| {
            if split_input.len().ne(&1) {
                split_input[1..].contains(&r.name().as_str())
            } else {
                true
            }
        })
        .for_each(|r| println!("{:#?}", r))
}

pub fn sort(split_input: &Vec<&str>) {
    let index = calendar_index!();
    let mut active_calendar = active_calendar!(index);
    let active_calendar_reference = active_calendar_reference!(index);

    if !(1..=3).contains(&split_input.len()) {
        warning(format!(
            "sort: Invalid number of arguments. Expected: 0 or 1. Got: {}",
            split_input.len() - 1
        ));
        return;
    }

    let mut events_std: Vec<Event> = active_calendar.events().to_vec();

    match split_input.len() {
        1 => {
            events_std.sort();
            println!("Sorted normally");
        }
        _ => match split_input[1].trim() {
            "name" => events_std.sort_by_key(|e| e.name()),
            "start" => events_std.sort_by_key(|e| e.start()),
            "end" => events_std.sort_by_key(|e| e.end()),
            "priority" => events_std.sort_by_key(|e| e.priority()),
            "difficulty" => events_std.sort_by_key(|e| e.difficulty()),
            _ => {
                warning(format!("sort: {} is not a valid key.", {
                    split_input[1].trim()
                }));
                return;
            }
        },
    }

    match split_input.get(2) {
        Some(arg) => match arg.trim() {
            "ascending" | "asc" | "a" => {}
            "descending" | "desc" | "d" | "rev" | "reverse" => events_std.reverse(),
            _ => {
                warning(format!(
                    "sort: {} is not a valid ordering argument",
                    split_input[2]
                ));
                return;
            }
        },
        None => {}
    }

    active_calendar.set_events(events_std);
    if let Err(e) = active_calendar.save(active_calendar_reference.path()) {
        print_err_msg(e, active_calendar_reference.path());
    }
}

pub fn duration(split_input: &Vec<&str>) {
    let active_calendar = active_calendar!();

    let name_arr = match split_input.len() {
        1 => {
            print!("Name: ");
            vec![get_valid_event_name()]
        }
        _ => split_input[1..].iter().map(|a| a.to_string()).collect(),
    };

    active_calendar.events().iter().for_each(|e| {
        if name_arr.contains(&e.name()) {
            println!("Duration of {}: {}", e.name(), duration_fmt(e.duration()))
        }
    })
}

pub fn until(split_input: &Vec<&str>) {
    let active_calendar = active_calendar!();

    let name_arr = match split_input.len() {
        1 => {
            print!("Name: ");
            vec![get_valid_event_name()]
        }
        _ => split_input[1..].iter().map(|a| a.to_string()).collect(),
    };

    active_calendar.events().iter().for_each(|e| {
        if name_arr.contains(&e.name()) {
            let now = Local::now();
            if now < e.start() {
                println!("Until {}: {}", e.name(), duration_fmt(e.start() - now))
            } else {
                println!(
                    "{} started {} ago",
                    e.name(),
                    duration_fmt((e.start() - now).neg())
                )
            }
        }
    });

    for name in &name_arr {
        if !active_calendar
            .events()
            .iter()
            .map(|e| e.name())
            .any(|x| x == *name)
        {
            warning(format!("until: No event named {}", name))
        }
    }
}

/// Generate and view
pub fn list(split_input: &Vec<&str>) {
    let mut span = parse_into_duration(&CONFIG.default_calendar_span);

    let active_calendar = active_calendar!();

    match split_input.len() {
        1 => {}
        2 => {
            span = parse_into_duration(split_input[1]);
        }
        _ => warning(format!(
            "list: Invalid number of arguments. Expected: 0 or 1. Got: {}",
            split_input.len() - 1
        )),
    }

    
    let re_days = regex::Regex::new(
	"^[0-9]+(d| +d|days| +days)$"
    ).unwrap();
    
    // if the user typed something like '3d', round the duration
    // to full days for convenience
    let mut end_date = Local::now() + span;
    if split_input.len() == 2 && re_days.is_match(split_input[1]) {
	end_date = round_to_full_day(end_date);
    }
    
    generate_until(active_calendar, end_date)
        .iter()
        .for_each(|e| println!("{e}"))
}

/// Generate, output to a file
pub fn write(split_input: &Vec<&str>) {
    // write filename - default span
    // write 10h filename

    let filename: String;
    let span: Duration;

    match split_input.len() {
        2 => {
            span = parse_into_duration(&CONFIG.default_calendar_span);
            filename = split_input[1].to_string();
        }
        3 => {
            span = parse_into_duration(split_input[1]);
            filename = split_input[2].to_string();
        }
        _ => {
            warning(format!(
                "write: Invalid number of arguments. Expected: 1 or 2. Got: {}",
                split_input.len() - 1
            ));
            return;
        }
    }

    let current_dir = match std::env::current_dir() {
        Ok(d) => d,
        Err(e) => {
            error(format!("Failed to get current directory.\n{e}"));
            return;
        }
    };

    let mut file = match std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(true)
        .open(current_dir.join(&filename))
    {
        Ok(f) => f,
        Err(e) => {
            error(format!(
                "Failed to create file {}.\n{e}",
                current_dir.join(filename).display()
            ));
            return;
        }
    };

    let re_days = regex::Regex::new(
	"^[0-9]+(d| +d|days| +days)$"
    ).unwrap();

    // if the user typed something like '3d', round the duration
    // to full days for convenience
    let mut end_date = Local::now() + span;
    if re_days.is_match(split_input[1]) {
	end_date = round_to_full_day(end_date);
    }

    let calendar = active_calendar!();
    
    let gen_events = generate_until(calendar, end_date);

    gen_events.iter().for_each(|e| {
        if let Err(e) = writeln!(&mut file, "{e}") {
            error(format!("Failed to write to file {filename}.\n{e}"));
        }
    })
}
