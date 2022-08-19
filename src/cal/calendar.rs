#![allow(dead_code)]

use crate::cli::messages::{warning, success};
use crate::{cal::calendar_index::CalendarIndex, cli::messages::error};
use crate::cal::calendar_ref::get_new_calendar_reference;
use crate::cal::event::EventJSON;
use crate::cal::getdata::get_valid_calendar_name;
use serde_derive::{Deserialize, Serialize};
use std::io::Write;
use crate::CONFIG;
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
    
    pub fn set_name(&mut self, name: String) {
	self.name = name;
    }

    pub fn save(&self, path: String) {
	let mut calendar_file = match std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&path)
	{
            Ok(file) => file,
            Err(e) => {
		error(format!("Failed to open {}.\n{}", path, e));
		std::process::exit(1);
            }
	};

	let calendar_json = match serde_json::to_string_pretty(&self) {
            Ok(result) => result,
            Err(e) => {
		error(format!("Failed to parse the calendar into string.\n{}", e));
		std::process::exit(1);
            }
	};

	match write!(calendar_file, "{}", calendar_json) {
            Ok(_) => (),
            Err(e) => {
		error(format!("Failed to write to {}.\n{}", path, e));
		std::process::exit(1);
            }
	}
    }
    pub fn add_event(&mut self, event: EventJSON) {
	self.events.push(event);
    }
}

pub enum CalendarReturnMessage {
    Abort,
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
            warning(format!("cal: Too many arguments provided. Expected: 0 or 1, Got: {}", split_input.len() - 1)); // do not count "cal" as an argument
            return;
        }
    };

    let mut calendar_index = CalendarIndex::get();
    if calendar_index.calendars.is_empty() {
        new_reference.active = true;
    }

    match calendar_index.add_entry(&new_reference) {
        Ok(_) => success("Added entry to calendar index.".to_string()),
        Err(_) => {
            error("Failed to add new calendar reference to calendar index.".to_string());
            return;
        }
    }
    calendar_index.save();
    success("Saved calendar index".to_string());
    new_reference.create_file()
}


/// Delete a calendar
pub fn removecal(split_input: &Vec<&str>) {
    let mut index = CalendarIndex::get();
    let name = match split_input.len() {
        1 => get_valid_calendar_name(),
        2 => split_input[1].to_string(),
        _ => {
            warning(format!("removecal: Too many arguments provided. Expected: 0 or 1. Got: {}", split_input.len() - 1));
            return;
        }
    };

    match index.delete_entry(name) {
        Ok(_) => (),
        Err(_) => return,
    }

    index.save();
    success("Successfully removed calendar".to_string());
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
            warning(format!("remove: Too many arguments provided. Expected: 1 or 2. Got: {}", split_input.len() - 1));
            return;
        }
    };
    let index = CalendarIndex::get();
    let mut active_calendar = index.active_calendar();
    active_calendar.events.retain(|event| event.name != name);
    active_calendar.save(index.active_calendar_reference().path);
}
