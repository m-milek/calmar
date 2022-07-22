use chrono::{Local, Date, NaiveTime};

use crate::repl::get_input;
use crate::validator::*;

use super::{parse_into_date, parse_into_time};

/*
Return a non-empty string
*/
pub fn get_name() -> String {
    let mut input = get_input();
    while  input.is_empty() {
	println!("Event name cannot be and empty string.")
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
        println!("{} is not a valid date.", input);
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
        println!("Entered time is not valid.");
        print!("Start time: ");
        input = get_input();
    }
    input
}

/*
Return a valid duration
*/
pub fn get_duration() -> String{
    let mut input = get_input();
    while !validate_duration(&input) {
        println!("Entered duration is not valid.");
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
    while !validate_date(&input) && parse_into_date(&input) > *start_date {
        println!("Entered date is not valid.");
        print!("End date: ");
        input = get_input();
    }
    input
}

/*
Return a valid time equal or greater than start time
TODO: Different error messages (same as in `get_end_date`)
*/
pub fn get_end_time(start_time: &NaiveTime) -> String {
    let mut input = get_input();
    while !validate_time(&input) && parse_into_time(&input) <= *start_time  {
        println!("Entered time is not valid.");
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
        println!("Entered difficulty is not valid.");
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
        println!("Entered priority is not valid.");
        print!("Priority: ");
        input = get_input();
    }
    input
}
