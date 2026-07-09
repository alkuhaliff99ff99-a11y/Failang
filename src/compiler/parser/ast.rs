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
    // عقدة استدعاء الدالة الجديدة (مثل: احسب(س، 5))
    Call {
        callee: Box<Expr>,    // اسم الدالة أو التعبير المؤدي لها
        paren: Token,         // قوس الإغلاق لتحديد مكان الخطأ إذا حدث
        arguments: Vec<Expr>, // الوسائط الممررة
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
    // عقدة تعريف الدالة الجديدة (مثل: دالة احسب(س) { ... })
    Function {
        name: Token,
        params: Vec<Token>, // المتغيرات المستقبلة لها
        body: Vec<Stmt>,    // الأكواد داخل جسم الدالة
    },
    // جملة الإرجاع الجديدة (مثل: عد س + 1)
    Return {
        keyword: Token,
        value: Option<Expr>,
    },
}
