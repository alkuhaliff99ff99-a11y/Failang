use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use super::value::Value;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, Value>,
    pub enclosing: Option<Arc<Mutex<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_with_enclosing(enclosing: Arc<Mutex<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<Value, String> {
        if let Some(value) = self.values.get(name) {
            return Ok(value.clone());
        }
        if let Some(ref enclosing) = self.enclosing {
            return enclosing.lock().unwrap().get(name);
        }
        Err(format!("متغير غير معرف '{}'", name))
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            return Ok(());
        }
        if let Some(ref enclosing) = self.enclosing {
            return enclosing.lock().unwrap().assign(name, value);
        }
        Err(format!("محاولة تعيين قيمة لمتغير غير معرف '{}'", name))
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
