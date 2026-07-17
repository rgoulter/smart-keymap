use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

#[test]
fn sticky_layer_deactivates_after_timeout_without_key_press() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.layered.sticky_timeout = 100,
                layers = [
                    [K.layer_mod.sticky 1, K.A, K.B],
                    [K.TTTT, K.K, K.L],
                ],
            }
        "#
    ));

    // Act — tap sticky layer mod, wait past timeout, press base key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    for _ in 0..101 {
        keymap.tick();
    }

    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert — sticky expired; key is base layer A, not layered K
    let expected_reports: &[[u8; 8]] = &[[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, KC_A, 0, 0, 0, 0, 0]];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn sticky_layer_still_applies_if_key_pressed_before_timeout() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.layered.sticky_timeout = 100,
                layers = [
                    [K.layer_mod.sticky 1, K.A, K.B],
                    [K.TTTT, K.K, K.L],
                ],
            }
        "#
    ));

    // Act — tap sticky, wait under timeout, press layered key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    for _ in 0..50 {
        keymap.tick();
    }

    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert — sticky still active
    let expected_reports: &[[u8; 8]] = &[[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, KC_K, 0, 0, 0, 0, 0]];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn sticky_layer_without_timeout_config_remains_active() {
    // Assemble — omit sticky_timeout (default None)
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.sticky 1, K.A, K.B],
                    [K.TTTT, K.K, K.L],
                ],
            }
        "#
    ));

    // Act — tap sticky, wait a long time, press key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    for _ in 0..500 {
        keymap.tick();
    }

    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert — sticky still active without configured timeout
    let expected_reports: &[[u8; 8]] = &[[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, KC_K, 0, 0, 0, 0, 0]];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
