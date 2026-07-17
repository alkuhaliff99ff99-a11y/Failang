use crate::stdlib::registry;
use crate::stdlib::runtime as stdlib_runtime;

pub fn call(module: &str, function: &str, args: Vec<String>) -> Option<String> {
    registry::find(function)?;

    stdlib_runtime::call(module, function, args)
}

pub fn exists(name: &str) -> bool {
    registry::find(name).is_some()
}
