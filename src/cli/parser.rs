use crate::cal::calendar::{
    removecal, remove
};
use crate::cal::calendar_index::{set, CalendarIndex};
use crate::cal::event::{Event, get_new_event, edit_event};
use crate::cal::{help, self};
use crate::cli::repl::get_input;
use super::messages::warning;

pub fn yesno(prompt: &str) -> bool {
    print!("{}", prompt);
    matches!(get_input().trim().to_lowercase().as_str(), "yes" | "y")
}

/*
Call event creation with name given optionally
 */
/// Create a new event and save it to the active calednar.
fn add(split_input: &Vec<&str>) {
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
fn edit(split_input: &[&str]) {
    for event_name in split_input[1..].iter() {
        edit_event(event_name);
    }
}

/// Display events in the active calendar
fn list(_split_input: &[&str]) {
    let index = CalendarIndex::get();
    let active_calendar = index.active_calendar();
    for event in &active_calendar.events {
        println!("{:#?}\n", event.to_standard_event());
    }
}


fn clear(split_input: &Vec<&str>) {
    match split_input.len() {
        1 => {
            println!("\u{001b}c");
        }
        _ => {
            warning(format!("clear: Invalid number of arguments. Expected: 0. Got: {}", split_input.len() - 1));
        }
    }
}

/// Handle input and call appropriate functions.
pub fn parse(input: String) {
    let split_input: Vec<&str> = input.split_whitespace().collect();
    match split_input[0].trim() {
        "add" | "a" => add(&split_input),
        "cal" | "c" => cal::calendar::cal(&split_input),
        "clear" => clear(&split_input),
        "edit" | "e" => edit(&split_input),
        "help" | "h" => help::print_help(split_input[0]),
        "remove" | "rm" | "r" => remove(&split_input),
        "removecal" | "rmcal" | "rc" => removecal(&split_input),
        "set" | "s" => set(&split_input),
        "list" | "l" | "ls" => list(&split_input),
	"listcal" | "lc" => CalendarIndex::get().list(),
        "quit" | "q" => std::process::exit(0),
        _ => warning(format!("Unknown command: {}", split_input[0].trim())),
    }
}
