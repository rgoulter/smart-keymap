use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

#[test]
fn opposite_hand_press_resolves_hold() {
    // Key 0 is TH; only key 2 may trigger hold (simulating opposite hand).
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.interrupt_response = "HoldOnKeyPress",
                config.tap_hold.timeout = 200,
                keys = [
                    K.A & K.hold K.LeftCtrl & { hold_trigger_key_positions = [2] },
                    K.B,
                    K.C,
                ],
            }
        "#
    ));

    // Act — interrupt with key 2 (allowed trigger)
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    keymap.tick_until_no_scheduled_events();

    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, KC_C, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn same_hand_press_does_not_resolve_hold() {
    // Key 0 is TH; only key 2 may trigger hold. Key 1 is "same hand".
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.interrupt_response = "HoldOnKeyPress",
                config.tap_hold.timeout = 200,
                keys = [
                    K.A & K.hold K.LeftCtrl & { hold_trigger_key_positions = [2] },
                    K.B,
                    K.C,
                ],
            }
        "#
    ));

    // Act — press TH, same-hand B, release both (should be taps, not hold)
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.tick_until_no_scheduled_events();

    // TH self-release resolves as tap (not hold); B is still held during A.
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_A, 0, 0, 0, 0, 0],
        [0, 0, KC_A, KC_B, 0, 0, 0, 0],
        [0, 0, KC_B, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn timeout_still_resolves_hold_with_positional_triggers() {
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.interrupt_response = "HoldOnKeyPress",
                config.tap_hold.timeout = 200,
                keys = [
                    K.A & K.hold K.LeftCtrl & { hold_trigger_key_positions = [2] },
                    K.B,
                    K.C,
                ],
            }
        "#
    ));

    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..250 {
        keymap.tick();
    }

    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
