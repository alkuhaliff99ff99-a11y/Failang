#[derive(Debug, Clone)]
pub struct DiagnosticError {
    pub arabic: String,
    pub english: String,
}

impl DiagnosticError {
    pub fn new(arabic: &str, english: &str) -> Self {
        Self {
            arabic: arabic.to_string(),
            english: english.to_string(),
        }
    }

    pub fn display(&self) -> String {
        format!(
            "❌ خطأ أثناء التشغيل | Runtime Error:\nعربي: {}\nEn: {}",
            self.arabic, self.english
        )
    }
}

pub fn translate(message: &str) -> DiagnosticError {
    DiagnosticError::new(message, message)
}

impl DiagnosticError {
    pub fn display_parse_error(token: &crate::compiler::lexer::Token, message: &str) -> String {
        format!(
            "[FSL Parser Error]\n\nالسطر: {}\nالرمز: {}\n\nالمشكلة:\n{}\n\nEnglish:\n{}",
            token.line, token.lexeme, message, message
        )
    }
}
