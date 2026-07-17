use fsl::stdlib::registry;

#[test]
fn test_bilingual_stdlib_names() {
    let abs_en = registry::find("abs");
    let abs_ar = registry::find("مطلق");

    assert!(abs_en.is_some());
    assert!(abs_ar.is_some());

    assert_eq!(
        abs_en.unwrap().module,
        abs_ar.unwrap().module
    );
}
