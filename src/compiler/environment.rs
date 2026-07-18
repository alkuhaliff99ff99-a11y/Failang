use std::collections::HashMap;
use crate::compiler::ast::LiteralValue;

pub struct Environment {
    values: HashMap<String, LiteralValue>,
}

impl Environment {
    pub fn new() -> Self {
        Self { values: HashMap::new() }
    }

    pub fn define(&mut self, name: String, value: LiteralValue) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &String) -> LiteralValue {
        match self.values.get(name) {
            Some(value) => value.clone(),
            None => panic!("خطأ: المتغير '{}' غير معرف!", name),
        }
    }
}
