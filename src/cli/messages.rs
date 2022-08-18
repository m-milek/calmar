#![allow(dead_code)]
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
