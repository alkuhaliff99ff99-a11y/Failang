use std::fs;
use std::path::Path;

pub struct Formatter;

impl Formatter {
    pub fn format_file<P: AsRef<Path>>(path: P) -> Result<(), String> {
        let content =
            fs::read_to_string(&path).map_err(|e| format!("Failed to read file: {}", e))?;

        let formatted = Self::format_source(&content);

        fs::write(&path, formatted).map_err(|e| format!("Failed to write file: {}", e))?;

        Ok(())
    }

    pub fn format_source(source: &str) -> String {
        let mut formatted_lines = Vec::new();

        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                formatted_lines.push(String::new());
                continue;
            }

            let leading_spaces = line.len() - line.trim_start().len();
            let indent = " ".repeat(leading_spaces);

            // تنسيق السطر مع حماية النصوص البرمجية داخل علامات الاقتباس
            let formatted_line = Self::format_line_safely(trimmed);

            formatted_lines.push(format!("{}{}", indent, formatted_line));
        }

        let mut result = formatted_lines.join("\n");
        if !result.ends_with('\n') && !result.is_empty() {
            result.push('\n');
        }
        result
    }

    // تقسيم السطر لتنسيق ما خارج علامات الاقتباس فقط
    fn format_line_safely(line: &str) -> String {
        let mut parts = Vec::new();
        let mut in_string = false;
        let mut current_part = String::new();

        for ch in line.chars() {
            if ch == '"' {
                if in_string {
                    // نهاية النص، نغلق النص ونضيفه بدون أي تنسيق
                    current_part.push(ch);
                    parts.push((current_part.clone(), true));
                    current_part.clear();
                    in_string = false;
                } else {
                    // بداية النص، ننسق ما قبله أولاً ثم نفتحه
                    if !current_part.is_empty() {
                        parts.push((current_part.clone(), false));
                        current_part.clear();
                    }
                    current_part.push(ch);
                    in_string = true;
                }
            } else {
                current_part.push(ch);
            }
        }

        if !current_part.is_empty() {
            parts.push((current_part, in_string));
        }

        // الآن ننسق الأجزاء التي ليست نصوصاً برمجية فقط
        let mut final_line = String::new();
        for (part, is_str) in parts {
            if is_str {
                final_line.push_str(&part);
            } else {
                final_line.push_str(&Self::format_operators(&part));
            }
        }
        final_line
    }

    fn format_operators(part: &str) -> String {
        let mut formatted = part.to_string();

        // 1. تنسيق العمليات الحسابية الأساسية (+, -, *, /)
        let math_ops = vec!["+", "-", "*", "/"];
        for op in math_ops {
            let temp_placeholder = format!("__{}__", op);
            formatted = formatted.replace(op, &temp_placeholder);
            formatted = formatted
                .replace(&format!(" {}", temp_placeholder), &temp_placeholder)
                .replace(&format!("{} ", temp_placeholder), &temp_placeholder)
                .replace(&temp_placeholder, &format!(" {} ", op));
        }

        // 2. تنسيق عامل التعيين "=" بحذر (بشرط ألا يكون جزءاً من "==" أو "!=" أو ">=")
        // لتجنب تعقيد التعبيرات النمطية، سنقوم بحماية "==" أولاً بوضع علامة مؤقتة لها
        formatted = formatted.replace("==", "__DOUBLE_EQUAL__");
        formatted = formatted.replace("!=", "__NOT_EQUAL__");
        formatted = formatted.replace(">=", "__GREATER_EQUAL__");
        formatted = formatted.replace("<=", "__LESS_EQUAL__");

        // الآن ننسق الـ "=" الفردية بأمان
        formatted = formatted.replace("=", " __EQUAL__ ");
        // تنظيف أي مسافات ثنائية نتجت حولها
        while formatted.contains("  __EQUAL__") {
            formatted = formatted.replace("  __EQUAL__", " __EQUAL__");
        }
        while formatted.contains("__EQUAL__  ") {
            formatted = formatted.replace("__EQUAL__  ", "__EQUAL__ ");
        }
        formatted = formatted.replace("__EQUAL__", "=");

        // استعادة الرموز المركبة كما هي دون المساس بها
        formatted = formatted.replace("__DOUBLE_EQUAL__", "==");
        formatted = formatted.replace("__NOT_EQUAL__", "!=");
        formatted = formatted.replace("__GREATER_EQUAL__", ">=");
        formatted = formatted.replace("__LESS_EQUAL__", "<=");

        // 3. توحيد الفراغات حول "يساوي" و "مساوي" العربية
        formatted = formatted
            .replace(" يساوي ", "يساوي")
            .replace("يساوي", " يساوي ");
        formatted = formatted
            .replace(" مساوي ", "مساوي")
            .replace("مساوي", " مساوي ");

        // تنظيف الفراغات المتكررة الناتجة عن التنسيق
        while formatted.contains("  ") {
            formatted = formatted.replace("  ", " ");
        }

        formatted
    }
}
