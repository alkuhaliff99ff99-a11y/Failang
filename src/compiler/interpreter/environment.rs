use super::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    // النطاق الأب (خارجي) إن وجد
    enclosing: Option<Rc<RefCell<Environment>>>,
    // جدول المتغيرات الخاص بهذا النطاق الحالي
    values: HashMap<String, Value>,
}

impl Environment {
    // إنشاء النطاق العالمي (Global Scope)
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    // إنشاء نطاق فرعي متداخل (Local Scope)
    pub fn new_with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Self {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }

    // تعريف متغير جديد أو تحديث قيمته في النطاق الحالي
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    // جلب قيمة متغير بالبحث تصاعدياً من النطاق الحالي للأب
    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.values.get(name) {
            return Some(value.clone());
        }

        if let Some(ref enclosing) = self.enclosing {
            return enclosing.borrow().get(name);
        }

        None
    }
}
