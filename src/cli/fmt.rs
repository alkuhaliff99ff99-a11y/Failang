use crate::tools::formatter::Formatter;
use std::env;
use std::process;

pub fn format() {
    let args: Vec<String> = env::args().collect();
    
    // سيتوقع الأمر تمرير مسار الملف مثل: fsl fmt filename.fsl
    // في بيئة التشغيل عبر cargo run، تكون المعاملات كالتالي:
    // args[0] = target/debug/fsl
    // args[1] = fmt
    // args[2] = tests/language/format_test.fsl
    if args.len() < 3 {
        println!("❌ خطأ: يرجى تحديد مسار الملف لتنسيقه.");
        println!("الاستخدام: fsl fmt [path/to/file.fsl]");
        process::exit(1);
    }

    let file_path = &args[2];
    println!("🧹 جاري تنسيق وترتيب الملف: {} ...", file_path);

    match Formatter::format_file(file_path) {
        Ok(_) => println!("✨ تم تنسيق وترتيب الملف بنجاح!"),
        Err(e) => {
            eprintln!("❌ فشل التنسيق: {}", e);
            process::exit(1);
        }
    }
}
