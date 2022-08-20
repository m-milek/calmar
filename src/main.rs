mod cal {
    pub mod calendar;
    pub mod calendar_index;
    pub mod calendar_ref;
    pub mod event;
    pub mod getdata;
    pub mod help;
    pub mod validator;
    pub mod util;
}
mod cli {
    pub mod config;
    pub mod commands;
    pub mod display;
    pub mod messages;
    pub mod parser;
    pub mod repl;
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
