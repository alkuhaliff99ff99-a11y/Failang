use crate::compiler::ast::{Expr, Stmt, LiteralValue};
use crate::compiler::tokens::TokenKind;
use crate::compiler::environment::Environment;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { environment: Environment::new() }
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for statement in statements {
            self.execute(statement);
        }
    }

    fn execute(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Var { name, initializer } => {
                let value = self.evaluate(initializer);
                self.environment.define(name, value);
            }
            Stmt::Print(expr) => {
                let value = self.evaluate(expr);
                println!("{}", self.stringify(value));
            }
            Stmt::Expression(expr) => {
                self.evaluate(expr);
            }
        }
    }

    fn evaluate(&self, expr: Expr) -> LiteralValue {
        match expr {
            Expr::Literal(value) => value,
            Expr::Variable(name) => self.environment.get(&name),
            Expr::Binary { left, operator, right } => {
                let left_val = self.evaluate(*left);
                let right_val = self.evaluate(*right);

                match (left_val, operator.kind, right_val) {
                    (LiteralValue::Number(l), TokenKind::Plus, LiteralValue::Number(r)) => LiteralValue::Number(l + r),
                    (LiteralValue::Number(l), TokenKind::Minus, LiteralValue::Number(r)) => LiteralValue::Number(l - r),
                    (LiteralValue::Number(l), TokenKind::Star, LiteralValue::Number(r)) => LiteralValue::Number(l * r),
                    (LiteralValue::Number(l), TokenKind::Slash, LiteralValue::Number(r)) => {
                        if r == 0.0 { panic!("خطأ: لا يمكن القسمة على صفر!"); }
                        LiteralValue::Number(l / r)
                    },
                    (LiteralValue::String(l), TokenKind::Plus, LiteralValue::String(r)) => LiteralValue::String(format!("{}{}", l, r)),
                    _ => panic!("خطأ: عملية حسابية غير صالحة في السطر {}", operator.line),
                }
            }
        }
    }

    fn stringify(&self, value: LiteralValue) -> String {
        match value {
            LiteralValue::Number(n) => n.to_string(),
            LiteralValue::String(s) => s,
            LiteralValue::Boolean(b) => if b { "صح".to_string() } else { "خطأ".to_string() },
            LiteralValue::Null => "عدم".to_string(),
        }
    }
}
