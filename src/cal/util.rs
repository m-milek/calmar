use colored::Colorize;

use crate::cli::repl::get_input;


pub fn uppercase_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

pub fn select_in_range(prompt: &str, max: usize) -> usize {
    let displayed_range = match max {
        1 => 1.to_string(),
        _ => 1.to_string() + "-" + max.to_string().as_str(),
    };

    loop {
        print!("{} [{}]: ", prompt, displayed_range);
        match get_input().parse::<usize>() {
            Ok(num) => match (1..=max).contains(&num) {
                true => {
                    return num;
                }
                false => println!("{}", "Number not in range".yellow().bold()),
            },
            Err(_) => {
                println!(
                    "{}",
                    "Invalid input. Enter a non-negative number".yellow().bold()
                );
            }
        }
    }
}
