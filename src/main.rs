use std::env;
use std::fs;
use std::process;

mod compiler;
mod repl;

use compiler::interpreter::Interpreter;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() > 2 {
        println!("الاستخدام: fsl [اسم_الملف.fsl]");
        process::exit(1);
    } else if args.len() == 2 {
        let file_path = &args[1];

        if !file_path.ends_with(".fsl") {
            eprintln!("[FSL:System] خطأ: يجب تشغيل ملف بامتداد .fsl");
            process::exit(1);
        }

        run_file(file_path);
    } else {
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
