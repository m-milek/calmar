use chrono::{DateTime, Duration, Local};
use serde_derive::{Deserialize, Serialize};
use serde_with::serde_as;
use std::fmt::{self, Display, Formatter};
use struct_field_names_as_array::FieldNamesAsArray;

use super::calmar_trait::CalendarDataType;

#[serde_with::serde_as]
#[derive(Debug, PartialEq, Eq, FieldNamesAsArray, Serialize, Deserialize, Clone)]
pub struct Event {
    name: String,
    start: DateTime<Local>,
    end: DateTime<Local>,
    #[serde_as(as = "serde_with::DurationSeconds<i64>")]
    repeat: Duration,
    priority: u8,
    difficulty: u8,
    exceptions: Vec<DateTime<Local>>,
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.start == other.start {
            return Some(self.name.cmp(&other.name));
        }
        Some(self.start.cmp(&other.start))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.start, &self.name).cmp(&(other.start, &other.name))
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Name: {} | Start: {} | End: {} | Repeat: {} | Priority: {} | Difficulty: {}",
            self.name,
            self.start,
            self.end,
            self.repeat(),
            self.priority,
            self.difficulty
        )
    }
}

impl CalendarDataType for Event {
    fn name(&self) -> String {
        self.name.clone()
    }
    fn priority(&self) -> u8 {
        self.priority
    }
}

impl Event {
    pub fn new(
        name: String,
        start: DateTime<Local>,
        end: DateTime<Local>,
        repeat: Duration,
        priority: u8,
        difficulty: u8,
        exceptions: Vec<DateTime<Local>>,
    ) -> Self {
        Event {
            name,
            start,
            end,
            repeat,
            priority,
            difficulty,
            exceptions,
        }
    }

    pub fn is_happening_on(&self, t: DateTime<Local>) -> bool {
        self.start <= t && t < self.end
    }
    pub fn will_happen_today(&self) -> bool {
        let now = Local::now();
        self.start.date() == now.date() && self.start.time() >= now.time()
    }

    pub fn start(&self) -> DateTime<Local> {
        self.start
    }
    pub fn end(&self) -> DateTime<Local> {
        self.end
    }
    pub fn repeat(&self) -> Duration {
        self.repeat
    }
    pub fn difficulty(&self) -> u8 {
        self.difficulty
    }
    pub fn exceptions(&self) -> &Vec<DateTime<Local>> {
        &self.exceptions
    }
    pub fn exceptions_mut(&mut self) -> &mut Vec<DateTime<Local>> {
        &mut self.exceptions
    }

    pub fn set_name(&mut self, name: &String) {
        self.name = name.to_string()
    }
    pub fn set_start(&mut self, new_start: &DateTime<Local>) {
        self.start = *new_start
    }
    pub fn set_end(&mut self, new_end: &DateTime<Local>) {
        self.end = *new_end
    }
    pub fn set_repeat(&mut self, d: &Duration) {
        self.repeat = *d
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
