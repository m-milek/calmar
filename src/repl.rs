use std::io::{self, Write};
pub mod parser;

fn get_input() -> String {
    let mut input = String::new();
    io::stdout().flush().expect("flush stdout");
    io::stdin()
        .read_line(&mut input)
        .expect("read a line from stdin");
    input.pop();
    input
}

pub fn run() {
    let mut input: String;
    loop {
        print!("[calmar] ");
        input = get_input();
        match input.as_str() {
            "" => (),
            _ => parser::parse(input),
        }
    }
}
