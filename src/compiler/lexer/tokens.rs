#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Number,
    String,
    Identifier,
    
    // الكلمات المفتاحية
    Let,     // دع / let
    Print,   // اطبع / print
    If,      // إذا / if
    Else,    // وإلا / else
    While,   // كرر / while
    
    // الرموز
    Plus,    // +
    Less,    // <
    Equal,   // =
    LBrace,  // {
    RBrace,  // }
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub value: Option<String>,
}

impl Token {
    pub fn new(token_type: TokenType, value: Option<String>) -> Self {
        Token { token_type, value }
    }
}
