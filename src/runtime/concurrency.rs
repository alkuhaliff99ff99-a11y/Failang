use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

// تمثيل لقفل البيانات المشتركة لحمايتها من الـ Data Races
#[derive(Clone)]
pub struct SharedData<T> {
    data: Arc<Mutex<T>>,
}

impl<T> SharedData<T> {
    pub fn new(val: T) -> Self {
        Self {
            data: Arc::new(Mutex::new(val)),
        }
    }

    // تعديل القيمة بأمان تام باستخدام القفل (Mutex)
    pub fn update<F>(&self, f: F) -> Result<(), String>
    where
        F: FnOnce(&mut T),
    {
        let mut lock = self.data.lock().map_err(|_| "فشل في الحصول على قفل الذاكرة المشتركة".to_string())?;
        f(&mut lock);
        Ok(())
    }
}

// قنوات الاتصال الآمنة بين الخيوط (Channels)
pub struct FailangChannel<T> {
    sender: mpsc::Sender<T>,
    receiver: Arc<Mutex<mpsc::Receiver<T>>>,
}

impl<T> FailangChannel<T> {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Self {
            sender: tx,
            receiver: Arc::new(Mutex::new(rx)),
        }
    }

    // إرسال رسالة إلى خيط آخر
    pub fn send(&self, msg: T) -> Result<(), String> {
        self.sender.send(msg).map_err(|_| "فشل في إرسال البيانات عبر القناة".to_string())
    }

    // استقبال رسالة بأمان
    pub fn recv(&self) -> Result<T, String> {
        let rx = self.receiver.lock().map_err(|_| "فشل قفل المستقبل".to_string())?;
        rx.recv().map_err(|_| "القناة مغلقة، لا يمكن الاستقبال".to_string())
    }
}

// دالة لتشغيل المهام في الخلفية بالتوازي
pub fn spawn_task<F>(f: F) -> thread::JoinHandle<()>
where
    F: FnOnce() + Send + 'static,
{
    thread::spawn(f)
}

// دالة لإيقاف الخيط مؤقتاً
pub fn sleep_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}
