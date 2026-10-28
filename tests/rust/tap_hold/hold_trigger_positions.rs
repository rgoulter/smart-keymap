//! Positional hold triggers (ZMK-style opposite-hand trigger positions).
//!
//! Layout used by these fixtures (indices in `keys`):
//! - 0: tap-hold (e.g. A / LeftCtrl) — the key under test
//! - 1: "same hand" letter (B) — not a hold trigger position
//! - 2: "opposite hand" letter (C) — hold trigger position

use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

/// Default profile: HoldOnKeyPress; only keymap index 2 is a hold trigger position.
macro_rules! keymap_triggers_only_index_2 {
    () => {
        ObservedKeymap::new(keymap!(
            r#"
                let K = import "keys.ncl" in
                {
                    config.tap_hold = {
                        interrupt_response = "HoldOnKeyPress",
                        timeout = 200,
                        hold_trigger_key_positions = [2],
                    },
                    keys = [
                        K.A & K.hold K.LeftCtrl,
                        K.B,
                        K.C,
                    ],
                }
            "#
        ))
    };
}

#[test]
fn opposite_hand_press_resolves_hold() {
    // Assemble: TH on index 0; only index 2 is a hold trigger position.
    let mut keymap = keymap_triggers_only_index_2!();

    // Act: press TH, then opposite-hand key (index 2).
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    keymap.tick_until_no_scheduled_events();

    // Assert: interrupt forced hold (LeftCtrl), then C down with mod.
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, KC_C, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, keymap.distinct_reports().reports());
}

#[test]
fn same_hand_press_does_not_resolve_hold() {
    // Assemble: TH on index 0; index 2 is a trigger position — index 1 is "same hand".
    let mut keymap = keymap_triggers_only_index_2!();

    // Act: press TH, same-hand B, release both (no timeout).
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.tick_until_no_scheduled_events();

    // Assert: TH resolved as tap (A), not hold; B is a normal press/release.
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_A, 0, 0, 0, 0, 0],
        [0, 0, KC_A, KC_B, 0, 0, 0, 0],
        [0, 0, KC_B, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, keymap.distinct_reports().reports());
}

#[test]
fn timeout_still_resolves_hold_with_positional_triggers() {
    // Assemble: hold trigger positions set, but we never press a trigger key.
    let mut keymap = keymap_triggers_only_index_2!();

    // Act: press TH alone and wait past timeout.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..250 {
        keymap.tick();
    }

    // Assert: timeout still resolves as hold regardless of hold trigger positions.
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, keymap.distinct_reports().reports());
}

#[test]
fn named_profile_trigger_position_resolves_hold() {
    // Assemble: default profile ignores interrupts; named profile has hold trigger positions.
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold = {
                    interrupt_response = "Ignore",
                    timeout = 200,
                    profiles = {
                        opposite = {
                            interrupt_response = "HoldOnKeyPress",
                            hold_trigger_key_positions = [2],
                        },
                    },
                },
                keys = [
                    K.A & K.hold K.LeftCtrl & K.tap_hold_profile "opposite",
                    K.B,
                    K.C,
                ],
            }
        "#
    ));

    // Act: press named-profile TH (index 0), then trigger position (index 2).
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    keymap.tick_until_no_scheduled_events();

    // Assert: named profile's hold trigger positions forced hold.
    #[rustfmt::skip]
    let expected: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, KC_C, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected, keymap.distinct_reports().reports());
}
