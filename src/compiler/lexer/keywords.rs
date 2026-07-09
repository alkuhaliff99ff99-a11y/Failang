use super::token::TokenKind;

pub fn lookup_keyword(word: &str) -> Option<TokenKind> {
    match word {
        // English Keywords
        "let" => Some(TokenKind::Let),
        "fn" => Some(TokenKind::Fn),
        "if" => Some(TokenKind::If),
        "else" => Some(TokenKind::Else),
        "while" => Some(TokenKind::While),
        "for" => Some(TokenKind::For),
        "return" => Some(TokenKind::Return),
        "true" => Some(TokenKind::True),
        "false" => Some(TokenKind::False),
        "print" => Some(TokenKind::Print),

        // العربية Keywords
        "دع" => Some(TokenKind::Let),
        "دالة" => Some(TokenKind::Fn),
        "إذا" => Some(TokenKind::If),
        "وإلا" => Some(TokenKind::Else),
        "طالما" => Some(TokenKind::While),
        "لكل" => Some(TokenKind::For),
        "أرجع" => Some(TokenKind::Return),
        "صحيح" => Some(TokenKind::True),
        "خطأ" => Some(TokenKind::False),
        "اطبع" => Some(TokenKind::Print),

        _ => None,
    }
}
