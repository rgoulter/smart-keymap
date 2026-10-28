use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

// Extra autoshift: `layer |> AL.autoshift` folds existing `hold` into inner hold.
// `inner_hold` profile: no timeout, HoldOnKeyPress → `A` on tap, mod on interrupt.

#[test]
fn autoshift_hrm_tap_is_plain() {
    // Assemble -- HRM key folded into autoshift, tap should be plain
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let AL = import "extra/autoshift_layer.ncl" in
            let K = import "keys.ncl" in
            {
              config.tap_hold.profiles.inner_hold = {timeout = null, interrupt_response = "HoldOnKeyPress"},
              keys = [K.A & K.hold K.LeftAlt] |> AL.autoshift,
            }
        "#
    ));

    // Act -- tap HRM position
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.tick_until_no_scheduled_events();

    // Assert -- plain 'a'
    let expected: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_A, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected, keymap.distinct_reports().reports());
}

#[test]
fn autoshift_hrm_hold_is_shifted() {
    // Assemble -- HRM hold is reused as inner hold, outer hold is shifted
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let AL = import "extra/autoshift_layer.ncl" in
            let K = import "keys.ncl" in
            {
              config.tap_hold.profiles.inner_hold = {timeout = null, interrupt_response = "HoldOnKeyPress"},
              keys = [K.A & K.hold K.LeftAlt] |> AL.autoshift,
            }
        "#
    ));

    // Act -- hold past outer timeout, release without interrupt
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..250 {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.tick_until_no_scheduled_events();

    // Assert -- Shift + A
    let expected: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LSHFT, 0, KC_A, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected, keymap.distinct_reports().reports());
}

#[test]
fn autoshift_hrm_interrupt_is_mod() {
    // Assemble -- HRM key, interrupt should be the HRM mod
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let AL = import "extra/autoshift_layer.ncl" in
            let K = import "keys.ncl" in
            {
              config.tap_hold.profiles.inner_hold = {timeout = null, interrupt_response = "HoldOnKeyPress"},
              keys = [K.A & K.hold K.LeftAlt, K.B] |> AL.autoshift,
            }
        "#
    ));

    // Act -- press HRM, interrupt with B
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    for _ in 0..250 {
        keymap.tick();
    }
    keymap.tick_until_no_scheduled_events();

    // Assert -- Alt held (from HRM), regardless of Shift on B
    let reports = keymap.distinct_reports().reports().to_vec();
    let has_alt = reports.iter().any(|r| r[0] & MOD_LALT != 0);
    let has_ctrl = reports.iter().any(|r| r[0] & MOD_LCTL != 0);
    assert!(has_alt, "expected reused Alt, got {:02X?}", reports);
    assert!(!has_ctrl, "should not have Ctrl, got {:02X?}", reports);
}

#[test]
fn autoshift_plain_tap_is_plain() {
    // Assemble -- plain key autoshifted, tap should be plain
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let AL = import "extra/autoshift_layer.ncl" in
            let K = import "keys.ncl" in
            {
              config.tap_hold.profiles.inner_hold = {timeout = null, interrupt_response = "HoldOnKeyPress"},
              keys = [K.A] |> AL.autoshift,
            }
        "#
    ));

    // Act -- tap plain
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.tick_until_no_scheduled_events();

    // Assert -- plain 'a'
    let expected: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_A, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected, keymap.distinct_reports().reports());
}

#[test]
fn autoshift_plain_hold_is_shifted() {
    // Assemble -- plain autoshift, hold should be shifted
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let AL = import "extra/autoshift_layer.ncl" in
            let K = import "keys.ncl" in
            {
              config.tap_hold.profiles.inner_hold = {timeout = null, interrupt_response = "HoldOnKeyPress"},
              keys = [K.A] |> AL.autoshift,
            }
        "#
    ));

    // Act -- hold past outer 200ms
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..250 {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.tick_until_no_scheduled_events();

    // Assert -- Shift + A
    let expected: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LSHFT, 0, KC_A, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected, keymap.distinct_reports().reports());
}
