use super::error::LexError;
use super::keywords::lookup_keyword;
use super::token::{Token, TokenKind};

pub struct Lexer {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(source_code: &str) -> Self {
        Self {
            source: source_code.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: 1,
        }
    }

    // --- تفعيل المحرك الرئيسي للحلقة ---
    pub fn scan_tokens(mut self) -> Result<Vec<Token>, LexError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        // إضافة رمز نهاية الملف عند اكتمال التحليل
        self.tokens.push(Token::new(
            TokenKind::EOF,
            String::new(),
            self.line,
            self.column,
        ));
        Ok(self.tokens)
    }

    fn scan_token(&mut self) -> Result<(), LexError> {
        let c = self.advance();
        match c {
            // الرموز البسيطة (Punctuation)
            '(' => self.add_token(TokenKind::LeftParen),
            ')' => self.add_token(TokenKind::RightParen),
            '{' => self.add_token(TokenKind::LeftBrace),
            '}' => self.add_token(TokenKind::RightBrace),
            '[' => self.add_token(TokenKind::LeftBracket),
            ']' => self.add_token(TokenKind::RightBracket),
            ',' => self.add_token(TokenKind::Comma),
            '.' => self.add_token(TokenKind::Dot),
            ':' => self.add_token(TokenKind::Colon),
            ';' => self.add_token(TokenKind::Semicolon),

            // العمليات الحسابية والمعاملات المركبة
            '+' => self.add_token(TokenKind::Plus),
            '-' => {
                if self.match_char('>') {
                    self.add_token(TokenKind::Arrow)
                } else {
                    self.add_token(TokenKind::Minus)
                }
            }
            '*' => self.add_token(TokenKind::Star),
            '%' => self.add_token(TokenKind::Percent),

            '=' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::EqualEqual)
                } else {
                    self.add_token(TokenKind::Equal)
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::BangEqual)
                } else {
                    self.add_token(TokenKind::Bang)
                }
            }
            '<' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::LessEqual)
                } else {
                    self.add_token(TokenKind::Less)
                }
            }
            '>' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::GreaterEqual)
                } else {
                    self.add_token(TokenKind::Greater)
                }
            }

            // التعليقات أو علامة القسمة
            '/' => {
                if self.match_char('/') {
                    // تعليق سطر واحد: استهلك الحروف حتى نهاية السطر
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_char('*') {
                    // تعليق متعدد الأسطر
                    self.multi_line_comment()?;
                } else {
                    self.add_token(TokenKind::Slash)
                }
            }

            // الفراغات والمسافات (يتم تجاهلها مع تتبع الأسطر)
            ' ' | '\r' | '\t' => {}
            '\n' => {} // دالة advance تتكفل بالسطر تلقائياً

            // النصوص والأرقام والمعرفات
            '"' => self.string_literal()?,

            _ => {
                if c.is_ascii_digit() {
                    self.number_literal();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    return Err(LexError::InvalidCharacter {
                        ch: c,
                        line: self.line,
                        column: self.column - 1,
                    });
                }
            }
        }
        Ok(())
    }

    fn multi_line_comment(&mut self) -> Result<(), LexError> {
        let start_line = self.line;
        let start_col = self.column - 2;

        while !self.is_at_end() {
            if self.peek() == '*' && self.peek_next() == '/' {
                self.advance(); // استهلاك '*'
                self.advance(); // استهلاك '/'
                return Ok(());
            }
            self.advance();
        }
        Err(LexError::UnterminatedComment {
            line: start_line,
            column: start_col,
        })
    }

    pub fn string_literal(&mut self) -> Result<(), LexError> {
        let start_line = self.line;
        let start_col = self.column - 1;

        while self.peek() != '"' && !self.is_at_end() {
            self.advance();
        }

        if self.is_at_end() {
            return Err(LexError::UnterminatedString {
                line: start_line,
                column: start_col,
            });
        }

        self.advance(); // علامة الإغلاق
        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_with_lexeme(TokenKind::String, value);
        Ok(())
    }

    pub fn number_literal(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            self.advance(); // استهلاك '.'
            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value: String = self.source[self.start..self.current].iter().collect();
        self.add_token_with_lexeme(TokenKind::Number, value);
    }

    pub fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        let kind = match lookup_keyword(&text) {
            Some(k) => k,
            None => TokenKind::Identifier,
        };
        self.add_token_with_lexeme(kind, text);
    }

    // دوال حركة المؤشر والمساعدات التابعة للجزء الثاني والثالث
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn advance(&mut self) -> char {
        let ch = self.source[self.current];
        self.current += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        ch
    }

    pub fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    pub fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    pub fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }
        self.advance();
        true
    }

    pub fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }
    pub fn is_alpha_numeric(&self, c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    pub fn add_token(&mut self, kind: TokenKind) {
        let lexeme: String = self.source[self.start..self.current].iter().collect();
        let col = self.column - (self.current - self.start);
        self.tokens.push(Token::new(kind, lexeme, self.line, col));
    }

    pub fn add_token_with_lexeme(&mut self, kind: TokenKind, lexeme: String) {
        let col = self.column - (self.current - self.start);
        self.tokens.push(Token::new(kind, lexeme, self.line, col));
    }
}

// --- وحدة الاختبارات الآلية للتأكد من الكود الثنائي ---
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arabic_and_english_code() {
        let input = r#"
        دع الاسم = "فيصل"
        اطبع(الاسم)
        إذا الاسم == "فيصل"
            اطبع("مرحباً")
        وإلا
            اطبع("أهلاً")

        let name = "Faisal"
        print(name)
        "#;

        let lexer = Lexer::new(input);
        let tokens = lexer.scan_tokens().unwrap();

        // فحص عينة من الرموز المستخرجة للتأكد من سلامتها
        assert_eq!(tokens[0].kind, TokenKind::Let); // "دع"
        assert_eq!(tokens[1].kind, TokenKind::Identifier); // "الاسم"
        assert_eq!(tokens[2].kind, TokenKind::Equal); // "="
        assert_eq!(tokens[3].kind, TokenKind::String); // ""فيصل""
    }
}
