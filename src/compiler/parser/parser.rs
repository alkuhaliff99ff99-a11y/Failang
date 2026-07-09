use super::ast::{Expr, Stmt};
use crate::compiler::lexer::{Token, TokenKind};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }
        statements
    }

    fn declaration(&mut self) -> Option<Stmt> {
        if self.match_kinds(&[TokenKind::Function]) {
            return Some(self.function_declaration());
        }
        if self.match_kinds(&[TokenKind::Let]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn function_declaration(&mut self) -> Stmt {
        let name = self
            .consume(TokenKind::Identifier, "Expected function name.")
            .clone();
        self.consume(TokenKind::LeftParen, "Expected '(' after function name.");

        let mut params = Vec::new();
        if !self.check(&TokenKind::RightParen) {
            loop {
                params.push(
                    self.consume(TokenKind::Identifier, "Expected parameter name.")
                        .clone(),
                );
                if !self.match_kinds(&[TokenKind::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenKind::RightParen, "Expected ')' after parameters.");
        self.consume(TokenKind::LeftBrace, "Expected '{' before function body.");

        let body = self.block_statement();
        Stmt::Function { name, params, body }
    }

    fn var_declaration(&mut self) -> Option<Stmt> {
        let name = self
            .consume(TokenKind::Identifier, "Expected variable name.")
            .clone();
        let mut initializer = None;
        if self.match_kinds(&[TokenKind::Equal]) {
            initializer = Some(self.expression());
        }
        self.match_kinds(&[TokenKind::Semicolon]);
        Some(Stmt::Var { name, initializer })
    }

    fn statement(&mut self) -> Option<Stmt> {
        if self.match_kinds(&[TokenKind::If]) {
            return Some(self.if_statement());
        }
        if self.match_kinds(&[TokenKind::While]) {
            return Some(self.while_statement());
        }
        if self.match_kinds(&[TokenKind::Print]) {
            return Some(self.print_statement());
        }
        if self.match_kinds(&[TokenKind::Return]) {
            return Some(self.return_statement());
        }
        if self.match_kinds(&[TokenKind::LeftBrace]) {
            return Some(Stmt::Block(self.block_statement()));
        }
        Some(self.expression_statement())
    }

    fn return_statement(&mut self) -> Stmt {
        let keyword = self.previous().clone();
        let mut value = None;
        if !self.check(&TokenKind::Semicolon) && !self.check(&TokenKind::RightBrace) {
            value = Some(self.expression());
        }
        self.match_kinds(&[TokenKind::Semicolon]);
        Stmt::Return { keyword, value }
    }

    fn print_statement(&mut self) -> Stmt {
        let has_paren = self.match_kinds(&[TokenKind::LeftParen]);
        let value = self.expression();
        if has_paren {
            self.consume(TokenKind::RightParen, "Expected ')' after print value.");
        }
        self.match_kinds(&[TokenKind::Semicolon]);
        Stmt::Print(value)
    }

    fn if_statement(&mut self) -> Stmt {
        let condition = self.expression();
        let then_branch = Box::new(
            self.statement()
                .unwrap_or(Stmt::Expression(Expr::Literal(String::new()))),
        );
        let mut else_branch = None;
        if self.match_kinds(&[TokenKind::Else]) {
            else_branch = Some(Box::new(
                self.statement()
                    .unwrap_or(Stmt::Expression(Expr::Literal(String::new()))),
            ));
        }
        Stmt::If {
            condition,
            then_branch,
            else_branch,
        }
    }

    fn while_statement(&mut self) -> Stmt {
        let condition = self.expression();
        let body = Box::new(
            self.statement()
                .unwrap_or(Stmt::Expression(Expr::Literal(String::new()))),
        );
        Stmt::While { condition, body }
    }

    fn block_statement(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if let Some(stmt) = self.declaration() {
                statements.push(stmt);
            }
        }
        self.consume(TokenKind::RightBrace, "Expected '}' after block.");
        statements
    }

    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.match_kinds(&[TokenKind::Semicolon]);
        Stmt::Expression(expr)
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_kinds(&[TokenKind::EqualEqual, TokenKind::BangEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.match_kinds(&[
            TokenKind::Less,
            TokenKind::LessEqual,
            TokenKind::Greater,
            TokenKind::GreaterEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_kinds(&[TokenKind::Plus, TokenKind::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_kinds(&[TokenKind::Star, TokenKind::Slash, TokenKind::Percent]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_kinds(&[TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Unary {
                operator,
                right: Box::new(right),
            };
        }
        self.call()
    }

    fn call(&mut self) -> Expr {
        let mut expr = self.primary();
        loop {
            if self.match_kinds(&[TokenKind::LeftParen]) {
                expr = self.finish_call(expr);
            } else {
                break;
            }
        }
        expr
    }

    fn finish_call(&mut self, callee: Expr) -> Expr {
        let mut arguments = Vec::new();
        if !self.check(&TokenKind::RightParen) {
            loop {
                arguments.push(self.expression());
                if !self.match_kinds(&[TokenKind::Comma]) {
                    break;
                }
            }
        }
        let paren = self
            .consume(TokenKind::RightParen, "Expected ')' after arguments.")
            .clone();
        Expr::Call {
            callee: Box::new(callee),
            paren,
            arguments,
        }
    }

    fn primary(&mut self) -> Expr {
        if self.match_kinds(&[TokenKind::Number, TokenKind::String]) {
            return Expr::Literal(self.previous().lexeme.clone());
        }
        if self.match_kinds(&[TokenKind::Identifier]) {
            return Expr::Variable(self.previous().clone());
        }
        if self.match_kinds(&[TokenKind::LeftParen]) {
            let expr = self.expression();
            self.consume(TokenKind::RightParen, "Expected ')' after expression.");
            return Expr::Grouping(Box::new(expr));
        }

        // 🛠️ ميزة قراءة المصفوفات الجديدة قواعدياً: [ تعبير، تعبير، ... ]
        if self.match_kinds(&[TokenKind::LeftBracket]) {
            let mut elements = Vec::new();
            if !self.check(&TokenKind::RightBracket) {
                loop {
                    elements.push(self.expression());
                    if !self.match_kinds(&[TokenKind::Comma]) {
                        break;
                    }
                }
            }
            let bracket = self
                .consume(
                    TokenKind::RightBracket,
                    "Expected ']' after array elements.",
                )
                .clone();
            return Expr::Array { bracket, elements };
        }

        Expr::Literal(String::new())
    }

    fn match_kinds(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();
                return true;
            }
        }
        false
    }

    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().kind == kind
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::EOF
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
    fn consume(&mut self, kind: TokenKind, _message: &str) -> &Token {
        if self.check(&kind) {
            return self.advance();
        }
        self.advance()
    }
}
