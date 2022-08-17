use colored::Colorize;
use serde_derive::{Serialize, Deserialize};
use std::path::PathBuf;
use crate::{cli::repl::get_input, cal::{getdata::get_dir_path, calendar::default_or_custom_save_path}};
use super::calendar_index::get_calendar_index;

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
    CalendarReference {
        name,
        path: path_to_calendar_string.to_owned(),
        active: false,
    }
}


/// Returns a `CalendarReference` currently set as active in `$HOME/.config/calmar/index.json`.
pub fn get_active_calendar_reference() -> CalendarReference {
    let mut index = get_calendar_index();
    index
        .calendars
        .retain(|calendar_reference| calendar_reference.active);

    match index.calendars.len() {
        1 => index.calendars[0].clone(),
        _ => {
            println!(
                "{}",
                format!(
                    "{} calendars are set as active. There must be exactly one.",
                    index.calendars.len()
                )
                .red()
                .bold()
            );
            std::process::exit(1);
        }
    }
}
