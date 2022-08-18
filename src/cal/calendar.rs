use crate::cal::calendar_index::CalendarIndex;
use crate::cal::calendar_ref::get_new_calendar_reference;
use crate::cal::event::EventJSON;
use crate::cal::getdata::get_valid_calendar_name;
use crate::cal::savedata::save_new_calendar;
use colored::Colorize;
use serde_derive::{Deserialize, Serialize};
use std::io::Write;
use crate::CONFIG;
use super::calendar_ref::get_active_calendar_reference;
use super::getdata::get_valid_event_name;

/// Holds its own name and a vector of `Event` structs.
/// # Use
/// An empty `Calendar` may be created with `Calendar::new("foo")`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Calendar {
    pub name: String,
    pub events: Vec<EventJSON>,
}

impl Calendar {
    /// Create an empty `Calendar` with a given `name`.
    pub fn new(name: &str) -> Self {
        Calendar {
            name: name.to_string(),
            events: Vec::<EventJSON>::new(),
        }
    }
    pub fn rename(&mut self, name: String) {
	self.name = name;
    }
}

pub enum CalendarReturnMessage {
    Abort,
}


pub fn save_calendar(calendar: Calendar, path: String) {
    let mut calendar_file = match std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(&path)
    {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to open {path}.\n{e}");
            std::process::exit(1);
        }
    };

    let calendar_json = match serde_json::to_string_pretty(&calendar) {
        Ok(result) => result,
        Err(e) => {
            println!("Failed to parse the calendar into string.\n{e}");
            std::process::exit(1);
        }
    };

    match write!(calendar_file, "{}", calendar_json) {
        Ok(_) => (),
        Err(e) => {
            println!("Failed to write to {path}.\n{e}");
            std::process::exit(1);
        }
    }
}


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

    let mut calendar_index = CalendarIndex::get();
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
    calendar_index.save();
    println!("{}", "Saved calendar index".green().bold());
    save_new_calendar(new_reference);
}


/// Delete a calendar
pub fn removecal(split_input: &Vec<&str>) {
    let mut index = CalendarIndex::get();
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

    index.save();
    println!("{}", "Successfully removed calendar".green().bold());
}

pub fn default_or_custom_save_path(input: String) -> String {
    if input.trim().is_empty() {
        return CONFIG.default_path.clone();
    }
    input
}

/// Delete an event from the active calendar
pub fn remove(split_input: &Vec<&str>) {
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
    let index = CalendarIndex::get();
    let mut active_calendar = index.get_active_calendar();
    active_calendar.events.retain(|event| event.name != name);
    save_calendar(active_calendar, get_active_calendar_reference().path);
}
