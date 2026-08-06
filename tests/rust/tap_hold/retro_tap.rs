use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

#[test]
fn long_hold_alone_resolves_as_tap_with_retro_tap() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.retro_tap = true,
                config.tap_hold.timeout = 200,
                config.tap_hold.interrupt_response = "HoldOnKeyPress",
                keys = [
                    K.A & K.hold K.LeftCtrl,
                ],
            }
        "#
    ));

    // Act — press, wait past timeout, release without other keys
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..250 {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.tick_until_no_scheduled_events();

    // Assert — retro-tap: alone release is always tap
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
fn interrupt_press_still_resolves_hold_with_retro_tap() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.retro_tap = true,
                config.tap_hold.timeout = 200,
                config.tap_hold.interrupt_response = "HoldOnKeyPress",
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B,
                ],
            }
        "#
    ));

    // Act — press TH, interrupt with B
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.tick_until_no_scheduled_events();

    // Assert — hold still activates on interrupting press
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, KC_B, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn interrupt_after_timeout_resolves_hold_with_retro_tap() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.retro_tap = true,
                config.tap_hold.timeout = 200,
                config.tap_hold.interrupt_response = "HoldOnKeyPress",
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B,
                ],
            }
        "#
    ));

    // Act — hold past timeout, then interrupt
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..250 {
        keymap.tick();
    }
    // Still no output (pending)
    assert_eq!(
        &[[0, 0, 0, 0, 0, 0, 0, 0]],
        keymap.distinct_reports().reports()
    );

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.tick_until_no_scheduled_events();

    // Assert — hold activates when another key is pressed after timeout
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, KC_B, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
