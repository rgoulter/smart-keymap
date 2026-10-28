//! Classic tri-layer via `conditional_layers`: lower (MO 1) + raise (MO 2) → adjust (3).
//!
//! Keymap indices: 0 = lower, 1 = raise, 2 = letter (A / B / C / D by layer).

use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

#[test]
fn letter_is_base_when_no_mods_held() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                conditional_layers = [
                    { then_layer = 3, if_layers = [1, 2] },
                ],
                layers = [
                    [K.layer_mod.hold 1, K.layer_mod.hold 2, K.A],
                    [K.TTTT, K.TTTT, K.B],
                    [K.TTTT, K.TTTT, K.C],
                    [K.TTTT, K.TTTT, K.D],
                ],
            }
        "#
    ));

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 2 });

    // Assert
    let expected_report: [u8; 8] = [0, 0, KC_A, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, keymap.boot_keyboard_report());
}

#[test]
fn lower_alone_does_not_activate_adjust() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                conditional_layers = [
                    { then_layer = 3, if_layers = [1, 2] },
                ],
                layers = [
                    [K.layer_mod.hold 1, K.layer_mod.hold 2, K.A],
                    [K.TTTT, K.TTTT, K.B],
                    [K.TTTT, K.TTTT, K.C],
                    [K.TTTT, K.TTTT, K.D],
                ],
            }
        "#
    ));

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 }); // lower
    keymap.handle_input(input::Event::Press { keymap_index: 2 }); // letter

    // Assert
    let expected_report: [u8; 8] = [0, 0, KC_B, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, keymap.boot_keyboard_report());
}

#[test]
fn raise_alone_does_not_activate_adjust() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                conditional_layers = [
                    { then_layer = 3, if_layers = [1, 2] },
                ],
                layers = [
                    [K.layer_mod.hold 1, K.layer_mod.hold 2, K.A],
                    [K.TTTT, K.TTTT, K.B],
                    [K.TTTT, K.TTTT, K.C],
                    [K.TTTT, K.TTTT, K.D],
                ],
            }
        "#
    ));

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 1 }); // raise
    keymap.handle_input(input::Event::Press { keymap_index: 2 }); // letter

    // Assert: layer 2 only → C (not adjust D)
    let expected_report: [u8; 8] = [0, 0, KC_C, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, keymap.boot_keyboard_report());
}

#[test]
fn lower_and_raise_activate_adjust() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                conditional_layers = [
                    { then_layer = 3, if_layers = [1, 2] },
                ],
                layers = [
                    [K.layer_mod.hold 1, K.layer_mod.hold 2, K.A],
                    [K.TTTT, K.TTTT, K.B],
                    [K.TTTT, K.TTTT, K.C],
                    [K.TTTT, K.TTTT, K.D],
                ],
            }
        "#
    ));

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 }); // lower
    keymap.handle_input(input::Event::Press { keymap_index: 1 }); // raise
    keymap.handle_input(input::Event::Press { keymap_index: 2 }); // letter

    // Assert
    let expected_report: [u8; 8] = [0, 0, KC_D, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, keymap.boot_keyboard_report());
}

#[test]
fn init_resets_active_layers_before_raise() {
    // Assemble: hold lower (the Ceedling isolation case)
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                conditional_layers = [
                    { then_layer = 3, if_layers = [1, 2] },
                ],
                layers = [
                    [K.layer_mod.hold 1, K.layer_mod.hold 2, K.A],
                    [K.TTTT, K.TTTT, K.B],
                    [K.TTTT, K.TTTT, K.C],
                    [K.TTTT, K.TTTT, K.D],
                ],
            }
        "#
    ));
    keymap.handle_input(input::Event::Press { keymap_index: 0 }); // lower

    // Act: init must clear lower so raise alone is not tri-layer
    keymap.init();
    keymap.handle_input(input::Event::Press { keymap_index: 1 }); // raise
    keymap.handle_input(input::Event::Press { keymap_index: 2 }); // letter

    // Assert
    let expected_report: [u8; 8] = [0, 0, KC_C, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, keymap.boot_keyboard_report());
}

#[test]
fn releasing_if_layer_deactivates_adjust() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                conditional_layers = [
                    { then_layer = 3, if_layers = [1, 2] },
                ],
                layers = [
                    [K.layer_mod.hold 1, K.layer_mod.hold 2, K.A],
                    [K.TTTT, K.TTTT, K.B],
                    [K.TTTT, K.TTTT, K.C],
                    [K.TTTT, K.TTTT, K.D],
                ],
            }
        "#
    ));
    keymap.handle_input(input::Event::Press { keymap_index: 0 }); // lower
    keymap.handle_input(input::Event::Press { keymap_index: 1 }); // raise

    // Act
    keymap.handle_input(input::Event::Release { keymap_index: 0 }); // release lower
    keymap.handle_input(input::Event::Press { keymap_index: 2 }); // letter

    // Assert: only raise remains → C
    let expected_report: [u8; 8] = [0, 0, KC_C, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, keymap.boot_keyboard_report());
}
