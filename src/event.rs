use chrono::{DateTime, Duration, Local, TimeZone};

#[derive(Debug)]
pub struct Event {
    pub name: String,
    pub start: DateTime<Local>,
    pub duration: Duration,
    pub end: DateTime<Local>,
    pub priority: u8,
    pub difficulty: u8,
}

impl Event {
    pub fn empty() -> Self {
        Event {
            name: "None".to_owned(),
            start: Local.ymd(1970, 1, 1).and_hms(0, 0, 0),
            duration: Duration::hours(1),
            end: Local.ymd(1970, 1, 1).and_hms(0, 0, 0),
            priority: 0,
            difficulty: 0,
        }
    }
}
