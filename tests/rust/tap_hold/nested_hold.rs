use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

// Outer tap = 'a' (KC_A) with timeout-only resolution.
// Inner is a second tap-hold: tap = 'A' (Shift+KC_A), hold = LeftAlt.
// Inner uses a profile with no timeout and HoldOnKeyPress.
//
// Nickel for outer hold (inner):
//   K.A & K.LeftShift & K.hold K.LeftAlt & K.tap_hold_profile "inner_hold"
//
// Config:
//   config.tap_hold.timeout = 200, interrupt_response = "Ignore" (outer)
//   config.tap_hold.profiles.inner_hold = { timeout = null, interrupt_response = "HoldOnKeyPress" }

macro_rules! nested_keymap {
    () => {
        ObservedKeymap::new(keymap!(
            r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold = {
                    timeout = 200,
                    interrupt_response = "Ignore",
                    profiles = {
                        inner_hold = {
                            timeout = null,
                            interrupt_response = "HoldOnKeyPress",
                        },
                    },
                },
                keys = [
                    K.A & K.hold (K.A & K.LeftShift & K.hold K.LeftAlt & K.tap_hold_profile "inner_hold"),
                    K.B,
                ],
            }
        "#
        ))
    };
}

#[test]
fn nested_tap_hold_outer_tap_is_plain_a() {
    // Assemble -- outer 'a', nested hold is another tap-hold (inner profile)
    let mut keymap = nested_keymap!();

    // Act -- quick tap of outer (press+release before timeout) → plain 'a'
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.tick_until_no_scheduled_events();

    // Assert -- reports a plain 'a' tap, not shifted, not Alt
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_A, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn nested_tap_hold_outer_hold_via_timeout_gives_inner_tap_shifted_a() {
    // Assemble
    let mut keymap = nested_keymap!();

    // Act -- hold outer past its 200ms timeout, then release.
    // Outer timeout resolves outer → inner pending (profile: no timeout, HoldOnKeyPress).
    // Inner stays pending (no timeout). Releasing without interrupting key resolves inner as tap → Shift+A.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..250 {
        keymap.tick();
    }
    // After outer timeout, Alt should NOT yet be held; inner is pending.
    // Release outer.
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.tick_until_no_scheduled_events();

    // Assert -- shifted 'A' (LeftShift + KC_A)
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LSHFT, 0, KC_A, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn nested_tap_hold_outer_hold_via_interrupt_then_inner_hold_gives_alt() {
    // Assemble -- HoldOnKeyPress on inner, Ignore on outer.
    // Press outer, then press B before outer timeout.
    // Outer (Ignore) stays pending until timeout, then replays B press into inner.
    // Inner (HoldOnKeyPress, no timeout) sees B press → resolves as hold → Alt.
    let mut keymap = nested_keymap!();

    // Act -- press outer, quickly press B (interrupt), then wait for outer timeout
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    // Outer is Ignore, so both outer and inner remain pending until timeout.
    // Tick past outer timeout (200ms) — this should resolve outer→inner, then inner→hold (Alt) via replayed B press.
    for _ in 0..250 {
        keymap.tick();
    }
    keymap.tick_until_no_scheduled_events();

    // Assert -- Alt is held while B is pressed
    let reports = keymap.distinct_reports().reports().to_vec();
    let has_alt = reports.iter().any(|r| r[0] & MOD_LALT != 0);
    let has_alt_b = reports
        .iter()
        .any(|r| r[0] & MOD_LALT != 0 && r.contains(&KC_B));
    assert!(
        has_alt,
        "expected Alt held after nested interrupt, reports: {:02X?}",
        reports
    );
    assert!(
        has_alt_b,
        "expected Alt+B chord after nested interrupt, reports: {:02X?}",
        reports
    );
}

#[test]
fn nested_tap_hold_outer_hold_timeout_then_inner_interrupt_gives_alt() {
    // Assemble
    let mut keymap = nested_keymap!();

    // Act -- hold outer past timeout (250 ticks), then press B.
    // Outer → inner pending (no timeout). Inner pending + B press → Alt.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..250 {
        keymap.tick();
    }
    // Outer has timed out, inner is now pending (still no Alt yet). Confirm not yet Alt+B.
    // Now interrupt inner with B.
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.tick_until_no_scheduled_events();

    // Assert -- Alt held with B
    let reports = keymap.distinct_reports().reports().to_vec();
    let has_alt = reports.iter().any(|r| r[0] == MOD_LALT);
    let has_alt_b = reports
        .iter()
        .any(|r| r[0] == MOD_LALT && r.contains(&KC_B));
    assert!(
        has_alt,
        "expected Alt after inner interrupt, reports: {:02X?}",
        reports
    );
    assert!(has_alt_b, "expected Alt+B, reports: {:02X?}", reports);
}
