#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Nil,
    Bool(bool),
    Number(f64),
    // استخدام Rc (Reference Counting) يضمن مشاركة وتنظيف النصوص في الذاكرة بشكل آمن وممتاز
    String(std::rc::Rc<String>),
}

#[derive(Debug, Clone)]
pub enum OpCode {
    Constant(Value),
    Add,
    Subtract,
    Multiply,
    Divide,
    Pop,
    Return,
}

pub struct VM {
    chunk: Vec<OpCode>,
    ip: usize, // Instruction Pointer
    stack: Vec<Value>,
}

impl VM {
    pub fn new(chunk: Vec<OpCode>) -> Self {
        Self {
            chunk,
            ip: 0,
            stack: Vec::new(),
        }
    }

    // إضافة قيمة للمكدس
    pub fn push(&mut self, value: Value) {
        self.stack.push(value);
    }

    // سحب قيمة من المكدس (مع ضمان التحرير الآمن للذاكرة بفضل نظام Rust الذكي)
    pub fn pop(&mut self) -> Result<Value, String> {
        self.stack.pop().ok_or_else(|| "خطأ في الذاكرة: محاولة السحب من مكدس فارغ (Stack Underflow)".to_string())
    }

    // تشغيل الـ VM
    pub fn run(&mut self) -> Result<Value, String> {
        while self.ip < self.chunk.len() {
            let instruction = &self.chunk[self.ip];
            self.ip += 1;

            match instruction {
                OpCode::Constant(value) => {
                    self.push(value.clone());
                }
                OpCode::Pop => {
                    self.pop()?; // يسحب القيمة ويقوم بتحريرها من الذاكرة فوراً إذا لم تستخدم
                }
                OpCode::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    match (a, b) {
                        (Value::Number(n1), Value::Number(n2)) => self.push(Value::Number(n1 + n2)),
                        (Value::String(s1), Value::String(s2)) => {
                            let concatenated = format!("{}{}", s1, s2);
                            self.push(Value::String(std::rc::Rc::new(concatenated)));
                        }
                        _ => return Err("خطأ في التشغيل: الأنواع غير متطابقة لعملية الجمع".to_string()),
                    }
                }
                OpCode::Return => {
                    if let Some(val) = self.stack.last() {
                        return Ok(val.clone());
                    }
                    return Ok(Value::Nil);
                }
                _ => todo!("بقية العمليات سيتم إضافتها لاحقاً"),
            }
        }
        Ok(Value::Nil)
    }
}
