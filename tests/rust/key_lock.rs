use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

#[test]
fn key_lock_holds_next_key_until_pressed_again() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.key_lock,
                    K.A,
                    K.B,
                ],
            }
        "#
    ));

    // Act: arm key lock, tap A (should stay held after release)
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    // Press B while A is locked
    keymap.handle_input(input::Event::Press { keymap_index: 2 });

    keymap.tick_until_no_scheduled_events();

    // Assert: A still held (via virtual key) with B pressed
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_A, 0, 0, 0, 0, 0],
        // release A: still held via virtual key (no distinct change if same report)
        [0, 0, KC_A, KC_B, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn key_lock_unlocks_when_same_key_pressed_again() {
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.key_lock,
                    K.A,
                    K.B,
                ],
            }
        "#
    ));

    // Arm and lock A
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    // Unlock A by pressing it again
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    // Press B alone
    keymap.handle_input(input::Event::Press { keymap_index: 2 });

    keymap.tick_until_no_scheduled_events();

    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_A, 0, 0, 0, 0, 0],
        // unlock press: A still held by physical press after VK release
        // unlock release: A gone
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_B, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn key_lock_holds_modifier() {
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.key_lock,
                    K.LeftShift,
                    K.A,
                ],
            }
        "#
    ));

    // Arm, lock LeftShift, type A (should be shifted)
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.handle_input(input::Event::Press { keymap_index: 2 });

    keymap.tick_until_no_scheduled_events();

    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LSHFT, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LSHFT, 0, KC_A, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn key_lock_toggle_watching_cancels_without_locking() {
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.key_lock,
                    K.A,
                ],
            }
        "#
    ));

    // Arm then disarm without locking
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    // Tap A — should not stay held
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_A, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
