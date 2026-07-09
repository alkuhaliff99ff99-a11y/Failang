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
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
    // عقدة المصفوفة الجديدة التي تحتوي على عناصرها الداخلية كـ تعبيرات
    Array {
        bracket: Token,      // قوس الإغلاق ] لتحديد السطر عند حدوث خطأ قواعدي
        elements: Vec<Expr>, // العناصر المخزنة بداخل المصفوفة
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
