mod diagnostics;
pub mod tools;
use std::env;

mod cli;
mod compiler;
mod repl;
mod runtime;

fn main() {
    let args: Vec<String> = env::args().collect();
    cli::commands::execute(&args);
}
