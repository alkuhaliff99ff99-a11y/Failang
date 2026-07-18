use crate::compiler::interpreter::Interpreter;
use std::fs;
use std::process;

pub fn run_file(path: &str) {
    if !path.ends_with(".fsl") {
        eprintln!("[FSL:System] خطأ: يجب تشغيل ملف .fsl");
        process::exit(1);
    }

    let source = fs::read_to_string(path).unwrap_or_else(|_| {
        eprintln!("[FSL:System] خطأ: تعذر قراءة الملف '{}'", path);
        process::exit(1);
    });

    let mut interpreter = Interpreter::new();
    interpreter.load_standard_library();
    crate::repl::execute(&source, &mut interpreter);
}
