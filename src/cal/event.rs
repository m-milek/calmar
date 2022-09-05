#![allow(dead_code)]

use chrono::{DateTime, Local};
use serde_derive::{Deserialize, Serialize};
use std::str::FromStr;
use struct_field_names_as_array::FieldNamesAsArray;

#[derive(Debug, PartialEq, Eq, Ord)]
pub struct Event {
    name: String,
    start: DateTime<Local>,
    end: DateTime<Local>,
    priority: u8,
    difficulty: u8,
}

#[derive(Debug, Serialize, Deserialize, Clone, FieldNamesAsArray)]
pub struct EventJSON {
    name: String,
    start: String,
    end: String,
    priority: u8,
    difficulty: u8,
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
	if self.start == other.start {
	    return Some(self.name.cmp(&other.name))
	}
	Some(self.start.cmp(&other.start))
    }
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

    pub fn new(
	name: String,
	start: DateTime<Local>,
	end: DateTime<Local>,
	priority: u8,
	difficulty: u8
    ) -> Self {
	Event { name, start, end, priority, difficulty}
    }

    pub fn name(&self) -> &String {
	&self.name
    }
    pub fn start(&self) -> &DateTime<Local> {
	&self.start
    }
    pub fn end(&self) -> &DateTime<Local> {
	&self.end
    }
    pub fn priority(&self) -> u8 {
	self.priority
    }
    pub fn difficulty(&self) -> u8 {
	self.difficulty
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

    pub fn name(&self) -> &String {
	&self.name
    }
    pub fn start(&self) -> &String {
	&self.start
    }
    pub fn end(&self) -> &String {
	&self.end
    }
    pub fn priority(&self) -> u8 {
	self.priority
    }
    pub fn difficulty(&self) -> u8 {
	self.difficulty
    }
    pub fn parsed_start(&self) -> Result<DateTime<Local>, chrono::ParseError> {
        DateTime::<Local>::from_str(&self.start)
    }
    pub fn parsed_end(&self) -> Result<DateTime<Local>, chrono::ParseError> {
        DateTime::<Local>::from_str(&self.end)
    }
    pub fn duration(&self) {}

    // Setters
    pub fn set_name(&mut self, n: &String) {
	self.name = n.to_string()
    }
    pub fn set_start(&mut self, s: &String) {
	self.start = s.to_string()
    }
    pub fn set_end(&mut self, e: &String) {
	self.end = e.to_string()
    }
    pub fn set_priority(&mut self, p: u8) {
	self.priority = p
    }
    pub fn set_difficulty(&mut self, d: u8) {
	self.difficulty = d
    }
    
}
