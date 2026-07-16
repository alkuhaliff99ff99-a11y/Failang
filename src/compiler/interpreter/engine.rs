use crate::compiler::parser::ast::{Expr, Stmt};
use crate::compiler::lexer::TokenKind;
use crate::compiler::interpreter::environment::Environment;
use crate::compiler::interpreter::value::Value;
use crate::diagnostics::error::translate;
use crate::diagnostics::reporter::report;

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
        let mut env = Environment::new();
        env.define("طول".to_string(), Value::Builtin("length".to_string()));
        env.define("نوع".to_string(), Value::Builtin("type".to_string()));
        env.define("أضف".to_string(), Value::Builtin("append".to_string()));
        env.define("أول".to_string(), Value::Builtin("first".to_string()));
        env.define("آخر".to_string(), Value::Builtin("last".to_string()));
        env.define("يحتوي".to_string(), Value::Builtin("contains".to_string()));
        env.define("قطع".to_string(), Value::Builtin("slice".to_string()));
        env.define("استبدل".to_string(), Value::Builtin("replace".to_string()));
        env.define("نص".to_string(), Value::Builtin("string".to_string()));
        env.define("رقم".to_string(), Value::Builtin("number".to_string()));
        Self { environment: env }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), String> {
        for stmt in statements {
            if let Err(cf) = self.execute(stmt) {
                match cf {
                    ControlFlow::Error(e) => {
                        let diagnostic = translate(&e);
                        report(&diagnostic);
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
                    _ => println!("{}", val),
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
                    .map_err(ControlFlow::Error)
            }
            Expr::IndexAssign { callee, index, value } => {
                let evaluated_val = self.evaluate(value)?;
                let evaluated_index = self.evaluate(index)?;

                let idx = match evaluated_index {
                    Value::Number(n) => n as usize,
                    _ => return Err(ControlFlow::Error("يجب أن يكون الفهرس رقماً صحيحاً.".to_string())),
                };

                if let Expr::Variable(name) = &**callee {
                    let mut arr_val = self.environment.get(&name.lexeme)
                        .map_err(ControlFlow::Error)?;

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
                            .map_err(ControlFlow::Error)?;
                        Ok(evaluated_val)
                    } else {
                        Err(ControlFlow::Error("لا يمكن تعديل فهرس لمتغير ليس مصفوفة.".to_string()))
                    }
                } else {
                    Err(ControlFlow::Error("الهدف المحدد للتعديل غير صالح.".to_string()))
                }
            },
            Expr::Assign { name, value } => {
                let val = self.evaluate(value)?;
                self.environment.assign(&name.lexeme, val.clone())
                    .map_err(ControlFlow::Error)?;
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

                if operator.kind == TokenKind::EqualEqual || operator.lexeme == "يساوي" {
                    return Ok(Value::Boolean(l == r));
                }
                if operator.kind == TokenKind::BangEqual {
                    return Ok(Value::Boolean(l != r));
                }

                match (l, r) {
                    (Value::Number(lv), Value::Number(rv)) => {
                        match operator.kind {
                            TokenKind::Less => Ok(Value::Boolean(lv < rv)),
                            TokenKind::LessEqual => Ok(Value::Boolean(lv <= rv)),
                            TokenKind::Greater => Ok(Value::Boolean(lv > rv)),
                            TokenKind::GreaterEqual => Ok(Value::Boolean(lv >= rv)),
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
                    (Value::Boolean(lb), Value::Boolean(rb)) => {
                        match operator.kind {
                            TokenKind::AndAnd => Ok(Value::Boolean(lb && rb)),
                            TokenKind::OrOr => Ok(Value::Boolean(lb || rb)),
                            _ => Err(ControlFlow::Error(format!("مشغل غير مدعوم: {:?}", operator.kind))),
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
                    Value::Builtin(name) => {
                        match name.as_str() {
                            "length" => {
                                if evaluated_args.len() != 1 {
                                    return Err(ControlFlow::Error("دالة طول تحتاج إلى وسيط واحد.".to_string()));
                                }
                                match &evaluated_args[0] {
                                    Value::Array(items) => Ok(Value::Number(items.len() as f64)),
                                    Value::String(text) => Ok(Value::Number(text.chars().count() as f64)),
                                    _ => Err(ControlFlow::Error("دالة طول تستقبل مصفوفة أو نصاً فقط.".to_string())),
                                }
                            }
                            "type" => {
                                if evaluated_args.len() != 1 {
                                    return Err(ControlFlow::Error("دالة نوع تحتاج إلى وسيط واحد.".to_string()));
                                }
                                match &evaluated_args[0] {
                                    Value::Number(_) => Ok(Value::String("رقم".to_string())),
                                    Value::String(_) => Ok(Value::String("نص".to_string())),
                                    Value::Boolean(_) => Ok(Value::String("منطقي".to_string())),
                                    Value::Array(_) => Ok(Value::String("مصفوفة".to_string())),
                                    Value::Function { .. } => Ok(Value::String("دالة".to_string())),
                                    Value::Builtin(_) => Ok(Value::String("دالة_مدمجة".to_string())),
                                    Value::Nil => Ok(Value::String("عدم".to_string())),
                                }
                            }
                            "string" => {
                                if evaluated_args.len() != 1 {
                                    return Err(ControlFlow::Error("دالة نص تحتاج إلى وسيط واحد.".to_string()));
                                }
                                Ok(Value::String(evaluated_args[0].to_string()))
                            }
                            "number" => {
                                if evaluated_args.len() != 1 {
                                    return Err(ControlFlow::Error("دالة رقم تحتاج إلى وسيط واحد.".to_string()));
                                }
                                match &evaluated_args[0] {
                                    Value::Number(n) => Ok(Value::Number(*n)),
                                    Value::String(s) => {
                                        if let Ok(n) = s.trim().parse::<f64>() {
                                            Ok(Value::Number(n))
                                        } else {
                                            Err(ControlFlow::Error(format!("تعذر تحويل النص '{}' إلى رقم.", s)))
                                        }
                                    }
                                    _ => Err(ControlFlow::Error("دالة رقم تستقبل رقماً أو نصاً قابلاً للتحويل فقط.".to_string())),
                                }
                            }
                            _ => Err(ControlFlow::Error(format!("دالة مدمجة غير معرفة: {}", name))),
                        }
                    }
                    Value::Function { params, body, name: _ } => {
                        if evaluated_args.len() != params.len() {
                            return Err(ControlFlow::Error(format!(
                                "خطأ: عدد المعاملات غير متطابق. المتوقع: {}، الممرر: {}.",
                                params.len(),
                                evaluated_args.len()
                            )));
                        }
                        
                        let mut local_env = Environment::new_with_enclosing(std::sync::Arc::new(std::sync::Mutex::new(self.environment.clone())));
                        for (param, arg) in params.iter().zip(evaluated_args.iter()) {
                            local_env.define(param.lexeme.clone(), arg.clone());
                        }
                        let previous_env = self.environment.clone();
                        self.environment = local_env;
                        
                        let result = self.execute(&Stmt::Block(body.clone()));
                        self.environment = previous_env;
                        match result {
                            Ok(_) => Ok(Value::Nil),
                            Err(ControlFlow::Return(val)) => Ok(val),
                            Err(e) => Err(e),
                        }
                    }
                    _ => Err(ControlFlow::Error("لا يمكن استدعاء كائن غير قابل للاستدعاء.".to_string())),
                }
            }
        }
    }

    fn is_truthy(&self, val: &Value) -> bool {
        match val {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0,
            Value::String(s) => !s.is_empty(),
            Value::Array(items) => !items.is_empty(),
            _ => true,
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
