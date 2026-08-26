use smart_keymap::input;
use smart_keymap::key::KeyboardModifiers;
use smart_keymap::keymap::ObservedKeymap;
use smart_keymap_macros::keymap;

use crate::hid_keycodes::*;

/// LEFT_CTRL modifier byte as used in HID report `report[0]`.
const MOD_LEFT_CTRL: u8 = KeyboardModifiers::LEFT_CTRL_U8;
/// LEFT_GUI modifier byte.
const MOD_LEFT_GUI: u8 = KeyboardModifiers::LEFT_GUI_U8;

/// Returns true if any HID report contains `modifier` byte.
fn has_report_with_mod(reports: &[[u8; 8]], modifier: u8) -> bool {
    reports.iter().any(|r| r[0] == modifier)
}

/// Returns true if any report has `modifier` and `key_code` together
///  (e.g. mod + A flash).
fn has_report_with_mod_and_key(reports: &[[u8; 8]], modifier: u8, key_code: u8) -> bool {
    reports.iter().any(|r| r[0] == modifier && r[2] == key_code)
}

#[test]
fn default_none_still_silent_until_settle() {
    // Assemble -- default NoOutput pending_output
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
        let K = import "keys.ncl" in
        { keys = [ K.A & K.hold K.LeftCtrl ] }
    "#
    ));

    // Act -- press TH
    keymap.handle_input(input::Event::Press { keymap_index: 0 });

    // Assert -- silent while pending
    let reports = keymap.distinct_reports().reports().to_vec();
    assert_eq!(
        vec![[0, 0, 0, 0, 0, 0, 0, 0]],
        reports,
        "should be silent while pending"
    );

    // Act -- tick to timeout
    for _ in 0..210 {
        keymap.tick();
    }

    // Assert -- after timeout should be hold
    let reports2 = keymap.distinct_reports().reports().to_vec();
    assert!(
        has_report_with_mod(&reports2, MOD_LEFT_CTRL),
        "after timeout should be hold"
    );
}

#[test]
fn hold_pending_shows_mod_after_first_tick() {
    // Assemble -- tap-hold with Hold pending_output
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
        let K = import "keys.ncl" in
        {
            config.tap_hold.pending_output = "Hold",
            keys = [ K.A & K.hold K.LeftCtrl ],
        }
    "#
    ));

    // Act -- press, tick once
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.tick();

    // Assert -- mod from first tick
    let reports = keymap.distinct_reports().reports().to_vec();
    assert!(
        has_report_with_mod(&reports, MOD_LEFT_CTRL),
        "speculative hold should appear after first tick, got {:?}",
        reports
    );
}

#[test]
fn hold_pending_timeout_stays_hold() {
    // Assemble -- tap-hold with Hold pending_output
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
        let K = import "keys.ncl" in
        {
            config.tap_hold.pending_output = "Hold",
            keys = [ K.A & K.hold K.LeftCtrl ],
        }
    "#
    ));

    // Act -- press, tick, then timeout
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.tick();
    for _ in 0..210 {
        keymap.tick();
    }

    // Assert -- still hold after timeout
    let reports = keymap.distinct_reports().reports().to_vec();
    assert!(has_report_with_mod(&reports, MOD_LEFT_CTRL));
}

#[test]
fn hold_pending_quick_release_cancels_then_tap() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
        let K = import "keys.ncl" in
        {
            config.tap_hold.pending_output = "Hold",
            keys = [ K.A & K.hold K.LeftCtrl ],
        }
    "#
    ));

    // Act -- press and tick to show speculative hold
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.tick();
    let before = keymap.distinct_reports().reports().to_vec();
    assert!(
        has_report_with_mod(&before, MOD_LEFT_CTRL),
        "speculative hold should be present"
    );

    // Act -- quick release before timeout
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.tick_until_no_scheduled_events();
    let reports = keymap.distinct_reports().reports().to_vec();

    // Assert -- should have tap A without mod, and no Ctrl+A flash
    assert!(
        reports.iter().any(|r| r[2] == KC_A && r[0] == 0),
        "tap A without mod after cancel"
    );
    let has_flash = has_report_with_mod_and_key(&reports, MOD_LEFT_CTRL, KC_A);
    assert!(
        !has_flash,
        "should not have Ctrl+A flash, reports {:?}",
        reports
    );
}

#[test]
fn hold_pending_interrupt_mod_already_down() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
        let K = import "keys.ncl" in
        {
            config.tap_hold.pending_output = "Hold",
            config.tap_hold.interrupt_response = "HoldOnKeyPress",
            keys = [ K.A & K.hold K.LeftCtrl, K.B ],
        }
    "#
    ));

    // Act -- press TH, tick, then interrupt with B
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.tick();
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.tick_until_no_scheduled_events();
    let reports = keymap.distinct_reports().reports().to_vec();

    // Assert -- mod already down when B taps (distinct: MOD then MOD+B)
    assert!(has_report_with_mod(&reports, MOD_LEFT_CTRL));
    assert!(
        has_report_with_mod_and_key(&reports, MOD_LEFT_CTRL, KC_B),
        "B should be with mod, reports {:?}",
        reports
    );
}

#[test]
fn gui_hold_refused_speculation() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
        let K = import "keys.ncl" in
        {
            config.tap_hold.pending_output = "Hold",
            keys = [ K.A & K.hold K.LeftGUI ],
        }
    "#
    ));

    // Act -- press GUI hold
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.tick();
    let reports = keymap.distinct_reports().reports().to_vec();

    // Assert -- should remain silent (GUI not speculated)
    assert!(
        !has_report_with_mod(&reports, MOD_LEFT_GUI),
        "GUI should not speculate, reports {:?}",
        reports
    );
}
