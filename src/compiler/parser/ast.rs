use crate::compiler::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(String),
    Variable(Token),
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
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    Expression(Expr),
    // إضافة النطاقات المغلقة { ... } للتحكم في Scope المتغيرات
    Block(Vec<Stmt>),
    // جملة إذا / وإلا (If / Else)
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    // حلقة طالما / أثناء (While Loop)
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
}
