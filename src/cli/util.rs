use std::fmt::Display;

use crate::{cli::repl::get_input, warning, CONFIG};
use chrono::{DateTime, Duration, Local, Timelike, NaiveDateTime};

pub fn uppercase_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

pub fn select_in_range<S: Display>(prompt: S, max: usize) -> usize {
    let displayed_range = match max {
        1 => 1.to_string(),
        _ => 1.to_string() + "-" + max.to_string().as_str(),
    };

    loop {
        match get_input(format!("{} [{}]: ", prompt, displayed_range).as_str(), None)
            .parse::<usize>()
        {
            Ok(num) => match (1..=max).contains(&num) {
                true => {
                    return num;
                }
                false => warning!("Number not in range"),
            },
            Err(_) => {
                warning!("Invalid input. Enter a non-negative number");
            }
        }
    }
}

pub fn default_or_custom_save_path(input: String) -> String {
    if input.trim().is_empty() {
        return CONFIG.default_path.clone();
    }
    input
}

pub fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let v1: Vec<char> = s1.chars().collect();
    let v2: Vec<char> = s2.chars().collect();

    if v1.is_empty() {
        return v2.len();
    }
    if v2.is_empty() {
        return v1.len();
    }

    fn min3<T: Ord>(a: T, b: T, c: T) -> T {
        use std::cmp::min;
        min(a, min(b, c))
    }

    fn delta(x: char, y: char) -> usize {
        if x == y {
            0
        } else {
            1
        }
    }

    let mut column: Vec<usize> = (0..v1.len() + 1).collect();
    for x in 1..v2.len() + 1 {
        column[0] = x;
        let mut lastdiag = x - 1;
        for y in 1..v1.len() + 1 {
            let olddiag = column[y];
            column[y] =
                min3(column[y] + 1, column[y - 1] + 1, lastdiag + delta(v1[y - 1], v2[x - 1]));
            lastdiag = olddiag;
        }
    }
    column[v1.len()]
}

pub fn round_to_full_day(d: NaiveDateTime) -> NaiveDateTime {
    d.with_hour(23)
        .unwrap()
        .with_minute(59)
        .unwrap()
        .with_second(59)
        .unwrap()
}

/// Returns current time, but with seconds and nanosecond zeroed
pub fn get_now_even() -> DateTime<Local> {
    Local::now()
        .with_second(0)
        .unwrap()
        .with_nanosecond(0)
        .unwrap()
}

pub fn duration_fmt(duration: Duration) -> String {
    if duration.num_seconds() < 60 {
        format!("{}s", duration.num_seconds())
    } else if duration.num_minutes() < 60 {
        format!(
            "{}m {}s",
            duration.num_minutes(),
            duration.num_seconds() - duration.num_minutes() * 60
        )
    } else if duration.num_hours() < 24 {
        let num_h = duration.num_hours();
        // remaining minutes after accounting for the whole hours (occurs further into the function as well)
        let num_m = duration.num_minutes() - num_h * 60;
        format!("{}h {}m", num_h, num_m)
    } else {
        let num_d = duration.num_days();
        let num_h = duration.num_hours() - num_d * 24;
        let num_m = duration.num_minutes() - num_h * 60 - num_d * 24 * 60;
        format!("{}d {}h {}m", num_d, num_h, num_m)
    }
}
