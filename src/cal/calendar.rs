use crate::cal::calmar_trait::CalendarDataType;
use crate::cal::{calmar_error::CalmarError, event::Event};
use core::fmt;
use serde_derive::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    io::Write,
};

use super::deadline::Deadline;

/// Holds its own name and a vector of `Event` structs.
/// # Use
/// An empty `Calendar` may be created with `Calendar::new("foo")`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Calendar {
    name: String,
    events: Vec<Event>,
    deadlines: Vec<Deadline>,
}

impl Display for Calendar {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Caledndar name: {} | Events: {:?} | Number of events: {}",
            self.name,
            self.events
                .iter()
                .map(|e| e.name())
                .collect::<Vec<String>>(),
            self.events.len()
        )
    }
}

impl Calendar {
    pub fn new(name: &str) -> Self {
        Calendar {
            name: name.to_string(),
            events: vec![],
            deadlines: vec![],
        }
    }

    pub fn events(&self) -> &Vec<Event> {
        &self.events
    }
    pub fn events_mut(&mut self) -> &mut Vec<Event> {
        &mut self.events
    }
    pub fn deadlines(&self) -> &Vec<Deadline> {
        &self.deadlines
    }
    pub fn deadlines_mut(&mut self) -> &mut Vec<Deadline> {
        &mut self.deadlines
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
            return Err(CalmarError::WriteFile { e });
        }
        Ok(())
    }

    pub fn add_event(&mut self, event: Event) {
        self.events.push(event)
    }
    pub fn add_deadline(&mut self, deadline: Deadline) {
        self.deadlines.push(deadline)
    }

    pub fn set_events(&mut self, events: Vec<Event>) {
        self.events = events;
    }
}
