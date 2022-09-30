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
mod test {
    pub mod test;
}
use crate::cli::{
    config::{get_config, Config},
    functions::{check_calmar_dir, check_config},
    help::print_startup_message,
    repl,
};
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
    check_calmar_dir();
    check_config();
    repl::run();
    std::process::exit(0);
}
