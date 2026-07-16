use std::fs;
use std::process::Command;

pub fn run_tests() {
    println!("=== Failang Test Runner | مشغل اختبارات Failang ===");

    let dirs = ["tests/language", "tests/runtime"];

    let mut failed = false;

    for dir in dirs {
        if let Ok(entries) = fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();

                if path.extension().and_then(|s| s.to_str()) == Some("fsl") {
                    println!("--- {} ---", path.display());

                    let result = Command::new("cargo")
                        .args(["run", "--"])
                        .arg(path.to_str().unwrap())
                        .status();

                    match result {
                        Ok(status) if status.success() => {}
                        _ => {
                            println!("❌ فشل الاختبار | Test failed: {}", path.display());
                            failed = true;
                        }
                    }
                }
            }
        }
    }

    if failed {
        println!("❌ بعض الاختبارات فشلت | Some tests failed");
    } else {
        println!("✅ جميع الاختبارات نجحت | All tests passed");
    }
}
