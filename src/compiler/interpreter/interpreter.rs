use super::environment::Environment;
use super::value::Value;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::{Expr, Stmt};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct ReturnException {
    pub value: Value,
}

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), ReturnException> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), ReturnException> {
        match stmt {
            Stmt::Expression(expr) => {
                self.evaluate(expr);
            }
            Stmt::Var { name, initializer } => {
                let mut value = Value::Nil;
                if let Some(init_expr) = initializer {
                    value = self.evaluate(init_expr);
                }
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), value);
            }
            Stmt::Block(statements) => {
                let previous = self.environment.clone();
                let local_env = Rc::new(RefCell::new(Environment::new_with_enclosing(
                    previous.clone(),
                )));

                self.environment = local_env;
                let result = self.interpret(statements);
                self.environment = previous;
                result?;
            }
            Stmt::Print(expr) => {
                let value = self.evaluate(expr);
                println!("{}", value);
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if self.is_truthy(&self.evaluate(condition)) {
                    self.execute(then_branch)?;
                } else if let Some(else_stmt) = else_branch {
                    self.execute(else_stmt)?;
                }
            }
            Stmt::While { condition, body } => {
                while self.is_truthy(&self.evaluate(condition)) {
                    self.execute(body)?;
                }
            }
            Stmt::Function { name, params, body } => {
                let param_names = params.iter().map(|p| p.lexeme.clone()).collect();
                let function_value = Value::Function {
                    name: name.lexeme.clone(),
                    params: param_names,
                    body: body.clone(),
                };
                self.environment
                    .borrow_mut()
                    .define(name.lexeme.clone(), function_value);
            }
            Stmt::Return { value, .. } => {
                let return_val = if let Some(expr) = value {
                    let v = self.evaluate(expr);
                    v
                } else {
                    Value::Nil
                };
                return Err(ReturnException { value: return_val });
            }
        }
        Ok(())
    }

    fn is_truthy(&self, value: &Value) -> bool {
        match value {
            Value::Nil => false,
            Value::Boolean(b) => *b,
            _ => true,
        }
    }

    pub fn evaluate(&self, expr: &Expr) -> Value {
        match expr {
            Expr::Literal(lexeme) => {
                if let Ok(num) = lexeme.parse::<f64>() {
                    Value::Number(num)
                } else {
                    Value::String(lexeme.clone())
                }
            }
            Expr::Variable(name) => self
                .environment
                .borrow()
                .get(&name.lexeme)
                .unwrap_or(Value::Nil),
            Expr::Grouping(inner) => self.evaluate(inner),
            Expr::Unary { operator, right } => {
                let right_val = self.evaluate(right);
                match operator.kind {
                    TokenKind::Minus => {
                        if let Value::Number(n) = right_val {
                            Value::Number(-n)
                        } else {
                            Value::Nil
                        }
                    }
                    TokenKind::Bang => Value::Boolean(!self.is_truthy(&right_val)),
                    _ => Value::Nil,
                }
            }
            Expr::Binary {
                left,
                operator,
                right,
            } => {
                let left_val = self.evaluate(left);
                let right_val = self.evaluate(right);

                match operator.kind {
                    TokenKind::Plus => {
                        if let (Value::Number(l), Value::Number(r)) = (&left_val, &right_val) {
                            Value::Number(l + r)
                        } else if let (Value::String(l), Value::String(r)) = (&left_val, &right_val)
                        {
                            Value::String(format!("{}{}", l, r))
                        } else {
                            Value::Nil
                        }
                    }
                    TokenKind::Minus => {
                        if let (Value::Number(l), Value::Number(r)) = (left_val, right_val) {
                            Value::Number(l - r)
                        } else {
                            Value::Nil
                        }
                    }
                    TokenKind::Star => {
                        if let (Value::Number(l), Value::Number(r)) = (left_val, right_val) {
                            Value::Number(l * r)
                        } else {
                            Value::Nil
                        }
                    }
                    TokenKind::Slash => {
                        if let (Value::Number(l), Value::Number(r)) = (left_val, right_val) {
                            Value::Number(l / r)
                        } else {
                            Value::Nil
                        }
                    }
                    TokenKind::EqualEqual => Value::Boolean(left_val == right_val),
                    TokenKind::BangEqual => Value::Boolean(left_val != right_val),
                    TokenKind::Less => {
                        if let (Value::Number(l), Value::Number(r)) = (left_val, right_val) {
                            Value::Boolean(l < r)
                        } else {
                            Value::Nil
                        }
                    }
                    TokenKind::Greater => {
                        if let (Value::Number(l), Value::Number(r)) = (left_val, right_val) {
                            Value::Boolean(l > r)
                        } else {
                            Value::Nil
                        }
                    }
                    _ => Value::Nil,
                }
            }
            Expr::Call {
                callee, arguments, ..
            } => {
                let callee_val = self.evaluate(callee);

                if let Value::Function { params, body, .. } = callee_val {
                    let mut evaluated_args = Vec::new();
                    for arg in arguments {
                        evaluated_args.push(self.evaluate(arg));
                    }

                    let previous = self.environment.clone();
                    let local_env = Rc::new(RefCell::new(Environment::new_with_enclosing(
                        previous.clone(),
                    )));

                    for (param, arg_val) in params.iter().zip(evaluated_args.iter()) {
                        local_env
                            .borrow_mut()
                            .define(param.clone(), arg_val.clone());
                    }

                    let mut interpreter_state = Interpreter {
                        environment: local_env,
                    };
                    let mut return_value = Value::Nil;

                    if let Err(exception) = interpreter_state.interpret(&body) {
                        return_value = exception.value;
                    }

                    return_value
                } else {
                    Value::Nil
                }
            }
            // 🛠️ تفعيل معالجة وحساب قيم المصفوفة الفعلي داخل المفسر
            Expr::Array { elements, .. } => {
                let mut evaluated_elements = Vec::new();
                for element in elements {
                    evaluated_elements.push(self.evaluate(element));
                }
                Value::Array(evaluated_elements)
            }
        }
    }
}
