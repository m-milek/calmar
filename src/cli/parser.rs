use crate::cli::{
    commands::{
        add, cal, clear, duration, edit, list, listcal, remove, removecal, set, sort, until,
    },
    help::print_help,
};

use super::{commands::{raw, write, date, time}, functions::handle_unknown_command, help::print_version};

/// Handle input and call appropriate functions.
pub fn parse(input: String) {
    let split_input: Vec<&str> = input.split_whitespace().collect();
    match split_input[0].trim() {
        "add" | "a" => add(&split_input),
        "cal" | "c" => cal(&split_input),
        "clear" => clear(&split_input),
	"date" | "D" => date(),
        "duration" | "d" => duration(&split_input),
        "edit" | "e" => edit(&split_input),
        "help" | "h" => print_help(&split_input),
        "list" | "l" | "ls" => list(&split_input),
        "listcal" | "lc" => listcal(&split_input),
        "raw" | "R" => raw(&split_input),
        "remove" | "rm" | "r" => remove(&split_input),
        "removecal" | "rmcal" | "rc" => removecal(&split_input),
        "set" | "s" => set(&split_input),
        "sort" | "S" => sort(&split_input),
	"time" | "T" => time(),
        "until" | "u" => until(&split_input),
        "quit" | "q" => std::process::exit(0),
	"version" | "v" => print_version(),
        "write" | "w" => write(&split_input),
        _ => handle_unknown_command(split_input[0])
    }
}
