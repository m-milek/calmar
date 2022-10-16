use unicode_segmentation::UnicodeSegmentation;

use crate::{
    cli::{
        commands::{
            add, cal, clear, date, duration, edit, list, listcal, mkconfig, mkindex, raw, remove,
            removecal, set, sort, time, until, update, update_index, write,
        },
        functions::handle_unknown_command,
        help::print_help,
        help::print_version,
    },
    error, CONFIG,
};
use std::ops::Range;

use super::commands::{
    backup, briefing, deadline, edit_cal, except, ls_deadlines, remove_deadline,
};

/// Handle input and call appropriate functions.
pub fn parse(input: String) {
    if !check_quotes(&input) {
        error!("Mismatched quotes");
        return;
    }
    let quote_parsed = handle_quotes(input);
    let split_input: Vec<&str> = quote_parsed.iter().map(|s| &**s).collect();
    match split_input[0].trim() {
        "add" | "a" => add(&split_input),
        "backup" | "b" => backup(&split_input),
        "briefing" | "br" => briefing(),
        "cal" | "c" => cal(&split_input),
        "clear" => clear(&split_input),
        "date" | "D" => date(),
        "deadline" | "dead" | "de" => deadline(&split_input),
        "duration" | "d" => duration(&split_input),
        "edit" | "e" => edit(&split_input),
        "edit-calendar" | "edit-cal" | "ec" => edit_cal(&split_input),
        "except" | "x" => except(&split_input),
        "help" | "h" => print_help(&split_input),
        "list" | "l" | "ls" => list(&split_input),
        "listcal" | "lc" => listcal(&split_input),
        "list-deadlines" | "ld" => ls_deadlines(&split_input),
        "mkindex" => mkindex(),
        "mkconfig" => mkconfig(),
        "raw" | "R" => raw(&split_input),
        "remove" | "rm" | "r" => remove(&split_input),
        "removecal" | "rmcal" | "rc" => removecal(&split_input),
        "remove-deadline" | "rmd" | "rd" => remove_deadline(&split_input),
        "set" | "s" => set(&split_input),
        "sort" | "S" => sort(&split_input),
        "time" | "T" => time(),
        "until" | "u" => until(&split_input),
        "update" | "U" => update(),
        "update-index" | "Ui" => update_index(),
        "quit" | "q" => std::process::exit(0),
        "version" | "v" => print_version(),
        "write" | "w" => write(&split_input),
        _ => handle_unknown_command(split_input[0]),
    }
}

fn check_quotes(input: &str) -> bool {
    let chars: Vec<char> = input.chars().collect();
    if chars.iter().filter(|c| **c == '\"').count() % 2 != 0 {
        return false;
    }
    true
}

fn handle_quotes(input: String) -> Vec<String> {
    let mut out: Vec<String> = vec![];
    let mut quoted_ranges: Vec<Range<usize>> = vec![];
    let mut non_quoted_ranges: Vec<Range<usize>> = vec![];
    let chars = input.graphemes(true).collect::<Vec<&str>>();
    let mut known_quote = vec![false; chars.len()];
    let quotation_symbol = "\"";

    for (i, c) in chars.iter().enumerate() {
        if c == &quotation_symbol && !known_quote[i] {
            // now we're in a quoted part
            known_quote[i] = true;
            for j in i..chars.len() {
                // if we find the closing quote
                if chars[j] == quotation_symbol && i != j && !known_quote[j] {
                    known_quote[j] = true;
                    // i+1 to skip the opening quote
                    quoted_ranges.push(i + 1..j);
                    let mut x = String::new();
                    for i in i + 1..j {
                        x.push_str(chars[i])
                    }
                    out.push(x.clone());
                    break;
                }
            }
        } else if !quoted_ranges.iter().any(|r| r.contains(&i))
            && !non_quoted_ranges.iter().any(|r| r.contains(&i))
            && chars[i] != quotation_symbol
            && chars[i] != " "
        {
            // we want to add a non-quoted string
            for j in i..chars.len() {
                // if we find a space
                if chars[j] == " " || j == chars.len() - 1 || chars[j] == quotation_symbol {
                    let mut x = String::new();
                    // i..j since we don't skip a quote here
                    non_quoted_ranges.push(i..j);
                    for p in i..=j {
                        x.push_str(chars[p])
                    }
                    if x.ends_with(" ") || chars[j] == quotation_symbol {
                        x.pop();
                    }
                    out.push(x.clone());
                    break;
                }
            }
        }
        // last iteration
        if i == chars.len() - 1 && chars[i] != quotation_symbol && chars.len() != 1 {
            if input.split_ascii_whitespace().last().unwrap().len() != 1 {
                out.pop();
            }
        }
    }
    out
}
