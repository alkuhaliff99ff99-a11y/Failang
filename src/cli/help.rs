pub fn show() {
    println!("=== Failang CLI | أوامر Failang ===");
    println!();

    println!("Usage:");
    println!("    fsl <command> [options]");
    println!();

    println!("Commands:");
    println!("    run   : تشغيل ملف | Run file");
    println!("            مثال: fsl run main.fsl");
    println!();

    println!("    test  : تشغيل الاختبارات | Run tests");
    println!("            مثال: fsl test");
    println!();

    println!("    build : فحص البناء | Check build");
    println!("            مثال: fsl build");
    println!();

    println!("    new   : إنشاء مشروع | Create project");
    println!("            مثال: fsl new project_name");
    println!();

    println!("    fmt   : تنسيق الكود | Format code");
    println!("            مثال: fsl fmt main.fsl");
    println!();

    println!("    help  : عرض المساعدة | Show help");
    println!();

    println!("Options:");
    println!("    --help       عرض المساعدة");
    println!("    --version    عرض الإصدار");
}
