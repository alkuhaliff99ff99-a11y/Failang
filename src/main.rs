use std::env;
use std::fs;
use std::process;

mod compiler;
mod repl;

use compiler::interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 2 {
        println!("الاستخدام: cargo run [اسم_الملف.fsl]");
        process::exit(1);
    } else if args.len() == 2 {
        let file_path = &args[1];
        run_file(file_path);
    } else {
        // تشغيل الـ REPL من وحدته المستقلة الجديدة
        repl::run_repl();
    }
}

fn run_file(path: &str) {
    let source_code = fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!("[FSL:System] خطأ: تعذر قراءة الملف المستهدف '{}'", path);
        process::exit(1);
    });

    let mut interpreter = Interpreter::new();
    repl::execute(&source_code, &mut interpreter);
}
