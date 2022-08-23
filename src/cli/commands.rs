use crate::{
    cal::{calendar_index::CalendarIndex, event::Event},
    cli::functions::{edit_event, get_new_event},
    CONFIG,
};

use super::{
    functions::get_new_calendar_reference,
    getdata::{get_valid_calendar_name, get_valid_event_name},
    messages::{error, print_err_msg, success, warning},
};

/*
Given 'name' of a new calendar, the function gets the home directory,
verifies the existence of a $HOME/.calmar directory,
creates a JSON file with the given 'name' under $HOME/.calmar.
If file named 'name' already exists, it asks the user for confirmation.
 */
/// Create a new calendar and save it to the calendar index.
pub fn cal(split_input: &Vec<&str>) {
    let mut new_reference = match split_input.len() {
        1 => get_new_calendar_reference(None),
        2 => get_new_calendar_reference(Some(split_input[1].to_owned())),
        _ => {
            warning(format!(
                "cal: Too many arguments provided. Expected: 0 or 1, Got: {}",
                split_input.len() - 1
            )); // do not count "cal" as an argument
            return;
        }
    };

    let mut index = match CalendarIndex::get() {
        Ok(i) => i,
        Err(e) => {
            print_err_msg(e, &CONFIG.index_path);
            return;
        }
    };

    if index.calendars().is_empty() {
        new_reference.set_active()
    }

    match index.add_entry(&new_reference) {
        Ok(_) => success("Added entry to calendar index.".to_string()),
        Err(_) => {
            error("Failed to add new calendar reference to calendar index.".to_string());
            return;
        }
    }
    if let Err(e) = index.save() {
        print_err_msg(e, &CONFIG.index_path);
        return;
    };
    success("Saved calendar index".to_string());
    if let Err(e) = new_reference.create_file() {
        print_err_msg(e, &new_reference.path());
        return;
    }
}

/// Delete a calendar
pub fn removecal(split_input: &Vec<&str>) {
    let mut index = match CalendarIndex::get() {
        Ok(i) => i,
        Err(e) => {
            print_err_msg(e, &CONFIG.index_path);
            return;
        }
    };
    let name = match split_input.len() {
        1 => get_valid_calendar_name(),
        2 => split_input[1].to_string(),
        _ => {
            warning(format!(
                "removecal: Too many arguments provided. Expected: 0 or 1. Got: {}",
                split_input.len() - 1
            ));
            return;
        }
    };

    match index.delete_entry(name) {
        Ok(_) => (),
        Err(_) => return,
    }

    if let Err(e) = index.save() {
        print_err_msg(e, &CONFIG.index_path);
        return;
    };
    success("Successfully removed calendar".to_string());
}

pub fn default_or_custom_save_path(input: String) -> String {
    if input.trim().is_empty() {
        return CONFIG.default_path.clone();
    }
    input
}

/// Delete an event from the active calendar
pub fn remove(split_input: &Vec<&str>) {
    let name = match split_input.len() {
        1 => get_valid_event_name(),
        2 => split_input[1].to_owned(),
        _ => {
            warning(format!(
                "remove: Too many arguments provided. Expected: 1 or 2. Got: {}",
                split_input.len() - 1
            ));
            return;
        }
    };

    let index = match CalendarIndex::get() {
        Ok(i) => i,
        Err(e) => {
            print_err_msg(e, &CONFIG.index_path);
            return;
        }
    };

    let path = match index.active_calendar_reference() {
        Ok(r) => r,
        Err(e) => {
            print_err_msg(e, &String::new());
            return;
        }
    }
    .path().clone();

    let mut active_calendar = match index.active_calendar() {
        Ok(c) => c,
        Err(e) => {
            print_err_msg(e, &path);
            return;
        }
    };

    active_calendar.events_mut().retain(|event| event.name().ne(&name));

    if let Err(e) = active_calendar.save(&path) {
        print_err_msg(e, &path)
    }
}

/// Change the active calednar
pub fn set(split_input: &Vec<&str>) {
    let mut index = match CalendarIndex::get() {
        Ok(i) => i,
        Err(e) => {
            print_err_msg(e, &CONFIG.index_path);
            return;
        }
    };
    let name = match split_input.len() {
        1 => get_valid_event_name(),
        2 => split_input[1].to_string(),
        _ => {
            warning(format!(
                "set: Too many arguments provided. Expected: 1 or 2. Got: {}",
                split_input.len()
            ));
            return;
        }
    };

    if !index.contains_one_named(&name) {
        return;
    }

    match index.number_of_active_calendars() {
        0 => {
            warning("No calendars are set as active. Please correct this and retry.".to_string());
        }
        1 => {
            index.set_active(name);
            if let Err(e) = index.save() {
                print_err_msg(e, &CONFIG.index_path);
                return;
            };
        }
        _ => {
            warning(
                "More than one calendar is set as active. Please correct this and retry."
                    .to_string(),
            );
        }
    }
}

/*
Call event creation with name given optionally
 */
/// Create a new event and save it to the active calednar.
pub fn add(split_input: &Vec<&str>) {
    let new_event: Event = match split_input.len() {
        1 => get_new_event(None),
        2 => get_new_event(Some(split_input[1].to_owned())),
        _ => {
            warning(format!(
                "add: Too many arguments provided. Expected: 0 or 1, Got: {}",
                split_input.len() - 1
            ));
            // do not count "add" as an argument
            return;
        }
    };
    let index = match CalendarIndex::get() {
        Ok(r) => r,
        Err(e) => {
            print_err_msg(e, &CONFIG.index_path);
            return;
        }
    };

    let path = match index.active_calendar_reference() {
        Ok(r) => r,
        Err(e) => {
            print_err_msg(e, &String::new());
            return;
        }
    }
    .path().clone();

    let mut active_calendar = match index.active_calendar() {
        Ok(cal) => cal,
        Err(e) => {
            print_err_msg(e, &path);
            return;
        }
    };

    active_calendar.add_event(new_event.to_event_json());

    if let Err(e) = active_calendar.save(&path) {
        print_err_msg(e, &path)
    }
}

/*
Edit attributes of a given event and save it
*/
pub fn edit(split_input: &[&str]) {
    split_input[1..].iter().for_each(|e| edit_event(e))
}

/// Display events in the active calendar
pub fn list(split_input: &[&str]) {
    let index = match CalendarIndex::get() {
        Ok(i) => i,
        Err(e) => {
            print_err_msg(e, &CONFIG.index_path);
            return;
        }
    };

    
    let path = match index.active_calendar_reference() {
        Ok(r) => r,
        Err(e) => {
            print_err_msg(e, &String::new());
            return;
        }
    }
    .path().clone();

    let active_calendar = match index.active_calendar() {
        Ok(c) => c,
        Err(e) => {
            print_err_msg(e, &path);
            return;
        }
    };

    active_calendar
        .events()
        .iter()
        .filter(|e| {
            if split_input.len().ne(&1) {
                split_input[1..].contains(&e.name().as_str())
            } else {
                true
            }
        })
        .for_each(|e| println!("{:#?}", e.to_standard_event()))
}

/// Clear the screen
pub fn clear(split_input: &Vec<&str>) {
    match split_input.len() {
        1 => {
            println!("\u{001b}c");
        }
        _ => {
            warning(format!(
                "clear: Invalid number of arguments. Expected: 0. Got: {}",
                split_input.len() - 1
            ));
        }
    }
}

// List calendars and their properties
pub fn listcal(split_input: &Vec<&str>) {
    let index = match CalendarIndex::get() {
        Ok(i) => i,
        Err(e) => {
            print_err_msg(e, &CONFIG.index_path);
            return;
        }
    };

    index.calendars()
        .iter()
        .filter(|r| {
	    if split_input.len().ne(&1) {
		split_input[1..].contains(&r.name().as_str())
	    }
	    else {true}
	})
        .for_each(|r| println!("{:#?}", r))
}
