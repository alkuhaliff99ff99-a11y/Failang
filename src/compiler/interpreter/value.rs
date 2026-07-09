#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    // تمثيل الدالة كقيمة قابلة للتداول داخل النظام
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<crate::compiler::parser::Stmt>,
    },
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", if *b { "صحيح" } else { "خطأ" }),
            Value::Nil => write!(f, "عدم"),
            Value::Function { name, .. } => write!(f, "<دالة {}>", name),
        }
    }
}
