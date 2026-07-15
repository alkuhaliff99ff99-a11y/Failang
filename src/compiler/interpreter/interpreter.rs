use crate::compiler::parser::ast::{Expr, Stmt};
use crate::compiler::lexer::{Token, TokenKind};
use crate::compiler::interpreter::environment::Environment;
use crate::compiler::interpreter::value::Value;

// هيكل للتحكم في تدفق البرنامج عند مواجهة دالة الإرجاع "return"
#[derive(Debug, Clone)]
pub enum ControlFlow {
    Return(Value),
    Error(String),
}

pub struct Interpreter {
    pub environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self { environment: Environment::new() }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), String> {
        for stmt in statements {
            if let Err(cf) = self.execute(stmt) {
                match cf {
                    ControlFlow::Error(e) => {
                        println!("❌ خطأ أثناء التنفيذ: {}", e);
                        return Err(e);
                    }
                    ControlFlow::Return(_) => {
                        println!("❌ خطأ أثناء التنفيذ: تم استخدام 'ارجع' خارج نطاق الدالة.");
                        return Err("ارجع خارج دالة".to_string());
                    }
                }
            }
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), ControlFlow> {
        match stmt {
            Stmt::Expression(expr) => { self.evaluate(expr)?; }
            Stmt::Print(expr) => {
                let val = self.evaluate(expr)?;
                match val {
                    Value::Number(n) => println!("{}", n),
                    Value::String(s) => println!("{}", s),
                    Value::Boolean(b) => println!("{}", if b { "صواب" } else { "خطأ" }),
                    Value::Nil => println!("عدم"),
                    _ => println!("{}", val), // استخدام طباعة Display المخصصة التي صممناها سابقاً
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
            Stmt::If { condition, then_branch, else_branch } => {
                let cond_val = self.evaluate(condition)?;
                if self.is_truthy(&cond_val) {
                    self.execute(then_branch)?;
                } else if let Some(else_stmt) = else_branch {
                    self.execute(else_stmt)?;
                }
            }
            Stmt::Block(statements) => {
                for statement in statements {
                    self.execute(statement)?;
                }
            }
            Stmt::Function { name, params, body } => {
                let function = Value::Function {
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                };
                self.environment.define(name.lexeme.clone(), function);
            }
            Stmt::Return { keyword: _, value } => {
                let mut return_val = Value::Nil;
                if let Some(expr) = value {
                    return_val = self.evaluate(expr)?;
                }
                return Err(ControlFlow::Return(return_val));
            }
        }
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, ControlFlow> {
        match expr {
            Expr::Literal(val) => {
                if let Ok(n) = val.trim().parse::<f64>() {
                    return Ok(Value::Number(n));
                }
                if val == "true" || val == "صواب" { return Ok(Value::Boolean(true)); }
                if val == "false" || val == "خطأ" { return Ok(Value::Boolean(false)); }
                if val == "nil" || val == "عدم" { return Ok(Value::Nil); }
                
                let cleaned = if val.starts_with('"') && val.ends_with('"') {
                    val[1..val.len()-1].to_string()
                } else {
                    val.clone()
                };
                Ok(Value::String(cleaned))
            }
            Expr::Variable(name) => {
                self.environment.get(&name.lexeme)
                    .map_err(|e| ControlFlow::Error(e))
            }
            Expr::IndexAssign { callee, index, value } => {
                let evaluated_val = self.evaluate(&value)?;
                let evaluated_index = self.evaluate(index)?;
                
                let idx = match evaluated_index {
                    Value::Number(n) => n as usize,
                    _ => return Err(ControlFlow::Error("يجب أن يكون الفهرس رقماً صحيحاً.".to_string())),
                };

                if let Expr::Variable(name) = &**callee {
                    let mut arr_val = self.environment.get(&name.lexeme)
                        .map_err(|e| ControlFlow::Error(e))?;

                    if let Value::Array(ref mut elements) = arr_val {
                        if idx >= elements.len() {
                            return Err(ControlFlow::Error(format!(
                                "خارج حدود المصفوفة: طول المصفوفة هو {} والفهرس المطلوب هو {}.",
                                elements.len(),
                                idx
                            )));
                        }
                        elements[idx] = evaluated_val.clone();
                        self.environment.assign(&name.lexeme, Value::Array(elements.clone()))
                            .map_err(|e| ControlFlow::Error(e))?;
                        Ok(evaluated_val)
                    } else {
                        Err(ControlFlow::Error("لا يمكن تعديل فهرس لمتغير ليس مصفوفة.".to_string()))
                    }
                } else {
                    Err(ControlFlow::Error("الهدف المحدد للتعديل غير صالح.".to_string()))
                }
            },
            &Expr::Assign { ref name, ref value } => {
                let val = self.evaluate(&value)?;
                self.environment.assign(&name.lexeme, val.clone())
                    .map_err(|e| ControlFlow::Error(e))?;
                Ok(val)
            }
            Expr::Unary { operator, right } => {
                let right_val = self.evaluate(right)?;
                match operator.kind {
                    TokenKind::Minus => {
                        if let Value::Number(n) = right_val {
                            Ok(Value::Number(-n))
                        } else {
                            Err(ControlFlow::Error("خطأ: لا يمكن استخدام علامة السالب إلا مع الأرقام.".to_string()))
                        }
                    }
                    TokenKind::Bang => {
                        Ok(Value::Boolean(!self.is_truthy(&right_val)))
                    }
                    _ => Err(ControlFlow::Error("عملية أحادية غير مدعومة.".to_string()))
                }
            }
            Expr::Binary { left, operator, right } => {
                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;
                match (l, r) {
                    (Value::Number(lv), Value::Number(rv)) => {
                        match operator.kind {
                            TokenKind::Less => Ok(Value::Boolean(lv < rv)),
                            TokenKind::Greater => Ok(Value::Boolean(lv > rv)),
                            TokenKind::Plus => Ok(Value::Number(lv + rv)),
                            TokenKind::Minus => Ok(Value::Number(lv - rv)),
                            TokenKind::Star => Ok(Value::Number(lv * rv)),
                            TokenKind::Slash => {
                                if rv == 0.0 { 
                                    return Err(ControlFlow::Error("خطأ: لا يمكن القسمة على صفر".to_string())); 
                                }
                                Ok(Value::Number(lv / rv))
                            }
                            _ => Err(ControlFlow::Error(format!("مشغل غير مدعوم: {:?}", operator.kind))),
                        }
                    }
                    (Value::String(mut lv), Value::String(rv)) => {
                        if operator.kind == TokenKind::Plus || operator.lexeme == "+" {
                            lv.push_str(&rv);
                            Ok(Value::String(lv))
                        } else {
                            Err(ControlFlow::Error("العملية الوحيدة المتاحة للنصوص هي الجمع (+)".to_string()))
                        }
                    }
                    (Value::String(mut lv), Value::Number(rv)) => {
                        if operator.kind == TokenKind::Plus || operator.lexeme == "+" {
                            lv.push_str(&rv.to_string());
                            Ok(Value::String(lv))
                        } else {
                            Err(ControlFlow::Error("العملية الوحيدة المتاحة للنصوص هي الجمع (+)".to_string()))
                        }
                    }
                    (left_val, right_val) => Err(ControlFlow::Error(format!("خطأ حسابي: لا يمكن إجراء عملية بين {:?} و {:?}", left_val, right_val))),
                }
            }
            Expr::Grouping(inner) => self.evaluate(inner),
            Expr::Array { bracket: _, elements } => {
                let mut evaluated_elements = Vec::new();
                for el in elements {
                    evaluated_elements.push(self.evaluate(el)?);
                }
                Ok(Value::Array(evaluated_elements))
            }
            Expr::Index { callee, bracket: _, index } => {
                let array_val = self.evaluate(callee)?;
                let index_val = self.evaluate(index)?;

                match (array_val, index_val) {
                    (Value::Array(elements), Value::Number(idx)) => {
                        if idx < 0.0 || idx.fract() != 0.0 {
                            return Err(ControlFlow::Error("خطأ: يجب أن يكون مؤشر المصفوفة رقماً صحيحاً موجباً.".to_string()));
                        }
                        let u_idx = idx as usize;
                        if u_idx >= elements.len() {
                            return Err(ControlFlow::Error(format!(
                                "خطأ: تجاوز حدود المصفوفة. الطول هو {} ولكن تم طلب العنصر ذو المؤشر {}.",
                                elements.len(),
                                u_idx
                            )));
                        }
                        Ok(elements[u_idx].clone())
                    }
                    (Value::Array(_), _) => {
                        Err(ControlFlow::Error("خطأ: يجب أن يكون مؤشر المصفوفة رقماً.".to_string()))
                    }
                    _ => Err(ControlFlow::Error("خطأ: لا يمكن إجراء عملية الفهرسة على كائن ليس مصفوفة.".to_string())),
                }
            }
            Expr::Call { callee, paren: _, arguments } => {
                let callee_val = self.evaluate(callee)?;
                
                let mut evaluated_args = Vec::new();
                for arg in arguments {
                    evaluated_args.push(self.evaluate(arg)?);
                }

                match callee_val {
                    Value::Function { name: _, params, body } => {
                        if evaluated_args.len() != params.len() {
                            return Err(ControlFlow::Error(format!(
                                "خطأ في تمرير المعاملات: المتوقع {} وسيطاً، ولكن تم تمرير {}.",
                                params.len(),
                                evaluated_args.len()
                            )));
                        }

                        let previous_env = self.environment.clone();
                        let mut local_env = Environment::new_with_enclosing(std::sync::Arc::new(std::sync::Mutex::new(previous_env.clone())));
                        for (param, arg_val) in params.iter().zip(evaluated_args.iter()) {
                            local_env.define(param.lexeme.clone(), arg_val.clone());
                        }

                        self.environment = local_env;

                        let mut return_value = Value::Nil;
                        for stmt in &body {
                            if let Err(cf) = self.execute(stmt) {
                                match cf {
                                    ControlFlow::Return(val) => {
                                        return_value = val;
                                        break;
                                    }
                                    ControlFlow::Error(e) => {
                                        self.environment = previous_env;
                                        return Err(ControlFlow::Error(e));
                                    }
                                }
                            }
                        }

                        self.environment = previous_env;
                        Ok(return_value)
                    }
                    _ => Err(ControlFlow::Error("لا يمكن استدعاء هذا الكائن كدالة.".to_string())),
                }
            }
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
