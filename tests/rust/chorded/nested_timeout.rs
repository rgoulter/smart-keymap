//! Nested pending timeouts are counted from the physical press.
//!
//! Chorded wrap tap-hold or tap-dance; without backdating, hold waits
//!  for the chord timeout plus the inner timeout.
//!
//! A *resolved chord* whose binding is tap-hold is a different path:
//!  the completing press is a fresh `new_pressed_key` on the activator
//!  index, so that hold clock starts when the chord is completed.

use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

#[test]
fn passthrough_hold_resolves_at_tap_hold_timeout_not_sum() {
    // Assemble -- equal 200ms chord + tap-hold timeouts.
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.chorded.timeout = 200,
                config.tap_hold.timeout = 200,
                chords = [
                    { indices = [0, 1], key = K.C },
                ],
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B,
                ],
            }
        "#
    ));

    // Act -- press the chorded tap-hold; ObservedKeymap::handle_input
    //  already ticks once, so 198 more ticks is t=199.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..198 {
        keymap.tick();
    }

    // Assert -- still pending just before the chord timeout.
    #[rustfmt::skip]
    let expected_before: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_before, keymap.distinct_reports().reports());

    // Act -- one more tick reaches t=200: chord passthroughs and the
    //  inner tap-hold timeout is already expired, so hold is Immediate.
    keymap.tick();

    // Assert -- hold, not another 200ms of pending.
    #[rustfmt::skip]
    let expected_hold: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_hold, keymap.distinct_reports().reports());
}

#[test]
fn aux_passthrough_hold_resolves_at_tap_hold_timeout_not_sum() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.chorded.timeout = 200,
                config.tap_hold.timeout = 200,
                chords = [
                    { indices = [0, 1], key = K.C },
                ],
                keys = [
                    K.A,
                    K.B & K.hold K.LeftShift,
                ],
            }
        "#
    ));

    // Act -- hold the auxiliary chorded tap-hold until t=200.
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    for _ in 0..199 {
        keymap.tick();
    }

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LSHFT, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, keymap.distinct_reports().reports());
}

#[test]
fn shorter_chord_timeout_leaves_remaining_tap_hold_delay() {
    // Assemble -- chord 50ms, tap-hold 200ms: hold at 200 from press.
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.chorded.timeout = 50,
                config.tap_hold.timeout = 200,
                chords = [
                    { indices = [0, 1], key = K.C },
                ],
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B,
                ],
            }
        "#
    ));

    // Act -- t=50: chord passthroughs; tap-hold still has 150ms remaining.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..49 {
        keymap.tick();
    }

    // Assert -- not hold yet (would be if we Immediate-fired the inner timeout).
    #[rustfmt::skip]
    let expected_pending: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_pending, keymap.distinct_reports().reports());

    // Act -- remaining tap-hold delay; hold at t=200 from the press.
    for _ in 0..150 {
        keymap.tick();
    }

    // Assert
    #[rustfmt::skip]
    let expected_hold: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_hold, keymap.distinct_reports().reports());
}

#[test]
fn shorter_tap_hold_timeout_resolves_when_chord_passthroughs() {
    // Assemble -- chord 200ms, tap-hold 50ms: cannot hold before the chord
    //  decides; at passthrough the inner timeout is already expired.
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.chorded.timeout = 200,
                config.tap_hold.timeout = 50,
                chords = [
                    { indices = [0, 1], key = K.C },
                ],
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B,
                ],
            }
        "#
    ));

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..199 {
        keymap.tick();
    }

    // Assert -- hold as soon as the chord passthroughs, not 50ms later.
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, keymap.distinct_reports().reports());
}

#[test]
fn layered_passthrough_hold_resolves_at_tap_hold_timeout_not_sum() {
    // Assemble -- chorded → layered → tap-hold (layered adds no timeout).
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.chorded.timeout = 200,
                config.tap_hold.timeout = 200,
                chords = [
                    { indices = [0, 1], key = K.E },
                ],
                layers = [
                    [K.A & K.hold K.LeftCtrl, K.B],
                    [K.F & K.hold K.LeftShift, K.D],
                ],
            }
        "#
    ));

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..199 {
        keymap.tick();
    }

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, keymap.distinct_reports().reports());
}

#[test]
fn tap_dance_under_chord_resolves_at_its_timeout_not_sum() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.chorded.timeout = 200,
                config.tap_dance.timeout = 200,
                chords = [
                    { indices = [0, 1], key = K.E },
                ],
                keys = [
                    K.A & { tap_dances = [K.B] },
                    K.C,
                ],
            }
        "#
    ));

    // Act -- hold the chorded tap-dance; at t=200 chord passthroughs and
    //  the first tap-dance timeout is already expired.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..199 {
        keymap.tick();
    }

    // Assert -- first tap-dance definition (A), not another 200ms wait.
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_A, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, keymap.distinct_reports().reports());
}
