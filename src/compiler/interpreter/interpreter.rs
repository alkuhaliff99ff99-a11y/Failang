use crate::runtime::stdlib;
use std::cell::RefCell;
use std::rc::Rc;
use crate::compiler::ast::expression::Expr;
use crate::compiler::interpreter::environment::Environment;
use crate::compiler::interpreter::value::Value;

pub struct Interpreter {
    // البيئة النشطة حالياً أثناء التشغيل
    pub environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }

    // دالة مساعدة لتنفيذ كتلة برمجية (Block) في Scope مخصص لها
    pub fn execute_block(&mut self, statements: &[Stmt], new_env: Environment) -> Result<(), String> {
        // حفظ البيئة السابقة
        let previous = Rc::clone(&self.environment);
        
        // الانتقال للبيئة الجديدة
        self.environment = Rc::new(RefCell::new(new_env));

        // تنفيذ الجمل داخل الـ Scope الجديد
        for statement in statements {
            if let Err(e) = self.execute(statement) {
                // استعادة البيئة السابقة حتى لو حدث خطأ
                self.environment = previous;
                return Err(e);
            }
        }

        // استعادة البيئة السابقة بعد الانتهاء بنجاح
        self.environment = previous;
        Ok(())
    }


    pub fn call_stdlib(
        &self,
        module: &str,
        function: &str,
        args: Vec<String>,
    ) -> Option<String> {
        stdlib::call(module, function, args)
    }
}
