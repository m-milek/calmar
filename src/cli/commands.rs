use crate::{cal::{calendar_ref::get_new_calendar_reference, calendar_index::CalendarIndex, getdata::{get_valid_calendar_name, get_valid_event_name}, event::{get_new_event, Event, edit_event}}, CONFIG};

use super::messages::{warning, success, error};



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
            warning(format!("cal: Too many arguments provided. Expected: 0 or 1, Got: {}", split_input.len() - 1)); // do not count "cal" as an argument
            return;
        }
    };

    let mut calendar_index = CalendarIndex::get();
    if calendar_index.calendars.is_empty() {
        new_reference.active = true;
    }

    match calendar_index.add_entry(&new_reference) {
        Ok(_) => success("Added entry to calendar index.".to_string()),
        Err(_) => {
            error("Failed to add new calendar reference to calendar index.".to_string());
            return;
        }
    }
    calendar_index.save();
    success("Saved calendar index".to_string());
    new_reference.create_file()
}


/// Delete a calendar
pub fn removecal(split_input: &Vec<&str>) {
    let mut index = CalendarIndex::get();
    let name = match split_input.len() {
        1 => get_valid_calendar_name(),
        2 => split_input[1].to_string(),
        _ => {
            warning(format!("removecal: Too many arguments provided. Expected: 0 or 1. Got: {}", split_input.len() - 1));
            return;
        }
    };

    match index.delete_entry(name) {
        Ok(_) => (),
        Err(_) => return,
    }

    index.save();
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
            warning(format!("remove: Too many arguments provided. Expected: 1 or 2. Got: {}", split_input.len() - 1));
            return;
        }
    };
    let index = CalendarIndex::get();
    let mut active_calendar = index.active_calendar();
    active_calendar.events.retain(|event| event.name != name);
    active_calendar.save(index.active_calendar_reference().path);
}


/// Change the active calednar
pub fn set(split_input: &Vec<&str>) {
    let mut index = CalendarIndex::get();
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
            return;
        }
        1 => (),
        _ => {
            warning(
                "More than one calendar is set as active. Please correct this and retry."
                    .to_string(),
            );
            return;
        }
    }

    index.set_active(name);
    index.save()
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
            warning(format!("add: Too many arguments provided. Expected: 0 or 1, Got: {}", split_input.len() - 1));
	    // do not count "add" as an argument
            return;
        }
    };
    let index = CalendarIndex::get();
    let mut active_calendar = index.active_calendar();
    let path = index.active_calendar_reference().path;
    active_calendar.add_event(new_event.to_event_json());
    active_calendar.save(path);
}

/*
Edit attributes of a given event and save it
*/
pub fn edit(split_input: &[&str]) {
    for event_name in split_input[1..].iter() {
        edit_event(event_name);
    }
}

/// Display events in the active calendar
pub fn list(_split_input: &[&str]) {
    let index = CalendarIndex::get();
    let active_calendar = index.active_calendar();
    for event in &active_calendar.events {
        println!("{:#?}\n", event.to_standard_event());
    }
}


pub fn clear(split_input: &Vec<&str>) {
    match split_input.len() {
        1 => {
            println!("\u{001b}c");
        }
        _ => {
            warning(format!("clear: Invalid number of arguments. Expected: 0. Got: {}", split_input.len() - 1));
        }
    }
}
