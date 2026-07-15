mod diagnostics;
use std::env;

mod compiler;
mod repl;
mod cli;


fn main() {
    let args: Vec<String> = env::args().collect();
    cli::commands::execute(&args);
}

