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

    // فصل الإعلانات عن الجمل العادية لأولويات النطاق
    fn declaration(&mut self) -> Option<Stmt> {
        if self.match_kinds(&[TokenKind::Let]) {
            return self.var_declaration();
        }
        self.statement()
    }

    fn var_declaration(&mut self) -> Option<Stmt> {
        let name = self
            .consume(TokenKind::Identifier, "Expected variable name.")
            .clone();

        let mut initializer = None;
        if self.match_kinds(&[TokenKind::Equal]) {
            initializer = Some(self.expression());
        }

        // دعم اختياري للفاصلة المنقوطة
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
        if self.match_kinds(&[TokenKind::LeftBrace]) {
            return Some(Stmt::Block(self.block_statement()));
        }
        Some(self.expression_statement())
    }

    // تحليل جملة إذا / وإلا
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

    // تحليل حلقة طالما
    fn while_statement(&mut self) -> Stmt {
        let condition = self.expression();
        let body = Box::new(
            self.statement()
                .unwrap_or(Stmt::Expression(Expr::Literal(String::new()))),
        );
        Stmt::While { condition, body }
    }

    // تحليل البلوكات المتداخلة { ... }
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
        self.primary()
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
