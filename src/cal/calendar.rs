#![allow(dead_code)]
use crate::cal::event::EventJSON;
use serde_derive::{Deserialize, Serialize};
use std::io::Write;
use super::calmar_error::CalmarError;

pub enum CalendarReturnMessage {
    Abort,
}

/// Holds its own name and a vector of `Event` structs.
/// # Use
/// An empty `Calendar` may be created with `Calendar::new("foo")`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Calendar {
    name: String,
    events: Vec<EventJSON>,
}

impl Calendar {
    /// Create an empty `Calendar` with a given `name`.
    pub fn new(name: &str) -> Self {
        Calendar {
            name: name.to_string(),
            events: Vec::<EventJSON>::new(),
        }
    }

    // Getters
    pub fn name(&self) -> &String {
	&self.name
    }

    pub fn events(&self) -> &Vec<EventJSON> {
	&self.events
    }
    pub fn events_mut(&mut self) -> &mut Vec<EventJSON> {
	&mut self.events
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name
    }

    pub fn save(&self, path: &String) -> Result<(), CalmarError> {
        let mut calendar_file = match std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&path)
        {
            Ok(file) => file,
            Err(e) => return Err(CalmarError::ReadFile { e }),
        };

        let calendar_json = match serde_json::to_string_pretty(&self) {
            Ok(result) => result,
            Err(e) => return Err(CalmarError::ParseJSON { e }),
        };

        if let Err(e) = write!(calendar_file, "{}", calendar_json) {
	    return Err(CalmarError::WriteFile { e })
        }
	Ok(())
    }

    pub fn add_event(&mut self, event: EventJSON) {
        self.events.push(event)
    }

    pub fn set_events(&mut self, events: Vec<EventJSON>) {
	self.events = events;
    }
}

