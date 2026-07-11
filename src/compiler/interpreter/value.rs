use crate::compiler::parser::Stmt;
use crate::compiler::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    // نوع الدالة الجديد: يحتوي على الاسم، المعاملات، والجسم
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => {
                if *b { write!(f, "صحيح") } else { write!(f, "خطأ") }
            }
            Value::Nil => write!(f, "عدم"),
            Value::Function { name, .. } => write!(f, "<دالة {}>", name.lexeme),
        }
    }
}
