use colored::Colorize;
use std::io::{self, Write};
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

/*
Continously get input and handle it until the process ends
*/
pub fn run() {
    let mut input;
    loop {
        print!("{}", "[calmar] ".bold());
        input = get_input();
        match input.as_str() {
            "" => (),
            _ => parser::parse(input),
        }
    }
}
