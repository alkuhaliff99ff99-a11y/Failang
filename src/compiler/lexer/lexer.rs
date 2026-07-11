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

    pub fn scan_tokens(mut self) -> Result<Vec<Token>, LexError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

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

            '+' => {
                if self.match_char('+') {
                    self.add_token(TokenKind::PlusPlus)
                } else if self.match_char('=') {
                    self.add_token(TokenKind::PlusEq)
                } else {
                    self.add_token(TokenKind::Plus)
                }
            }
            '-' => {
                if self.match_char('>') {
                    self.add_token(TokenKind::Arrow)
                } else if self.match_char('-') {
                    self.add_token(TokenKind::MinusMinus)
                } else if self.match_char('=') {
                    self.add_token(TokenKind::MinusEq)
                } else {
                    self.add_token(TokenKind::Minus)
                }
            }
            '*' => {
                if self.match_char('*') {
                    if self.match_char('=') {
                        self.add_token(TokenKind::PowerEq)
                    } else {
                        self.add_token(TokenKind::Power)
                    }
                } else if self.match_char('=') {
                    self.add_token(TokenKind::StarEq)
                } else {
                    self.add_token(TokenKind::Star)
                }
            }
            '%' => {
                if self.match_char('=') {
                    self.add_token(TokenKind::PercentEq)
                } else {
                    self.add_token(TokenKind::Percent)
                }
            }

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

            '&' => {
                if self.match_char('&') {
                    self.add_token(TokenKind::AndAnd)
                } else {
                    return Err(LexError::InvalidCharacter {
                        ch: c,
                        line: self.line,
                        column: self.column - 1,
                    });
                }
            }
            '|' => {
                if self.match_char('|') {
                    self.add_token(TokenKind::OrOr)
                } else {
                    return Err(LexError::InvalidCharacter {
                        ch: c,
                        line: self.line,
                        column: self.column - 1,
                    });
                }
            }

            '/' => {
                if self.match_char('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_char('*') {
                    self.multi_line_comment()?;
                } else if self.match_char('=') {
                    self.add_token(TokenKind::SlashEq)
                } else {
                    self.add_token(TokenKind::Slash)
                }
            }

            ' ' | '\r' | '\t' | '\n' => {}

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
                self.advance();
                self.advance();
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

        self.advance();
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
            self.advance();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arabic_and_english_code() {
        let input = r#"دع الاسم = "فيصل"
        ثابت س = 10
        س += 5
        س++
        2 ** 8
        إذا صحيح && ليس خطأ
        "#;

        let lexer = Lexer::new(input);
        let tokens = lexer.scan_tokens().unwrap();

        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[4].kind, TokenKind::Const);
        assert_eq!(tokens[9].kind, TokenKind::PlusEq);
        assert_eq!(tokens[12].kind, TokenKind::PlusPlus);
        assert_eq!(tokens[15].kind, TokenKind::Power);
        assert_eq!(tokens[18].kind, TokenKind::If);
        assert_eq!(tokens[20].kind, TokenKind::AndAnd);
        assert_eq!(tokens[21].kind, TokenKind::Not);
    }
}
