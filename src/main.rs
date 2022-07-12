mod event;
mod repl;
mod test;
mod validator;
mod config;
use lazy_static::lazy_static;
use crate::config::{Config, get_config};

lazy_static!{
    pub static ref CONFIG: Config = get_config();
}

fn main() {
    repl::run();
    std::process::exit(0);
}
