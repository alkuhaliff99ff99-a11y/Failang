use fsl::compiler::ast::{Expr, Stmt};
use fsl::compiler::lexer::Lexer;
use fsl::compiler::parser::Parser;

fn parse_source(source: &str) -> Vec<Stmt> {
    let lexer = Lexer::new(source);
    let tokens = lexer.scan_tokens().expect("Lexer failed");

    let mut parser = Parser::new(tokens);

    parser.parse().expect("Parser failed")
}

#[test]
fn test_if_statement() {
    let source = r#"
let x = 10

if x > 5
    print "big"
"#;

    let ast = parse_source(source);

    assert_eq!(ast.len(), 2);

    match &ast[1] {
        Stmt::If { .. } => {}
        _ => panic!("Expected If statement"),
    }
}

#[test]
fn test_function_declaration_and_call() {
    let source = r#"
fn add(a, b)
    return a + b

print add(1, 2)
"#;

    let ast = parse_source(source);

    assert_eq!(ast.len(), 2);

    match &ast[0] {
        Stmt::Function { params, body, .. } => {
            assert_eq!(params.len(), 2);
            assert!(!body.is_empty());
        }
        _ => panic!("Expected Function declaration"),
    }
}

#[test]
fn test_while_loop() {
    let source = r#"
let i = 0

while i < 3
    print i
    i = i + 1
"#;

    let ast = parse_source(source);

    assert_eq!(ast.len(), 2);

    match &ast[1] {
        Stmt::While { .. } => {}
        _ => panic!("Expected While statement"),
    }
}

#[test]
fn test_array_literal() {
    let source = r#"
let nums = [1, 2, 3]

print nums
"#;

    let ast = parse_source(source);

    assert_eq!(ast.len(), 2);

    match &ast[0] {
        Stmt::Var {
            initializer: Some(expr),
            ..
        } => match expr {
            Expr::Array { elements, .. } => {
                assert_eq!(elements.len(), 3);
            }
            _ => panic!("Expected Array expression"),
        },
        _ => panic!("Expected variable declaration"),
    }
}

#[test]
fn test_array_indexing() {
    let source = r#"
let nums = [10, 20, 30]

print nums[1]
"#;

    let ast = parse_source(source);

    assert_eq!(ast.len(), 2);

    match &ast[1] {
        Stmt::Print(expr) => match expr {
            Expr::Index { .. } => {}
            _ => panic!("Expected Index expression"),
        },
        _ => panic!("Expected Print statement"),
    }
}

#[test]
fn test_array_index_assignment() {
    let source = r#"
let nums = [1, 2, 3]

nums[1] = 99
"#;

    let ast = parse_source(source);

    assert_eq!(ast.len(), 2);

    match &ast[1] {
        Stmt::Expression(expr) => match expr {
            Expr::IndexAssign { .. } => {}
            _ => panic!("Expected IndexAssign expression"),
        },
        _ => panic!("Expected expression statement"),
    }
}
