use std::sync::{Arc, Mutex};
use super::environment::Environment;
use super::value::Value;
use crate::compiler::parser::{Expr, Stmt};
use crate::compiler::lexer::TokenKind;

pub struct Interpreter {
    environment: Arc<Mutex<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Arc::new(Mutex::new(Environment::new())),
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), String> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
            }
            Stmt::Print(expr) => {
                let value = self.evaluate(expr)?;
                println!("{}", value);
            }
            Stmt::Var { name, initializer } => {
                let value = match initializer {
                    Some(init) => self.evaluate(init)?,
                    None => Value::Nil,
                };
                self.environment.lock().unwrap().define(name.lexeme.clone(), value);
            }
            Stmt::Block(statements) => {
                let previous = self.environment.clone();
                let new_env = Environment::new_with_enclosing(previous.clone());
                self.environment = Arc::new(Mutex::new(new_env));
                
                let mut result = Ok(());
                for statement in statements {
                    if let Err(e) = self.execute(statement) {
                        result = Err(e);
                        break;
                    }
                }
                
                self.environment = previous;
                result?;
            }
            Stmt::If { condition, then_branch, else_branch } => {
                let cond_val = self.evaluate(condition)?;
                if self.is_truthy(&cond_val) {
                    self.execute(then_branch)?;
                } else if let Some(else_stmt) = else_branch {
                    self.execute(else_stmt)?;
                }
            }
            Stmt::While { condition, body } => {
                loop {
                    let cond_val = self.evaluate(condition)?;
                    if !self.is_truthy(&cond_val) {
                        break;
                    }
                    self.execute(body)?;
                }
            }
            Stmt::Function { name, params, body } => {
                let function = Value::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                };
                self.environment.lock().unwrap().define(name.lexeme.clone(), function);
            }
            Stmt::Return { value, .. } => {
                let return_val = match value {
                    Some(expr) => self.evaluate(expr)?,
                    None => Value::Nil,
                };
                // تغليف القيمة مباشرة داخل إشارة نصية موحدة لمنع أخطاء الترميز
                let encoded = match return_val {
                    Value::Number(n) => format!("NUM:{}", n),
                    Value::Boolean(b) => format!("BOOL:{}", b),
                    Value::String(s) => format!("STR:{}", s),
                    Value::Nil => "NIL:".to_string(),
                    _ => "NIL:".to_string(),
                };
                return Err(format!("__RETURN_SIGNAL__{}", encoded));
            }
        }
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::Literal(lexeme) => {
                if lexeme == "true" || lexeme == "صحيح" { return Ok(Value::Boolean(true)); }
                if lexeme == "false" || lexeme == "خطأ" { return Ok(Value::Boolean(false)); }
                if let Ok(n) = lexeme.parse::<f64>() { return Ok(Value::Number(n)); }
                let cleaned = lexeme.trim_matches('"').to_string();
                Ok(Value::String(cleaned))
            }
            Expr::Variable(name) => self.environment.lock().unwrap().get(&name.lexeme),
            Expr::Call { callee, arguments, .. } => {
                let callee_val = self.evaluate(callee)?;
                
                if let Value::Function { params, body, .. } = callee_val {
                    if arguments.len() != params.len() {
                        return Err(format!("خطأ: عدد الوسائط غير متطابق. المتوقع {} والممرر {}", params.len(), arguments.len()));
                    }

                    let mut evaluated_args = Vec::new();
                    for arg in arguments {
                        evaluated_args.push(self.evaluate(arg)?);
                    }

                    let previous = self.environment.clone();
                    let mut closure_env = Environment::new_with_enclosing(previous.clone());
                    
                    for (param, arg_val) in params.iter().zip(evaluated_args) {
                        closure_env.define(param.lexeme.clone(), arg_val);
                    }

                    self.environment = Arc::new(Mutex::new(closure_env));
                    
                    let mut return_value = Value::Nil;
                    for statement in &body {
                        if let Err(e) = self.execute(statement) {
                            if e.starts_with("__RETURN_SIGNAL__") {
                                let inner = e.trim_start_matches("__RETURN_SIGNAL__");
                                if inner.starts_with("NUM:") {
                                    let val = inner.trim_start_matches("NUM:").parse().unwrap_or(0.0);
                                    return_value = Value::Number(val);
                                } else if inner.starts_with("BOOL:") {
                                    let val = inner.trim_start_matches("BOOL:").parse().unwrap_or(false);
                                    return_value = Value::Boolean(val);
                                } else if inner.starts_with("STR:") {
                                    let val = inner.trim_start_matches("STR:").to_string();
                                    return_value = Value::String(val);
                                } else {
                                    return_value = Value::Nil;
                                }
                                break;
                            } else {
                                self.environment = previous;
                                return Err(e);
                            }
                        }
                    }

                    self.environment = previous;
                    return Ok(return_value);
                } else {
                    return Err("يمكن فقط استدعاء الدوال!".to_string());
                }
            }
            Expr::Binary { left, operator, right } => {
                if operator.kind == TokenKind::Equal {
                    if let Expr::Variable(ref name) = **left {
                        let val = self.evaluate(right)?;
                        self.environment.lock().unwrap().assign(&name.lexeme, val.clone())?;
                        return Ok(val);
                    }
                }

                let left_val = self.evaluate(left)?;
                let right_val = self.evaluate(right)?;

                match (&left_val, &operator.kind, &right_val) {
                    (Value::Number(l), TokenKind::Plus, Value::Number(r)) => Ok(Value::Number(l + r)),
                    (Value::Number(l), TokenKind::Minus, Value::Number(r)) => Ok(Value::Number(l - r)),
                    (Value::Number(l), TokenKind::Star, Value::Number(r)) => Ok(Value::Number(l * r)),
                    (Value::Number(l), TokenKind::Slash, Value::Number(r)) => {
                        if *r == 0.0 { return Err("خطأ: القسمة على صفر!".to_string()); }
                        Ok(Value::Number(l / r))
                    }
                    (Value::Number(l), TokenKind::Power, Value::Number(r)) => Ok(Value::Number(l.powf(*r))),
                    
                    (Value::Number(l), TokenKind::EqualEqual, Value::Number(r)) => Ok(Value::Boolean(l == r)),
                    (Value::Number(l), TokenKind::Greater, Value::Number(r)) => Ok(Value::Boolean(l > r)),
                    (Value::Number(l), TokenKind::Less, Value::Number(r)) => Ok(Value::Boolean(l < r)),
                    (Value::Number(l), TokenKind::GreaterEqual, Value::Number(r)) => Ok(Value::Boolean(l >= r)),
                    (Value::Number(l), TokenKind::LessEqual, Value::Number(r)) => Ok(Value::Boolean(l <= r)),

                    (Value::Boolean(l), TokenKind::AndAnd, Value::Boolean(r)) => Ok(Value::Boolean(*l && *r)),
                    (Value::Boolean(l), TokenKind::OrOr, Value::Boolean(r)) => Ok(Value::Boolean(*l || *r)),

                    _ => Err(format!("عملية غير مدعومة بين القيم في السطر {}", operator.line)),
                }
            }
            Expr::Grouping(inner) => self.evaluate(inner),
            _ => Err("تعبير برمي غير مدعوم حالياً".to_string()),
        }
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            _ => true,
        }
    }
}
