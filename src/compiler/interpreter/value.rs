#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<crate::compiler::parser::Stmt>,
    },
    Array(Vec<Value>),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number(n) => write!(f, "{}", n),
            Value::String(s) => write!(f, "{}", s),
            Value::Boolean(b) => write!(f, "{}", if *b { "صحيح" } else { "خطأ" }),
            Value::Nil => write!(f, "عدم"),
            Value::Function { name, .. } => write!(f, "<دالة {}>", name),
            Value::Array(elements) => {
                let mut result = String::new();
                result.push('[');
                for (i, el) in elements.iter().enumerate() {
                    if i > 0 {
                        result.push_str(", "); // استخدام الفاصلة القياسية يمنع انهيار الاتجاهات في المحاكيات البرمجية
                    }
                    result.push_str(&format!("{}", el));
                }
                result.push(']');
                write!(f, "{}", result)
            }
        }
    }
}
