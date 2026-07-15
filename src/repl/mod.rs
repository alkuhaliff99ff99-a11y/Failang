use std::io::{self, Write};
use crate::compiler::interpreter::Interpreter;
use crate::compiler::lexer::Lexer;
use crate::compiler::parser::Parser;

// أكواد ألوان ANSI بسيطة ومضمونة التوافق
const GREEN: &str = "\x1b[32m";
const CYAN: &str = "\x1b[36m";
const YELLOW: &str = "\x1b[33m";
const RESET: &str = "\x1b[0m";

pub fn run_repl() {
    let mut interpreter = Interpreter::new();
    
    println!("{}", YELLOW);
    println!("=================================================");
    println!("        Failang (FSL) Interactive Shell          ");
    println!("                 Version 0.1.0                   ");
    println!("          اكتب :help للمساعدة والتعليمات          ");
    println!("=================================================");
    println!("{}", RESET);

    loop {
        // طباعة المؤشر البسيط باللون الأخضر المضمون
        print!("{}fsl>{} ", GREEN, RESET);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(0) => {
                println!("\nوداعاً!");
                break;
            }
            Ok(_) => {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    continue;
                }

                // معالجة الأوامر الخاصة بالطرفية التي تبدأ بـ :
                if trimmed.starts_with(':') {
                    match trimmed {
                        ":exit" | ":خروج" => {
                            println!("وداعاً!");
                            break;
                        }
                        ":help" | ":مساعدة" => {
                            print_help();
                            continue;
                        }
                        ":clear" | ":مسح" => {
                            print!("{}[2J{}[1;1H", 27 as char, 27 as char); // مسح الشاشة
                            io::stdout().flush().unwrap();
                            continue;
                        }
                        _ => {
                            println!("{}أمر غير معروف. اكتب :help للمساعدة.{}", YELLOW, RESET);
                            continue;
                        }
                    }
                }

                // تنفيذ الكود المدخل
                execute(trimmed, &mut interpreter);
            }
            Err(error) => {
                eprintln!("[FSL:System] خطأ في قراءة المدخلات: {}", error);
                break;
            }
        }
    }
}

// دالة التنفيذ الموحدة لمنع التكرار وتحسين معالجة الأخطاء
pub fn execute(source: &str, interpreter: &mut Interpreter) {
    let lexer = Lexer::new(source);
    let tokens = match lexer.scan_tokens() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("{}[FSL:Lexer] خطأ معجمي: {:?}{}", YELLOW, e, RESET);
            return;
        }
    };

    let mut parser = Parser::new(tokens);
    let statements = parser.parse(); // ملاحظة: سنقوم بتحويل الـ parser لإرجاع Result في مرحلة قادمة

    let _ = interpreter.interpret(&statements);
}

fn print_help() {
    println!("{}", CYAN);
    println!("الأوامر المتاحة في طرفية Failang:");
    println!("  :help    - عرض هذه المساعدة");
    println!("  :clear   - مسح شاشة الطرفية");
    println!("  :exit    - الخروج من الطرفية");
    println!("{}", RESET);
}
