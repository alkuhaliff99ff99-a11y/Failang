use crate::compiler::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // رقم (مثل: 5 أو 3.14)
    Literal(String),

    // متغير (مثل: الاسم أو x)
    Variable(Token),

    // عملية أحادية (مثل: -5 أو !خطأ)
    Unary {
        operator: Token,
        right: Box<Expr>,
    },

    // عملية ثنائية (مثل: 5 + 3 أو س == ص)
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },

    // الأقواس لتحديد الأولية (مثل: (5 + 3) * 2)
    Grouping(Box<Expr>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    // جملة تعبيرية (ينتهي بها السطر بـ ; أو بدون)
    Expression(Expr),

    // الإعلان عن متغير (مثل: دع س = 10)
    Var {
        name: Token,
        initializer: Option<Expr>,
    },
}
