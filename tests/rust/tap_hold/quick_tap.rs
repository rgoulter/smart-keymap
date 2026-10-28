use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

#[test]
fn re_press_within_quick_tap_ms_forces_tap() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.quick_tap_ms = 175,
                config.tap_hold.timeout = 200,
                keys = [
                    K.A & K.hold K.LeftCtrl,
                ],
            }
        "#
    ));

    // Act — first tap
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    // Re-press soon after first press and hold past timeout
    for _ in 0..50 {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..250 {
        keymap.tick();
    }

    // Assert — second press stays tap (not hold) despite long hold
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_A, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_A, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn re_press_after_quick_tap_ms_allows_hold() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.quick_tap_ms = 100,
                config.tap_hold.timeout = 200,
                keys = [
                    K.A & K.hold K.LeftCtrl,
                ],
            }
        "#
    ));

    // Act — first tap
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    // Wait past quick_tap window, then hold
    for _ in 0..150 {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..250 {
        keymap.tick();
    }

    // Assert — second press resolves as hold after timeout
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_A, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
