#![allow(dead_code)]
use colored::Colorize;

use crate::{cal::calmar_error::CalmarError, CONFIG};

pub fn success(s: String) {
    if CONFIG.print_success_messages {
        println!("{}", s.green().bold())
    }
}

pub fn warning(s: String) {
    if CONFIG.print_warning_messages {
        eprintln!("{}", s.yellow().bold())
    }
}

pub fn error(s: String) {
    if CONFIG.print_error_messages {
        eprintln!("{}", s.red().bold())
    }
}

pub fn print_err_msg(err: CalmarError, info: &String) {
    match err {
        CalmarError::ReadFile { e } => error(format!("Failed to read {} \n{}", info, e)),
        CalmarError::ParseJSON { e } => error(format!("Failed to parse {} as JSON.\n{}", info, e)),
        CalmarError::WriteFile { e } => error(format!("Failed to write to {}.\n{}", info, e)),
        CalmarError::CreateFile { e } => {
            error(format!("Failed to create file at {}.\n{}", info, e))
        }
        CalmarError::ToJSON { e } => error(format!(
            "Failed to serialize struct to JSON.\n{:#?}\n{}",
            info, e
        )),
        CalmarError::ActiveCalendarCount { e } => error(format!(
            "There are {} calendars set as 'active'. There should be exactly one.",
            e
        )),
    }
}
