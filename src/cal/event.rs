use crate::cal::calendar::Calendar;
use chrono::{DateTime, Local};
use serde_derive::{Deserialize, Serialize};
use std::io::Write;
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
}

impl EventJSON {
    #[allow(dead_code)]
    pub fn to_standard_event(&self) -> Event {
        Event {
            name: self.name.clone(),
            start: DateTime::<Local>::from_str(&self.start)
                .expect("Failed to parse start datetime from string"),
            end: DateTime::<Local>::from_str(&self.start)
                .expect("Failed to parse end datetime from string"),
            priority: self.priority,
            difficulty: self.difficulty,
        }
    }
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
