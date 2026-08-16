use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

#[test]
fn first_press_holds_alt_then_taps_tab() {
    // Assemble -- Alt-Tab tri-state on index 0
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.tri_state.alt_tab,
                    K.A,
                ],
            }
        "#
    ));

    // Act -- tap the tri-state key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.tick_until_no_scheduled_events();

    // Assert -- first report is the Alt+Tab chord; Alt stays after release
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LALT, 0, KC_TAB, 0, 0, 0, 0, 0],
        [MOD_LALT, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn repress_taps_tab_again_while_alt_held() {
    // Assemble -- Alt-Tab tri-state on index 0
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.tri_state.alt_tab,
                    K.A,
                ],
            }
        "#
    ));

    // Act -- tap the tri-state key twice
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.tick_until_no_scheduled_events();

    // Assert -- two Tab pulses, Alt held throughout
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LALT, 0, KC_TAB, 0, 0, 0, 0, 0],
        [MOD_LALT, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LALT, 0, KC_TAB, 0, 0, 0, 0, 0],
        [MOD_LALT, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn other_key_releases_alt() {
    // Assemble -- Alt-Tab tri-state, then A
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.tri_state.alt_tab,
                    K.A,
                ],
            }
        "#
    ));

    // Act -- open the session, then tap A
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.tick_until_no_scheduled_events();

    // Assert -- A interrupts: hold is released in the same turn as A,
    //  so the host sees A, not Alt+A
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LALT, 0, KC_TAB, 0, 0, 0, 0, 0],
        [MOD_LALT, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_A, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn custom_hold_tap_ctrl_tab() {
    // Assemble -- custom Ctrl+Tab tri-state
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.tri_state.custom {
                        hold = K.LeftCtrl,
                        tap = K.Tab,
                    },
                ],
            }
        "#
    ));

    // Act -- tap the tri-state key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.tick_until_no_scheduled_events();

    // Assert -- first report is the Ctrl+Tab chord; Ctrl stays
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, KC_TAB, 0, 0, 0, 0, 0],
        [MOD_LCTL, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
