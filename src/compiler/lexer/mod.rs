use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // الرموز المفردة والمزدوجة
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket, // [ و ] الجديدة للمصفوفات
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,
    Percent,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // القيم
    Identifier,
    String,
    Number,

    // الكلمات المفتاحية
    Let,
    If,
    Else,
    While,
    Print,
    True,
    False,
    Function,
    Return,

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
        keywords.insert("دع".to_string(), TokenKind::Let);
        keywords.insert("let".to_string(), TokenKind::Let);
        keywords.insert("إذا".to_string(), TokenKind::If);
        keywords.insert("if".to_string(), TokenKind::If);
        keywords.insert("وإلا".to_string(), TokenKind::Else);
        keywords.insert("else".to_string(), TokenKind::Else);
        keywords.insert("طالما".to_string(), TokenKind::While);
        keywords.insert("while".to_string(), TokenKind::While);
        keywords.insert("اطبع".to_string(), TokenKind::Print);
        keywords.insert("print".to_string(), TokenKind::Print);
        keywords.insert("صحيح".to_string(), TokenKind::True);
        keywords.insert("true".to_string(), TokenKind::True);
        keywords.insert("خطأ".to_string(), TokenKind::False);
        keywords.insert("false".to_string(), TokenKind::False);
        keywords.insert("دالة".to_string(), TokenKind::Function);
        keywords.insert("fn".to_string(), TokenKind::Function);
        keywords.insert("عد".to_string(), TokenKind::Return);
        keywords.insert("return".to_string(), TokenKind::Return);

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
            '[' => self.add_token(TokenKind::LeftBracket), // التعرف على [
            ']' => self.add_token(TokenKind::RightBracket), // التعرف على ]
            ',' | '،' => self.add_token(TokenKind::Comma), // دعم الفاصلة العربية والإنجليزية
            '.' => self.add_token(TokenKind::Dot),
            '-' => self.add_token(TokenKind::Minus),
            '+' => self.add_token(TokenKind::Plus),
            ';' => self.add_token(TokenKind::Semicolon),
            '*' => self.add_token(TokenKind::Star),
            '%' => self.add_token(TokenKind::Percent),
            '!' => {
                let kind = if self.match_char('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                };
                self.add_token(kind);
            }
            '=' => {
                let kind = if self.match_char('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };
                self.add_token(kind);
            }
            '<' => {
                let kind = if self.match_char('=') {
                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                };
                self.add_token(kind);
            }
            '>' => {
                let kind = if self.match_char('=') {
                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                };
                self.add_token(kind);
            }
            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenKind::Slash);
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => self.line += 1,
            '"' => self.string_token()?,
            _ => {
                if c.is_digit(10) {
                    self.number_token();
                } else if c.is_alphabetic() || c == '_' {
                    self.identifier_token();
                } else {
                    return Err(format!("رموز غير مدعومة في السطر {}: '{}'", self.line, c));
                }
            }
        }
        Ok(())
    }

    fn string_token(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            return Err(format!("نص غير مغلق في السطر {}", self.line));
        }
        self.advance();
        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_with_lexeme(TokenKind::String, value);
        Ok(())
    }

    fn number_token(&mut self) {
        while self.peek().is_digit(10) {
            self.advance();
        }
        if self.peek() == '.' && self.peek_next().is_digit(10) {
            self.advance();
            while self.peek().is_digit(10) {
                self.advance();
            }
        }
        let value: String = self.source[self.start..self.current].iter().collect();
        self.add_token_with_lexeme(TokenKind::Number, value);
    }

    fn identifier_token(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }
        let text: String = self.source[self.start..self.current].iter().collect();
        let kind = self
            .keywords
            .get(&text)
            .cloned()
            .unwrap_or(TokenKind::Identifier);
        self.add_token_with_lexeme(kind, text);
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
    fn add_token(&mut self, kind: TokenKind) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        self.tokens.push(Token {
            kind,
            lexeme,
            line: self.line,
        });
    }
    fn add_token_with_lexeme(&mut self, kind: TokenKind, lexeme: String) {
        self.tokens.push(Token {
            kind,
            lexeme,
            line: self.line,
        });
    }
}
