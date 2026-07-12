use std::env;
use std::fs;
use std::process;

mod compiler;

use compiler::interpreter::Interpreter;
use compiler::lexer::Lexer;
use compiler::parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("الاستخدام: cargo run <اسم_الملف.fsl>");
        process::exit(1);
    }

    let file_path = &args[1];
    run_file(file_path);
}

fn run_file(path: &str) {
    let source_code = fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!("خطأ: تعذر قراءة الملف المستهدف '{}'", path);
        process::exit(1);
    });

    let lexer = Lexer::new(&source_code);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("خطأ معجمي (Lexical Error): {:?}", e);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let statements = parser.parse();

    let mut interpreter = Interpreter::new();
    let _ = interpreter.interpret(&statements);
}
