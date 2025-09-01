#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ui/01-pass.rs");
    t.compile_fail("tests/ui/02-fail-nickel-error.rs");
}
