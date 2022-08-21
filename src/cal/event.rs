#![allow(dead_code)]

use chrono::{DateTime, Local};
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;
use struct_field_names_as_array::FieldNamesAsArray;

#[derive(Debug)]
pub struct Event {
    pub name: String,
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
    pub priority: u8,
    pub difficulty: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, FieldNamesAsArray)]
pub struct EventJSON {
    pub name: String,
    pub start: String,
    pub end: String,
    pub priority: u8,
    pub difficulty: u8,
}

impl Event {
    pub fn to_event_json(&self) -> EventJSON {
        EventJSON {
            name: self.name.clone(),
            start: self.start.to_string(),
            end: self.end.to_string(),
            priority: self.priority,
            difficulty: self.difficulty,
        }
    }
    pub fn set_name(){}
    pub fn set_start_date(){}
    pub fn set_start_time(){}
    pub fn set_start(){}
    pub fn set_end_date(){}
    pub fn set_end_time(){}
    pub fn set_end(){}
    pub fn set_duration(){}
    pub fn set_priority(){}
    pub fn set_difficulty(){}

    pub fn duration(){}
}

impl EventJSON {
    #[allow(dead_code)]
    pub fn to_standard_event(&self) -> Event {
        Event {
            name: self.name.clone(),
            start: DateTime::<Local>::from_str(&self.start)
                .expect("Failed to parse start datetime from string"),
            end: DateTime::<Local>::from_str(&self.end)
                .expect("Failed to parse end datetime from string"),
            priority: self.priority,
            difficulty: self.difficulty,
        }
    }
    pub fn parsed_start(&self) -> Result<DateTime<Local>, chrono::ParseError> {
        DateTime::<Local>::from_str(&self.start)
    }
    pub fn parsed_end(&self) -> Result<DateTime<Local>, chrono::ParseError> {
        DateTime::<Local>::from_str(&self.end)
    }
    
    pub fn duration(&self) {}
}
