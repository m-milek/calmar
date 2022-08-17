use crate::cal::calendar::{
    get_active_calendar, removecal, remove
};
use crate::cal::calendar_index::set;
use crate::cal::calendar_ref::get_active_calendar_reference;
use crate::cal::event::{Event, get_new_event, edit_event};
use crate::cal::{help, self};
use crate::cal::savedata::save_event;
use crate::cli::repl::get_input;
use colored::Colorize;

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
            println!(
                "{}",
                format!(
                    "add: Too many arguments provided. Expected: 0 or 1, Got: {}",
                    split_input.len() - 1
                )
                .yellow()
                .bold()
            ); // do not count "add" as an argument
            return;
        }
    };
    match save_event(new_event, get_active_calendar_reference()) {
        true => {
            println!("{}", "Successfully saved new event.".green().bold());
        }
        false => {
            println!("{}", "Failed to save new event.".red().bold());
        }
    }
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
    let active_calendar = get_active_calendar();
    for event in active_calendar.events {
        println!("{:#?}\n", event.to_standard_event());
    }
}


fn clear(split_input: &Vec<&str>) {
    match split_input.len() {
        1 => {
            println!("\u{001b}c");
        }
        _ => {
            println!(
                "{}",
                format!(
                    "clear: Invalid number of arguments. Expected: 0. Got: {}",
                    split_input.len() - 1
                )
                .yellow()
                .bold()
            );
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
        "quit" | "q" => std::process::exit(0),
        _ => println!(
            "{}",
            format!("Unknown command: {}", split_input[0].trim())
                .yellow()
                .bold()
        ),
    }
}
