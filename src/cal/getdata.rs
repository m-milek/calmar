use crate::cal::calendar::get_calendar_index;
use crate::cal::validator::*;
use crate::cli::parser::{parse_into_date, parse_into_time};
use crate::cli::repl::get_input;
use chrono::{Date, Local, NaiveTime};
use colored::Colorize;

/*
Return a non-empty string
*/
pub fn get_name() -> String {
    let mut input = get_input();
    while input.is_empty() {
        println!(
            "{}",
            "Event name cannot be and empty string.".yellow().bold()
        );
        print!("Name: ");
        input = get_input();
    }
    input
}

/*
Return a valid date
*/
pub fn get_start_date() -> String {
    let mut input = get_input();
    while !validate_date(&input) {
        println!(
            "{}",
            format!("{input} is not a valid date.").yellow().bold()
        );
        print!("Start date: ");
        input = get_input();
    }
    input
}

/*
Return a valid time
*/
pub fn get_start_time() -> String {
    let mut input = get_input();
    while !validate_time(&input) {
        println!(
            "{}",
            format!("{input} is not a valid time input.")
                .yellow()
                .bold()
        );
        print!("Start time: ");
        input = get_input();
    }
    input
}

/*
Return a valid duration
*/
pub fn get_duration() -> String {
    let mut input = get_input();
    while !validate_duration(&input) {
        println!(
            "{}",
            format!("{input} is not a valid duration input")
                .yellow()
                .bold()
        );
        print!("Duration: ");
        input = get_input();
    }
    input
}

/*
Return a valid date equal or greater than start date
TODO: Different errors depending on error type (match expression)
*/
pub fn get_end_date(start_date: &Date<Local>) -> String {
    let mut input = get_input();
    while !validate_date(&input) || &parse_into_date(&input) < start_date {
        println!(
            "{}",
            format!("{input} is not a valid date input.")
                .yellow()
                .bold()
        );
        // we have to handle errors differently if the second condition is false
        print!("End date: ");
        input = get_input();
    }
    input
}

/*
Return a valid time equal or greater than start time
TODO: Different error messages (same as in `get_end_date`)
*/
pub fn get_end_time(
    start_date: &Date<Local>,
    start_time: &NaiveTime,
    end_date: &Date<Local>,
) -> String {
    let mut input = get_input();
    while !validate_time(&input)
        || ((start_date == end_date) && (&parse_into_time(&input) <= start_time))
    {
        println!(
            "{}",
            format!("{input} is not a valid time input.")
                .yellow()
                .bold()
        );
        print!("End time: ");
        input = get_input();
    }
    input
}

/*
Return a valid difficulty
*/
pub fn get_difficulty() -> String {
    let mut input = get_input();
    while !validate_difficulty(&input) {
        println!(
            "{}",
            format!("{input} is not a valid difficulty input.")
                .yellow()
                .bold()
        );
        print!("Difficulty: ");
        input = get_input();
    }
    input
}

/*
Return a valid priority
*/
pub fn get_priority() -> String {
    let mut input = get_input();
    while !validate_priority(&input) {
        println!("{}", "Entered priority is not valid.".yellow().bold());
        print!("Priority: ");
        input = get_input();
    }
    input
}

pub fn get_dir_path() -> String {
    let mut input = get_input();
    while !validate_dir_path(&input) {
        println!(
            "{}",
            format!("{input} is not a valid directory path")
                .yellow()
                .bold()
        );
        print!("Path: ");
        input = get_input();
    }
    input
}

pub fn get_valid_calendar_name() -> String {
    let mut input = get_input();
    while input.is_empty() {
        println!(
            "{}",
            "Calendar name cannot be an empty string.".yellow().bold()
        );
        print!("Name: ");
        input = get_input()
    }
    input
}

pub fn get_number_of_active_calendars() -> i32 {
    let mut calendars = get_calendar_index().calendars;
    calendars.retain(|calendar| calendar.active);
    calendars.len() as i32
}
