use smart_keymap_macros::keymap;

// This is not a real test, but rather a compile-time check
// that the macro expands to valid code.
// The code itself is not executed.
fn main() {
    let _ = keymap!(
        r#"
        {
            keys = [
                { key_code = 4 },
            ],
        }
        "#
    );
}
