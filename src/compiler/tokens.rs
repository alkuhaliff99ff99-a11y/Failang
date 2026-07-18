#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Let, Fn, If, Else, While, Return, Print, True, False, Null,
    Identifier(String), Number(f64), String(String),
    Plus, Minus, Star, Slash, Assign, Equal, NotEqual, Greater, Less,
    LParen, RParen, LBrace, RBrace, LBracket, RBracket, Comma, Colon, Semicolon,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,
    pub line: usize,
}
