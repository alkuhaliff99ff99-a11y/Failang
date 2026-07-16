use crate::compiler::interpreter::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc; // تأكد من مطابقة مسار الـ Value لديك

#[derive(Clone, Debug)]
pub struct Environment {
    // جدول المتغيرات الخاصة بالـ Scope الحالي
    values: HashMap<String, Value>,
    // مؤشر آمن للـ Scope الأب (العلوي)
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    // إنشاء Scope رئيسي (Global Scope)
    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    // إنشاء Scope فرعي (Local Scope) يشير إلى Scope أب
    pub fn new_with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }
    }

    // تعريف متغير جديد في الـ Scope الحالي حصراً
    pub fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    // جلب قيمة متغير (البحث يبدأ من الـ Scope الحالي صعوداً للأب)
    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.values.get(name) {
            return Some(value.clone());
        }

        // إذا لم يجده، يبحث في الـ Scope الأب تراجعياً
        if let Some(ref enclosing) = self.enclosing {
            return enclosing.borrow().get(name);
        }

        None
    }

    // تعديل قيمة متغير موجود مسبقاً (يبحث صعوداً للأب لتعديله في مكانه الأصلي)
    pub fn assign(&mut self, name: &str, value: Value) -> Result<(), String> {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            return Ok(());
        }

        if let Some(ref enclosing) = self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
        }

        Err(format!("المتغير '{}' غير معرف برمجياً.", name))
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
