use fsl::compiler::interpreter::Interpreter;
use fsl::compiler::lexer::Lexer;
use fsl::compiler::parser::Parser;

#[test]
fn test_print_literal() {
    let source = "print 42";
    let lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().expect("Lexer error");
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().expect("Parse error");
    let mut interpreter = Interpreter::new();
    assert!(interpreter.interpret(&statements).is_ok());
}

#[test]
fn test_let_and_print() {
    // استخدم الفاصلة المنقوطة لفصل الجملتين في سطر واحد
    let source = "let x = 10; print x";
    let lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().expect("Lexer error");
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().expect("Parse error");
    let mut interpreter = Interpreter::new();
    assert!(interpreter.interpret(&statements).is_ok());
}
