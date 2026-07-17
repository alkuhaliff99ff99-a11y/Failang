use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
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
    AndAnd,
    OrOr,
    Arrow,
    Colon,

    // التوكنز الجديدة للتنسيق الحر والمسافات البادئة
    Newline,
    Indent,
    Dedent,

    Identifier,
    String,
    Number,
    Let,
    Const,
    Var,
    If,
    Else,
    While,
    Print,
    True,
    False,
    Function,
    Return,
    Eof,
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

    // متغيرات تتبع المسافات البادئة والأسطر الجديدة
    indent_stack: Vec<usize>,
    at_line_start: bool,
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
        keywords.insert("اكتب".to_string(), TokenKind::Print);
        keywords.insert("print".to_string(), TokenKind::Print);
        keywords.insert("صحيح".to_string(), TokenKind::True);
        keywords.insert("صواب".to_string(), TokenKind::True);
        keywords.insert("خطأ".to_string(), TokenKind::False);
        keywords.insert("دالة".to_string(), TokenKind::Function);
        keywords.insert("عد".to_string(), TokenKind::Return);
        keywords.insert("أرجع".to_string(), TokenKind::Return);
        keywords.insert("ارجع".to_string(), TokenKind::Return);

        // كلمات المقارنة الطبيعية في Failang
        keywords.insert("اصغر".to_string(), TokenKind::Less);
        keywords.insert("يساوي".to_string(), TokenKind::EqualEqual);
        keywords.insert("مساوي".to_string(), TokenKind::EqualEqual);
        keywords.insert("لا".to_string(), TokenKind::BangEqual);
        keywords.insert("و".to_string(), TokenKind::AndAnd);
        keywords.insert("أو".to_string(), TokenKind::OrOr);
        keywords.insert("او".to_string(), TokenKind::OrOr);

        Self {
            source: source.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
            indent_stack: vec![0], // نبدأ بمستوى مسافة 0 كأصل ثابت
            at_line_start: true,   // الملف يبدأ دائماً من بداية السطر الأول
        }
    }

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, String> {
        while !self.is_at_end() {
            self.start = self.current;

            // 1. فحص وحساب المسافات البادئة إذا كنا في بداية سطر جديد
            if self.at_line_start {
                let mut spaces = 0;
                while !self.is_at_end() && (self.peek() == ' ' || self.peek() == '\t') {
                    if self.peek() == ' ' {
                        spaces += 1;
                    } else if self.peek() == '\t' {
                        spaces += 4; // نعتبر التبويب Tab بمثابة 4 مسافات
                    }
                    self.advance();
                }

                // إذا وجدنا سطر فارغ تماماً أو تعليق، لا نغير مستويات الـ Indent
                if self.is_at_end()
                    || self.peek() == '\n'
                    || (self.peek() == '/' && self.peek_next() == '/')
                {
                    self.at_line_start = true;
                    if !self.is_at_end() {
                        self.advance();
                    } // لتجاوز السطر الجديد الفارغ
                    continue;
                }

                let current_indent = *self.indent_stack.last().unwrap_or(&0);

                if spaces > current_indent {
                    self.indent_stack.push(spaces);
                    self.add_token(TokenKind::Indent);
                } else if spaces < current_indent {
                    while spaces < *self.indent_stack.last().unwrap_or(&0) {
                        self.indent_stack.pop();
                        self.add_token(TokenKind::Dedent);
                    }
                }
                self.at_line_start = false;
            }

            // 2. معالجة التوكن العادي
            if !self.is_at_end() {
                self.start = self.current;
                self.scan_token()?;
            }
        }

        // عند نهاية الملف (Eof)، نقوم بإغلاق أي كتل مفتوحة (Dedent) وتوليد التوكنز المقابلة لها
        while self.indent_stack.len() > 1 {
            self.indent_stack.pop();
            self.add_token(TokenKind::Dedent);
        }

        self.tokens.push(Token {
            kind: TokenKind::Eof,
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
                let kind = if self.match_char('+') {
                    TokenKind::Plus
                } else if self.match_char('=') {
                    TokenKind::Equal
                } else {
                    TokenKind::Plus
                };
                self.add_token(kind);
            }
            '-' => {
                let kind = if self.match_char('>') {
                    TokenKind::Arrow
                } else {
                    TokenKind::Minus
                };
                self.add_token(kind);
            }
            '*' => self.add_token(TokenKind::Star),
            '%' => self.add_token(TokenKind::Percent),
            '=' => {
                let kind = if self.match_char('=') {
                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                };
                self.add_token(kind);
            }
            '!' => {
                let kind = if self.match_char('=') {
                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
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
            '&' => {
                if self.match_char('&') {
                    self.add_token(TokenKind::AndAnd);
                } else {
                    return Err(format!("Error: single & at line {}", self.line));
                }
            }
            '|' => {
                if self.match_char('|') {
                    self.add_token(TokenKind::OrOr);
                } else {
                    return Err(format!("Error: single | at line {}", self.line));
                }
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
            ' ' | '\r' | '\t' | '\u{200E}' | '\u{200F}' | '\u{202B}' | '\u{202C}' => {}
            '\n' => {
                self.line += 1;
                self.add_token(TokenKind::Newline);
                self.at_line_start = true;
            }
            '"' => self.string_token()?,
            _ => {
                if self.is_digit(c) {
                    self.number_token();
                } else if c.is_alphabetic() || c == '_' || ('ا'..='ي').contains(&c) {
                    self.identifier_token();
                } else {
                    return Err(format!("رموز غير مدعومة في السطر {}: '{}'", self.line, c));
                }
            }
        }
        Ok(())
    }

    fn is_digit(&self, c: char) -> bool {
        c.is_ascii_digit() || ('٠'..='٩').contains(&c)
    }

    fn string_token(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err("سلسلة نصية غير مغلقة.".to_string());
        }

        self.advance(); // لتجاوز الرمز " المغلق

        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_with_lexeme(TokenKind::String, value);
        Ok(())
    }

    fn number_token(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();
            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let value: String = self.source[self.start..self.current].iter().collect();
        self.add_token_with_lexeme(TokenKind::Number, value);
    }

    fn identifier_token(&mut self) {
        while self.peek().is_alphanumeric()
            || self.peek() == '_'
            || (self.peek() >= 'ا' && self.peek() <= 'ي')
        {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        let kind = self
            .keywords
            .get(&text)
            .cloned()
            .unwrap_or(TokenKind::Identifier);
        self.add_token(kind);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
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

    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        if self.source[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
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
            lexeme: lexeme.clone(),
            line: self.line,
        });
    }
}
