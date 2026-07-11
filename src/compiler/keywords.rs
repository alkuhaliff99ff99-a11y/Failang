use super::token::TokenKind;

pub fn lookup_keyword(word: &str) -> Option<TokenKind> {
    match word {
        // English Keywords
        "let" => Some(TokenKind::Let),
        "const" => Some(TokenKind::Const),
        "var" => Some(TokenKind::Var),
        "fn" => Some(TokenKind::Fn),
        "if" => Some(TokenKind::If),
        "else" => Some(TokenKind::Else),
        "while" => Some(TokenKind::While),
        "for" => Some(TokenKind::For),
        "return" => Some(TokenKind::Return),
        "true" => Some(TokenKind::True),
        "false" => Some(TokenKind::False),
        "print" => Some(TokenKind::Print),
        "and" => Some(TokenKind::And),
        "or" => Some(TokenKind::Or),
        "not" => Some(TokenKind::Not),

        // العربية Keywords
        "دع" => Some(TokenKind::Let),
        "ثابت" => Some(TokenKind::Const),
        "متغير" => Some(TokenKind::Var),
        "دالة" => Some(TokenKind::Fn),
        "إذا" => Some(TokenKind::If),
        "وإلا" => Some(TokenKind::Else),
        "طالما" => Some(TokenKind::While),
        "لكل" => Some(TokenKind::For),
        "أرجع" => Some(TokenKind::Return),
        "صحيح" => Some(TokenKind::True),
        "خطأ" => Some(TokenKind::False),
        "اطبع" => Some(TokenKind::Print),
        "و" => Some(TokenKind::And),
        "أو" => Some(TokenKind::Or),
        "ليس" => Some(TokenKind::Not),

        _ => None,
    }
}
