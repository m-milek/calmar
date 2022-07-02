use colored::*;

pub fn print_help() {
    println!(
"
The list of available commands:

{}, {} -- display events or calendars
{}, {} -- create events or calendars
{}, {} -- remove specified events or calendars

Type \"help\" followed by command name for full documentation.
",

"show".bold(), "sho, sh, s".dimmed(),
"new".bold(), "ne, n".dimmed(),
"remove".bold(), "rem, re, r".dimmed());
}

