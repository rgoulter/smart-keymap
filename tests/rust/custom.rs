use smart_keymap::input;

use smart_keymap_macros::keymap;

#[test]
fn key_press_once_and_hold_resolves_as_first_definition() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.custom 255,
                ],
            }
        "#
    );

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });

    // Assert
    let expected_custom_codes: &[u8] = &[255];
    let actual_custom_codes = keymap.report_output().pressed_custom_codes();
    assert_eq!(expected_custom_codes, actual_custom_codes);
}
