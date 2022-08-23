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
        self.name = name
    }

    pub fn save(&self, path: &String) -> Result<(), CalmarError> {
        let mut calendar_file = match std::fs::OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(&path)
        {
            Ok(file) => file,
            Err(e) => return Err(CalmarError::OpenFile { e }),
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
}

