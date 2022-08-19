use crate::cli::{messages::warning, repl::get_input};

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
                false => warning("Number not in range".to_string()),
            },
            Err(_) => {
                warning("Invalid input. Enter a non-negative number".to_string());
            }
        }
    }
}
