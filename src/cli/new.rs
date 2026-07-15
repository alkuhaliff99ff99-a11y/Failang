use std::fs;

pub fn create(name: &str) {
    println!("=== Failang New | إنشاء مشروع ===");
    let dirs = [
        format!("{}/src", name),
        format!("{}/tests", name),
        format!("{}/examples", name),
    ];

    for dir in dirs {
        if let Err(e) = fs::create_dir_all(&dir) {
            println!("❌ خطأ | Error: {}", e);
            return;
        }
    }

    let main_file = format!("{}/src/main.fsl", name);
    if let Err(e) = fs::write(
        &main_file,
        "متغير رسالة = \"مرحبا من Failang\"\nاطبع رسالة\n"
    ) {
        println!("❌ خطأ | Error: {}", e);
        return;
    }

    let config = format!(
        "[package]\nname = \"{}\"\nversion = \"0.1.0\"\n",
        name
    );
    if let Err(e) = fs::write(
        format!("{}/failang.toml", name),
        config
    ) {
        println!("❌ خطأ | Error: {}", e);
        return;
    }

    if let Err(e) = fs::write(
        format!("{}/README.md", name),
        format!("# {}\n\nFailang Project\n", name)
    ) {
        println!("❌ خطأ | Error: {}", e);
        return;
    }

    println!("✅ تم إنشاء المشروع | Project created: {}", name);
}
