use crate::runtime::value::Value;

pub fn length(value: &str) -> usize {
    value.chars().count()
}

pub fn empty(value: &str) -> bool {
    value.is_empty()
}

pub fn call(name: &str, args: &[Value]) -> Option<Result<Value, String>> {
    match name {
        "length" | "len" | "طول" => {
            if args.len() != 1 {
                return Some(Err("length() requires one argument".to_string()));
            }

            match &args[0] {
                Value::String(s) => Some(Ok(Value::Number(length(s) as f64))),
                Value::Array(items) => Some(Ok(Value::Number(items.len() as f64))),
                _ => Some(Err("length() accepts text or array".to_string())),
            }
        }

        "empty" | "فارغ" => {
            if args.len() != 1 {
                return Some(Err("empty() requires one argument".to_string()));
            }

            match &args[0] {
                Value::String(s) => Some(Ok(Value::Boolean(empty(s)))),
                _ => Some(Err("empty() accepts text only".to_string())),
            }
        }

        _ => None,
    }
}
