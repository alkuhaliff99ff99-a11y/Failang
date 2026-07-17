use fsl::compiler::interpreter::Interpreter;
use fsl::compiler::lexer::Lexer;
use fsl::compiler::parser::Parser;

fn run_failang(source: &str) {
    let lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().expect("Lexer failed");

    let mut parser = Parser::new(tokens);
    let program = parser.parse().expect("Parser failed");

    let mut interpreter = Interpreter::new();
    interpreter.interpret(&program).expect("Runtime failed");
}

#[test]
fn test_collection_builtins_bilingual() {
    run_failang(
        r#"
        print len([1,2,3])
        print طول([1,2,3])

        print first([10,20,30])
        print أول([10,20,30])

        print last([10,20,30])
        print آخر([10,20,30])

        print contains([1,2,3],2)
        print يحتوي([1,2,3],5)
        "#,
    );
}
