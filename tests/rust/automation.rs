#[test]
fn test_simple_1char_string_macro() {
    // This test demonstrates using smart_keymap::keymap::Keymap directly,
    //  by using the keymap! macro.

    // Assemble
    use smart_keymap::input;
    use smart_keymap::keymap;

    use keymap::DistinctReports;

    let mut keymap = smart_keymap_macros::keymap!(
        r#"
        let K = import "keys.ncl" in

        let MY_MACRO = K.string_macro "a" in
        {
            keys = [
                MY_MACRO,
            ],
        }
        "#
    );
    let mut actual_reports = DistinctReports::new();

    // Act -- tap macro key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn test_simple_string_macro() {
    // This test demonstrates using smart_keymap::keymap::Keymap directly,
    //  by using the keymap! macro.

    // Assemble
    use smart_keymap::input;
    use smart_keymap::keymap;

    use keymap::DistinctReports;

    let mut keymap = smart_keymap_macros::keymap!(
        r#"
        let K = import "keys.ncl" in

        let MY_MACRO = K.string_macro "abc" in
        {
            keys = [
                MY_MACRO,
            ],
        }
        "#
    );
    let mut actual_reports = DistinctReports::new();

    // Act -- tap macro key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x05, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x06, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}
