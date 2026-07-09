use super::error::LexError;
use super::keywords::lookup_keyword;
use super::token::Token;
use super::token_kind::TokenKind;

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

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, LexError> {
        // سيتم تفعيل الحلقة الكاملة في الجزء الرابع والأخير
        Ok(self.tokens)
    }

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

    // --- الجزء الثالث: مستشعرات قراءة الرموز المعقدة ---

    // 1. قراءة السلاسل النصية ودعم الـ UTF-8 (العربية والإنجليزية)
    pub fn string_literal(&mut self) -> Result<(), LexError> {
        let start_line = self.line;
        let start_col = self.column - 1; // موقع علامة الاقتباس الافتتاحية

        while self.peek() != '"' && !self.is_at_end() {
            self.advance();
        }

        if self.is_at_end() {
            return Err(LexError::UnterminatedString {
                line: start_line,
                column: start_col,
            });
        }

        // استهلاك علامة الاقتباس الإغلاقية
        self.advance();

        // استخراج النص البرميجي بدون علامات الاقتباس
        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token_with_lexeme(TokenKind::String, value);
        Ok(())
    }

    // 2. قراءة الأرقام الصحيحة والعشرية (Floats)
    pub fn number_literal(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // التحقق من وجود نقطة عشرية متبوعة برقم (لتجنب الخلط مع استدعاء الدوال مثل 1.print)
        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            // استهلاك النقطة العشرية "."
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        let value: String = self.source[self.start..self.current].iter().collect();
        self.add_token_with_lexeme(TokenKind::Number, value);
    }

    // 3. قراءة المعرفات والكلمات المفتاحية الثنائية (العربية والإنجليزية)
    pub fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();

        // فحص ما إذا كانت الكلمة مفتاحية (مثل "دع" أو "let") وإلا نعتبرها معرف عادي
        let kind = match lookup_keyword(&text) {
            Some(k) => k,
            None => TokenKind::Identifier,
        };

        self.add_token_with_lexeme(kind, text);
    }

    // مساعدات التحقق من الحروف والأرقام لدعم الـ UTF-8 كاملاً
    pub fn is_alpha(&self, c: char) -> bool {
        c.is_alphabetic() || c == '_'
    }

    pub fn is_alpha_numeric(&self, c: char) -> bool {
        c.is_alphanumeric() || c == '_'
    }

    // دوال إضافة التوكنز إلى المصفوفة
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
