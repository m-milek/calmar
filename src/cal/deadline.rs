use std::fmt::{Formatter, Display, Result};
use chrono::{Local, DateTime};
use serde_derive::{Serialize, Deserialize};

use super::calmar_trait::CalendarDataType;

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Deadline {
    name: String,
    date: DateTime<Local>,
    priority: u8
}

impl Display for Deadline {
    fn fmt(&self, f: &mut Formatter) -> Result {
	write!(
	    f,
	    // name, date, time, priority, days left
	    "{}\t{}\t{}\t{}\t{}",
	    self.name,
	    self.date.date_naive(),
	    self.date.time(),
	    self.priority,
	    {
		let days = (self.date - Local::now()).num_days();
		if days > 0 {
		    format!("In {days} days")
		} else if days < 0 {
		    format!("{} days ago", days.abs())
		} else {
		    "Today".to_string()
		}
	    }.as_str()
	)
    }
}

impl CalendarDataType for Deadline {
    fn name(&self) -> String {
	self.name.clone()
    }
    fn priority(&self) -> u8 {
	self.priority
    }
}

impl Deadline {
    pub fn new(name: String, date: DateTime<Local>, priority: u8) -> Self {
	Deadline { name, date, priority }
    }
    pub fn date(&self) -> DateTime<Local> {
	self.date
    }
}
