mod cal {
    pub mod calendar;
    pub mod calendar_index;
    pub mod calendar_ref;
    pub mod calmar_error;
    pub mod event;
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
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

fn main() {
    repl::run();
    std::process::exit(0);
}
