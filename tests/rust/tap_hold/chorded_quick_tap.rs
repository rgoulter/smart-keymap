//! Tap-hold under a chorded key, with `quick_tap_ms` set.
//!
//! After idle the chord waits, then passthroughs to tap-hold. That
//! replacement `new_pressed_key` must still be able to resolve as hold.

use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

macro_rules! keymap_under_test {
    () => {
        ObservedKeymap::new(keymap!(
            r#"
                let K = import "keys.ncl" in
                {
                    config.tap_hold.interrupt_response = "HoldOnKeyTap",
                    config.tap_hold.quick_tap_ms = 200,
                    config.tap_hold.timeout = 200,
                    config.chorded.required_idle_time = 125,
                    chords = [
                        {
                            indices = [1, 2],
                            key = K.Tab & K.hold (K.layer_mod.hold 1),
                        },
                    ],
                    layers = [
                        [
                            K.A,
                            K.Escape & K.hold (K.layer_mod.hold 2),
                            K.Space & K.hold (K.layer_mod.hold 1),
                        ],
                        [
                            K.Left,
                            K.TTTT,
                            K.TTTT,
                        ],
                        [
                            K.Right,
                            K.TTTT,
                            K.TTTT,
                        ],
                    ],
                }
            "#
        ))
    };
}

/// Chord idle gate passthrough at t = 0: timeout-hold still activates the layer.
#[test]
fn first_press_timeout_hold_activates_layer() {
    let mut keymap = keymap_under_test!();

    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    for _ in 0..201 {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Press { keymap_index: 0 });

    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_LEFT, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, keymap.distinct_reports().reports());
}

/// After 2000 ms idle the chord waits, then passthroughs. Timeout-hold
/// of the inner tap-hold must activate the layer.
#[test]
fn after_idle_timeout_hold_activates_layer() {
    let mut keymap = keymap_under_test!();

    for _ in 0..2000 {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    for _ in 0..401 {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Press { keymap_index: 0 });

    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_LEFT, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, keymap.distinct_reports().reports());
}

/// After 2000 ms idle: HoldOnKeyTap of the inner tap-hold applies the layer.
#[test]
fn after_idle_hold_on_key_tap_activates_layer() {
    let mut keymap = keymap_under_test!();

    for _ in 0..2000 {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.tick_until_no_scheduled_events();

    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_LEFT, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, keymap.distinct_reports().reports());
}
