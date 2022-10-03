use crate::{cli::{repl::get_input, util::get_now_even, validator::*}, warning, CONFIG};
use chrono::{Date, Duration, Local, NaiveTime, TimeZone};

/*
Return a valid date
*/
pub fn get_start_date() -> Date<Local> {
    let prompt: &str = "Start Date: ";
    let mut input = get_input(prompt, None);
    while !validate_date(&input) {
        warning!("{input} is not a valid date.");
        input = get_input(prompt, None);
    }
    parse_into_date(input.as_str())
}

/*
Return a valid time
*/
pub fn get_start_time() -> NaiveTime {
    let prompt: &str = "Start Time: ";
    let mut input = get_input(prompt, None);
    while !validate_time(&input) {
        warning!("{input} is not a valid time input.");
        input = get_input(prompt, None);
    }
    parse_into_time(input.as_str())
}

/*
Return a valid duration
*/
pub fn get_duration() -> Duration {
    let prompt: &str = "Duration: ";
    let mut input = get_input(prompt, None);
    while !validate_duration(&input) {
        warning!("{input} is not a valid duration input");
        input = get_input(prompt, None);
    }
    parse_into_duration(input.as_str())
}

/*
Return a valid date equal or greater than start date
TODO: Different errors depending on error type (match expression)
*/
pub fn get_end_date(start_date: &Date<Local>) -> Date<Local> {
    let prompt: &str = "End Date: ";
    let mut input = get_input(prompt, None);
    while !validate_date(&input) || &parse_into_date(&input) < start_date {
        warning!("{input} is not a valid date input.");
        // we have to handle errors differently if the second condition is false
        input = get_input(prompt, None);
    }
    parse_into_date(input.as_str())
}

/*
Return a valid time equal or greater than start time
TODO: Different error messages (same as in `get_end_date`)
*/
pub fn get_end_time(
    start_date: &Date<Local>,
    start_time: &NaiveTime,
    end_date: &Date<Local>,
) -> NaiveTime {
    let prompt: &str = "End Time: ";
    let mut input = get_input(prompt, None);
    while !validate_time(&input)
        || ((start_date == end_date) && (&parse_into_time(&input) <= start_time))
    {
        warning!("{input} is not a valid time input.");
        input = get_input(prompt, None);
    }
    parse_into_time(input.as_str())
}

pub fn get_repeat() -> Duration {
    let prompt: &str = "Repeat: ";
    let mut input = get_input(prompt, None);
    while !validate_duration(&input) {
        warning!("{input} is not a valid duration input");
        input = get_input(prompt, None);
    }
    parse_into_duration(input.as_str())
}

/*
Return a valid difficulty
*/
pub fn get_difficulty() -> u8 {
    let prompt: &str = "Difficulty: ";
    let mut input = get_input(prompt, None);
    while !validate_difficulty(&input) {
        warning!("{input} is not a valid difficulty input.");
        input = get_input(prompt, None);
    }
    input.parse::<u8>().unwrap()
}

/*
Return a valid priority
*/
pub fn get_priority() -> u8 {
    let prompt: &str = "Priority: ";
    let mut input = get_input(prompt, None);
    while !validate_priority(&input) {
        warning!("Entered priority is not valid.");
        input = get_input(prompt, None);
    }
    input.parse::<u8>().unwrap()
}

pub fn get_dir_path() -> String {
    let prompt: &str = "Path: ";
    let mut input = get_input(prompt, get_home_dir().to_str());
    while !validate_dir_path(&input) {
        warning!("{input} is not a valid directory path");
        input = get_input(prompt, get_home_dir().to_str());
    }
    input
}

pub fn get_valid_calendar_name() -> String {
    let prompt: &str = "Calendar Name: ";
    let mut input = get_input(prompt, None);
    while input.is_empty() {
        warning!("Calendar name cannot be an empty string.");
        input = get_input(prompt, None);
    }
    input
}

pub fn get_valid_event_name() -> String {
    let prompt: &str = "Event Name: ";
    let mut input = get_input(prompt, None);
    while input.is_empty() {
        warning!("Event name cannot be an empty string");
        input = get_input(prompt, None);
    }
    input
}

pub fn parse_into_date(input: &str) -> Date<Local> {
    if input.trim().is_empty() {
        return Local::now().date();
    }

    let split_string: Vec<&str> = input.split('/').collect();

    Local.ymd(
        split_string[2].parse().expect("A number was given as year"),
        split_string[1]
            .parse()
            .expect("A number was given as month"),
        split_string[0].parse().expect("A number was given as day"),
    )
}

pub fn parse_into_time(input: &str) -> NaiveTime {
    if input.trim().is_empty() {
        return get_now_even().time();
    }

    let split_string: Vec<&str> = input.split(':').collect();
    NaiveTime::from_hms(
        split_string[0].parse().expect("A number was given as hour"),
        split_string[1]
            .parse()
            .expect("A number was given as minute"),
        0,
    )
}

/*
As of now, this only accepts input such as '3d', '40min' or '3h'
Eventually, support for a format like '1:20h' should be added.
*/
pub fn parse_into_duration(input: &str) -> Duration {
    if input.trim().is_empty() {
        return Duration::zero();
    }

    let input_lower = &input.to_lowercase();

    match (input_lower.contains('d'), input_lower.contains('h'), input_lower.contains('m')) {
        (true, false, false) => Duration::days(
            input_lower.split('d').collect::<Vec<&str>>()[0]
                .trim()
                .parse()
                .expect("Valid duration was given"),
        ),
        (false, true, false) => Duration::hours(
            input_lower.split('h').collect::<Vec<&str>>()[0]
                .trim()
                .parse()
                .expect("Valid duration was given"),
        ),
        (false, false, true) => Duration::minutes(
            input_lower.split('m').collect::<Vec<&str>>()[0]
                .trim()
                .parse()
                .expect("Valid duration was given"),
        ),
        (_, _, _) => panic!("Error parsing duration. This error should be unreachable"),
    }
}
