use chrono::{DateTime, Local, TimeZone};
use serde_derive::{Deserialize, Serialize};
use std::io::Write;
use std::str::FromStr;

use crate::calendar::Calendar;

#[derive(Debug)]
pub struct Event {
    pub name: String,
    pub start: DateTime<Local>,
    pub end: DateTime<Local>,
    pub priority: u8,
    pub difficulty: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventJSON {
    pub name: String,
    pub start: String,
    pub end: String,
    pub priority: u8,
    pub difficulty: u8,
}

impl Event {
    pub fn default() -> Self {
        Event {
            name: "None".to_owned(),
            start: Local.ymd(1970, 1, 1).and_hms(0, 0, 0),
            end: Local.ymd(1970, 1, 1).and_hms(0, 0, 0),
            priority: 0,
            difficulty: 0,
        }
    }

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
