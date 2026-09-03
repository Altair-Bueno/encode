#[test]
fn length_prefix_rejects_non_encodable_length_type() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/length_prefix_non_encodable_length.rs");
}

#[test]
fn from_error_rejects_non_convertible_error_type() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/from_error_non_convertible_error.rs");
}

#[test]
fn iter_rejects_non_iterator_inner_type() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/iter_non_iterator_inner.rs");
}

#[test]
fn le_rejects_non_numeric_inner_type() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/le_non_numeric_inner.rs");
}

#[test]
fn be_rejects_non_numeric_inner_type() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/be_non_numeric_inner.rs");
}
