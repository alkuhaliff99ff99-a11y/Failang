use fsl::compiler::interpreter::Interpreter;
use fsl::compiler::lexer::Lexer;
use fsl::compiler::parser::Parser;

#[test]
fn test_english_variable_and_math() {
    let source = "let x = 10 + 5; print x;";
    let lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().expect("Lexer error in English");
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().expect("Parser error in English");

    let mut interpreter = Interpreter::new();
    assert!(interpreter.interpret(&statements).is_ok());
}

#[test]
fn test_arabic_variable_and_math() {
    // نفس المنطق الرياضي والبرمجي تماماً بكلمات مفتاحية عربية
    let source = "متغير س = 10 + 5; اطبع س;";
    let lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().expect("Lexer error in Arabic");
    let mut parser = Parser::new(tokens);
    let statements = parser.parse().expect("Parser error in Arabic");

    let mut interpreter = Interpreter::new();
    assert!(interpreter.interpret(&statements).is_ok());
}
