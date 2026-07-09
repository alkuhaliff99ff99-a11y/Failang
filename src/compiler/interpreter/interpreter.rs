use super::value::Value;
use crate::compiler::lexer::TokenKind;
use crate::compiler::parser::Expr;

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    // الدالة الرئيسية لتقييم أي تعبير وإرجاع قيمته الحقيقية
    pub fn evaluate(&self, expr: &Expr) -> Value {
        match expr {
            Expr::Literal(lexeme) => {
                // محاولة تحويل النص إلى رقم، وإذا فشل يعتبر نصاً عادياً
                if let Ok(num) = lexeme.parse::<f64>() {
                    Value::Number(num)
                } else {
                    Value::String(lexeme.clone())
                }
            }
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
                    TokenKind::Bang => {
                        if let Value::Boolean(b) = right_val {
                            Value::Boolean(!b)
                        } else {
                            Value::Nil
                        }
                    }
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
                    // العمليات الحسابية
                    TokenKind::Plus => {
                        if let (Value::Number(l), Value::Number(r)) = (&left_val, &right_val) {
                            Value::Number(l + r)
                        } else if let (Value::String(l), Value::String(r)) = (&left_val, &right_val)
                        {
                            Value::String(format!("{}{}", l, r)) // دمج النصوص!
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

                    // المقارنات والتساوي
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
            _ => Value::Nil, // سيتم دعم المتغيرات في الجزء الثاني
        }
    }
}
