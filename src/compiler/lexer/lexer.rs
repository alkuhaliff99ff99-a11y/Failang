use super::error::LexError;
use super::token::Token;

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
        // سيتم تفعيل حلقة الفحص الشاملة في الأجزاء القادمة
        Ok(self.tokens)
    }

    // --- الجزء الثاني: محرك حركة المؤشر وإدارة التموضع المركزي ---

    // 1. التحقق من الوصول لنهاية الملف
    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // 2. التقدم خطوة للأمام وإدارة السطور والأعمدة مركزياً ودعم الـ UTF-8
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

    // 3. فحص الحرف الحالي دون استهلاكه (Lookahead 1)
    pub fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.source[self.current]
        }
    }

    // 4. فحص الحرف التالي للحالي دون استهلاكه (Lookahead 2) - بعد إصلاح الـ Type الملاحظ سابقاً
    pub fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source[self.current + 1]
        }
    }

    // 5. التقدم خطوة فقط إذا كان الحرف الحالي يطابق المتوقع (تستخدم للمعاملات المركبة مثل == أو <=)
    pub fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }

        // استهلاك الحرف عبر دالة advance لضمان تحديث الـ column بشكل سليم ومركزي
        self.advance();
        true
    }
}
