use smart_keymap::input;
use smart_keymap::keymap;

use smart_keymap_macros::keymap;

use keymap::DistinctReports;

#[test]
fn rolled_presses() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B,
                ],
            }
        "#
    );
    let mut actual_reports = DistinctReports::new();

    // Act
    // Roll the keys: press 0, press 1, release 0, release 1
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0x05, 0, 0, 0, 0],
        [0, 0, 0x05, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn rolled_presses_desc_keycodes() {
    // Assemble
    const K_G: u8 = 0x0A;
    const K_O: u8 = 0x12;

    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.O & K.hold K.LeftCtrl,
                    K.G,
                ],
            }
        "#
    );

    {
        let mut actual_reports = DistinctReports::new();

        // Roll the keys: press 0, press 1, release 0,
        keymap.handle_input(input::Event::Press { keymap_index: 0 });
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

        keymap.handle_input(input::Event::Press { keymap_index: 1 });
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

        keymap.handle_input(input::Event::Release { keymap_index: 0 });
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

        while keymap.has_scheduled_events() {
            keymap.tick();
            actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
        }

        let expected_reports: &[[u8; 8]] = &[
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, K_O, 0, 0, 0, 0, 0],
            [0, 0, K_O, K_G, 0, 0, 0, 0],
            [0, 0, K_G, 0, 0, 0, 0, 0],
        ];
        assert_eq!(expected_reports, actual_reports.reports());
    }

    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    // Act
    // Roll a second time
    {
        let mut actual_reports = DistinctReports::new();

        keymap.handle_input(input::Event::Press { keymap_index: 0 });
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

        keymap.handle_input(input::Event::Press { keymap_index: 1 });
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

        keymap.handle_input(input::Event::Release { keymap_index: 0 });
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

        while keymap.has_scheduled_events() {
            keymap.tick();
            actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
        }

        // Assert
        let expected_reports: &[[u8; 8]] = &[
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, K_O, 0, 0, 0, 0, 0],
            [0, 0, K_O, K_G, 0, 0, 0, 0],
            [0, 0, K_G, 0, 0, 0, 0, 0],
        ];
        assert_eq!(expected_reports, actual_reports.reports());
    }
}
