use crate::compiler::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(String),
    Variable(Token),
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Grouping(Box<Expr>),
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    Array {
        bracket: Token,
        elements: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression(Expr),
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
    Print(Expr),
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,

    },
    Return {
        keyword: Token,
        value: Option<Expr>,
    },
}