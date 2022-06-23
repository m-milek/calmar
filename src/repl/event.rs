use chrono::{Duration, Local, DateTime};

struct Event {
	name: String,
	start: DateTime<Local>,
	duration: Duration,
	end: DateTime<Local>,
	priority: u8,
	difficulty: u8,
}

impl Event {}
