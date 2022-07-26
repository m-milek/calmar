use crate::calendar::{Calendar, CalendarIndex, CalendarReference};
use crate::event::Event;
use crate::validator::get_home_dir;
use std::fmt::write;
use std::fs::read_to_string;
use std::io::Write;

/*
Takes an Event argument, converts it to a EventJSON helper struct, serializes it and saves to the currently selected calendar
 */
pub fn save_event(event: Event, calendar_ref: CalendarReference) -> bool {
    let file_content = match read_to_string(&calendar_ref.path) {
        Ok(content) => content,
        Err(e) => {
            println!("Failed to read {}.\n{}", calendar_ref.path, e);
            return false;
        }
    };

    let mut calendar: Calendar = match serde_json::from_str(&file_content) {
        Ok(result) => result,
        Err(e) => {
            println!(
                "Failed to parse {}. Check for syntax errors.\n{}",
                calendar_ref.path, e
            );
            return false;
        }
    };

    calendar.events.push(event.to_event_json());

    let calendar_json: String = match serde_json::ser::to_string_pretty(&calendar) {
        Ok(result) => result,
        Err(e) => {
            println!("Failed to parse Event to String.\n{}", e);
            return false;
        }
    };

    let mut new_file = match std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&calendar_ref.path)
    {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to open {}.\n{}", calendar_ref.path, e);
            return false;
        }
    };

    match write!(new_file, "{}", calendar_json) {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to write to {}.\n{}", calendar_ref.path, e);
            return false;
        }
    };

    //println!("Written to {}", calendar_ref.path);
    return true;
}

pub fn save_calendar_index(calendar_index: CalendarIndex) {
    let home_dir = get_home_dir();
    let mut index_file = match std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(home_dir.join(".config/calmar/index.json"))
    {
        Ok(file) => file,
        Err(e) => {
            println!(
                "Failed to open {}.\n{}",
                home_dir.join(".config/calmar/index.json").display(),
                e
            );
            std::process::exit(1);
        }
    };
    let calendar_index_json: String = match serde_json::ser::to_string_pretty(&calendar_index) {
        Ok(result) => result,
        Err(e) => {
            println!("Failed to serialize calendar index to string.\n{}", e);
            std::process::exit(1);
        }
    };

    match write!(index_file, "{}", calendar_index_json) {
        Ok(_) => (),
        Err(e) => {
            println!(
                "Failed to write to {}.\n{}",
                home_dir.join(".config/calmar/index.json").display(),
                e
            );
        }
    }
}

pub fn save_new_calendar(calendar_reference: CalendarReference) {
    let mut calendar_file = match std::fs::File::create(&calendar_reference.path) {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to create {}.\n{}", calendar_reference.path, e);
            return;
        }
    };

    let calendar_json: String =
        match serde_json::to_string_pretty(&Calendar::new(&calendar_reference.name)) {
            Ok(result) => result,
            Err(e) => {
                println!("Failed to serialize calendar to string.\n{}", e);
                return;
            }
        };

    match write!(calendar_file, "{}", calendar_json) {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to write to {}.\n{}", calendar_reference.name, e);
        }
    }
    println!("Written to {}.", calendar_reference.path);
}
