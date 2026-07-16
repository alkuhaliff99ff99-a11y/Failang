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

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Number,
    String,
    Boolean,
    Array,
    Function,
    Nil,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Number => write!(f, "رقم"),
            Type::String => write!(f, "نص"),
            Type::Boolean => write!(f, "منطقي"),
            Type::Array => write!(f, "مصفوفة"),
            Type::Function => write!(f, "دالة"),
            Type::Nil => write!(f, "عدم"),
        }
    }
}

impl Value {
    pub fn type_of(&self) -> Type {
        match self {
            Value::Number(_) => Type::Number,
            Value::String(_) => Type::String,
            Value::Boolean(_) => Type::Boolean,
            Value::Array(_) => Type::Array,
            Value::Function { .. } | Value::Builtin(_) => Type::Function,
            Value::Nil => Type::Nil,
        }
    }
}
