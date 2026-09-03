#[test]
fn length_prefix_rejects_non_encodable_length_type() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/length_prefix_non_encodable_length.rs");
}
