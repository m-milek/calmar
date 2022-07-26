use std::fs::read_to_string;

use crate::{event::EventJSON, validator::get_home_dir};
use home::home_dir;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Calendar {
    pub name: String,
    pub events: Vec<EventJSON>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarReference {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarIndex {
    pub current_calendar: CalendarReference,
    pub calendars: Vec<CalendarReference>,
}

impl Calendar {
    pub fn rename() {}
    pub fn delete() {}
    pub fn set_name() {}
    pub fn set_path() {}
}

impl CalendarIndex {
    pub fn add_entry() {}
    pub fn delete_entry() {}
    pub fn set_calendar() {}
}

pub fn get_calendar_index() -> CalendarIndex {

    let mut home = get_home_dir();
    home.push(".config/calmar/index.json");
    let index_file_path = home;

    let content = match read_to_string(&index_file_path) {
        Ok(result) => result,
        Err(e) => {
            println!("Failed to read {}.\n{}", index_file_path.display(), e);
            panic!();
        }
    };

    match serde_json::from_str(&content) {
        Ok(result) => result,
        Err(e) => {
            println!(
                "Failed to parse {} to CalendarIndex struct. Check for syntax errors.\n{}",
                index_file_path.display(),
                e
            );
            panic!();
        }
    }
}

pub fn get_current_calendar() -> Calendar {
    let index = get_calendar_index();
    let current_calendar_content = match read_to_string(&index.current_calendar.path) {
	Ok(content) => content,
	Err(e) => {
	    println!("Failed to read {}.\n{}", index.current_calendar.path, e);
	    std::process::exit(1);
	}
    };
    match serde_json::from_str(&current_calendar_content) {
	Ok(result) => result,
	Err(e) => {
	    println!("Failed to parse {} to Calendar struct. Check for syntax errors,\n{}", index.current_calendar.path, e);
	    std::process::exit(1);
	}
    }
}
