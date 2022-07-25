mod calendar;
mod config;
mod event;
mod repl;
mod test;
mod validator;
use crate::config::{get_config, Config};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

fn main() {
    repl::run();
    std::process::exit(0);
}
