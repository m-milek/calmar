use std::fs::read_to_string;
use colored::Colorize;
use serde_derive::{Serialize, Deserialize};
use crate::cli::parser::yesno;
use super::{calendar::{CalendarReturnMessage, check_if_calendar_exists}, validator::get_home_dir, calendar_ref::CalendarReference, getdata::{get_valid_event_name, get_number_of_active_calendars}, savedata::save_calendar_index};

/// Holds a vector of `CalendarReference` structs.
#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarIndex {
    pub calendars: Vec<CalendarReference>,
}


impl CalendarIndex {
    /// Adds a new `CalendarReference` to `self.calendars`.
    ///
    /// # Executed steps
    /// * Check for `CalendarReference`s with calendars named like the new one.
    /// Remove those entries and associated files if the user agrees.
    ///
    /// * Check for `CalendarReference`s with a path like the new one.
    /// Remove those entries and associated files if the user agrees.
    ///
    /// * Push the new `CalendarReference` to the `self.calendars`.
    pub fn add_entry(
        &mut self,
        new_calendar: &CalendarReference,
    ) -> Result<(), CalendarReturnMessage> {
        let mut already_saved_entry_names = Vec::<String>::new();
        for reference in &self.calendars {
            already_saved_entry_names.push(reference.name.clone());
        }

        if already_saved_entry_names.contains(&new_calendar.name) {
            if !yesno(
                format!(
                    "Calendar named {} already exists. Do you want to overwrite it? [y/N]: ",
                    new_calendar.name
                )
                .as_str(),
            ) {
                return Err(CalendarReturnMessage::Abort);
            } else {
                // Remove all calendar files with the same name
                for reference in &self.calendars {
                    if reference.name == new_calendar.name {
                        match std::fs::remove_file(&reference.path) {
                            Ok(_) => (),
                            Err(e) => {
                                println!(
                                    "{}",
                                    format!("Failed to delete file {}.\n{}", reference.path, e)
                                        .red()
                                        .bold()
                                );
                                std::process::exit(1);
                            }
                        }
                    }
                }
                // Remove all references with the same name
                self.calendars
                    .retain(|calendar| calendar.name != new_calendar.name);
            }
        }

        let mut already_saved_entry_paths = Vec::<String>::new();
        for reference in &self.calendars {
            already_saved_entry_paths.push(reference.path.clone());
        }

        if already_saved_entry_paths.contains(&new_calendar.path) {
            if !yesno(
                format!(
                    "Calendar with path {} already exists. Do you want to overwrite it?",
                    new_calendar.path
                )
                .as_str(),
            ) {
                return Err(CalendarReturnMessage::Abort);
            } else {
                // Remove all calendar files with the same path
                for reference in &self.calendars {
                    match std::fs::remove_file(&reference.path) {
                        Ok(_) => (),
                        Err(e) => {
                            println!(
                                "{}",
                                format!("Failed to delete file {}.\n{}", reference.path, e)
                                    .red()
                                    .bold()
                            );
                            std::process::exit(1);
                        }
                    }
                }
                // Remove all references with the same path
                self.calendars
                    .retain(|calendar| calendar.path != new_calendar.path);
            }
        }
        // Now the index is cleaned of any calendars named like the new one and the files are deleted.
        self.calendars.push(new_calendar.clone());
        Ok(())
    }

    /// Deletes an entry from `self.calendars` by name.
    /// Disallows unambigous situations where the number of `CalendarReference`s
    /// named `name` is not equal to one - returns `CalendarReturnMessage::Abort`.
    pub fn delete_entry(&mut self, name: String) -> Result<(), CalendarReturnMessage> {
        let mut tmp_reference_vec = self.calendars.clone();
        tmp_reference_vec.retain(|r| r.name == name);

        match tmp_reference_vec.len() {
            0 => {
                println!(
                    "{}",
                    format!("No calendar named {} found.", name).red().bold()
                );
                return Err(CalendarReturnMessage::Abort);
            }
            1 => match std::fs::remove_file(&tmp_reference_vec[0].path) {
                Ok(_) => (),
                Err(e) => {
                    println!(
                        "{}",
                        format!(
                            "Failed to remove file {}.\n{}",
                            tmp_reference_vec[0].path, e
                        )
                        .red()
                        .bold()
                    );
                    return Err(CalendarReturnMessage::Abort);
                }
            },
            _ => {
                println!("{}", format!("Multiple calendars named {} found. Please fix ~/.config/index.json before proceeding. Calendars must have unique names.", name). red().bold());
                return Err(CalendarReturnMessage::Abort);
            }
        }

        self.calendars.retain(|r| r.name != name);
        Ok(())
    }
}

/// Returns `CalendarIndex` struct set as active in `$HOME/.config/calmar/index.json`.
pub fn get_calendar_index() -> CalendarIndex {
    let mut home = get_home_dir();
    home.push(".config/calmar/index.json");
    let index_file_path = home;

    let content = match read_to_string(&index_file_path) {
        Ok(result) => result,
        Err(e) => {
            println!(
                "{}",
                format!("Failed to read {}.\n{}", index_file_path.display(), e)
                    .red()
                    .bold()
            );
            std::process::exit(1);
        }
    };

    match serde_json::from_str(&content) {
        Ok(result) => result,
        Err(e) => {
            println!(
                "{}",
                format!(
                    "Failed to parse {} to CalendarIndex struct. Check for syntax errors.\n{}",
                    index_file_path.display(),
                    e
                )
                .red()
                .bold()
            );
            panic!();
        }
    }
}


/// Change the active calednar
pub fn set(split_input: &Vec<&str>) {
    let name = match split_input.len() {
        1 => get_valid_event_name(),
        2 => split_input[1].to_string(),
        _ => {
            println!(
                "{}",
                format!(
                    "set: Too many arguments provided. Expected: 1 or 2. Got: {}",
                    split_input.len()
                )
                .yellow()
                .bold()
            );
            return;
        }
    };

    if !check_if_calendar_exists(&name) {
        return;
    }

    match get_number_of_active_calendars() {
        0 => {
            println!(
                "{}",
                "No calendars are set as active. Please correct this and retry."
                    .yellow()
                    .bold()
            );
            return;
        }
        1 => (),
        _ => {
            println!(
                "{}",
                "More than one calendar is set as active. Please correct this and retry."
                    .yellow()
                    .bold()
            );
            return;
        }
    }

    let mut index = get_calendar_index();
    // Set the currently active calendar as not active
    // Set the desired calendar as active
    for calendar in &mut index.calendars {
        if calendar.active {
            calendar.active = false;
        }
        if calendar.name == name {
            calendar.active = true;
        }
    }
    save_calendar_index(index)
}
