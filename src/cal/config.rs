use crate::cal::validator::get_home_dir;
use colored::Colorize;
use serde_derive::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub date_format: String,
    pub time_format: String,
    pub default_path: String,
    pub prompt_text: String,
    pub prompt_color: String,
    pub prompt_bold: bool,
    pub prompt_italic: bool,
    pub prompt_underline: bool,
}

impl Config {
    fn default(&self) -> Config {
        Config {
            date_format: "DD/MM/YYYY".to_string(),
            time_format: "HH:MM".to_string(),
            default_path: "/home/michal/.calmar".to_string(),
            prompt_text: "[calmar]".to_string(),
            /*
            Available colors:
            - black
            - red
            - green
            - yellow
            - blue
            - magenta
            - cyan
            - white
            - bright_*
             */
            prompt_color: "bright_white".to_string(),
            prompt_bold: true,
            prompt_italic: false,
            prompt_underline: false,
        }
    }
}

pub fn get_config() -> Config {
    let config_path = get_home_dir().join(".config/calmar/config.json");
    let config_file = match read_to_string(&config_path) {
        Ok(content) => content,
        Err(e) => {
            println!(
                "{}",
                format!("Failed to read {}.\n{}", config_path.display(), e)
                    .red()
                    .bold()
            );
            std::process::exit(1);
        }
    };
    match serde_json::from_str(&config_file) {
        Ok(config) => config,
        Err(e) => {
            println!(
                "{}",
                format!(
                    "Failed to parse {}. Check for syntax errors.\n{}",
                    config_path.display(),
                    e
                )
                .red()
                .bold()
            );
            std::process::exit(1);
        }
    }
}