mod event;
mod repl;
mod validator;

fn main() {
    repl::run();
    std::process::exit(0);
}
