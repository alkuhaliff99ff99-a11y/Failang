use crate::compiler::ast::{Expr, Stmt};
use crate::compiler::parser::token::Token;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum BorrowKind {
    Shared,
    Mutable,
}

#[derive(Debug, Clone)]
pub struct Borrow {
    pub name: String,
    pub kind: BorrowKind,
    pub token: Token,
}

#[derive(Debug, Default)]
pub struct Scope {
    pub borrows: HashMap<String, Vec<Borrow>>,
    pub moved_variables: HashMap<String, Token>,
}

pub struct BorrowChecker {
    scopes: Vec<Scope>,
}

impl BorrowChecker {
    pub fn new() -> Self {
        Self {
            scopes: vec![Scope::default()],
        }
    }

    fn enter_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    fn exit_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn check_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Block(statements) => {
                self.enter_scope();
                for statement in statements {
                    self.check_stmt(statement)?;
                }
                self.exit_scope();
            }
            Stmt::If { condition, then_branch, else_branch } => {
                self.check_expr(condition)?;
                self.check_stmt(then_branch)?;
                if let Some(else_stmt) = else_branch {
                    self.check_stmt(else_stmt)?;
                }
            }
            Stmt::While { condition, body } => {
                self.check_expr(condition)?;
                self.check_stmt(body)?;
            }
            Stmt::Print(expr) => {
                self.check_expr(expr)?;
            }
            Stmt::Return { value, .. } => {
                if let Some(expr) = value {
                    self.check_expr(expr)?;
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn check_expr(&mut self, _expr: &Expr) -> Result<(), String> {
        Ok(())
    }
}
