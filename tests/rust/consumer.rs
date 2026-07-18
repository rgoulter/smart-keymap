use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;
use smart_keymap_macros::keymap;

use crate::hid_keycodes::*;

#[test]
fn consumer_key() {
    // Assemble
    let mut keymap = smart_keymap_macros::keymap!(
        r#"
        let K = import "keys.ncl" in
        {
            keys = [
                K.PlayPause,
            ],
        }
        "#
    );

    // Act -- press 'PlayPause'
    assert!(keymap.report_output().pressed_consumer_codes().is_empty());
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.tick();
    let report_output = keymap.report_output();
    let pressed_codes = report_output.pressed_consumer_codes();
    assert_eq!(&[0xCD], pressed_codes.as_slice());
    // boot report should be empty
    assert_eq!([0u8; 8], report_output.as_hid_boot_keyboard_report());

    // Act -- release 'PlayPause'
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.tick();
    assert!(keymap.report_output().pressed_consumer_codes().is_empty());
}

#[test]
fn modified_consumer_key_reports_modifier_and_consumer_code() {
    // Keyboard modifiers via ObservedKeymap distinct reports
    let mut observed = ObservedKeymap::new(keymap!(
        r#"
        let K = import "keys.ncl" in
        {
            keys = [
                K.VolumeUp & K.LeftShift,
            ],
        }
        "#
    ));

    observed.handle_input(input::Event::Press { keymap_index: 0 });
    observed.tick_until_no_scheduled_events();

    let expected_reports: &[[u8; 8]] =
        &[[0, 0, 0, 0, 0, 0, 0, 0], [MOD_LSHFT, 0, 0, 0, 0, 0, 0, 0]];
    assert_eq!(expected_reports, observed.distinct_reports().reports());

    // Consumer code alongside modifiers
    let mut keymap = smart_keymap_macros::keymap!(
        r#"
        let K = import "keys.ncl" in
        {
            keys = [
                K.VolumeUp & K.LeftShift,
            ],
        }
        "#
    );
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.tick();
    let report_output = keymap.report_output();
    assert_eq!(&[0xE9], report_output.pressed_consumer_codes().as_slice());
    assert_eq!(
        [MOD_LSHFT, 0, 0, 0, 0, 0, 0, 0],
        report_output.as_hid_boot_keyboard_report()
    );
}

#[test]
fn modified_consumer_key_clears_on_release() {
    let mut keymap = smart_keymap_macros::keymap!(
        r#"
        let K = import "keys.ncl" in
        {
            keys = [
                K.VolumeUp & K.LeftShift,
            ],
        }
        "#
    );

    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.tick();
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.tick();

    let report_output = keymap.report_output();
    assert!(report_output.pressed_consumer_codes().is_empty());
    assert_eq!([0u8; 8], report_output.as_hid_boot_keyboard_report());
}
