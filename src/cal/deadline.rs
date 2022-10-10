use core::fmt;
use std::fmt::{Display, Formatter, write};

use chrono::{Local, DateTime};
use serde_derive::{Serialize, Deserialize};

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Deadline {
    name: String,
    date: DateTime<Local>,
    priority: u8
}

impl Display for Deadline {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
	write!(
	    f,
	    "{}\t{}\t{}",
	    self.name,
	    self.date.date_naive(),
	    self.date.time()
	)
    }
}

impl Deadline {
    pub fn new(name: String, date: DateTime<Local>, priority: u8) -> Self {
	Deadline { name, date, priority }
    }
    pub fn name(&self) -> String {
	self.name.clone()
    }
}
