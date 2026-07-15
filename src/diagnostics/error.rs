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
            "❌ خطأ أثناء التنفيذ | Runtime Error:\n{}\n{}",
            self.arabic,
            self.english
        )
    }
}

pub fn translate(message: &str) -> DiagnosticError {
    let english = match message {
        m if m.contains("لا يمكن القسمة على صفر") => {
            "Error: Division by zero"
        }

        m if m.contains("متغير غير معرف") => {
            "Error: Undefined variable"
        }

        m if m.contains("تجاوز حدود المصفوفة") => {
            "Error: Array index out of bounds"
        }

        m if m.contains("لا يمكن استدعاء هذا الكائن كدالة") => {
            "Error: Object is not callable"
        }

        m if m.contains("خطأ في تمرير المعاملات") => {
            "Error: Function argument mismatch"
        }

        _ => {
            "Error: Runtime failure"
        }
    };

    DiagnosticError::new(message, english)
}
