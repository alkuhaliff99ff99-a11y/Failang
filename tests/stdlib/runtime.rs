
use fsl::runtime::stdlib;

#[test]
fn test_stdlib_bridge() {
    let result = stdlib::call(
        "math",
        "abs",
        vec!["-50".to_string()],
    );

    assert_eq!(result.unwrap(), "50");
}

#[test]
fn test_arabic_stdlib_bridge() {
    let result = stdlib::call(
        "math",
        "مطلق",
        vec!["-20".to_string()],
    );

    assert_eq!(result.unwrap(), "20");
}

#[test]
fn test_stdlib_registry() {
    assert!(stdlib::exists("abs"));
    assert!(stdlib::exists("مطلق"));
}
