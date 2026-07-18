use std::collections::HashMap;
use std::str::Chars;
use std::iter::Peekable;
use crate::compiler::tokens::{Token, TokenKind};

pub struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
    keywords: HashMap<&'static str, TokenKind>,
    line: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut keywords = HashMap::new();
        keywords.insert("متغير", TokenKind::Let);
        keywords.insert("دع", TokenKind::Let);
        keywords.insert("let", TokenKind::Let);
        keywords.insert("دالة", TokenKind::Fn);
        keywords.insert("fn", TokenKind::Fn);
        keywords.insert("إذا", TokenKind::If);
        keywords.insert("if", TokenKind::If);
        keywords.insert("اطبع", TokenKind::Print);
        keywords.insert("اكتب", TokenKind::Print);
        keywords.insert("print", TokenKind::Print);
        keywords.insert("ارجع", TokenKind::Return);
        keywords.insert("أرجع", TokenKind::Return);
        keywords.insert("return", TokenKind::Return);
        keywords.insert("يساوي", TokenKind::Equal);

        Self {
            input: input.chars().peekable(),
            keywords,
            line: 1,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(&ch) = self.input.peek() {
            match ch {
                ' ' | '\t' | '\r' => { self.input.next(); }
                '\n' => {
                    self.line += 1;
                    self.input.next();
                }
                '#' => { // دعم تخطي التعليقات مباشرة هنا
                    self.input.next();
                    while let Some(&comment_ch) = self.input.peek() {
                        if comment_ch == '\n' { break; }
                        self.input.next();
                    }
                }
                '(' => tokens.push(self.make_token(TokenKind::LParen, "(")),
                ')' => tokens.push(self.make_token(TokenKind::RParen, ")")),
                '{' => tokens.push(self.make_token(TokenKind::LBrace, "{")),
                '}' => tokens.push(self.make_token(TokenKind::RBrace, "}")),
                '+' => tokens.push(self.make_token(TokenKind::Plus, "+")),
                '-' => tokens.push(self.make_token(TokenKind::Minus, "-")),
                '*' => tokens.push(self.make_token(TokenKind::Star, "*")),
                '/' => tokens.push(self.make_token(TokenKind::Slash, "/")),
                '=' => tokens.push(self.make_token(TokenKind::Assign, "=")),
                '"' => tokens.push(self.read_string()),
                '0'..='9' => tokens.push(self.read_number()),
                _ if ch.is_alphabetic() || ch == '_' => tokens.push(self.read_identifier()),
                _ => { self.input.next(); }
            }
        }
        tokens.push(Token { kind: TokenKind::EOF, lexeme: "".into(), line: self.line });
        tokens
    }

    fn make_token(&mut self, kind: TokenKind, lexeme: &str) -> Token {
        self.input.next();
        Token { kind, lexeme: lexeme.to_string(), line: self.line }
    }

    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        while let Some(&ch) = self.input.peek() {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(self.input.next().unwrap());
            } else {
                break;
            }
        }
        let kind = self.keywords.get(identifier.as_str())
            .cloned()
            .unwrap_or(TokenKind::Identifier(identifier.clone()));
        Token { kind, lexeme: identifier, line: self.line }
    }

    fn read_number(&mut self) -> Token {
        let mut number = String::new();
        while let Some(&ch) = self.input.peek() {
            if ch.is_ascii_digit() || ch == '.' {
                number.push(self.input.next().unwrap());
            } else {
                break;
            }
        }
        let val: f64 = number.parse().unwrap_or(0.0);
        Token { kind: TokenKind::Number(val), lexeme: number, line: self.line }
    }

    fn read_string(&mut self) -> Token {
        self.input.next();
        let mut string = String::new();
        while let Some(&ch) = self.input.peek() {
            if ch == '"' {
                self.input.next();
                break;
            }
            string.push(self.input.next().unwrap());
        }
        Token { kind: TokenKind::String(string.clone()), lexeme: string, line: self.line }
    }
}
