use crate::runtime::value::Value;
use crate::stdlib::math;
use crate::stdlib::registry;
use crate::stdlib::system;
use crate::stdlib::text;

pub fn call(module: &str, function: &str, args: Vec<Value>) -> Option<Result<Value, String>> {
    registry::find(function)?;

    match module {
        "math" => math::call(function, &args),
        "text" => text::call(function, &args),
        "system" => system::call(function, &args),
        _ => None,
    }
}

pub fn exists(name: &str) -> bool {
    registry::find(name).is_some()
}
