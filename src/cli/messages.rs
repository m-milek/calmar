#![allow(dead_code)]
use crate::cal::calmar_error::CalmarError;
use crate::CONFIG;
use colored::Colorize;

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

pub fn print_err_msg(err: CalmarError, additional_info: &String) {
    match err {
        CalmarError::OpenFile { e } => error(format!("Failed to read {} \n{}", additional_info, e)),
        CalmarError::ParseJSON { e } => error(format!(
            "Failed to parse {} as JSON.\n{}",
            additional_info, e
        )),
        CalmarError::WriteFile { e } => {
            error(format!("Failed to write to {}.\n{}", additional_info, e))
        }
    }
}
