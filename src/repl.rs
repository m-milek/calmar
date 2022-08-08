use colored::Colorize;
use std::io::{self, Write};

use crate::CONFIG;
pub mod parser;

/*
Perfom everything necessary to get clean input from stdin:
- flush stdout as recommended in the docs
- read line
- pop the '\n' character
- trim trailing whitespace
*/
fn get_input() -> String {
    let mut input = String::new();
    io::stdout().flush().expect("flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("read a line from stdin");
    input.pop();
    input.as_str().trim().to_owned()
}

fn print_prompt() {
    let prompt_text = &CONFIG.prompt_text;
    let mut prompt = prompt_text.white();

    if CONFIG.prompt_bold {
        prompt = prompt.bold();
    }

    if CONFIG.prompt_italic {
        prompt = prompt.italic();
    }

    if CONFIG.prompt_underline {
        prompt = prompt.underline();
    }

    // Colorize the prompt
    prompt = match CONFIG.prompt_color.as_str() {
        "black" => prompt.black(),
        "bright_black" => prompt.bright_black(),
        "red" => prompt.red(),
        "bright_red" => prompt.bright_red(),
        "green" => prompt.green(),
        "bright_green" => prompt.bright_green(),
        "yellow" => prompt.yellow(),
        "bright_yellow" => prompt.bright_yellow(),
        "blue" => prompt.blue(),
        "bright_blue" => prompt.bright_blue(),
        "magenta" => prompt.magenta(),
        "bright_magenta" => prompt.bright_magenta(),
        "cyan" => prompt.cyan(),
        "bright_cyan" => prompt.bright_cyan(),
        "white" => prompt,
        "bright_white" => prompt.bright_white(),
        _ => "INVALID CONFIG".red().bold(),
    };

    print!("{} ", prompt);
}

/*
Continously get input and handle it until the process ends
*/
pub fn run() {
    let mut input;
    loop {
        //print!("{}", "[calmar] ".bold());
        print_prompt();
        input = get_input();
        match input.as_str() {
            "" => (),
            _ => parser::parse(input),
        }
    }
}
