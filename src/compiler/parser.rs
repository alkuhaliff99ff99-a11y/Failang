use crate::compiler::tokens::{Token, TokenKind};
use crate::compiler::ast::{Expr, Stmt, LiteralValue};

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
        if self.match_token(TokenKind::Let) {
            self.var_declaration()
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Option<Stmt> {
        let name = if let TokenKind::Identifier(n) = &self.peek().kind {
            n.clone()
        } else {
            panic!("يتوقع وجود اسم متغير بعد كلمة 'متغير' في السطر {}", self.peek().line);
        };
        
        self.advance();
        self.consume(TokenKind::Assign, "يتوقع وجود '=' بعد اسم المتغير");
        
        let initializer = self.expression();
        Some(Stmt::Var { name, initializer })
    }

    fn statement(&mut self) -> Option<Stmt> {
        if self.match_token(TokenKind::Print) {
            let expr = self.expression();
            Some(Stmt::Print(expr))
        } else {
            let expr = self.expression();
            Some(Stmt::Expression(expr))
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.term();
        while self.match_token(TokenKind::Plus) || self.match_token(TokenKind::Minus) {
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
        let token = self.advance().clone();
        match token.kind {
            TokenKind::Number(n) => Expr::Literal(LiteralValue::Number(n)),
            TokenKind::String(s) => Expr::Literal(LiteralValue::String(s)),
            TokenKind::Identifier(name) => Expr::Variable(name),
            _ => panic!("تعبير غير صالح في السطر {}", token.line),
        }
    }

    fn match_token(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            return true;
        }
        false
    }

    fn consume(&mut self, kind: TokenKind, message: &str) {
        if self.check(kind) {
            self.advance();
        } else {
            panic!("{}", message);
        }
    }

    fn check(&self, kind: TokenKind) -> bool {
        if self.is_at_end() { return false; }
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(&kind)
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() { self.current += 1; }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().kind, TokenKind::EOF)
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
