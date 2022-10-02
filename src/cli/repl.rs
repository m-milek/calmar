use crate::{CONFIG, EDITOR_CONFIG, error};
use colored::{ColoredString, Colorize};

/*
Perfom everything necessary to get clean input from stdin:
- flush stdout as recommended in the docs
- read line
- pop the '\n' character
- trim trailing whitespace
 */
/// Get clean stdin input without trailing spaces and newline
pub fn get_input(prompt: &str) -> String {
    let mut rl = Editor::<()>::with_config(*EDITOR_CONFIG).unwrap();
    let readline = rl.readline(prompt);

    match readline {
        Ok(line) => line,
        Err(ReadlineError::Interrupted) => {
            println!("CTRL-C");
            std::process::exit(1);
        }
        Err(ReadlineError::Eof) => {
            println!("EOF");
            std::process::exit(1);
        }
        Err(err) => {
            println!("Error: {err}");
            std::process::exit(1);
        }
    }
}

/// Print a prompt as defined in config.json,
/// add a space at the end.
fn get_prompt() -> ColoredString {
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

    prompt
}
use rustyline::{error::ReadlineError, Editor};

/*
Continously get input and handle it until the process ends
 */
pub fn run() {
    let config = rustyline::config::Config::builder()
        .color_mode(rustyline::ColorMode::Enabled)
        .history_ignore_dups(true)
        .build();

    let mut rl = match Editor::<()>::with_config(config) {
        Ok(editor) => editor,
        Err(err) => {
            error!("Failed to construct rustyline::Editor with given config. Should be unreachable and checked beforehand.\n{}", err);
            return;
        }
    };
    if rl.load_history("history.txt").is_err() {
        println!("No previous history");
    }

    loop {
        let readline = rl.readline(&(get_prompt().to_string() + " "));

        match readline {
            Ok(line) => match line.as_str() {
                "" => {}
                _ => {
                    rl.add_history_entry(line.as_str());
                    if let Err(e) = rl.save_history("history.txt") {
                        error!("Failed to save command to history.\n{e}");
                        break;
                    };
                    crate::cli::parser::parse(line);
                }
            },
            Err(ReadlineError::Interrupted) => {
                break;
            }
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {err}");
                break;
            }
        }
    }
}
