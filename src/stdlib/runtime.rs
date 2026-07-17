use crate::stdlib::{math, system, text};

pub fn call(module: &str, function: &str, args: Vec<String>) -> Option<String> {
    match (module, function) {
        ("math", "abs") | ("math", "مطلق") => {
            let value = args.first()?.parse::<f64>().ok()?;
            Some(math::abs(value).to_string())
        }

        ("math", "max") | ("math", "أكبر") => {
            let a = args.first()?.parse::<f64>().ok()?;
            let b = args.get(1)?.parse::<f64>().ok()?;
            Some(math::max(a, b).to_string())
        }

        ("math", "min") | ("math", "أصغر") => {
            let a = args.first()?.parse::<f64>().ok()?;
            let b = args.get(1)?.parse::<f64>().ok()?;
            Some(math::min(a, b).to_string())
        }

        ("text", "length") | ("text", "طول") => Some(text::length(args.first()?).to_string()),

        ("system", "version") | ("system", "إصدار") => Some(system::version().to_string()),

        _ => None,
    }
}
