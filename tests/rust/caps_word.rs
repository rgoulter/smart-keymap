use smart_keymap::input;
use smart_keymap::keymap;

use smart_keymap_macros::keymap;

use keymap::DistinctReports;

#[test]
fn tap_caps_word_shifts_keyboard_keys() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.caps_word.toggle,
                    K.A,
                    K.B,
                    K.Space,
                ],
            }
        "#
    );
    let mut actual_reports = DistinctReports::new();

    // Act
    // Tap CapsWord
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    // Press "A"
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0x02, 0, 0, 0, 0, 0, 0, 0],
        [0x02, 0, 0x04, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn tap_caps_word_spc_deactivates_caps_word() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.caps_word.toggle,
                    K.A,
                    K.B,
                    K.Space,
                ],
            }
        "#
    );
    let mut actual_reports = DistinctReports::new();

    // Act
    // Tap CapsWord
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    // Tap "A"
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    // Tap "Spc"
    keymap.handle_input(input::Event::Press { keymap_index: 3 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    keymap.handle_input(input::Event::Release { keymap_index: 3 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    // Press "A"
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0x02, 0, 0, 0, 0, 0, 0, 0],
        [0x02, 0, 0x04, 0, 0, 0, 0, 0],
        [0x02, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x2C, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}
