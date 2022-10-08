# Calmar
A fast, easy to use calendar/scheduling program.
The project's aim is to create an easy to use, intuitive tool for keeping track of events, schedules and deadlines.
Calmar's will receive updates introducing more features that we find useful.

# Installation
`cargo` will be needed for the install process. For instructions on installing the Rust toolchain, visit [this link](https://www.rust-lang.org/tools/install).
`git clone https://gitlab.com/calmar-team/calmar.git`
`cd calmar`
`cargo build --release`
Copy `target/release/calmar` to a directry in `PATH`.

In the future, Calmar will be available in the [AUR](https://aur.archlinux.org/).

# Getting Started
Calmar is a CLI calendar program. It revolves around the concept of calendars containing your data.

## Creating calendars
To create a new calendar, run the program and use the `calendar` command (abbreviated as `cal` or `c`):
```
cal foo
```
Now you will be asked where to save the JSON file representing it. 
You can choose the default `$HOME/.calmar` directory by just pressing `Return`. Alternatively, you can type your path of choice.

## Adding data
Calendar data can be divided into two categories: Events and Deadlines (Deadlines are yet to be added in v1.1.0).

To add events, use the `add` command:
```
add bar
```
The `add` command takes any number of arguments. Quotes are supported if you'd wish to have spaces in event names:
```
add foo bar "Learning GNU Emacs"
```
## Displaying calendars
Calendars are just blueprints - the events they contain can have a set time after which they reoccur.

Displaying the calendar using the `list` command generates a real calendar from that blueprint and displays it.
By default, a calendar for 7 full days is generated. This is configurable in `$HOME/.config/calmar/config.json`.

## Help
Information about other commands and their usage is available in Calmar. Use the `help` command.

# Modifications
Calmar's structure is easy to comprehend and modify. If you wish to add a feature, just visit `src/cli/parser.rs` and add a pattern to the `match` statement, bind a function to it.

# Contributing
While we doubt it, maybe you'd be interested in contributing to Calmar.
All contributions are welcome!

# Authors
Michał Miłek
Artur Gulik

# Licensing
Calmar is licensed under the GNU GPLv3.
