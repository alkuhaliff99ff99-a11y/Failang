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
fn test_variable_and_print() {
    let source = r#"
let x = 10

print x
"#;

    run_failang(source);
}

#[test]
fn test_if_else_execution() {
    let source = r#"
let x = 10

if x > 5
    print "كبير"
else
    print "صغير"
"#;

    run_failang(source);
}

#[test]
fn test_while_loop_execution() {
    let source = r#"
let i = 0

while i < 5
    print i
    i = i + 1
"#;

    run_failang(source);
}

#[test]
fn test_array_index_execution() {
    let source = r#"
let numbers = [10, 20, 30]

print numbers[1]
"#;

    run_failang(source);
}

#[test]
fn test_function_execution() {
    let source = r#"
fn sum(a, b)
    return a + b

print sum(10, 20)
"#;

    run_failang(source);
}

#[test]
fn test_function_with_array_execution() {
    let source = r#"
fn sum(arr)
    let total = 0
    let i = 0

    while i < 3
        total = total + arr[i]
        i = i + 1

    return total

print sum([1, 2, 3])
"#;

    run_failang(source);
}

#[test]
fn test_type_casting_number() {
    // 1. تحويل نص يحتوي على رقم
    run_failang(
        r#"
    متغير س = رقم("123.45")
    اكتب(س)
    "#,
    );

    // 2. تحويل منطقي (صواب) إلى رقم
    run_failang(
        r#"
    متغير ص = رقم(صواب)
    اكتب(ص)
    "#,
    );

    // 3. تحويل عدم إلى رقم
    run_failang(
        r#"
    متغير ع = رقم(عدم)
    اكتب(ع)
    "#,
    );
}
