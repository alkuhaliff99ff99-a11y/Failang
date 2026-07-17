use std::process::Command;

#[test]
fn test_cli_help() {
    let output = Command::new(env!("CARGO_BIN_EXE_fsl"))
        .arg("--help")
        .output()
        .expect("failed to run fsl");

    assert!(output.status.success());

    let text = String::from_utf8_lossy(&output.stdout);

    assert!(text.contains("Failang CLI"));
    assert!(text.contains("run"));
    assert!(text.contains("build"));
}
