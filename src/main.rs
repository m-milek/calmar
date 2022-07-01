mod event;
mod repl;
mod verifier;

fn main() {
    repl::run();
    std::process::exit(0);
}
