use std::sync::{Arc, Mutex};
use super::environment::Environment;
use super::value::Value;
use crate::compiler::parser::{Expr, Stmt};
use crate::compiler::lexer::TokenKind;

pub struct Interpreter { environment: Arc<Mutex<Environment>> }

impl Interpreter {
    pub fn new() -> Self { Self { environment: Arc::new(Mutex::new(Environment::new())) } }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), String> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn call_native(&mut self, name: &str, args: Vec<Value>) -> Result<Value, String> {
        match name {
            "نوع" => {
                if args.len() != 1 { return Err("دالة 'نوع' تتطلب وسيطاً واحداً".to_string()); }
                Ok(Value::String(match &args[0] {
                    Value::Number(_) => "رقم", Value::String(_) => "نص",
                    Value::Boolean(_) => "منطق", Value::Array(_) => "مصفوفة", _ => "عدم"
                }.to_string()))
            }
            "طول" => {
                if args.len() != 1 { return Err("دالة 'طول' تتطلب وسيطاً واحداً".to_string()); }
                if let Value::Array(arr) = &args[0] { Ok(Value::Number(arr.len() as f64)) }
                else { Err("دالة 'طول' تعمل فقط مع المصفوفات".to_string()) }
            }
            _ => Err(format!("الدالة '{}' غير معرفة", name)),
        }
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<Option<Value>, String> {
        match stmt {
            Stmt::Expression(expr) => { self.evaluate(expr)?; }
            Stmt::Print(expr) => { println!("{}", self.evaluate(expr)?); }
            Stmt::Var { name, initializer } => {
                let val = match initializer { Some(init) => self.evaluate(init)?, None => Value::Nil };
                self.environment.lock().unwrap().define(name.lexeme.clone(), val);
            }
            // ربط الحقول الصحيحة المتوافقة مع ملف value.rs
            Stmt::Function { name, params, body } => {
                let function = Value::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                };
                self.environment.lock().unwrap().define(name.lexeme.clone(), function);
            }
            Stmt::Block(statements) => {
                let prev = self.environment.clone();
                self.environment = Arc::new(Mutex::new(Environment::new_with_enclosing(prev.clone())));
                for s in statements { if let Some(ret) = self.execute(s)? { self.environment = prev; return Ok(Some(ret)); } }
                self.environment = prev;
            }
            Stmt::If { condition, then_branch, else_branch } => {
                let cond = self.evaluate(condition)?;
                if self.is_truthy(&cond) { return self.execute(then_branch); }
                else if let Some(else_stmt) = else_branch { return self.execute(else_stmt); }
            }
            Stmt::While { condition, body } => {
                loop {
                    let cond = self.evaluate(condition)?;
                    if !self.is_truthy(&cond) { break; }
                    if let Some(ret) = self.execute(body)? { return Ok(Some(ret)); }
                }
            }
            Stmt::Return { value, .. } => {
                let ret = match value { Some(e) => self.evaluate(e)?, None => Value::Nil };
                return Ok(Some(ret));
            }
        }
        Ok(None)
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(l) => {
                if l == "true" || l == "صحيح" { Ok(Value::Boolean(true)) }
                else if l == "false" || l == "خطأ" { Ok(Value::Boolean(false)) }
                else if let Ok(n) = l.parse::<f64>() { Ok(Value::Number(n)) }
                else { Ok(Value::String(l.trim_matches('"').to_string())) }
            }
            Expr::Variable(n) => self.environment.lock().unwrap().get(&n.lexeme),
            Expr::Array { elements, .. } => {
                let mut arr = Vec::new();
                for e in elements { arr.push(self.evaluate(e)?); }
                Ok(Value::Array(arr))
            }
            Expr::Call { callee, arguments, .. } => {
                let func_val = self.evaluate(callee)?;
                let mut args = Vec::new();
                for arg in arguments { args.push(self.evaluate(arg)?); }

                if let Expr::Variable(name) = &**callee {
                    let env = self.environment.lock().unwrap();
                    if env.get(&name.lexeme).is_err() { drop(env); return self.call_native(&name.lexeme, args); }
                }

                if let Value::Function { params, body, .. } = func_val {
                    let prev = self.environment.clone();
                    self.environment = Arc::new(Mutex::new(Environment::new_with_enclosing(prev.clone())));
                    for (p, a) in params.iter().zip(args) { self.environment.lock().unwrap().define(p.lexeme.clone(), a); }
                    let mut res = Value::Nil;
                    for stmt in &body { if let Some(ret) = self.execute(stmt)? { res = ret; break; } }
                    self.environment = prev;
                    Ok(res)
                } else { Err("يمكن استدعاء الدوال فقط".to_string()) }
            }
            Expr::Binary { left, operator, right } => {
                let (l, r) = (self.evaluate(left)?, self.evaluate(right)?);
                match (&l, &operator.kind, &r) {
                    (Value::Number(a), TokenKind::Plus, Value::Number(b)) => Ok(Value::Number(a + b)),
                    (Value::Number(a), TokenKind::Minus, Value::Number(b)) => Ok(Value::Number(a - b)),
                    (Value::Number(a), TokenKind::Star, Value::Number(b)) => Ok(Value::Number(a * b)),
                    (Value::Number(a), TokenKind::Slash, Value::Number(b)) => Ok(Value::Number(a / b)),
                    _ => Err(format!("عملية غير مدعومة بين {:?} و {:?}", l, r)),
                }
            }
            _ => Err("تعبير غير مدعوم".to_string()),
        }
    }
    fn is_truthy(&self, val: &Value) -> bool { !matches!(val, Value::Nil | Value::Boolean(false)) }
}
