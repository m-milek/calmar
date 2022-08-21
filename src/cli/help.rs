use colored::*;

pub fn print_help(arg: &str) {
    match arg.trim().to_lowercase().as_str() {
        "" => {
            println!(
                "
        The list of available commands:

        {}, {} -- print this information or command documentation
        {}, {} -- display events or calendars
        {}, {} -- create events or calendars
        {}, {} -- remove specified events or calendars
        {}, {} -- exit the program

        Type \"help\" followed by command name for full documentation.
        ",
                "help".bold(),
                "hel, he, h".dimmed(),
                "show".bold(),
                "sho, sh, s".dimmed(),
                "new".bold(),
                "ne, n".dimmed(),
                "remove".bold(),
                "rem, re, r".dimmed(),
                "exit".bold(),
                "ex, e".dimmed()
            );
        }
        "show" | "sho" | "sh" | "s" => {
            println!(
                "
	        {}
	        ",
                "show, sho, sh, s".bold()
            );
        }
        _ => {
            println!("No documentation entries for: {}", arg);
        }
    }
}
