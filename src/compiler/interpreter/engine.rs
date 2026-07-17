use crate::compiler::interpreter::environment::Environment;
use crate::compiler::interpreter::value::Value;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::ast::{Expr, Stmt};
use crate::diagnostics::error::translate;
use crate::diagnostics::error::DiagnosticError;
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
        env.define("len".to_string(), Value::Builtin("length".to_string()));
        env.define("نوع".to_string(), Value::Builtin("type".to_string()));
        env.define("type".to_string(), Value::Builtin("type".to_string()));
        env.define("نص".to_string(), Value::Builtin("string".to_string()));
        env.define("str".to_string(), Value::Builtin("string".to_string()));
        env.define("رقم".to_string(), Value::Builtin("number".to_string()));
        env.define("number".to_string(), Value::Builtin("number".to_string()));
        env.define("أضف".to_string(), Value::Builtin("append".to_string()));
        env.define("append".to_string(), Value::Builtin("append".to_string()));
        env.define("أول".to_string(), Value::Builtin("first".to_string()));
        env.define("first".to_string(), Value::Builtin("first".to_string()));
        env.define("آخر".to_string(), Value::Builtin("last".to_string()));
        env.define("last".to_string(), Value::Builtin("last".to_string()));
        env.define("يحتوي".to_string(), Value::Builtin("contains".to_string()));
        env.define("contains".to_string(), Value::Builtin("contains".to_string()));
        env.define("استبدل".to_string(), Value::Builtin("replace".to_string()));
        env.define("replace".to_string(), Value::Builtin("replace".to_string()));
        env.define("قطع".to_string(), Value::Builtin("slice".to_string()));
        env.define("slice".to_string(), Value::Builtin("slice".to_string()));
        env.define("تقسيم".to_string(), Value::Builtin("split".to_string()));
        env.define("split".to_string(), Value::Builtin("split".to_string()));
        env.define("دمج".to_string(), Value::Builtin("concat".to_string()));
        env.define("concat".to_string(), Value::Builtin("concat".to_string()));
        env.define("قص_فراغات".to_string(), Value::Builtin("trim".to_string()));
        env.define("trim".to_string(), Value::Builtin("trim".to_string()));
        env.define("يبدأ_بـ".to_string(), Value::Builtin("starts_with".to_string()));
        env.define("starts_with".to_string(), Value::Builtin("starts_with".to_string()));
        env.define("ينتهي_بـ".to_string(), Value::Builtin("ends_with".to_string()));
        env.define("ends_with".to_string(), Value::Builtin("ends_with".to_string()));
        env.define("حرف_عند".to_string(), Value::Builtin("char_at".to_string()));
        env.define("char_at".to_string(), Value::Builtin("char_at".to_string()));
        env.define("موضع".to_string(), Value::Builtin("index_of".to_string()));
        env.define("index_of".to_string(), Value::Builtin("index_of".to_string()));
        env.define("احذف".to_string(), Value::Builtin("pop".to_string()));
        env.define("pop".to_string(), Value::Builtin("pop".to_string()));
        env.define("عكس".to_string(), Value::Builtin("reverse".to_string()));
        env.define("reverse".to_string(), Value::Builtin("reverse".to_string()));
        env.define("فارغ".to_string(), Value::Builtin("is_empty".to_string()));
        env.define("is_empty".to_string(), Value::Builtin("is_empty".to_string()));
        env.define("مطلق".to_string(), Value::Builtin("abs".to_string()));
        env.define("abs".to_string(), Value::Builtin("abs".to_string()));
        env.define("جذر".to_string(), Value::Builtin("sqrt".to_string()));
        env.define("sqrt".to_string(), Value::Builtin("sqrt".to_string()));
        env.define("قوة".to_string(), Value::Builtin("pow".to_string()));
        env.define("pow".to_string(), Value::Builtin("pow".to_string()));
        env.define("تقريب".to_string(), Value::Builtin("round".to_string()));
        env.define("round".to_string(), Value::Builtin("round".to_string()));
        env.define("أكبر_قيمة".to_string(), Value::Builtin("max".to_string()));
        env.define("max".to_string(), Value::Builtin("max".to_string()));
        env.define("أصغر_قيمة".to_string(), Value::Builtin("min".to_string()));
        env.define("min".to_string(), Value::Builtin("min".to_string()));
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
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
            }
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
            Stmt::While { condition, body } => loop {
                let cond_val = self.evaluate(condition)?;
                if !self.is_truthy(&cond_val) {
                    break;
                }
                self.execute(body)?;
            },
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
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
                if val == "true" || val == "صواب" {
                    return Ok(Value::Boolean(true));
                }
                if val == "false" || val == "خطأ" {
                    return Ok(Value::Boolean(false));
                }
                if val == "nil" || val == "عدم" {
                    return Ok(Value::Nil);
                }

                let cleaned = if val.starts_with('"') && val.ends_with('"') {
                    val[1..val.len() - 1].to_string()
                } else {
                    val.clone()
                };
                Ok(Value::String(cleaned))
            }
            Expr::Variable(name) => self
                .environment
                .get(&name.lexeme)
                .ok_or_else(|| ControlFlow::Error(format!("المتغير '{}' غير معرف.", name.lexeme))),
            Expr::IndexAssign {
                callee,
                index,
                value,
            } => {
                let evaluated_val = self.evaluate(value)?;
                let evaluated_index = self.evaluate(index)?;

                let idx = match evaluated_index {
                    Value::Number(n) => n as usize,
                    _ => {
                        return Err(ControlFlow::Error(
                            DiagnosticError::new(
                                "يجب أن يكون الفهرس رقماً صحيحاً.",
                                "Index must be an integer.",
                            )
                            .display(),
                        ))
                    }
                };

                if let Expr::Variable(name) = &**callee {
                    let mut arr_val = self.environment.get(&name.lexeme).ok_or_else(|| {
                        ControlFlow::Error(format!("المتغير '{}' غير معرف.", name.lexeme))
                    })?;

                    if let Value::Array(ref mut elements) = arr_val {
                        if idx >= elements.len() {
                            return Err(ControlFlow::Error(DiagnosticError::new(&format!("خارج حدود المصفوفة: طول المصفوفة هو {} والفهرس المطلوب هو {}.", elements.len(), idx), &format!("Index out of bounds: array length is {} and requested index is {}.", elements.len(), idx)).display()));
                        }
                        elements[idx] = evaluated_val.clone();
                        self.environment
                            .assign(&name.lexeme, Value::Array(elements.clone()))
                            .map_err(ControlFlow::Error)?;
                        Ok(evaluated_val)
                    } else {
                        Err(ControlFlow::Error(
                            DiagnosticError::new(
                                "لا يمكن تعديل فهرس لمتغير ليس مصفوفة.",
                                "Cannot assign an index to a non-array variable.",
                            )
                            .display(),
                        ))
                    }
                } else {
                    Err(ControlFlow::Error(
                        DiagnosticError::new(
                            "الهدف المحدد للتعديل غير صالح.",
                            "Invalid assignment target.",
                        )
                        .display(),
                    ))
                }
            }
            Expr::Assign { name, value } => {
                let val = self.evaluate(value)?;
                self.environment
                    .assign(&name.lexeme, val.clone())
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
                            Err(ControlFlow::Error(
                                DiagnosticError::new(
                                    "خطأ: لا يمكن استخدام علامة السالب إلا مع الأرقام.",
                                    "Error: Minus operator can only be used with numbers.",
                                )
                                .display(),
                            ))
                        }
                    }
                    TokenKind::Bang => Ok(Value::Boolean(!self.is_truthy(&right_val))),
                    _ => Err(ControlFlow::Error("عملية أحادية غير مدعومة.".to_string())),
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let l = self.evaluate(left)?;
                let r = self.evaluate(right)?;

                if operator.kind == TokenKind::EqualEqual || operator.lexeme == "يساوي" {
                    return Ok(Value::Boolean(l == r));
                }
                if operator.kind == TokenKind::BangEqual {
                    return Ok(Value::Boolean(l != r));
                }

                match (l, r) {
                    (Value::Number(lv), Value::Number(rv)) => match operator.kind {
                        TokenKind::Less => Ok(Value::Boolean(lv < rv)),
                        TokenKind::LessEqual => Ok(Value::Boolean(lv <= rv)),
                        TokenKind::Greater => Ok(Value::Boolean(lv > rv)),
                        TokenKind::GreaterEqual => Ok(Value::Boolean(lv >= rv)),
                        TokenKind::Plus => Ok(Value::Number(lv + rv)),
                        TokenKind::Minus => Ok(Value::Number(lv - rv)),
                        TokenKind::Star => Ok(Value::Number(lv * rv)),
                        TokenKind::Slash => {
                            if rv == 0.0 {
                                return Err(ControlFlow::Error(
                                    DiagnosticError::new(
                                        "خطأ: لا يمكن القسمة على صفر.",
                                        "Error: Cannot divide by zero.",
                                    )
                                    .display(),
                                ));
                            }
                            Ok(Value::Number(lv / rv))
                        }
                        _ => Err(ControlFlow::Error(format!(
                            "مشغل غير مدعوم: {:?}",
                            operator.kind
                        ))),
                    },
                    (Value::String(mut lv), Value::String(rv)) => {
                        if operator.kind == TokenKind::Plus || operator.lexeme == "+" {
                            lv.push_str(&rv);
                            Ok(Value::String(lv))
                        } else {
                            Err(ControlFlow::Error(
                                DiagnosticError::new(
                                    "العملية الوحيدة المتاحة للنصوص هي الجمع (+)",
                                    "The only available operation for strings is addition (+).",
                                )
                                .display(),
                            ))
                        }
                    }
                    (Value::String(mut lv), Value::Number(rv)) => {
                        if operator.kind == TokenKind::Plus || operator.lexeme == "+" {
                            lv.push_str(&rv.to_string());
                            Ok(Value::String(lv))
                        } else {
                            Err(ControlFlow::Error(
                                DiagnosticError::new(
                                    "العملية الوحيدة المتاحة للنصوص هي الجمع (+)",
                                    "The only available operation for strings is addition (+).",
                                )
                                .display(),
                            ))
                        }
                    }
                    (Value::Boolean(lb), Value::Boolean(rb)) => match operator.kind {
                        TokenKind::AndAnd => Ok(Value::Boolean(lb && rb)),
                        TokenKind::OrOr => Ok(Value::Boolean(lb || rb)),
                        _ => Err(ControlFlow::Error(format!(
                            "مشغل غير مدعوم: {:?}",
                            operator.kind
                        ))),
                    },
                    (left_val, right_val) => {
                        let left_type = left_val.type_of();
                        let right_type = right_val.type_of();
                        Err(ControlFlow::Error(DiagnosticError::new(
                            &format!("خطأ حسابي: لا يمكن إجراء العملية بين نوع ({}) ونوع ({}).", left_type, right_type),
                            &format!("Arithmetic error: Cannot perform operation between ({}) and ({}).", left_type, right_type)
                        ).display()))
                    }
                }
            }
            Expr::Grouping(inner) => self.evaluate(inner),
            Expr::Array {
                bracket: _,
                elements,
            } => {
                let mut evaluated_elements = Vec::new();
                for el in elements {
                    evaluated_elements.push(self.evaluate(el)?);
                }
                Ok(Value::Array(evaluated_elements))
            }
            Expr::Index {
                callee,
                bracket: _,
                index,
            } => {
                let array_val = self.evaluate(callee)?;
                let index_val = self.evaluate(index)?;

                match (array_val, index_val) {
                    (Value::Array(elements), Value::Number(idx)) => {
                        if idx < 0.0 || idx.fract() != 0.0 {
                            return Err(ControlFlow::Error(
                                DiagnosticError::new(
                                    "خطأ: يجب أن يكون مؤشر المصفوفة رقماً صحيحاً موجباً.",
                                    "Error: Array index must be a positive integer.",
                                )
                                .display(),
                            ));
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
                    (Value::Array(_), _) => Err(ControlFlow::Error(
                        DiagnosticError::new(
                            "خطأ: يجب أن يكون مؤشر المصفوفة رقماً.",
                            "Error: Array index must be a number.",
                        )
                        .display(),
                    )),
                    _ => Err(ControlFlow::Error(
                        DiagnosticError::new(
                            "خطأ: لا يمكن إجراء عملية الفهرسة على كائن ليس مصفوفة.",
                            "Error: Cannot index a non-array object.",
                        )
                        .display(),
                    )),
                }
            }
            Expr::Call {
                callee,
                paren: _,
                arguments,
            } => {
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
                                    return Err(ControlFlow::Error(
                                        DiagnosticError::new(
                                            "دالة طول تحتاج إلى وسيط واحد.",
                                            "len() function requires exactly one argument.",
                                        )
                                        .display(),
                                    ));
                                }
                                match &evaluated_args[0] {
                                    Value::Array(items) => Ok(Value::Number(items.len() as f64)),
                                    Value::String(text) => {
                                        Ok(Value::Number(text.chars().count() as f64))
                                    }
                                    _ => Err(ControlFlow::Error(
                                        DiagnosticError::new(
                                            "دالة طول تستقبل مصفوفة أو نصاً فقط.",
                                            "len() function only accepts arrays or strings.",
                                        )
                                        .display(),
                                    )),
                                }
                            }
                            "append" => {
                                if evaluated_args.len() != 2 {
                                    return Err(ControlFlow::Error("append يحتاج مصفوفة وعنصر".to_string()));
                                }
                                match &evaluated_args[0] {
                                    Value::Array(items) => {
                                        let mut result = items.clone();
                                        result.push(evaluated_args[1].clone());
                                        Ok(Value::Array(result))
                                    }
                                    _ => Err(ControlFlow::Error("append يعمل مع المصفوفات فقط".to_string()))
                                }
                            }
                            "first" => {
                                match &evaluated_args[0] {
                                    Value::Array(items) => Ok(items.first().cloned().unwrap_or(Value::Nil)),
                                    _ => Err(ControlFlow::Error("first يعمل مع المصفوفات فقط".to_string()))
                                }
                            }
                            "last" => {
                                match &evaluated_args[0] {
                                    Value::Array(items) => Ok(items.last().cloned().unwrap_or(Value::Nil)),
                                    _ => Err(ControlFlow::Error("last يعمل مع المصفوفات فقط".to_string()))
                                }
                            }
                            "contains" => {
                                if evaluated_args.len() != 2 {
                                    return Err(ControlFlow::Error("contains يحتاج قيمتين".to_string()));
                                }
                                match &evaluated_args[0] {
                                    Value::Array(items) => Ok(Value::Boolean(items.contains(&evaluated_args[1]))),
                                    Value::String(text) => {
                                        if let Value::String(find) = &evaluated_args[1] {
                                            Ok(Value::Boolean(text.contains(find)))
                                        } else {
                                            Ok(Value::Boolean(false))
                                        }
                                    }
                                    _ => Ok(Value::Boolean(false))
                                }
                            }
                            "type" => {
                                if evaluated_args.len() != 1 {
                                    return Err(ControlFlow::Error(
                                        DiagnosticError::new(
                                            "دالة نوع تحتاج إلى وسيط واحد.",
                                            "type() function requires exactly one argument.",
                                        )
                                        .display(),
                                    ));
                                }
                                match &evaluated_args[0] {
                                    Value::Number(_) => Ok(Value::String("رقم".to_string())),
                                    Value::String(_) => Ok(Value::String("نص".to_string())),
                                    Value::Boolean(_) => Ok(Value::String("منطقي".to_string())),
                                    Value::Array(_) => Ok(Value::String("مصفوفة".to_string())),
                                    Value::Function { .. } => Ok(Value::String("دالة".to_string())),
                                    Value::Builtin(_) => {
                                        Ok(Value::String("دالة_مدمجة".to_string()))
                                    }
                                    Value::Nil => Ok(Value::String("عدم".to_string())),
                                }
                            }
                            "string" => {
                                if evaluated_args.len() != 1 {
                                    return Err(ControlFlow::Error(
                                        DiagnosticError::new(
                                            "دالة نص تحتاج إلى وسيط واحد.",
                                            "str() function requires exactly one argument.",
                                        )
                                        .display(),
                                    ));
                                }
                                Ok(Value::String(evaluated_args[0].to_string()))
                            }
                            "number" => {
                                if evaluated_args.len() != 1 {
                                    return Err(ControlFlow::Error(
                                        DiagnosticError::new(
                                            "دالة رقم تحتاج إلى وسيط واحد.",
                                            "number() function requires exactly one argument.",
                                        )
                                        .display(),
                                    ));
                                }
                                match &evaluated_args[0] {
                                    Value::Number(n) => Ok(Value::Number(*n)),
                                    Value::Boolean(b) => {
                                        Ok(Value::Number(if *b { 1.0 } else { 0.0 }))
                                    }
                                    Value::Nil => Ok(Value::Number(0.0)),
                                    Value::String(s) => {
                                        if let Ok(n) = s.trim().parse::<f64>() {
                                            Ok(Value::Number(n))
                                        } else {
                                            Err(ControlFlow::Error(DiagnosticError::new(
                                        &format!("تعذر تحويل النص \"{}\" إلى رقم.", s),
                                        &format!("Could not convert string \"{}\" to number.", s)
                                    ).display()))
                                        }
                                    }
                                    other => {
                                        let t = other.type_of();
                                        Err(ControlFlow::Error(
                                            DiagnosticError::new(
                                                &format!("لا يمكن تحويل نوع ({}) إلى رقم.", t),
                                                &format!("Cannot convert type ({}) to number.", t),
                                            )
                                            .display(),
                                        ))
                                    }
                                }
                            }
                            "abs" => {
                                match evaluated_args.as_slice() {
                                    [Value::Number(n)] => Ok(Value::Number(n.abs())),
                                    _ => Err(ControlFlow::Error("مطلق يحتاج رقماً واحداً.".to_string())),
                                }
                            }
                            "max" => {
                                match evaluated_args.as_slice() {
                                    [Value::Number(a), Value::Number(b)] => Ok(Value::Number(a.max(*b))),
                                    _ => Err(ControlFlow::Error("أكبر يحتاج رقمين.".to_string())),
                                }
                            }
                            "min" => {
                                match evaluated_args.as_slice() {
                                    [Value::Number(a), Value::Number(b)] => Ok(Value::Number(a.min(*b))),
                                    _ => Err(ControlFlow::Error("أصغر يحتاج رقمين.".to_string())),
                                }
                            }
                            "sqrt" => {
                                match evaluated_args.as_slice() {
                                    [Value::Number(n)] => Ok(Value::Number(n.sqrt())),
                                    _ => Err(ControlFlow::Error("جذر يحتاج رقماً واحداً.".to_string())),
                                }
                            }
                            "round" => {
                                Ok(Value::Number(evaluated_args[0].to_string().parse::<f64>().unwrap().round()))
                            }
                            "pow" => {
                                match evaluated_args.as_slice() {
                                    [Value::Number(a), Value::Number(b)] => Ok(Value::Number(a.powf(*b))),
                                    _ => Err(ControlFlow::Error("قوة تحتاج رقمين.".to_string())),
                                }
                            }
                            "pop" => {
                                if evaluated_args.len() != 1 {
                                    return Err(ControlFlow::Error("pop يحتاج مصفوفة واحدة".to_string()));
                                }

                                match &evaluated_args[0] {
                                    Value::Array(items) => {
                                        match items.last() {
                                            Some(v) => Ok(v.clone()),
                                            None => Ok(Value::Nil)
                                        }
                                    }
                                    _ => Err(ControlFlow::Error("pop يعمل مع المصفوفات فقط".to_string()))
                                }
                            }
                            "reverse" => {
                                if evaluated_args.len() != 1 {
                                    return Err(ControlFlow::Error("reverse يحتاج مصفوفة واحدة".to_string()));
                                }

                                match &evaluated_args[0] {
                                    Value::Array(items) => {
                                        let mut result = items.clone();
                                        result.reverse();
                                        Ok(Value::Array(result))
                                    }
                                    _ => Err(ControlFlow::Error("reverse يعمل مع المصفوفات فقط".to_string()))
                                }
                            }
                            "is_empty" => {
                                if evaluated_args.len() != 1 {
                                    return Err(ControlFlow::Error("is_empty يحتاج قيمة واحدة".to_string()));
                                }

                                match &evaluated_args[0] {
                                    Value::Array(items) => Ok(Value::Boolean(items.is_empty())),
                                    Value::String(text) => Ok(Value::Boolean(text.is_empty())),
                                    _ => Err(ControlFlow::Error("is_empty يعمل مع النصوص والمصفوفات فقط".to_string()))
                                }
                            }
                            "char_at" => {
                                if evaluated_args.len() != 2 {
                                    return Err(ControlFlow::Error("char_at يحتاج نصاً ورقماً".to_string()));
                                }

                                match (&evaluated_args[0], &evaluated_args[1]) {
                                    (Value::String(text), Value::Number(index)) => {
                                        match text.chars().nth(*index as usize) {
                                            Some(c) => Ok(Value::String(c.to_string())),
                                            None => Err(ControlFlow::Error("الموضع خارج حدود النص".to_string()))
                                        }
                                    }
                                    _ => Err(ControlFlow::Error("char_at يعمل مع نص ورقم فقط".to_string()))
                                }
                            }
                            "index_of" => {
                                if evaluated_args.len() != 2 {
                                    return Err(ControlFlow::Error("index_of يحتاج نصين".to_string()));
                                }

                                match (&evaluated_args[0], &evaluated_args[1]) {
                                    (Value::String(text), Value::String(search)) => {
                                        match text.find(search) {
                                            Some(i) => Ok(Value::Number(i as f64)),
                                            None => Ok(Value::Number(-1.0))
                                        }
                                    }
                                    _ => Err(ControlFlow::Error("index_of يعمل مع النصوص فقط".to_string()))
                                }
                            }
                            "starts_with" | "ends_with" => {
                                if evaluated_args.len() != 2 {
                                    return Err(ControlFlow::Error("الدالة تحتاج نصين".to_string()));
                                }

                                match (&evaluated_args[0], &evaluated_args[1]) {
                                    (Value::String(text), Value::String(part)) => {
                                        let result = if name == "starts_with" {
                                            text.starts_with(part)
                                        } else {
                                            text.ends_with(part)
                                        };
                                        Ok(Value::Boolean(result))
                                    }
                                    _ => Err(ControlFlow::Error("الدالة تعمل مع النصوص فقط".to_string()))
                                }
                            }
                            "trim" => {
                                if evaluated_args.len() != 1 {
                                    return Err(ControlFlow::Error("trim يحتاج نصاً واحداً".to_string()));
                                }

                                match &evaluated_args[0] {
                                    Value::String(text) => {
                                        Ok(Value::String(text.trim().to_string()))
                                    }
                                    _ => Err(ControlFlow::Error("trim يعمل مع النصوص فقط".to_string()))
                                }
                            }
                            "split" => {
                                if evaluated_args.len() != 2 {
                                    return Err(ControlFlow::Error("split يحتاج نصاً وفاصلًا".to_string()));
                                }

                                match (&evaluated_args[0], &evaluated_args[1]) {
                                    (Value::String(text), Value::String(separator)) => {
                                        Ok(Value::Array(
                                            text.split(separator)
                                                .map(|s| Value::String(s.to_string()))
                                                .collect()
                                        ))
                                    }
                                    _ => Err(ControlFlow::Error("split يعمل مع النصوص فقط".to_string()))
                                }
                            }
                            "concat" => {
                                let mut result = String::new();

                                for arg in evaluated_args {
                                    match arg {
                                        Value::String(s) => result.push_str(&s),
                                        _ => return Err(ControlFlow::Error("concat يعمل مع النصوص فقط".to_string())),
                                    }
                                }

                                Ok(Value::String(result))
                            }
                            "replace" => {
                                if evaluated_args.len() != 3 {
                                    return Err(ControlFlow::Error("replace يحتاج نصاً ونص البحث والنص البديل".to_string()));
                                }

                                match (&evaluated_args[0], &evaluated_args[1], &evaluated_args[2]) {
                                    (Value::String(text), Value::String(old), Value::String(new)) => {
                                        Ok(Value::String(text.replace(old, new)))
                                    }
                                    _ => Err(ControlFlow::Error("replace يعمل مع النصوص فقط".to_string()))
                                }
                            }
                            "slice" => {
                                if evaluated_args.len() != 3 {
                                    return Err(ControlFlow::Error("slice يحتاج قيمة وبداية ونهاية".to_string()));
                                }

                                match &evaluated_args[0] {
                                    Value::String(text) => {
                                        let start = match &evaluated_args[1] {
                                            Value::Number(n) => *n as usize,
                                            _ => return Err(ControlFlow::Error("بداية القطع يجب أن تكون رقماً".to_string()))
                                        };

                                        let end = match &evaluated_args[2] {
                                            Value::Number(n) => *n as usize,
                                            _ => return Err(ControlFlow::Error("نهاية القطع يجب أن تكون رقماً".to_string()))
                                        };

                                        let result: String = text.chars().skip(start).take(end - start).collect();
                                        Ok(Value::String(result))
                                    }
                                    _ => Err(ControlFlow::Error("slice يعمل مع النصوص فقط".to_string()))
                                }
                            }
                            _ => Err(ControlFlow::Error(format!(
                                "دالة مدمجة غير معرفة: {}",
                                name
                            ))),
                        }
                    }
                    Value::Function {
                        params,
                        body,
                        name: _,
                    } => {
                        if evaluated_args.len() != params.len() {
                            return Err(ControlFlow::Error(format!(
                                "خطأ: عدد المعاملات غير متطابق. المتوقع: {}، الممرر: {}.",
                                params.len(),
                                evaluated_args.len()
                            )));
                        }

                        let mut local_env = Environment::new_with_enclosing(std::rc::Rc::new(
                            std::cell::RefCell::new(self.environment.clone()),
                        ));
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
                    _ => Err(ControlFlow::Error(
                        DiagnosticError::new(
                            "لا يمكن استدعاء كائن غير قابل للاستدعاء.",
                            "Cannot call a non-callable object.",
                        )
                        .display(),
                    )),
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
