use regex::Regex;

fn is_numeric(string: &str) -> bool {
    for char in string.chars() {
        if !char.is_numeric() {
            return false;
        }
    }
    true
}

fn str_to_num(str: &str) -> Result<u8, core::num::ParseIntError> {
    match str.trim().parse::<u8>() {
        Ok(num) => Ok(num),
        Err(err) => Err(err),
    }
}

/*
Verifies time.
Checks if time conforms to HH:MM
TODO: is the time valid?
*/
pub fn validate_time(time_string: &str) -> bool {
    let re = Regex::new("[0-9]{2}:[0-9]{2}").unwrap();
    re.is_match(time_string.trim())
}

/*
Verifies date.
Checks if the date conforms to DD/MM/YYYY (subject to change by config file)
TODO: Is the date valid?
*/
pub fn validate_date(date_string: &str) -> bool {
    let re = Regex::new(r"[0-9]{2}/[0-9]{2}/[0-9]{4}").unwrap();
    re.is_match(date_string.trim())
}

/*
Verifies a duration
Valid formats:
1h
1 h
1min
10 min
*/
pub fn validate_duration(duration_string: &str) -> bool {
    let re = Regex::new("^[0-9]+(min| +min|h| +h)$").unwrap();
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
