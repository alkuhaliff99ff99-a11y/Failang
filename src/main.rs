mod compiler;
use compiler::lexer::Lexer;
use compiler::parser::Parser;
use compiler::interpreter::Interpreter;

fn main() {
    let code = r#"
        متغير الاسم = "فاي"
        متغير س = 10
        let y = 20
        متغير المجموع = س + y
        
        اطبع "مرحباً بك في لغة: " + الاسم
        اطبع "ناتج الحساب هو: "
        print المجموع
    "#;

    // 1. Lexer: تحويل النص إلى رموز
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();

    // 2. Parser: تحويل الرموز إلى شجرة (AST)
    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    // 3. Interpreter: تنفيذ الشجرة
    let mut interpreter = Interpreter::new();
    println!("--- تشغيل Failang ---");
    interpreter.interpret(ast);
}
