use crate::{event::EventJSON, repl::parser::yesno, validator::get_home_dir};
use serde_derive::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Debug, Serialize, Deserialize)]
pub struct Calendar {
    pub name: String,
    pub events: Vec<EventJSON>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CalendarReference {
    pub name: String,
    pub path: String,
    pub active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CalendarIndex {
    pub calendars: Vec<CalendarReference>,
}

impl Calendar {
    pub fn new(name: &String) -> Self {
        Calendar {
            name: name.to_string(),
            events: Vec::<EventJSON>::new(),
        }
    }
    pub fn rename() {}
    pub fn delete() {}
    pub fn set_name() {}
    pub fn set_path() {}
}

pub enum CalendarReturnMessage {
    Abort,
}

impl CalendarIndex {
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
                                println!("Failed to delete file {}.\n{}", reference.path, e);
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
                            println!("Failed to delete file {}.\n{}", reference.path, e);
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

    pub fn delete_entry(&mut self, name: String) -> Result<(), CalendarReturnMessage> {

	let mut tmp_reference_vec = self.calendars.clone();
	tmp_reference_vec.retain(|r| r.name == name);
	
	match tmp_reference_vec.len() {
	    0 => {
		println!("No calendar named {} found.", name);
		return Err(CalendarReturnMessage::Abort)
	    }
	    1 => {
		match std::fs::remove_file(&tmp_reference_vec[0].path) {
		    Ok(_) => (),
		    Err(e) => {
			println!("Failed to remove file {}.\n{}", tmp_reference_vec[0].path, e);
			return Err(CalendarReturnMessage::Abort)
		    }
		}
	    },
	    _ => {
		println!("Multiple calendars named {} found. Please fix ~/.config/index.json before proceeding. Calendars must have unique names.", name);
		return Err(CalendarReturnMessage::Abort)
	    }
	}

	self.calendars.retain(|r| r.name != name);
	Ok(())
	
    }
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
            std::process::exit(1);
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
    let mut index = get_calendar_index();
    index
        .calendars
        .retain(|calendar_reference| calendar_reference.active);
    let current_calendar = match index.calendars.len() {
        1 => &index.calendars[0],
        _ => {
            println!(
                "{} calendars are set as active. There must be exactly one.",
                index.calendars.len()
            );
            std::process::exit(1);
        }
    };
    let current_calendar_content = match read_to_string(&current_calendar.path) {
        Ok(content) => content,
        Err(e) => {
            println!("Failed to read {}.\n{}", current_calendar.path, e);
            std::process::exit(1);
        }
    };
    match serde_json::from_str(&current_calendar_content) {
        Ok(result) => result,
        Err(e) => {
            println!(
                "Failed to parse {} to Calendar struct. Check for syntax errors,\n{}",
                current_calendar.path, e
            );
            std::process::exit(1);
        }
    }
}

pub fn get_active_calendar_reference() -> CalendarReference {
    let mut index = get_calendar_index();
    index
        .calendars
        .retain(|calendar_reference| calendar_reference.active);
    match index.calendars.len() {
        1 => index.calendars[0].clone(),
        _ => {
            println!(
                "{} calendars are set as active. There must be exactly one.",
                index.calendars.len()
            );
            std::process::exit(1);
        }
    }
}