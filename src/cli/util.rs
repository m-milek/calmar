use crate::cli::{messages::warning, repl::get_input};
use crate::CONFIG;

pub fn uppercase_first_letter(s: &str) -> String {
    s[0..1].to_uppercase() + &s[1..]
}

pub fn select_in_range(prompt: &str, max: usize) -> usize {
    let displayed_range = match max {
        1 => 1.to_string(),
        _ => 1.to_string() + "-" + max.to_string().as_str(),
    };

    loop {
        match get_input(format!("{} [{}]:", prompt, displayed_range).as_str()).parse::<usize>() {
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

pub fn yesno(prompt: &str) -> bool {
    print!("{}", prompt);
    matches!(
        get_input(prompt).trim().to_lowercase().as_str(),
        "yes" | "y"
    )
}

pub fn default_or_custom_save_path(input: String) -> String {
    if input.trim().is_empty() {
        return CONFIG.default_path.clone();
    }
    input
}
