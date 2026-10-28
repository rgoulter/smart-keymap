use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

/// Profile 0 (default) ignores interrupts; named profile forces hold-on-press.
#[test]
fn named_profile_uses_distinct_interrupt_response() {
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold = {
                    # default profile: ignore interrupts (hold only on timeout)
                    interrupt_response = "Ignore",
                    timeout = 200,
                    profiles = {
                        aggressive = {
                            interrupt_response = "HoldOnKeyPress",
                            timeout = 200,
                        },
                    },
                },
                keys = [
                    # default profile — interrupt does not force hold
                    K.A & K.hold K.LeftCtrl,
                    K.B,
                    # named profile — interrupt forces hold
                    K.C & K.hold K.LeftShift & K.tap_hold_profile "aggressive",
                ],
            }
        "#
    ));

    // Default-profile key: press TH, press B, release TH → tap (Ignore)
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.tick_until_no_scheduled_events();

    // Same order as interrupt_ignore::rolled_presses (pending TH, then B, release TH as tap).
    #[rustfmt::skip]
    let expected_after_default: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_A, 0, 0, 0, 0, 0],
        [0, 0, KC_A, KC_B, 0, 0, 0, 0],
        [0, 0, KC_B, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(
        expected_after_default,
        keymap.distinct_reports().reports(),
        "default profile should ignore interrupt"
    );

    // Reset reports by creating a fresh keymap for the second scenario clarity
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold = {
                    interrupt_response = "Ignore",
                    timeout = 200,
                    profiles = {
                        aggressive = {
                            interrupt_response = "HoldOnKeyPress",
                            timeout = 200,
                        },
                    },
                },
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B,
                    K.C & K.hold K.LeftShift & K.tap_hold_profile "aggressive",
                ],
            }
        "#
    ));

    // Profile key at index 2: press TH, press B → hold
    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 2 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.tick_until_no_scheduled_events();

    #[rustfmt::skip]
    let expected_aggressive: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LSHFT, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LSHFT, 0, KC_B, 0, 0, 0, 0, 0],
        [0, 0, KC_B, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(
        expected_aggressive,
        keymap.distinct_reports().reports(),
        "named profile should hold on interrupt press"
    );
}

#[test]
fn numeric_profile_index_selects_extra_profile() {
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold = {
                    interrupt_response = "Ignore",
                    timeout = 200,
                    profiles = {
                        # alphabetical: only one extra → index 1
                        hold_press = {
                            interrupt_response = "HoldOnKeyPress",
                            timeout = 200,
                        },
                    },
                },
                keys = [
                    K.A & K.hold K.LeftCtrl & { tap_hold_profile = 1 },
                    K.B,
                ],
            }
        "#
    ));

    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.tick_until_no_scheduled_events();

    #[rustfmt::skip]
    let expected: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, KC_B, 0, 0, 0, 0, 0],
        [0, 0, KC_B, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected, keymap.distinct_reports().reports());
}
