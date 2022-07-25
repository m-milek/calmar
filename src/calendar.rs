use home::home_dir;
use serde_derive::{Serialize, Deserialize};
use crate::event::EventJSON;

#[derive(Debug, Serialize, Deserialize)]
pub struct Calendar {
    pub name: String,
    pub events: Vec<EventJSON>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarReference {
    pub name: String,
    pub path: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarIndex {
    pub current_calendar: CalendarReference,
    pub calendars: Vec<CalendarReference>
}

impl Calendar {
    pub fn rename(){}
    pub fn delete(){}
    pub fn set_name(){}
    pub fn set_path(){}
}

impl CalendarIndex {
    pub fn add_entry(){}
    pub fn delete_entry(){}
    pub fn set_calendar(){}
}

pub fn get_calendar_index() -> CalendarIndex {
    let mut home = match home_dir() {
	Some(dir) => dir,
	None => {
	    println!("Failed to acquire HOME. Cannot locate calendar index file.");
	    panic!();
	} 
    };

    home.push(".config/calmar/index.json");
    let index_file_path = home;

    let content = match std::fs::read_to_string(&index_file_path) {
	Ok(result) => result,
	Err(e) => {
	    println!("Failed to read {}.\n{}", index_file_path.display(), e);
	    panic!();
	}
    };

    let index: CalendarIndex = match serde_json::from_str(&content) {
	Ok(result) => result,
	Err(e) => {
	    println!("Failed to parse {} to CalendarIndex struct. Check for syntax errors.\n{}", index_file_path.display(), e);
	    panic!();
	}
    };
    return index;
}
