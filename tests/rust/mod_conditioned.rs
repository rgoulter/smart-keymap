use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

#[test]
fn gresc_alone_sends_escape() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.gresc,
                    K.LeftShift,
                ],
            }
        "#
    ));

    // Act: press gresc alone
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.tick_until_no_scheduled_events();

    // Assert: Escape (base binding)
    let expected_reports: &[[u8; 8]] =
        &[[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, KC_ESCAPE, 0, 0, 0, 0, 0]];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn gresc_with_shift_sends_grave_without_shift() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.gresc,
                    K.LeftShift,
                ],
            }
        "#
    ));

    // Act: hold LeftShift, then press gresc
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.tick_until_no_scheduled_events();

    // Assert: Grave (morphed), Shift suppressed from the report
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LSHFT, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_GRAVE, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn gresc_release_restores_shift_in_report() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.gresc,
                    K.LeftShift,
                ],
            }
        "#
    ));

    // Act: Shift + gresc press, then release gresc (Shift still held)
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.tick_until_no_scheduled_events();

    // Assert: after morph release, Shift reappears in the report
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LSHFT, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_GRAVE, 0, 0, 0, 0, 0],
        [MOD_LSHFT, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn bspc_del_with_left_shift_sends_delete_without_shift() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.bspc_del,
                    K.LeftShift,
                    K.RightShift,
                ],
            }
        "#
    ));

    // Act: hold LeftShift, press bspc_del
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.tick_until_no_scheduled_events();

    // Assert: Delete with LeftShift suppressed
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LSHFT, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_DELETE, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn bspc_del_with_right_shift_keeps_right_shift() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.bspc_del,
                    K.LeftShift,
                    K.RightShift,
                ],
            }
        "#
    ));

    // Act: hold RightShift, press bspc_del
    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.tick_until_no_scheduled_events();

    // Assert: Delete morph, RightShift kept (in keep_mods)
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_RSHFT, 0, 0, 0, 0, 0, 0, 0],
        [MOD_RSHFT, 0, KC_DELETE, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn custom_record_shift_delete() {
    // Assemble: ad-hoc mod-conditioned record (base Backspace, morphed Delete on LeftShift)
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    {
                        base = K.Backspace,
                        morphed = K.Delete,
                        mods = { left_shift = true },
                    },
                    K.LeftShift,
                ],
            }
        "#
    ));

    // Act: alone → Backspace; then with Shift → Delete (Shift suppressed)
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.tick_until_no_scheduled_events();

    // Assert
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_BACKSPACE, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [MOD_LSHFT, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_DELETE, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
