use fsl::compiler::interpreter::Interpreter;
use fsl::compiler::lexer::Lexer;
use fsl::compiler::parser::Parser;

fn run_failang_error(source: &str) -> bool {
    let lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().expect("Lexer failed");

    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser failed");

    let mut interpreter = Interpreter::new();
    interpreter.interpret(&program).is_err()
}

#[test]
fn test_builtin_error_cases() {
    assert!(run_failang_error(
        r#"print طول(123)"#
    ));

    assert!(run_failang_error(
        r#"print رقم("abc")"#
    ));
}
