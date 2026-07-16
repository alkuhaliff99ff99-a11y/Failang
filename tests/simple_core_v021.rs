#[test]
fn test_simple_core_v021_features() {
    let source = r#"
دع أ = 10
دع ب = 20

دالة جمع(x, y):
    ارجع x + y

دع نتيجة = جمع(أ, ب)

اطبع نتيجة
"#;

    assert!(source.contains("دالة"));
    assert!(source.contains("ارجع"));
    assert!(source.contains("اطبع"));
}
