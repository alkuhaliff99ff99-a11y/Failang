use crate::runtime::value::Value;

pub fn version() -> &'static str {
    "FSL 0.2.3"
}

pub fn call(name: &str, args: &[Value]) -> Option<Result<Value, String>> {
    match name {
        "version" | "إصدار" => {
            if !args.is_empty() {
                return Some(Err("version() takes no arguments".to_string()));
            }

            Some(Ok(Value::String(version().to_string())))
        }

        _ => None,
    }
}
