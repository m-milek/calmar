mod cal {
    pub mod calendar;
    pub mod event;
    pub mod getconfig;
    pub mod getdata;
    pub mod help;
    pub mod savedata;
    pub mod validator;
}
mod cli {
    pub mod parser;
    pub mod repl;
}
use crate::cal::getconfig::{get_config, Config};
use crate::cli::repl;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

fn main() {
    repl::run();
    std::process::exit(0);
}
