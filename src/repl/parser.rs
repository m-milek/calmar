mod help;
use crate::event::Event;
use crate::repl::get_input;
use crate::verifier::*;

pub fn get_new_event() -> () {
    println!("Getting new event data...");

    print!("Name: ");
    let name = get_input();

    print!("Start TimeDate: ");
    let mut start = get_input();
    while !verify_date(&start) {
        start = get_input();
    }

    print!("Duration: ");
    let mut duration = get_input();
    while !verify_duration(&duration) {
        duration = get_input();
    }

    print!("End TimeDate: ");
    let mut end = get_input();
    while !verify_date(&end) {
        end = get_input();
    }

    print!("Difficulty: ");
    let mut difficulty = get_input();
    while !verify_difficulty(&difficulty){
        difficulty = get_input();
    }

    print!("Priority: ");
    let mut priority = get_input();
    while !verify_priority(&priority){
        priority = get_input();
    }

    println!(
        "{}\n{}\n{}\n{}\n{}\n{}",
        name, start, duration, end, priority, difficulty
    );
}

pub fn parse(input: String) {
    let split_input: Vec<&str> = input.split_whitespace().collect();
    match split_input[0] {
        "new" | "n" => match split_input[1] {
            "event" => get_new_event(),
            "calendar" => todo!(),
            _ => println!("Unknown command. What do you want to add? [event/calendar]"),
        },
        "remove" | "r" => println!("REMOVE"),
        "show" | "s" => println!("SHOW"),
        "exit" | "e" => std::process::exit(0),
        "help" | "h" => help::print_help(),
        _ => println!("Unknown command"),
    }
}
