use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    LeftParen, RightParen, LeftBrace, RightBrace, LeftBracket, RightBracket,
    Comma, Dot, Minus, Plus, Semicolon, Slash, Star, Percent,
    Bang, BangEqual, Equal, EqualEqual, Greater, GreaterEqual, Less, LessEqual,
    AndAnd, OrOr, Arrow, Colon,
    Identifier, String, Number,
    Let, Const, Var, If, Else, While, Print, True, False, Function, Return,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
}

pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenKind>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("let".to_string(), TokenKind::Let);
        keywords.insert("const".to_string(), TokenKind::Const);
        keywords.insert("var".to_string(), TokenKind::Var);
        keywords.insert("if".to_string(), TokenKind::If);
        keywords.insert("else".to_string(), TokenKind::Else);
        keywords.insert("while".to_string(), TokenKind::While);
        keywords.insert("print".to_string(), TokenKind::Print);
        keywords.insert("true".to_string(), TokenKind::True);
        keywords.insert("false".to_string(), TokenKind::False);
        keywords.insert("fn".to_string(), TokenKind::Function);
        keywords.insert("return".to_string(), TokenKind::Return);

        keywords.insert("دع".to_string(), TokenKind::Let);
        keywords.insert("ثابت".to_string(), TokenKind::Const);
        keywords.insert("متغير".to_string(), TokenKind::Var);
        keywords.insert("إذا".to_string(), TokenKind::If);
        keywords.insert("اذا".to_string(), TokenKind::If);
        keywords.insert("وإلا".to_string(), TokenKind::Else);
        keywords.insert("والا".to_string(), TokenKind::Else);
        keywords.insert("طالما".to_string(), TokenKind::While);
        keywords.insert("اطبع".to_string(), TokenKind::Print);
        keywords.insert("صحيح".to_string(), TokenKind::True);
        keywords.insert("صواب".to_string(), TokenKind::True);
        keywords.insert("خطأ".to_string(), TokenKind::False);
        keywords.insert("دالة".to_string(), TokenKind::Function);
        keywords.insert("عد".to_string(), TokenKind::Return);
        keywords.insert("أرجع".to_string(), TokenKind::Return);
        keywords.insert("ارجع".to_string(), TokenKind::Return);

        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }
        self.tokens.push(Token {
            kind: TokenKind::EOF,
            lexeme: "".to_string(),
            line: self.line,
        });
        Ok(self.tokens)
    }

    fn scan_token(&mut self) -> Result<(), String> {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenKind::LeftParen),
            ')' => self.add_token(TokenKind::RightParen),
            '{' => self.add_token(TokenKind::LeftBrace),
            '}' => self.add_token(TokenKind::RightBrace),
            '[' => self.add_token(TokenKind::LeftBracket),
            ']' => self.add_token(TokenKind::RightBracket),
            ',' | '،' => self.add_token(TokenKind::Comma),
            '.' => self.add_token(TokenKind::Dot),
            ';' => self.add_token(TokenKind::Semicolon),
            ':' | '：' => self.add_token(TokenKind::Colon),
            '+' => {
                let kind = if self.match_char('+') { TokenKind::Plus }
                           else if self.match_char('=') { TokenKind::Equal }
                           else { TokenKind::Plus };
                self.add_token(kind);
            }
            '-' => {
                let kind = if self.match_char('>') { TokenKind::Arrow }
                           else { TokenKind::Minus };
                self.add_token(kind);
            }
            '*' => self.add_token(TokenKind::Star),
            '%' => self.add_token(TokenKind::Percent),
            '=' => {
                let kind = if self.match_char('=') { TokenKind::EqualEqual }
                           else { TokenKind::Equal };
                self.add_token(kind);
            }
            '!' => {
                let kind = if self.match_char('=') { TokenKind::BangEqual }
                           else { TokenKind::Bang };
                self.add_token(kind);
            }
            '<' => {
                let kind = if self.match_char('=') { TokenKind::LessEqual }
                           else { TokenKind::Less };
                self.add_token(kind);
            }
            '>' => {
                let kind = if self.match_char('=') { TokenKind::GreaterEqual }
                           else { TokenKind::Greater };
                self.add_token(kind);
            }
            '&' => {
                if self.match_char('&') { self.add_token(TokenKind::AndAnd); }
                else { return Err(format!("Error: single & at line {}", self.line)); }
            }
            '|' => {
                if self.match_char('|') { self.add_token(TokenKind::OrOr); }
                else { return Err(format!("Error: single | at line {}", self.line)); }
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() { self.advance(); }
                } else {
                    self.add_token(TokenKind::Slash);
                }
            }
            ' ' | '\r' | '\t' | '\u{200E}' | '\u{200F}' | '\u{202B}' | '\u{202C}' => {}
            '\n' => self.line += 1,
            '"' => self.string_token()?,
            _ => {
                if self.is_digit(c) {
                    self.number_token();
                } else if c.is_alphabetic() || c == '_' || (c >= 'ا' && c <= 'ي') {
                    self.identifier_token();
                } else {
                    return Err(format!("رموز غير مدعومة في السطر {}: '{}'", self.line, c));
                }
            }
        }
        Ok(())
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_digit(10) || (c >= '٠' && c <= '٩')
    }

    fn string_token(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' { self.line += 1; }
            self.advance();
        }
        if self.is_at_end() { return Err(format!("نص غير مغلق في السطر {}", self.line)); }
        self.advance();
        let value: String = self.source[self.start + 1..self.current - 1].iter().collect();
        self.add_token_with_lexeme(TokenKind::String, value);
        Ok(())
    }

    fn number_token(&mut self) {
        while self.is_digit(self.peek()) { self.advance(); }
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) { self.advance(); }
        }
        let raw: String = self.source[self.start..self.current].iter().collect();
        // تحويل الأرقام العربية الشرقية لغربية ليفهمها نظام التشغيل والمفسر بمرونة
        let normalized: String = raw.chars().map(|c| {
            match c {
                '٠' => '0', '١' => '1', '٢' => '2', '٣' => '3', '٤' => '4',
                '٥' => '5', '٦' => '6', '٧' => '7', '٨' => '8', '٩' => '9',
                _ => c
            }
        }).collect();
        self.add_token_with_lexeme(TokenKind::Number, normalized);
    }

    fn identifier_token(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' || (self.peek() >= 'ا' && self.peek() <= 'ي') {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].iter().collect();
        let kind = self.keywords.get(&text).cloned().unwrap_or(TokenKind::Identifier);
        self.add_token_with_lexeme(kind, text);
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected { return false; }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() { return '\0'; }
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() { return '\0'; }
        self.source[self.current + 1]
    }

    fn is_at_end(&self) -> bool { self.current >= self.source.len() }

    fn add_token(&mut self, kind: TokenKind) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token { kind, lexeme, line: self.line });
    }

    fn add_token_with_lexeme(&mut self, kind: TokenKind, lexeme: String) {
        self.tokens.push(Token { kind, lexeme, line: self.line });
    }
}
