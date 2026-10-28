//! Chorded tap-hold required_idle is from the physical press, not chord timeout.

use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

#[test]
fn chorded_tap_hold_required_idle_insufficient_remains_tap_after_timeout() {
    // Assemble -- chord 200ms, tap-hold 200ms, required_idle 100ms
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.chorded.timeout = 200,
                config.tap_hold.timeout = 200,
                config.tap_hold.required_idle_time = 100,
                chords = [
                    { indices = [0, 1], key = K.C },
                ],
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B,
                    K.D,
                ],
            }
        "#
    ));

    // Act -- D tap, then 50ms later press chorded tap-hold (insufficient 50 < 100)
    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    keymap.handle_input(input::Event::Release { keymap_index: 2 });

    for _ in 0..50 {
        keymap.tick();
    }

    keymap.handle_input(input::Event::Press { keymap_index: 0 });

    // Hold past chord timeout (200 from press) -- if required_idle were
    //  checked at passthrough time, idle would appear as 250 and hold.
    for _ in 0..201 {
        keymap.tick();
    }

    // Assert -- still tap, not hold (idle was insufficient at the physical press)
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_D, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_A, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, keymap.distinct_reports().reports());
}

#[test]
fn chorded_tap_hold_required_idle_sufficient_holds_after_timeout() {
    // Assemble -- same configs, sufficient idle 150 >= 100
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.chorded.timeout = 200,
                config.tap_hold.timeout = 200,
                config.tap_hold.required_idle_time = 100,
                chords = [
                    { indices = [0, 1], key = K.C },
                ],
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B,
                    K.D,
                ],
            }
        "#
    ));

    // Act -- D tap, then 150ms later press chorded tap-hold (sufficient)
    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    keymap.handle_input(input::Event::Release { keymap_index: 2 });

    for _ in 0..150 {
        keymap.tick();
    }

    keymap.handle_input(input::Event::Press { keymap_index: 0 });

    for _ in 0..201 {
        keymap.tick();
    }

    // Assert -- hold after the tap-hold timeout counted from the physical press
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_D, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, keymap.distinct_reports().reports());
}
