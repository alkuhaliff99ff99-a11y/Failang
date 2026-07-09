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

    // الدالة الرئيسية لبدء التحليل
    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            if let Some(stmt) = self.statement() {
                statements.push(stmt);
            }
        }
        statements
    }

    // تحليل الجمل (حالياً تدعم الجمل التعبيرية فقط)
    fn statement(&mut self) -> Option<Stmt> {
        Some(Stmt::Expression(self.expression()))
    }

    // 1. التعبيرات (الأقل أولوية حالياً)
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    // 2. التحقق من التساوي (==, !=)
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

    // 3. التحقق من المقارنات (<, <=, >, >=)
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

    // 4. الجمع والطرح (+, -)
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

    // 5. الضرب والقسمة وباقي القسمة (*, /, %)
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

    // 6. العمليات الأحادية (مثل -5 أو !خطأ)
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

    // 7. العقد الأولية (الأعلى أولوية: الأرقام، النصوص، المتغيرات، والأقواس)
    fn primary(&mut self) -> Expr {
        if self.match_kinds(&[TokenKind::Number, TokenKind::String]) {
            return Expr::Literal(self.previous().lexeme.clone());
        }

        if self.match_kinds(&[TokenKind::Identifier]) {
            return Expr::Variable(self.previous().clone());
        }

        if self.match_kinds(&[TokenKind::LeftParen]) {
            let expr = self.expression();
            // استهلاك قوس الإغلاق لضمان تكافؤ الأقواس
            self.consume(TokenKind::RightParen, "Expected ')' after expression.");
            return Expr::Grouping(Box::new(expr));
        }

        // حالة افتراضية لتفادي التوقف (سيتم استبدالها بمعالجة الأخطاء الاحترافية)
        Expr::Literal(String::new())
    }

    // --- أدوات حركة مؤشر الـ Parser ومطابقة الرموز ---

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
        // حالياً نتقدم بشكل صامت عند الخطأ البرمجي وسنخصصها لاحقاً
        self.advance()
    }
}
