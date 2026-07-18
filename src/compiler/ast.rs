use crate::compiler::tokens::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Literal(LiteralValue),
    Variable(String),
}

#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Var {
        name: String,
        initializer: Expr,
    },
    Print(Expr),
    Expression(Expr),
}
