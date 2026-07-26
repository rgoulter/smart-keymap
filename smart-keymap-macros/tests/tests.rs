#[test]
fn ui_pass() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/01-pass.rs");
}

#[test]
fn ui_compile_fail() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/ui/02-fail-nickel-error.rs");
}
