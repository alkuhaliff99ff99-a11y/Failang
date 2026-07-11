use std::fmt;
use crate::compiler::lexer::Token;
use crate::compiler::parser::Stmt;

#[derive(Clone, Debug)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    Array(Vec<Value>), // إضافة نوع المصفوفة هنا
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", if *b { "صحيح" } else { "خطأ" }),
            Value::Function { name, .. } => write!(f, "<دالة {}>", name.lexeme),
            Value::Nil => write!(f, "عدم"),
            Value::Array(elements) => {
                let mut result = String::from("[");
                for (i, el) in elements.iter().enumerate() {
                    result.push_str(&format!("{}", el));
                    if i < elements.len() - 1 {
                        result.push_str(", ");
                    }
                }
                result.push(']');
                write!(f, "{}", result)
            }
        }
    }
}
