use crate::hid_keycodes::*;
use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;
use smart_keymap_macros::keymap;

#[test]
fn press_modified_hold_reports_as_modifier() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.hold 1 & { modifiers = { left_shift = true } }, K.A],
                    [K.TTTT, K.B],
                ],
            }
        "#
    ));

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    let expected_reports: &[[u8; 8]] =
        &[[0, 0, 0, 0, 0, 0, 0, 0], [MOD_LSHFT, 0, 0, 0, 0, 0, 0, 0]];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn press_modified_hold_modifies_layer() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.hold 1 & { modifiers = { left_shift = true } }, K.A],
                    [K.TTTT, K.B],
                ],
            }
        "#
    ));

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LSHFT, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LSHFT, 0, KC_B, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
