use crate::runtime::value::Value;

pub fn abs(value: f64) -> f64 {
    value.abs()
}

pub fn max(a: f64, b: f64) -> f64 {
    a.max(b)
}

pub fn min(a: f64, b: f64) -> f64 {
    a.min(b)
}

pub fn call(name: &str, args: &[Value]) -> Option<Result<Value, String>> {
    match name {
        "abs" | "مطلق" => {
            if args.len() != 1 {
                return Some(Err("abs() requires one argument".to_string()));
            }

            match &args[0] {
                Value::Number(n) => Some(Ok(Value::Number(abs(*n)))),
                _ => Some(Err("abs() requires a number".to_string())),
            }
        }

        "max" | "أكبر" => {
            if args.len() != 2 {
                return Some(Err("max() requires two arguments".to_string()));
            }

            match (&args[0], &args[1]) {
                (Value::Number(a), Value::Number(b)) => Some(Ok(Value::Number(max(*a, *b)))),
                _ => Some(Err("max() requires numbers".to_string())),
            }
        }

        "min" | "أصغر" => {
            if args.len() != 2 {
                return Some(Err("min() requires two arguments".to_string()));
            }

            match (&args[0], &args[1]) {
                (Value::Number(a), Value::Number(b)) => Some(Ok(Value::Number(min(*a, *b)))),
                _ => Some(Err("min() requires numbers".to_string())),
            }
        }

        _ => None,
    }
}
