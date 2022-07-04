mod event;
mod repl;
mod validator;
mod test;

fn main() {
    repl::run();
    std::process::exit(0);
}
