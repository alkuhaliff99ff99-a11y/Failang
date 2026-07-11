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
        // دعم صريح لـ دالة و fn
        if self.check(&TokenKind::Function) || self.peek().lexeme == "دالة" || self.peek().lexeme == "fn" {
            self.advance();
            return Some(self.function_declaration());
        }
        // دعم صريح للمتغيرات
        if self.check(&TokenKind::Let) || self.check(&TokenKind::Var) || 
           self.peek().lexeme == "متغير" || self.peek().lexeme == "دع" || self.peek().lexeme == "let" {
            self.advance();
            return self.var_declaration();
        }
        self.statement()
    }

    fn function_declaration(&mut self) -> Stmt {
        let name = self.consume(TokenKind::Identifier, "Expected function name.").clone();
        self.consume(TokenKind::LeftParen, "Expected '(' after function name.");
        let mut params = Vec::new();
        if !self.check(&TokenKind::RightParen) {
            loop {
                params.push(self.consume(TokenKind::Identifier, "Expected parameter name.").clone());
                if !self.match_kinds(&[TokenKind::Comma]) { break; }
            }
        }
        self.consume(TokenKind::RightParen, "Expected ')' after parameters.");
        
        // التحقق من القوس المفتوح { للكتلة البرمجية
        self.consume(TokenKind::LeftBrace, "Expected '{' before function body.");
        let body = self.block_statement();
        Stmt::Function { name, params, body }
    }

    fn var_declaration(&mut self) -> Option<Stmt> {
        let name = self.consume(TokenKind::Identifier, "Expected variable name.").clone();
        let mut initializer = None;
        if self.match_kinds(&[TokenKind::Equal]) || self.peek().lexeme == "=" {
            if self.peek().lexeme == "=" { self.advance(); }
            initializer = Some(self.expression());
        }
        self.match_kinds(&[TokenKind::Semicolon]);
        Some(Stmt::Var { name, initializer })
    }

    fn statement(&mut self) -> Option<Stmt> {
        if self.check(&TokenKind::If) || self.peek().lexeme == "إذا" || self.peek().lexeme == "if" {
            self.advance();
            return Some(self.if_statement());
        }
        if self.check(&TokenKind::While) || self.peek().lexeme == "طالما" || self.peek().lexeme == "while" {
            self.advance();
            return Some(self.while_statement());
        }
        if self.check(&TokenKind::Print) || self.peek().lexeme == "اطبع" || self.peek().lexeme == "print" {
            self.advance();
            return Some(self.print_statement());
        }
        if self.check(&TokenKind::Return) || self.peek().lexeme == "ارجع" || self.peek().lexeme == "return" {
            self.advance();
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
        if has_paren { self.consume(TokenKind::RightParen, "Expected ')' after print value."); }
        self.match_kinds(&[TokenKind::Semicolon]);
        Stmt::Print(value)
    }

    fn if_statement(&mut self) -> Stmt {
        let condition = self.expression();
        
        // التحقق مما إذا كان هناك قوس بداية للكتلة، إن لم يكن، نستهلكه للاحتياط
        if self.check(&TokenKind::LeftBrace) { self.advance(); }
        let then_branch = Box::new(Stmt::Block(self.block_statement()));
        
        let mut else_branch = None;
        if self.peek().lexeme == "والا" || self.peek().lexeme == "إلا" || self.check(&TokenKind::Else) {
            self.advance();
            if self.check(&TokenKind::LeftBrace) { self.advance(); }
            else_branch = Some(Box::new(Stmt::Block(self.block_statement())));
        }
        Stmt::If { condition, then_branch, else_branch }
    }

    fn while_statement(&mut self) -> Stmt {
        let condition = self.expression();
        if self.check(&TokenKind::LeftBrace) { self.advance(); }
        let body = Box::new(Stmt::Block(self.block_statement()));
        Stmt::While { condition, body }
    }

    fn block_statement(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            if let Some(stmt) = self.declaration() { statements.push(stmt); }
        }
        self.consume(TokenKind::RightBrace, "Expected '}' after block.");
        statements
    }

    fn expression_statement(&mut self) -> Stmt {
        let expr = self.expression();
        self.match_kinds(&[TokenKind::Semicolon]);
        Some(expr).map(Stmt::Expression).unwrap()
    }

    fn expression(&mut self) -> Expr { self.assignment() }

    fn assignment(&mut self) -> Expr {
        let expr = self.or();
        if self.match_kinds(&[TokenKind::Equal]) || self.peek().lexeme == "=" {
            let equals = if self.peek().lexeme == "=" { self.advance().clone() } else { self.previous().clone() };
            let value = self.assignment();
            if let Expr::Variable(name) = expr {
                return Expr::Binary {
                    left: Box::new(Expr::Variable(name)),
                    operator: equals,
                    right: Box::new(value),
                };
            }
        }
        expr
    }

    fn or(&mut self) -> Expr {
        let mut expr = self.and();
        while self.match_kinds(&[TokenKind::OrOr]) || self.peek().lexeme == "أو" {
            let operator = self.advance().clone();
            let right = self.and();
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        expr
    }

    fn and(&mut self) -> Expr {
        let mut expr = self.equality();
        while self.match_kinds(&[TokenKind::AndAnd]) || self.peek().lexeme == "و" {
            let operator = self.advance().clone();
            let right = self.equality();
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        expr
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();
        while self.match_kinds(&[TokenKind::EqualEqual, TokenKind::BangEqual]) || self.peek().lexeme == "==" {
            let operator = self.advance().clone();
            let right = self.comparison();
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        expr
    }

    fn comparison(&mut self) -> Expr {
        let mut expr = self.term();
        while self.match_kinds(&[TokenKind::Less, TokenKind::LessEqual, TokenKind::Greater, TokenKind::GreaterEqual]) {
            let operator = self.previous().clone();
            let right = self.term();
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        expr
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();
        while self.match_kinds(&[TokenKind::Plus, TokenKind::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor();
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.power();
        while self.match_kinds(&[TokenKind::Star, TokenKind::Slash, TokenKind::Percent]) {
            let operator = self.previous().clone();
            let right = self.power();
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        expr
    }

    fn power(&mut self) -> Expr {
        let mut expr = self.unary();
        while self.match_kinds(&[TokenKind::Power]) {
            let operator = self.previous().clone();
            let right = self.unary();
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        expr
    }

    fn unary(&mut self) -> Expr {
        if self.match_kinds(&[TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary();
            return Expr::Unary { operator, right: Box::new(right) };
        }
        self.call()
    }

    fn call(&mut self) -> Expr {
        let mut expr = self.primary();
        loop {
            if self.match_kinds(&[TokenKind::LeftParen]) { expr = self.finish_call(expr); }
            else { break; }
        }
        expr
    }

    fn finish_call(&mut self, callee: Expr) -> Expr {
        let mut arguments = Vec::new();
        if !self.check(&TokenKind::RightParen) {
            loop {
                arguments.push(self.expression());
                if !self.match_kinds(&[TokenKind::Comma]) { break; }
            }
        }
        let paren = self.consume(TokenKind::RightParen, "Expected ')' after arguments.").clone();
        Expr::Call { callee: Box::new(callee), paren, arguments }
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
            if self.check(kind) { self.advance(); return true; }
        }
        false
    }

    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() { return false; }
        &self.peek().kind == kind
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() { self.current += 1; }
        self.previous()
    }

    fn is_at_end(&self) -> bool { self.peek().kind == TokenKind::EOF }
    fn peek(&self) -> &Token { &self.tokens[self.current] }
    fn previous(&self) -> &Token { &self.tokens[self.current - 1] }
    fn consume(&mut self, kind: TokenKind, _msg: &str) -> &Token {
        if self.check(&kind) { return self.advance(); }
        self.advance()
    }
}
