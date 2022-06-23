use std::io::{self, Write};
mod event;
mod help;

pub fn run() {
    let mut input = String::new();
    loop {
        input.clear();
        print!("[calmar] ");
        io::stdout().flush().expect("Failed to flush stdout");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        input.pop(); // pops the newline character '\n'
        match &input as &str {
            "" => (),
            "exit" => break,
            "help" => help::print_help(),
            _ => println!("Undefined command"),
        }
    }
}
