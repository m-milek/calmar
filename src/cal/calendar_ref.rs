use colored::Colorize;
use serde_derive::{Serialize, Deserialize};
use std::path::PathBuf;
use crate::{
    cli::repl::get_input,
	    cal::{getdata::get_dir_path,
		  calendar::default_or_custom_save_path}};
use crate::cal::calendar::Calendar;
use std::io::Write;

/// Holds a "pointer" to a file containing a `Calendar` struct.
/// # Fields
/// `name`: name of the calendar in file under `path`
/// `path`: path to the file containing a `Calendar` struct
/// `active`: determines if the `Calendar` under `path` is currently selected.
/// There can be only one active calendar.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CalendarReference {
    pub name: String,
    pub path: String,
    pub active: bool,
}

impl CalendarReference {
    pub fn new(name: String, path: String, active: bool) -> Self {
	CalendarReference {
            name,
            path,
            active,
	}
    }
    pub fn set_name(&mut self, name: String) {
	self.name = name
    }
    pub fn set_path(&mut self, path: String) {
	self.path = path
    }
    pub fn set_active(&mut self) {
	self.active = true
    }
    pub fn set_inactive(&mut self) {
	self.active = false
    }
    pub fn create_file(&self) {
	
	let mut calendar_file = match std::fs::File::create(&self.path) {
            Ok(file) => file,
            Err(e) => {
		println!(
                    "{}",
                    format!("Failed to create {}.\n{}", self.path, e)
			.red()
			.bold()
		);
		return;
            }
	};

	let calendar_json: String =
            match serde_json::to_string_pretty(&Calendar::new(self.name.as_str())) {
		Ok(result) => result,
		Err(e) => {
                    println!(
			"{}",
			format!("Failed to serialize calendar to string.\n{}", e)
                            .red()
                            .bold()
                    );
                    return;
		}
            };

	match write!(calendar_file, "{}", calendar_json) {
            Ok(_) => (),
            Err(e) => {
		println!(
                    "{}",
                    format!("Failed to write to {}.\n{}", self.name, e)
			.red()
			.bold()
		);
            }
	}
	println!(
            "{}",
            format!("Written to {}.", self.path)
		.green()
		.bold()
	);
    }
}

/// Create a calendar reference and return it.
pub fn get_new_calendar_reference(name: Option<String>) -> CalendarReference {
    let name = match name {
        Some(name) => name,
        None => {
            print!("Name: ");
            get_input()
        }
    };

    print!("Path: ");
    let path = default_or_custom_save_path(get_dir_path());
    let mut path_to_calendar = PathBuf::from(path).join(&name);
    path_to_calendar.set_extension("json");
    let path_to_calendar_string = match path_to_calendar.to_str() {
        Some(string) => string,
        None => {
            println!(
                "{}",
                format!(
                    "Failed to convert {} to string.",
                    path_to_calendar.display()
                )
                .red()
                .bold()
            );
            std::process::exit(1);
        }
    };
    CalendarReference::new(name, path_to_calendar_string.to_owned(), false)
}

