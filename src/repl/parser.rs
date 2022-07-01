mod help;
use chrono::{Duration, Local, TimeZone};

use crate::event::Event;
use crate::repl::get_input;
use crate::verifier::*;

pub fn get_new_event() -> Event {
    println!("Getting new event data...");

    print!("Name: ");
    let name = get_input();

    print!("Start date: ");
    let mut start_date = get_input();
    while !verify_date(&start_date) {
        println!("Entered date is not valid.");
        print!("Start date: ");
        start_date = get_input();
    }
    let split_start_date: Vec<&str> = start_date.split('/').collect();
    println!("{:?}", split_start_date);

    print!("Start time: ");
    let mut start_time = get_input();
    while !verify_date(&start_time) {
        println!("Entered time is not valid.");
        print!("Start time: ");
        start_time = get_input();
    }
    let split_start_time: Vec<&str> = start_time.split(':').collect();
    println!("{:?}", split_start_time);

    print!("Duration: ");
    let mut duration = get_input();
    while !verify_duration(&duration) {
        println!("Entered duration is not valid.");
        print!("Duration: ");
        duration = get_input();
    }

    print!("End date: ");
    let mut end_date = get_input();
    while !verify_date(&end_date) {
        println!("Entered date is not valid.");
        print!("End date: ");
        end_date = get_input();
    }
    let split_end_date: Vec<&str> = end_date.split('/').collect();
    println!("{:?}", split_end_date);

    print!("End time: ");
    let mut end_time = get_input();
    while !verify_date(&end_time) {
        println!("Entered time is not valid.");
        print!("End time: ");
        end_time = get_input();
    }
    let split_end_time: Vec<&str> = end_time.split(':').collect();
    println!("{:?}", split_end_time);

    print!("Difficulty: ");
    let mut difficulty = get_input();
    while !verify_difficulty(&difficulty) {
        println!("Entered difficulty is not valid.");
        print!("Difficulty: ");
        difficulty = get_input();
    }

    print!("Priority: ");
    let mut priority = get_input();
    while !verify_priority(&priority) {
        println!("Entered priority is not valid.");
        print!("Priority: ");
        priority = get_input();
    }

    println!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        name, start_date, start_time, duration, end_date, end_time, priority, difficulty
    );

    Event {
        name: name,
        start: Local
            .ymd(
                split_start_date[2].trim().parse().expect("Wanted a number"),
                split_start_date[1].trim().parse().expect("Wanted a number"),
                split_start_date[0].trim().parse().expect("Wanted a number"),
            )
            .and_hms(
                split_start_time[0].trim().parse().expect("Wanted a number"),
                split_start_time[1].trim().parse().expect("Wanted a number"),
                0,
            ),
        duration: Duration::hours(1),
        end: Local
            .ymd(
                split_end_date[2].trim().parse().expect("Wanted a number"),
                split_end_date[1].trim().parse().expect("Wanted a number"),
                split_end_date[0].trim().parse().expect("Wanted a number"),
            )
            .and_hms(
                split_end_time[0].trim().parse().expect("Wanted a number"),
                split_end_time[1].trim().parse().expect("Wanted a number"),
                0,
            ),
        priority: priority
            .trim()
            .parse()
            .expect("Wanted a number as priority"),
        difficulty: difficulty
            .trim()
            .parse()
            .expect("Wanted a number as difficulty"),
    }
}

pub fn parse(input: String) {
    let split_input: Vec<&str> = input.split_whitespace().collect();
    let mut new_event = Event::empty();
    match split_input[0] {
        "new" | "n" => match split_input[1] {
            "event" => new_event = get_new_event(),
            "calendar" => todo!(),
            _ => println!("Unknown command. What do you want to create? [event/calendar]"),
        },
        "remove" | "r" => println!("REMOVE"),
        "show" | "s" => println!("SHOW"),
        "exit" | "e" => std::process::exit(0),
        "help" | "h" => help::print_help(),
        _ => println!("Unknown command"),
    }
    println!("{:?}", new_event);
}
