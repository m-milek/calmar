use crate::cal::calendar_index::CalendarIndex;
use crate::cli::help;
use super::commands::{cal, clear, edit, add, remove, removecal, set, list};
use super::messages::warning;

/// Handle input and call appropriate functions.
pub fn parse(input: String) {
    let split_input: Vec<&str> = input.split_whitespace().collect();
    match split_input[0].trim() {
        "add" | "a" => add(&split_input),
        "cal" | "c" => cal(&split_input),
        "clear" => clear(&split_input),
        "edit" | "e" => edit(&split_input),
        "help" | "h" => help::print_help(split_input[0]),
        "remove" | "rm" | "r" => remove(&split_input),
        "removecal" | "rmcal" | "rc" => removecal(&split_input),
        "set" | "s" => set(&split_input),
        "list" | "l" | "ls" => list(&split_input),
	"listcal" | "lc" => CalendarIndex::get().list(),
        "quit" | "q" => std::process::exit(0),
        _ => warning(format!("Unknown command: {}", split_input[0].trim())),
    }
}
