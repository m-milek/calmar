#![allow(dead_code)]
use chrono::{DateTime, Local, Duration};
use serde_derive::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsArray;

#[derive(Debug, PartialEq, Eq, Ord, FieldNamesAsArray, Serialize, Deserialize, Clone)]
pub struct Event {
    name: String,
    start: DateTime<Local>,
    end: DateTime<Local>,
    priority: u8,
    difficulty: u8,
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.start == other.start {
            return Some(self.name.cmp(&other.name));
        }
        Some(self.start.cmp(&other.start))
    }
}

impl Event {
    pub fn new(
        name: String,
        start: DateTime<Local>,
        end: DateTime<Local>,
        priority: u8,
        difficulty: u8,
    ) -> Self {
        Event {
            name,
            start,
            end,
            priority,
            difficulty,
        }
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

    pub fn set_name(&mut self, name: &String) {
        self.name = name.to_string()
    }
    pub fn set_start_date() {}
    pub fn set_start_time() {}
    pub fn set_start(&mut self, new_start: &DateTime<Local>) {
        self.start = *new_start
    }
    pub fn set_end_date() {}
    pub fn set_end_time() {}
    pub fn set_end(&mut self, new_end: &DateTime<Local>) {
        self.end = *new_end
    }
    pub fn set_priority(&mut self, p: u8) {
        self.priority = p
    }
    pub fn set_difficulty(&mut self, d: u8) {
        self.difficulty = d
    }

    pub fn duration(&self) -> Duration {
	self.end - self.start
    }
}
