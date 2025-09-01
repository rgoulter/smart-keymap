use smart_keymap_macros::keymap;

fn main() {
    let _ = keymap!(
        r#"
        {
            keys = [
                { invalid_field = 4 },
            ],
        }
        "#
    );
}
