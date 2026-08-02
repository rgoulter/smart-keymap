use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

#[test]
fn lock_keeps_layer_after_hold_released() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.hold 1, K.layer_mod.lock, K.A],
                    [K.TTTT, K.TTTT, K.B],
                ],
            }
        "#
    ));

    // Act: hold layer, lock, release hold, press letter
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 2 });

    // Assert
    let expected_report: [u8; 8] = [0, 0, KC_B, 0, 0, 0, 0, 0];
    let actual_report = keymap.boot_keyboard_report();
    assert_eq!(expected_report, actual_report);
}

#[test]
fn lock_again_unlocks_layer() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.hold 1, K.layer_mod.lock, K.A],
                    [K.TTTT, K.TTTT, K.B],
                ],
            }
        "#
    ));

    // Act: lock layer, unlock with lock again, press letter
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.handle_input(input::Event::Press { keymap_index: 2 });

    // Assert
    let expected_report: [u8; 8] = [0, 0, KC_A, 0, 0, 0, 0, 0];
    let actual_report = keymap.boot_keyboard_report();
    assert_eq!(expected_report, actual_report);
}

#[test]
fn hold_while_locked_unlocks() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.hold 1, K.layer_mod.lock, K.A],
                    [K.TTTT, K.TTTT, K.B],
                ],
            }
        "#
    ));

    // Act: lock, then tap hold again to unlock
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 2 });

    // Assert
    let expected_report: [u8; 8] = [0, 0, KC_A, 0, 0, 0, 0, 0];
    let actual_report = keymap.boot_keyboard_report();
    assert_eq!(expected_report, actual_report);
}

#[test]
fn lock_layer_activates_specific_layer() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.lock_layer 1, K.A],
                    [K.TTTT, K.B],
                ],
            }
        "#
    ));

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    // Assert
    let expected_report: [u8; 8] = [0, 0, KC_B, 0, 0, 0, 0, 0];
    let actual_report = keymap.boot_keyboard_report();
    assert_eq!(expected_report, actual_report);
}

#[test]
fn lock_layer_twice_deactivates() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.lock_layer 1, K.A],
                    [K.TTTT, K.B],
                ],
            }
        "#
    ));

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    // Assert
    let expected_report: [u8; 8] = [0, 0, KC_A, 0, 0, 0, 0, 0];
    let actual_report = keymap.boot_keyboard_report();
    assert_eq!(expected_report, actual_report);
}
