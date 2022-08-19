use std::fs::read_to_string;
use colored::Colorize;
use serde_derive::{Serialize, Deserialize};
use crate::cli::parser::yesno;
use super::{calendar::{CalendarReturnMessage, Calendar},
	    validator::get_home_dir,
	    calendar_ref::CalendarReference,
	    getdata::get_valid_event_name};
use std::io::Write;

/// Holds a vector of `CalendarReference` structs.
#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarIndex {
    pub calendars: Vec<CalendarReference>,
}


impl CalendarIndex {

    /// Returns `CalendarIndex` struct from `$HOME/.config/calmar/index.json`.
    pub fn get() -> Self {
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

    pub fn contains_one_named(&self, name: &String) -> bool {
	let num = self.calendars.iter().filter(|r| r.name == *name).count();
	match num {
            0 => {
		println!(
                    "{}",
                    format!("No calendars named {}.", name).yellow().bold()
		);
		false
            }
            1 => true,
            _ => {
		println!(
                    "{}",
                    format!(
			"More than one calendars named {}. Please correct this and retry.",
			name
                    )
			.yellow()
			.bold()
		);
		false
            }
	}
    }

    /// Returns `Calendar` struct parsed from the file pointed at by a `CalendarReference`
    /// currently set as active in `$HOME/.config/calmar/index.json`.
    pub fn active_calendar(&self) -> Calendar {
	let num = self.calendars.iter().filter(|r| r.active).count();

	let current_calendar = match num {
            1 => &self.calendars[self.calendars.iter().position(|r| r.active).unwrap()],
            _ => {
		println!(
                    "{}",
                    format!(
			"{} calendars are set as active. There must be exactly one.",
			num
                    )
			.red()
			.bold()
		);
		std::process::exit(1);
            }
	};
	
	let current_calendar_content = match read_to_string(&current_calendar.path) {
            Ok(content) => content,
            Err(e) => {
		println!(
                    "{}",
                    format!("Failed to read {}.\n{}", current_calendar.path, e)
			.red()
			.bold()
		);
		std::process::exit(1);
            }
	};
	
	match serde_json::from_str(&current_calendar_content) {
            Ok(result) => result,
            Err(e) => {
		println!(
                    "{}",
                    format!(
			"Failed to parse {} to Calendar struct. Check for syntax errors,\n{}",
			current_calendar.path, e
                    )
			.red()
			.bold()
		);
		std::process::exit(1);
            }
	}
    }

    /// Returns a `CalendarReference` currently set as active in `$HOME/.config/calmar/index.json`.
    pub fn active_calendar_reference(&self) -> CalendarReference {
	let mut refs = self.calendars.clone();
	let num = refs.iter().filter(|r| r.active).count();
	match num {
            1 => {
		refs.retain(|r| r.active);
		refs[0].clone()
	    },
            _ => {
		println!(
                    "{}",
                    format!(
			"{} calendars are set as active. There must be exactly one.",
			num
                    )
			.red()
			.bold()
		);
		std::process::exit(1);
            }
	}
    }

    pub fn number_of_active_calendars(&self) -> usize {
	self.calendars.iter().filter(|c| c.active).count()
    }
    
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

    pub fn save(&self) {	
	let home_dir = get_home_dir();
	let mut index_file = match std::fs::OpenOptions::new()
	    .write(true)
	    .truncate(true)
	    .open(home_dir.join(".config/calmar/index.json"))
	{
	    Ok(file) => file,
	    Err(e) => {
		println!(
		    "{}",
		    format!(
			"Failed to open {}.\n{}",
			home_dir.join(".config/calmar/index.json").display(),
			e
		    )
			.red()
			.bold()
		);
		std::process::exit(1);
	    }
	};
	let calendar_index_json: String = match serde_json::ser::to_string_pretty(&self) {
	    Ok(result) => result,
	    Err(_e) => {
		println!(
		    "{}",
		    "Failed to serialize calendar index to string.\n{e}"
			.red()
			.bold()
		);
		std::process::exit(1);
	    }
	};

	match write!(index_file, "{}", calendar_index_json) {
	    Ok(_) => (),
	    Err(e) => {
		println!(
		    "{}",
		    format!(
			"Failed to write to {}.\n{}",
			home_dir.join(".config/calmar/index.json").display(),
			e
		    )
			.red()
			.bold()
		);
	    }
	}
    }

    pub fn set_active(&mut self, name: String) {
	// Set the currently active calendar as not active
	// Set the desired calendar as active

	for r in &mut self.calendars {
            if r.active {
		r.set_inactive()
            }
            if r.name == name {
		r.set_active()
            }
	}
    }

    pub fn list(&self) {
	self.calendars.iter().for_each(|c| println!("{}", c.name))
    }
}



/// Change the active calednar
pub fn set(split_input: &Vec<&str>) {
    let mut index = CalendarIndex::get();
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

    if !index.contains_one_named(&name) {
        return;
    }

    match index.number_of_active_calendars() {
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

    index.set_active(name);
    index.save()
}

