use std::process::Command;

pub fn build() {
    println!("=== Failang Build | بناء Failang ===");

    let result = Command::new("cargo")
        .arg("check")
        .status();

    match result {
        Ok(s) if s.success() => {
            println!("✅ البناء ناجح | Build successful");
        }
        _ => {
            println!("❌ فشل البناء | Build failed");
        }
    }
}
