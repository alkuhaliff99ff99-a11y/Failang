use super::ast::{Expr, Stmt};
use crate::compiler::lexer::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct ParseError {
    pub token: Token,
    pub message: String,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {

    fn skip_newlines(&mut self) {
        while self.match_kinds(&[TokenKind::Newline]) {}
    }

    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    // الدالة الرئيسية تعود الآن بـ Result
    pub fn parse(&mut self) -> Result<Vec<Stmt>, Vec<ParseError>> {
        let mut statements = Vec::new();
        let mut errors = Vec::new();

        while !self.is_at_end() {

            // تجاهل الأسطر الجديدة بين التعليمات
            while self.match_kinds(&[TokenKind::Newline]) {}

            if self.is_at_end() {
                break;
            }

            match self.declaration() {
                Ok(stmt) => {
                    if let Some(s) = stmt {
                        statements.push(s);
                    }
                }
                Err(err) => {
                    errors.push(err);
                    self.synchronize(); // التعافي الذاتي من الخطأ النحوي لمواصلة التحليل
                }
            }
        }

        if errors.is_empty() {
            Ok(statements)
        } else {
            Err(errors)
        }
    }

    fn declaration(&mut self) -> Result<Option<Stmt>, ParseError> {
        self.skip_newlines();

        if self.match_kinds(&[TokenKind::Indent, TokenKind::Dedent]) {
            return Ok(None);
        }

        if self.check(&TokenKind::Function) || self.peek().lexeme == "دالة" || self.peek().lexeme == "fn" {
            self.advance();
            return self.function_declaration().map(Some);
        }
        if self.check(&TokenKind::Let) || self.check(&TokenKind::Var) || 
           self.peek().lexeme == "متغير" || self.peek().lexeme == "دع" || self.peek().lexeme == "let" {
            self.advance();
            return self.var_declaration().map(Some);
        }
        self.statement().map(Some)
    }

    fn function_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(
            TokenKind::Identifier,
            "متوقع اسم الدالة بعد الإعلان."
        )?.clone();

        let mut params = Vec::new();

        // دعم الصيغة القديمة: دالة جمع(أ, ب)
        if self.match_kinds(&[TokenKind::LeftParen]) {
            if !self.check(&TokenKind::RightParen) {
                loop {
                    let param = self.consume(
                        TokenKind::Identifier,
                        "متوقع اسم المعامل."
                    )?.clone();

                    params.push(param);

                    if !self.match_kinds(&[TokenKind::Comma]) {
                        break;
                    }
                }
            }

            self.consume(
                TokenKind::RightParen,
                "متوقع قوس إغلاق."
            )?;

            self.match_kinds(&[TokenKind::Colon]);
        }
        // دعم الصيغة الجديدة:
        // دالة جمع أ ب
        else {
            while self.check(&TokenKind::Identifier) {
                params.push(self.advance().clone());
            }
        }

        self.consume(
            TokenKind::Newline,
            "متوقع سطر جديد بعد تعريف الدالة."
        )?;

        let body = self.block_statement()?;

        Ok(Stmt::Function {
            name,
            params,
            body,
        })
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        let name = self.consume(TokenKind::Identifier, "متوقع اسم المتغير بعد الإعلان عنه.")?.clone();
        let mut initializer = None;
        if self.match_kinds(&[TokenKind::Equal]) || self.peek().lexeme == "=" {
            if self.peek().lexeme == "=" { self.advance(); }
            initializer = Some(self.expression()?);
        }
        self.match_kinds(&[TokenKind::Semicolon]);
        Ok(Stmt::Var { name, initializer })
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.check(&TokenKind::If) || self.peek().lexeme == "إذا" || self.peek().lexeme == "if" {
            self.advance();
            return self.if_statement();
        }
        if self.check(&TokenKind::While) || self.peek().lexeme == "طالما" || self.peek().lexeme == "while" {
            self.advance();
            return self.while_statement();
        }
        if self.check(&TokenKind::Print) || self.peek().lexeme == "اطبع" || self.peek().lexeme == "print" {
            self.advance();
            return self.print_statement();
        }
        if self.check(&TokenKind::Return) || self.peek().lexeme == "ارجع" || self.peek().lexeme == "return" {
            self.advance();
            return self.return_statement();
        }
        if self.match_kinds(&[TokenKind::LeftBrace]) {
            return Ok(Stmt::Block(self.block_statement()?));
        }
        self.expression_statement()
    }

    fn return_statement(&mut self) -> Result<Stmt, ParseError> {
        let keyword = self.previous().clone();
        let mut value = None;
        if !self.check(&TokenKind::Semicolon) && !self.check(&TokenKind::RightBrace) && !self.is_at_end() {
            value = Some(self.expression()?);
        }
        self.match_kinds(&[TokenKind::Semicolon]);
        Ok(Stmt::Return { keyword, value })
    }

    fn print_statement(&mut self) -> Result<Stmt, ParseError> {
        let has_paren = self.match_kinds(&[TokenKind::LeftParen]);
        let value = self.expression()?;
        if has_paren { 
            self.consume(TokenKind::RightParen, "متوقع قوس مغلق ')' بعد تعبير الطباعة.")?; 
        }
        self.match_kinds(&[TokenKind::Semicolon]);
        Ok(Stmt::Print(value))
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        let condition = self.expression()?;

        self.consume(
            TokenKind::Newline,
            "متوقع سطر جديد بعد شرط إذا."
        )?;

        let then_branch =
            Box::new(Stmt::Block(self.block_statement()?));

        let mut else_branch = None;

        if self.check(&TokenKind::Else)
            || self.peek().lexeme == "والا"
            || self.peek().lexeme == "إلا"
        {
            self.advance();

            self.consume(
                TokenKind::Newline,
                "متوقع سطر جديد بعد وإلا."
            )?;

            else_branch =
                Some(Box::new(
                    Stmt::Block(self.block_statement()?)
                ));
        }

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        let condition = self.expression()?;

        self.consume(
            TokenKind::Newline,
            "متوقع سطر جديد بعد شرط طالما."
        )?;

        let body =
            Box::new(Stmt::Block(self.block_statement()?));

        Ok(Stmt::While {
            condition,
            body,
        })
    }

    fn block_statement(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();

        self.skip_newlines();

        self.consume(
            TokenKind::Indent,
            "متوقع بداية كتلة برمجية بعد المسافة البادئة."
        )?;

        self.skip_newlines();

        while !self.check(&TokenKind::Dedent) && !self.is_at_end() {
            if let Some(stmt) = self.declaration()? {
                statements.push(stmt);
            }

            self.skip_newlines();
        }

        self.consume(
            TokenKind::Dedent,
            "متوقع نهاية الكتلة البرمجية."
        )?;

        Ok(statements)
    }

    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.match_kinds(&[TokenKind::Semicolon]);
        Ok(Stmt::Expression(expr))
    }

    fn expression(&mut self) -> Result<Expr, ParseError> { 
        self.assignment() 
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;
        if self.match_kinds(&[TokenKind::Equal]) || self.peek().lexeme == "=" {
            if self.peek().lexeme == "=" { self.advance(); }
            let value = self.assignment()?;
            match expr {
                Expr::Variable(name) => {
                    return Ok(Expr::Assign {
                        name,
                        value: Box::new(value),
                    });
                }
                Expr::Index { callee, index, .. } => {
                    return Ok(Expr::IndexAssign {
                        callee,
                        index,
                        value: Box::new(value),
                    });
                }
                _ => {
                    return Err(ParseError {
                        token: self.previous().clone(),
                        message: "هدف الإسناد (Assignment target) غير صالح.".to_string(),
                    });
                }
            }
        }
        Ok(expr)
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;
        while self.match_kinds(&[TokenKind::OrOr]) || self.peek().lexeme == "أو" {
            if self.peek().lexeme == "أو" { self.advance(); }
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;
        while self.match_kinds(&[TokenKind::AndAnd]) || self.peek().lexeme == "و" {
            if self.peek().lexeme == "و" { self.advance(); }
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;
        while self.match_kinds(&[TokenKind::BangEqual, TokenKind::EqualEqual]) ||
              self.peek().lexeme == "==" || self.peek().lexeme == "!=" {
            if self.peek().lexeme == "==" || self.peek().lexeme == "!=" { self.advance(); }
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;
        while self.match_kinds(&[TokenKind::Greater, TokenKind::GreaterEqual, TokenKind::Less, TokenKind::LessEqual]) ||
              self.peek().lexeme == ">" || self.peek().lexeme == "<" || self.peek().lexeme == ">=" || self.peek().lexeme == "<=" {
            let lex = self.peek().lexeme.clone();
            if lex == ">" || lex == "<" || lex == ">=" || lex == "<=" { self.advance(); }
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;
        while self.match_kinds(&[TokenKind::Minus, TokenKind::Plus]) || self.peek().lexeme == "+" || self.peek().lexeme == "-" {
            let lex = self.peek().lexeme.clone();
            if lex == "+" || lex == "-" { self.advance(); }
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;
        while self.match_kinds(&[TokenKind::Slash, TokenKind::Star]) || self.peek().lexeme == "*" || self.peek().lexeme == "/" {
            let lex = self.peek().lexeme.clone();
            if lex == "*" || lex == "/" { self.advance(); }
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Expr::Binary { left: Box::new(expr), operator, right: Box::new(right) };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_kinds(&[TokenKind::Bang, TokenKind::Minus]) || self.peek().lexeme == "!" || self.peek().lexeme == "-" {
            let lex = self.peek().lexeme.clone();
            if lex == "!" || lex == "-" { self.advance(); }
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Expr::Unary { operator, right: Box::new(right) });
        }
        self.call()
    }

    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary()?;
        loop {
            if self.match_kinds(&[TokenKind::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else if self.match_kinds(&[TokenKind::LeftBracket]) {
                expr = self.finish_index(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_index(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let bracket = self.previous().clone();
        let index = self.expression()?;
        self.consume(TokenKind::RightBracket, "متوقع قوس الإغلاق ']' بعد مؤشر/فهرس المصفوفة.")?;
        Ok(Expr::Index {
            callee: Box::new(callee),
            bracket,
            index: Box::new(index),
        })
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let mut arguments = Vec::new();
        if !self.check(&TokenKind::RightParen) {
            loop {
                arguments.push(self.expression()?);
                if !self.match_kinds(&[TokenKind::Comma]) { break; }
            }
        }
        let paren = self.consume(TokenKind::RightParen, "متوقع قوس الإغلاق ')' بعد المعاملات البرمجية الدخيلة.")?.clone();
        Ok(Expr::Call { callee: Box::new(callee), paren, arguments })
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_kinds(&[TokenKind::False]) || self.peek().lexeme == "خطأ" {
            if self.peek().lexeme == "خطأ" { self.advance(); }
            return Ok(Expr::Literal("false".to_string()));
        }
        if self.match_kinds(&[TokenKind::True]) || self.peek().lexeme == "صواب" {
            if self.peek().lexeme == "صواب" { self.advance(); }
            return Ok(Expr::Literal("true".to_string()));
        }
        if self.peek().lexeme == "عدم" {
            self.advance();
            return Ok(Expr::Literal("nil".to_string()));
        }
        if self.match_kinds(&[TokenKind::Number]) || self.peek().kind == TokenKind::Number {
            return Ok(Expr::Literal(self.previous().lexeme.clone()));
        }
        if self.match_kinds(&[TokenKind::String]) || self.peek().kind == TokenKind::String {
            return Ok(Expr::Literal(self.previous().lexeme.clone()));
        }
        if self.match_kinds(&[TokenKind::Identifier]) || self.peek().kind == TokenKind::Identifier {
            return Ok(Expr::Variable(self.previous().clone()));
        }
        if self.match_kinds(&[TokenKind::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenKind::RightParen, "متوقع قوس الإغلاق ')' بعد نهاية التعبير الحسابي.")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        if self.match_kinds(&[TokenKind::LeftBracket]) {
            let bracket = self.previous().clone();
            let mut elements = Vec::new();
            if !self.check(&TokenKind::RightBracket) {
                loop {
                    elements.push(self.expression()?);
                    if !self.match_kinds(&[TokenKind::Comma]) { break; }
                }
            }
            self.consume(TokenKind::RightBracket, "متوقع قوس الإغلاق ']' بعد عناصر المصفوفة.")?;
            return Ok(Expr::Array { bracket, elements });
        }

        // في حال حدوث رمز غير متوقع بدلاً من الـ panic نلقي بخطأ منظم
        Err(ParseError {
            token: self.peek().clone(),
            message: "تعبير غير متوقع أو رمز غير معروف.".to_string(),
        })
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
        if self.is_at_end() { return false; }
        &self.peek().kind == kind
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() { self.current += 1; }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::EOF
    }

    pub fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, kind: TokenKind, message: &str) -> Result<&Token, ParseError> {
        if self.check(&kind) { return Ok(self.advance()); }
        Err(ParseError {
            token: self.peek().clone(),
            message: message.to_string(),
        })
    }

    // تقنية الـ Synchronization لإعادة توجيه الـ Parser بعد الخطأ
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().kind == TokenKind::Semicolon { return; }

            match self.peek().kind {
                TokenKind::Function | TokenKind::Var | TokenKind::Let |
                TokenKind::If | TokenKind::While | TokenKind::Print | TokenKind::Return => return,
                _ => {}
            }

            let lex = &self.peek().lexeme;
            if lex == "دالة" || lex == "متغير" || lex == "إذا" || lex == "طالما" || lex == "اطبع" || lex == "ارجع" {
                return;
            }

            self.advance();
        }
    }
}
