use crate::{error, warning, CONFIG};
use chrono::{LocalResult, TimeZone, Utc};
use home::home_dir;
use regex::Regex;
use std::{path::PathBuf, str::FromStr};

pub fn get_home_dir() -> PathBuf {
    match home_dir() {
        Some(dir) => dir,
        None => {
            error!("Failed to get HOME directory.");
            std::process::exit(1);
        }
    }
}

pub fn is_numeric(string: &str) -> bool {
    if string.is_empty() {
        return false;
    }
    for char in string.chars() {
        if !char.is_numeric() {
            return false;
        }
    }
    true
}

fn str_to_num(string: &str) -> Result<i32, core::num::ParseIntError> {
    match string.trim().parse::<i32>() {
        Ok(num) => Ok(num),
        Err(err) => {
            error!("Failed to parse {string} to i32.\n{err}");
            Err(err)
        }
    }
}

pub fn validate_dir_path(path: &str) -> bool {
    if path.trim().is_empty() {
        return true;
    }

    let path = match PathBuf::from_str(path) {
        Ok(path) => path,
        Err(e) => {
            warning!("Failed to parse {path} as path.\n{e}");
            return false;
        }
    };
    path.is_dir()
}

/*
Verifies time.
Checks if time conforms to HH:MM
*/
pub fn validate_time(time_string: &str) -> bool {
    if time_string.trim().is_empty() {
        return true;
    }

    let re = Regex::new("^[0-9]{2}:[0-9]{2}$").unwrap();
    if !re.is_match(time_string.trim()) {
        warning!("Input does not conform to specified time format");
        return false;
    }
    let split_input: Vec<&str> = time_string.split(':').collect();
    let hours = str_to_num(split_input[0]).unwrap();
    let minutes = str_to_num(split_input[1]).unwrap();

    (0..=23).contains(&hours) && (0..=59).contains(&minutes)
}

/*
Verifies date.
Checks if the date conforms to DD/MM/YYYY (subject to change by config file)
*/
pub fn validate_date(date_string: &str) -> bool {
    if date_string.trim().is_empty() {
        return true;
    }
    let re = Regex::new(r"^[0-9]{2}/[0-9]{2}/[0-9]{4}$").unwrap();
    if !re.is_match(date_string.trim()) {
        warning!("Input does not conform to specified format");
        return false;
    }

    let split_input: Vec<&str> = date_string.split('/').collect();
    let day = str_to_num(split_input[0]).unwrap();
    let month = str_to_num(split_input[1]).unwrap();
    let year = str_to_num(split_input[2]).unwrap();

    if year < 1970 {
        return false;
    }

    matches!(
        Utc.ymd_opt(year, month.try_into().unwrap(), day.try_into().unwrap(),),
        LocalResult::Single(_)
    )
}

/*
Verifies a duration
Valid formats:
10h
10 h
10 hours
10hours
10min
10 min
1minutes
10 minutes
10 days
10days
*/
pub fn validate_duration(duration_string: &str) -> bool {
    if duration_string.trim().is_empty() {
        return true;
    }

    let re = Regex::new(
        "^[0-9]+(minutes| +minutes|min| +min|m| +m|h| +h|hours| +hours|d| +d|days| +days)$",
    )
    .unwrap();
    re.is_match(duration_string.trim())
}

pub fn validate_difficulty(difficulty: &str) -> bool {
    if is_numeric(difficulty) && !difficulty.is_empty() {
        let parsed: i32 = difficulty
            .trim()
            .parse()
            .expect("Parsable 32-bit number passed as difficulty");
        match (parsed >= 0, parsed <= 10) {
            (true, true) => return true,
            _ => return false,
        }
    }
    false
}

pub fn validate_priority(priority: &str) -> bool {
    if is_numeric(priority) && !priority.is_empty() {
        let parsed: i32 = priority
            .trim()
            .parse()
            .expect("Parsable 32-bit number passed as priority");
        match (parsed >= 0, parsed <= 10) {
            (true, true) => return true,
            _ => return false,
        }
    }
    false
}
