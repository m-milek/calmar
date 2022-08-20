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

