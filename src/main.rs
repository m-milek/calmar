mod cal {
    pub mod calendar;
    pub mod calendar_index;
    pub mod calendar_ref;
    pub mod calmar_error;
    pub mod event;
    pub mod macros;
}
mod cli {
    pub mod commands;
    pub mod config;
    pub mod display;
    pub mod functions;
    pub mod getdata;
    pub mod help;
    pub mod messages;
    pub mod parser;
    pub mod repl;
    pub mod util;
    pub mod validator;
}
use crate::cli::config::{get_config, Config};
use crate::cli::repl;
use cli::help::print_startup_message;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = get_config();
    pub static ref EDITOR_CONFIG: rustyline::Config = rustyline::config::Config::builder()
        .color_mode(rustyline::ColorMode::Enabled)
        .history_ignore_dups(true)
        .build();
}

fn main() {
    print_startup_message();
    repl::run();
    std::process::exit(0);
}
