#![allow(dead_code)]

use crate::cli::validator::get_home_dir;
use colored::Colorize;
use serde_derive::{Deserialize, Serialize};
use std::fs::read_to_string;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub date_format: String,
    pub time_format: String,
    pub default_path: String,
    pub index_path: String,
    pub default_calendar_span: String,
    pub print_success_messages: bool,
    pub print_warning_messages: bool,
    pub print_error_messages: bool,
    pub prompt_text: String,
    pub prompt_color: String,
    pub prompt_bold: bool,
    pub prompt_italic: bool,
    pub prompt_underline: bool,
}

impl Config {
    pub fn default() -> Self {
        Config {
            date_format: "DD/MM/YYYY".to_string(),
            time_format: "HH:MM".to_string(),
            default_path: get_home_dir().join(".calmar").to_str().unwrap().to_string(),
            index_path: get_home_dir().join(".config/calmar/index.json").to_str().unwrap().to_string(),
            default_calendar_span: "7d".to_string(),
            print_success_messages: true,
            print_warning_messages: true,
            print_error_messages: true,
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
	    eprintln!("{}",format!("Failed to read {}.\n{}.", config_path.display(), e).red().bold());
	    eprintln!("{}","Using default configuration. Use the \"mkconfig\" command to generate a configuration file.".yellow().bold());
	    return Config::default()
        }
    };
    match serde_json::from_str(&config_file) {
        Ok(config) => config,
        Err(e) => {
	    eprintln!("{}",format!("Failed to parse {}.\n{}.", config_path.display(), e).red().bold());
	    eprintln!("{}","Using default configuration. Use the \"mkconfig\" command to generate a configuration file.".yellow().bold());
	    return Config::default()
        }
    }
}
