use fsl::compiler::lexer::Lexer;
use fsl::compiler::parser::Parser;
use fsl::compiler::interpreter::Interpreter;

fn run_failang(source: &str) {
    let lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().expect("Lexer failed");

    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser failed");

    let mut interpreter = Interpreter::new();
    interpreter.interpret(&program).expect("Runtime failed");
}

#[test]
fn test_english_syntax() {
    let source = r#"
let x = 10
print x
"#;

    run_failang(source);
}

#[test]
fn test_arabic_syntax() {
    let source = r#"
متغير س = 10
اطبع س
"#;

    run_failang(source);
}

#[test]
fn test_mixed_syntax() {
    let source = r#"
let س = 20
print س
"#;

    run_failang(source);
}
