use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

#[test]
fn pressed_key_does_not_resolve_as_hold_on_timeout() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.timeout = null,
                config.tap_hold.interrupt_response = "HoldOnKeyPress",
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B,
                ],
            }
        "#
    ));

    // Act -- press the tap-hold key and wait longer than the default timeout.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..500 {
        keymap.tick();
    }

    // Assert -- still pending; no timeout was scheduled.
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn release_without_interrupt_resolves_as_tap() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.timeout = null,
                config.tap_hold.interrupt_response = "HoldOnKeyPress",
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B,
                ],
            }
        "#
    ));

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..500 {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_A, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn interrupting_press_still_resolves_as_hold() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.timeout = null,
                config.tap_hold.interrupt_response = "HoldOnKeyPress",
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B,
                ],
            }
        "#
    ));

    // Act -- press the TH key, then interrupt it with another press.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..500 {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, KC_B, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
