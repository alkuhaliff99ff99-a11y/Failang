use crate::compiler::parser::ast::{Expr, Stmt};
use crate::compiler::lexer::TokenKind;
use crate::compiler::interpreter::environment::Environment;
use crate::compiler::interpreter::value::Value;

pub struct Interpreter {
    pub environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { environment: Environment::new() }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), String> {
        for stmt in statements {
            if let Err(e) = self.execute(stmt) {
                println!("❌ خطأ أثناء التنفيذ: {}", e);
                return Err(e);
            }
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Expression(expr) => { self.evaluate(expr)?; }
            Stmt::Print(expr) => {
                let val = self.evaluate(expr)?;
                // طباعة نظيفة ومباشرة للمستخدم بدون تعقيد الـ Debug
                match val {
                    Value::Number(n) => println!("{}", n),
                    Value::String(s) => println!("{}", s),
                    Value::Boolean(b) => println!("{}", if b { "صواب" } else { "خطأ" }),
                    Value::Nil => println!("عدم"),
                    _ => println!("{:?}", val),
                }
            }
            Stmt::Var { name, initializer } => {
                let val = match initializer {
                    Some(expr) => self.evaluate(expr)?,
                    None => Value::Nil,
                };
                self.environment.define(name.lexeme.clone(), val);
            }
            Stmt::While { condition, body } => {
                loop {
                    let cond_val = self.evaluate(condition)?;
                    if !self.is_truthy(&cond_val) { break; }
                    self.execute(body)?;
                }
            }
            Stmt::Block(statements) => {
                for statement in statements {
                    self.execute(statement)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(val) => {
                if let Ok(n) = val.trim().parse::<f64>() {
                    return Ok(Value::Number(n));
                }
                Ok(Value::String(val.clone()))
            }
            Expr::Variable(name) => self.environment.get(&name.lexeme),
            Expr::Assign { name, value } => {
                let val = self.evaluate(value)?;
                self.environment.assign(&name.lexeme, val.clone())?;
                Ok(val)
            }
            Expr::Binary { left, operator, right } => {
                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;
                match (l, r) {
                    (Value::Number(lv), Value::Number(rv)) => {
                        match operator.kind {
                            TokenKind::Less => Ok(Value::Boolean(lv < rv)),
                            TokenKind::Plus => Ok(Value::Number(lv + rv)),
                            TokenKind::Minus => Ok(Value::Number(lv - rv)),
                            TokenKind::Star => Ok(Value::Number(lv * rv)),
                            TokenKind::Slash => {
                                if rv == 0.0 { return Err("خطأ: لا يمكن القسمة على صفر".to_string()); }
                                Ok(Value::Number(lv / rv))
                            }
                            _ => Err(format!("مشغل غير مدعوم: {:?}", operator.kind)),
                        }
                    }
                    (left_val, right_val) => Err(format!("خطأ حسابي: لا يمكن إجراء عملية بين {:?} و {:?}", left_val, right_val)),
                }
            }
            _ => Err("تعبير غير مدعوم".to_string()),
        }
    }

    fn is_truthy(&self, val: &Value) -> bool {
        match val {
            Value::Boolean(b) => *b,
            Value::Nil => false,
            _ => true,
        }
    }
}
