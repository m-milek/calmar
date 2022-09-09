use super::commands::{add, cal, clear, edit, list, listcal, remove, removecal, set, sort, duration, until};
use super::messages::warning;
use crate::cli::help;

/// Handle input and call appropriate functions.
pub fn parse(input: String) {
    let split_input: Vec<&str> = input.split_whitespace().collect();
    match split_input[0].trim() {
        "add" | "a" => add(&split_input),
        "cal" | "c" => cal(&split_input),
        "clear" => clear(&split_input),
	"duration" | "d" => duration(&split_input),
        "edit" | "e" => edit(&split_input),
        "help" | "h" => help::print_help(&split_input),
        "list" | "l" | "ls" => list(&split_input),
        "listcal" | "lc" => listcal(&split_input),
        "remove" | "rm" | "r" => remove(&split_input),
        "removecal" | "rmcal" | "rc" => removecal(&split_input),
        "set" | "s" => set(&split_input),
        "sort" | "S" => sort(&split_input),
	"until" | "u" => until(&split_input),
        "quit" | "q" => std::process::exit(0),
        _ => warning(format!("Unknown command: {}", split_input[0].trim())),
    }
}
