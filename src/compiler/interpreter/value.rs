use std::fmt;
use crate::compiler::lexer::Token;
use crate::compiler::parser::Stmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    Builtin(String),
    Array(Vec<Value>),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", if *b { "صواب" } else { "خطأ" }),
            Value::Function { name, .. } => write!(f, "<دالة {}>", name.lexeme),
            Value::Builtin(name) => write!(f, "<دالة_مدمجة {}>", name),
            Value::Array(elements) => {
                let elems: Vec<String> = elements.iter().map(|e| e.to_string()).collect();
                write!(f, "[{}]", elems.join(", "))
            }
            Value::Nil => write!(f, "عدم"),
        }
    }
}
