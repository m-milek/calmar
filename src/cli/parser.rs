use std::ops::Range;

use crate::cli::{
    commands::{
        add, cal, clear, duration, edit, list, listcal, remove, removecal, set, sort, until,
    },
    help::print_help,
};

use super::{commands::{raw, write, date, time, update}, functions::handle_unknown_command, help::print_version, messages::error};

/// Handle input and call appropriate functions.
pub fn parse(input: String) {
    let quote_parsed: Vec<String>;
    if check_quotes(&input) {
	quote_parsed = handle_quotes2(input);
    } else {
	error("Mismatched quotes".to_string());
	return;
    }
    let split_input: Vec<&str> = quote_parsed.iter().map(|s| &**s).collect();

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
	"update" | "U" => update(),
        "quit" | "q" => std::process::exit(0),
	"version" | "v" => print_version(),
        "write" | "w" => write(&split_input),
        _ => handle_unknown_command(split_input[0])
    }
}

pub fn check_quotes(input: &String) -> bool {
    let chars: Vec<char> = input.chars().collect();
    if chars.iter().filter(|c| **c =='\"').count() % 2 != 0 {
	return false
    }
    true
}

pub fn handle_quotes2(input: String) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    let chars: Vec<char> = input.chars().collect();
    let mut processed = vec![false; chars.len()];

    let mut quoted_ranges: Vec<Range<usize>> = vec![];
    let mut tmp = String::new();
    let quotation_symbol = '\"';

    // Determine ranges where quoted strings are
    for i in 0..chars.len() {
	if !processed[i] {
	    if chars[i] == quotation_symbol {
		for j in i+1..chars.len() {
		    if chars[j] == quotation_symbol && !processed[j] {
			quoted_ranges.push(i+1..j);
			out.push(input[i+1..j].to_string());
			processed[j] = true;
			break;
		    }
		}
	    } else if !chars[i].is_ascii_whitespace() && !quoted_ranges.iter().any(|r| r.contains(&i)) {
		tmp.push(chars[i]);
	    } else {
		if !tmp.is_empty() {
		    out.push(tmp.to_string());
		    tmp.clear();
		}
	    }
	}
	// fix an issue where if the last arg isn't in quotes, it doesn't get added.
	// if the second condition wasn't there, if the last arg was quoted, an empty
	// string would be added at the end of output
	if i == chars.len()-1 && chars[i] != quotation_symbol {
	    out.push(tmp.clone());
	}
    }
    out
}
