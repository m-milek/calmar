mod config;
mod event;
mod repl;
mod test;
mod validator;
mod calendar;
use crate::config::{get_config, Config};
use calendar::get_calendar_index;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref CONFIG: Config = get_config();
}

fn main() {
    repl::run();
    std::process::exit(0);
}
