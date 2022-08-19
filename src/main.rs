mod cal {
    pub mod calendar;
    pub mod calendar_index;
    pub mod calendar_ref;
    pub mod event;
    pub mod config;
    pub mod getdata;
    pub mod help;
    pub mod validator;
    pub mod util;
}
mod cli {
    pub mod parser;
    pub mod repl;
    pub mod display;
    pub mod messages;
}
use crate::cal::config::{get_config, Config};
use crate::cli::repl;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

fn main() {
    repl::run();
    std::process::exit(0);
}
