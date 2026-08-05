use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

#[test]
fn tlex_acts_as_base_and_deactivates_toggle_layer() {
    // Assemble: toggle layer 1, then tlex over base A
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.toggle 1, K.A],
                    [K.TTTT, K.tlex],
                ],
            }
        "#
    ));

    // Act: enable layer 1, press tlex key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    // Assert: this press is base A, and layer is off afterward
    let expected_report: [u8; 8] = [0, 0, KC_A, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, keymap.boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    // Second press of the same key still base (layer exited)
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    assert_eq!(expected_report, keymap.boot_keyboard_report());
}

#[test]
fn tlex_differs_from_full_transparency() {
    // Without tlex, pressing B on layer 1 then a transparent key leaves layer on.
    // With tlex on a different key pattern: after tlex, other layer-1 keys fall to base.
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.toggle 1, K.A, K.C],
                    [K.TTTT, K.tlex, K.B],
                ],
            }
        "#
    ));

    // Enable layer 1
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    // Layer-1 key B works
    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    let expected_b: [u8; 8] = [0, 0, KC_B, 0, 0, 0, 0, 0];
    assert_eq!(expected_b, keymap.boot_keyboard_report());
    keymap.handle_input(input::Event::Release { keymap_index: 2 });

    // tlex on index 1 → base A, deactivates layer 1
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    let expected_a: [u8; 8] = [0, 0, KC_A, 0, 0, 0, 0, 0];
    assert_eq!(expected_a, keymap.boot_keyboard_report());
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    // Same physical key that was B is now base C
    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    let expected_c: [u8; 8] = [0, 0, KC_C, 0, 0, 0, 0, 0];
    assert_eq!(expected_c, keymap.boot_keyboard_report());
}

#[test]
fn tlex_under_hold_layer_still_active_resolves_base_for_this_press() {
    // Hold-layer still held: tlex still continue-evals for this press and emits Deactivated.
    // Hold release also deactivates (idempotent).
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.hold 1, K.A],
                    [K.TTTT, K.tlex],
                ],
            }
        "#
    ));

    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    let expected_a: [u8; 8] = [0, 0, KC_A, 0, 0, 0, 0, 0];
    assert_eq!(expected_a, keymap.boot_keyboard_report());
}
