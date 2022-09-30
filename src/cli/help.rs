use crate::{cli::messages::warning, CONFIG, EDITOR_CONFIG};
use colored::*;

pub fn print_startup_message() {
    let str = format!(
        "{}
Copyright (C) 2022 Michał Miłek & Artur Gulik.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
To report issues, please see:
<https://gitlab.com/calmar-team/calmar/-/issues>
Find the Calmar documentation and other helpful resources at:
    <https://gitlab.com/calmar-team/calmar/-/wikis/home>

For help, type \"help\".",
        "Calmar v0.9.0".purple().bold()
    );
    println!("{str}");
}

pub fn print_version() {
    let str = format!(
        "{}
Copyright (C) 2022 Michał Miłek & Artur Gulik.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.",
        "Calmar v0.9.0".purple().bold()
    );
    println!("{str}");
}

pub fn print_help(split_input: &Vec<&str>) {
    let add_doc = format!(
        "
{}

Your calendars consist of events. `add` lets you add a new event to the calendar
set as active in your index.json file.

{}
> add
> add {}
> a
> a {}

Defined in {}
",
        "add, a".bold(),
        "Syntax".bold(),
        "event_name".italic(),
        "event_name".italic(),
        "src/cli/commands.rs".italic()
    );

    let cal_doc = format!(
        "
{}

Create a new, empty calendar and write it to your index.json file.
You can specify the name and path of the new calendar.

{}
> cal
> cal {}
> c
> c {}

Defined in {}
",
        "cal, c".bold(),
        "Syntax".bold(),
        "calendar_name".italic(),
        "calendar_name".italic(),
        "src/cli/commands.rs".italic()
    );

    let clear_doc = format!(
        "
{}

Print an ANSI escape code to clear the screen.

{}
> clear

Defined in {}
",
        "clear".bold(),
        "Syntax".bold(),
        "src/cli/commands.rs".italic()
    );

    let date_doc = format!(
        "
{}

Print current date.

{}
> date
",
        "date, D".bold(),
        "Syntax".bold()
    );

    let duration_doc = format!(
        "
{}

Print a formatted, human-readable duration of all events passed in as arguments.
If no arguments are given, the user is prompted for input.

{}
> duration
> duration [{}]...
> d
> d [{}]...

Defined in {}
",
        "duration, d".bold(),
        "Syntax".bold(),
        "event_name".italic(),
        "event_name".italic(),
        "src/cli/commands.rs".italic()
    );

    let edit_doc = format!(
        "
{}

Edit a property of all events passed in as arguments. If no arguments are given,
the user is prompted for input.

{}
> edit
> edit [{}]...
> e
> e [{}]...

Defined in {}
",
        "edit, e".bold(),
        "Syntax".bold(),
        "event_name".italic(),
        "event_name".italic(),
        "src/cli/commands.rs".italic()
    );

    let help_doc = format!(
        "
{}

Print a help page. If no arguments are given, print a general help page.
Otherwise, print a help page for a specific command.

{}
> help
> help {}
> h
> h {}

Defined in {}
",
        "help, h".bold(),
        "Syntax".bold(),
        "command_name".italic(),
        "command_name".italic(),
        "src/cli/help.rs".italic()
    );

    let list_doc = format!(
        "
{}

List all events in the active calendar that match one of the arguments passed in.
If no arguments have been passed, list all events in the active calendar.

{}
> list
> list [{}]...
> ls
> ls [{}]...
> l
> l [{}]...
",
        "list, ls, l".bold(),
        "Syntax".bold(),
        "event_name".italic(),
        "event_name".italic(),
        "event_name".italic()
    );

    let listcal_doc = format!(
        "
{}

List all calendars in your index.json file.
Currently, the index file is {}.

{}
> listcal
> lc
",
        "listcal, lc".bold(),
        &CONFIG.index_path.italic(),
        "Syntax".bold()
    );

    let raw_doc = format!(
        "
{}

Print a raw version of the events in the active calendar that match the arguments passed in.
If no arguments are passed, print the entire raw calendar.

{}
> raw
> raw [{}]...
> R
> R [{}]...

",
        "raw, R".bold(),
        "Syntax".bold(),
        "event_name".italic(),
        "event_name".italic()
    );

    let remove_doc = format!(
        "
{}

Remove events from the active calendar.
If no event names are passed in, the user is asked for input.

{}
> remove
> remove [{}]...
> rm
> rm [{}]...
> r
> r [{}]...

Defined in {}
",
        "remove, rm, r".bold(),
        "Syntax".bold(),
        "event_name".italic(),
        "event_name".italic(),
        "event_name".italic(),
        "src/cli/commands.rs".italic(),
    );

    let removecal_doc = format!(
        "
{}

Remove a calendar from index.json along with the associated calendar file.
Every calendar that matches one of the arguments will be removed.
If no arguments are provided, the user is asked for input.

{}
> removecal
> removecal [{}]...
> rmcal
> rmcal [{}]...
> rc
> rc [{}]...

Defined in {}
",
        "removecal, rmcal, rc".bold(),
        "Syntax".bold(),
        "calendar_name".italic(),
        "calendar_name".italic(),
        "calendar_name".italic(),
        "src/cli/commmands.rs".italic()
    );

    let set_doc = format!(
        "
{}

Set the active calendar.
The active calendar will be set to the calendar passed as argument.
If no arguments are passed in, user is asked for input.

{}
> set
> set {}
> s
> s {}

Defined in {}
",
        "set, s".bold(),
        "Syntax".bold(),
        "calendar_name".italic(),
        "calendar_name".italic(),
        "src/cli/commands.rs".italic()
    );

    let sort_doc = format!(
        "
{}

Sort events in the active calendar by default or by specified key and ordering.
Default ordering: Sort by start timedate, if equal - sort by name.
Ascending ordering is applied, unless stated otherwise.

{}
> sort
> sort {}
> sort {} {}
> S
> S {}
> S {} {}

{}
asc, ascending
desc, descending

Defined in {}
",
        "sort, S".bold(),
        "Syntax".bold(),
        "key".italic(),
        "key".italic(),
        "ordering".italic(),
        "key".italic(),
        "key".italic(),
        "ordering".italic(),
        "Ordering Syntax".bold(),
        "src/cli/commands.rs".italic()
    );

    let time_doc = format!(
        "
{}

Print current time.

{}
> time
",
        "time, T".bold(),
        "Syntax".bold()
    );

    let until_doc = format!(
        "
{}

Print time until the start of an event.
Multiple event names can be provided.

{}
> until [{}]...
> u [{}]...
",
        "until, u".bold(),
        "Syntax".bold(),
        "event_name".italic(),
        "event_name".italic()
    );

    let update_doc = format!(
	"
{}

Update the active calendar.
An event is removed if it is not recurring and already happended.
Start and end timedates of recurring events are updated to their occurence closest future in time to current timedate.
If a recurring event is currently happening, its start and end timedates are set to timedates of the current occurence.

{}
> update
> U
",
	"update, U".bold(),
	"Syntax".bold(),
    );

    let update_index_doc = format!(
	"
{}

Update the calendar index.
Remove all calendar references where their path no longer points to an existing file.

{}

> update-index
> Ui
",
	"update-index, Ui".bold(),
	"Syntax".bold()
    );

    let quit_doc = format!(
        "
{}

Quit the program.

{}
> quit
> q
",
        "quit, q".bold(),
        "Syntax".bold()
    );

    let version_doc = format!(
        "
{}

Print version information.

{}
> version
",
        "version, v".bold(),
        "Syntax".bold()
    );

    let write_doc = format!(
        "
{}

Generate a calendar for a given duration and write it to a new file.

{}
> write {}
> write {} {}
> w {}
> w {} {}
",
        "write, w".bold(),
        "Syntax".bold(),
        "filename".italic(),
        "duration".italic(),
        "filename".italic(),
        "filename".italic(),
        "duration".italic(),
        "filename".italic(),
    );

    match split_input.len() {
        1 => {
            println!(
                "
The list of available commands:

{}, {} -- add an event
{}, {} -- add a calendar
{} -- clear the screen
{}, {} -- print current date
{}, {} -- print the duration of an event
{}, {} -- edit an event
{}, {} -- print this information or command documentation
{}, {} -- list events
{}, {} -- list calendars
{}, {} -- print a raw calendar
{}, {} -- remove events
{}, {} -- remove calendars
{}, {} -- set the active calendar
{}, {} -- sort events
{}, {} -- print current time
{}, {} -- print time remaining until an event
{}, {} -- update the active calendar
{}, {} -- update the calendar index
{}, {} -- exit the program
{}, {} -- print version information
{}, {} -- write calendar to a file

Type \"help\" followed by command name for full documentation.

Your current keymap is \"{:#?}\".

Keyboard shortcut lists:
- Emacs: https://catonmat.net/ftp/readline-emacs-editing-mode-cheat-sheet.pdf
- vi: https://catonmat.net/ftp/bash-vi-editing-mode-cheat-sheet.pdf
        ",
                "add".bold(),
                "a".dimmed(),
                "cal".bold(),
                "c".dimmed(),
                "clear".bold(),
                "date".bold(),
                "D".dimmed(),
                "duration".bold(),
                "d".dimmed(),
                "edit".bold(),
                "e".dimmed(),
                "help".bold(),
                "h".dimmed(),
                "list".bold(),
                "ls, l".dimmed(),
                "listcal".bold(),
                "lc".dimmed(),
                "raw".bold(),
                "R".dimmed(),
                "remove".bold(),
                "rm, r".dimmed(),
                "removecal".bold(),
                "rmcal, rc".dimmed(),
                "set".bold(),
                "s".dimmed(),
                "sort".bold(),
                "S".dimmed(),
                "time".bold(),
                "T".dimmed(),
                "until".bold(),
                "u".dimmed(),
                "update".bold(),
                "U".dimmed(),
                "update-index".bold(),
                "Ui".dimmed(),
                "quit".bold(),
                "q".dimmed(),
                "version".bold(),
                "v".dimmed(),
                "write".bold(),
                "w".dimmed(),
                EDITOR_CONFIG.edit_mode(),
            );
        }
        2 => match split_input[1] {
            "add" | "a" => println!("{add_doc}"),
            "cal" | "c" => println!("{cal_doc}"),
            "clear" => println!("{clear_doc}"),
            "date" | "D" => println!("{date_doc}"),
            "duration" | "d" => println!("{duration_doc}"),
            "edit" | "e" => println!("{edit_doc}"),
            "help" | "h" => println!("{help_doc}"),
            "list" | "l" | "ls" => println!("{list_doc}"),
            "listcal" | "lc" => println!("{listcal_doc}"),
            "raw" | "R" => println!("{raw_doc}"),
            "remove" | "rm" | "r" => println!("{remove_doc}"),
            "removecal" | "rmcal" | "rc" => println!("{removecal_doc}"),
            "set" | "s" => println!("{set_doc}"),
            "sort" | "S" => println!("{sort_doc}"),
            "time" | "T" => println!("{time_doc}"),
            "until" | "u" => println!("{until_doc}"),
            "update" | "U" => println!("{update_doc}"),
	    "update-index" | "Ui" => println!("{update_index_doc}"),
            "quit" | "q" => println!("{quit_doc}"),
            "version" | "v" => println!("{version_doc}"),
            "write" | "w" => println!("{write_doc}"),
            _ => warning(format!(
                "help: No documentation for command \"{}\"",
                split_input[1]
            )),
        },
        _ => warning(format!(
            "help: Too many arguments provided. Expected: 0 or 1. Got: {}",
            split_input.len()
        )),
    }
}
