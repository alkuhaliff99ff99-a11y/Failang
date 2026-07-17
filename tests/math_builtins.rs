use fsl::compiler::interpreter::Interpreter;
use fsl::compiler::lexer::Lexer;
use fsl::compiler::parser::Parser;

fn run(source: &str) {
    let tokens = Lexer::new(source).scan_tokens().unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut interpreter = Interpreter::new();
    interpreter.interpret(&program).unwrap();
}

#[test]
fn test_math_builtins_bilingual() {
    run(r#"
    print abs(-5)
    print مطلق(-10)
    print max(3, 9)
    print أكبر_قيمة(7, 4)
    print min(2, 8)
    print أصغر_قيمة(6, 1)
    print sqrt(9)
    print جذر(16)
    print pow(2, 3)
    print قوة(3, 2)
    "#);
}
