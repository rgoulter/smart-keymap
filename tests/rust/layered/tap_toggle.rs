use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

#[test]
fn hold_tap_toggle_activates_layer_while_pressed() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.tap_toggle 1, K.A],
                    [K.TTTT, K.B],
                ],
            }
        "#
    ));

    // Act: hold TT and press the other key (interrupt → hold path)
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    // Assert
    let expected_report: [u8; 8] = [0, 0, KC_B, 0, 0, 0, 0, 0];
    let actual_report = keymap.boot_keyboard_report();
    assert_eq!(expected_report, actual_report);
}

#[test]
fn hold_tap_toggle_deactivates_layer_on_release() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.tap_toggle 1, K.A],
                    [K.TTTT, K.B],
                ],
            }
        "#
    ));

    // Act: hold TT, interrupt with layer key, release both, then press base key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    // Assert: back on base layer
    let expected_report: [u8; 8] = [0, 0, KC_A, 0, 0, 0, 0, 0];
    let actual_report = keymap.boot_keyboard_report();
    assert_eq!(expected_report, actual_report);
}

#[test]
fn tap_tap_toggle_activates_layer() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.tap_toggle 1, K.A],
                    [K.TTTT, K.B],
                ],
            }
        "#
    ));

    // Act: tap TT (no interrupt), then press the other key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    // Assert: layer stays toggled on
    let expected_report: [u8; 8] = [0, 0, KC_B, 0, 0, 0, 0, 0];
    let actual_report = keymap.boot_keyboard_report();
    assert_eq!(expected_report, actual_report);
}

#[test]
fn tap_tap_toggle_twice_deactivates_layer() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.tap_toggle 1, K.A],
                    [K.TTTT, K.B],
                ],
            }
        "#
    ));

    // Act: tap TT twice, then press the other key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    // Assert: layer toggled off
    let expected_report: [u8; 8] = [0, 0, KC_A, 0, 0, 0, 0, 0];
    let actual_report = keymap.boot_keyboard_report();
    assert_eq!(expected_report, actual_report);
}

#[test]
fn hold_of_toggled_on_layer_stays_active_after_release() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.tap_toggle 1, K.A],
                    [K.TTTT, K.B],
                ],
            }
        "#
    ));

    // Act: tap to toggle layer on, then hold TT with interrupt, release TT, press key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    // Layer is on. Hold TT and interrupt with another key, then release TT.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    // Assert: still on toggled layer
    let expected_report: [u8; 8] = [0, 0, KC_B, 0, 0, 0, 0, 0];
    let actual_report = keymap.boot_keyboard_report();
    assert_eq!(expected_report, actual_report);
}
