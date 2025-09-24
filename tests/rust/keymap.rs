mod caps_word;
mod chorded;
mod consumer;
mod custom;
mod layered;
mod sticky;
mod tap_dance;
mod tap_hold;

mod ms_per_tick;

#[test]
fn basic_keymap_expression() {
    // This test demonstrates using smart_keymap::keymap::Keymap directly.

    // Assemble
    use smart_keymap::input;
    use smart_keymap::keymap;

    use keymap::DistinctReports;

    let mut keymap = {
        use key_system::Context;
        use key_system::Ref;
        use smart_keymap::key::composite as key_system;
        const KEY_COUNT: usize = 1;
        const KEY_REFS: [Ref; KEY_COUNT] = [smart_keymap::key::composite::Ref::Keyboard(
            smart_keymap::key::keyboard::Ref::KeyCode(0x04),
        )];
        const CONTEXT: Context = Context::from_config(key_system::DEFAULT_CONFIG);

        smart_keymap::keymap::Keymap::new(
            KEY_REFS,
            CONTEXT,
            smart_keymap::key::composite::System::array_based(
                smart_keymap::key::callback::System::new([]),
                smart_keymap::key::chorded::System::new([], []),
                smart_keymap::key::keyboard::System::new([]),
                smart_keymap::key::layered::System::new([], []),
                smart_keymap::key::sticky::System::new([]),
                smart_keymap::key::tap_dance::System::new([]),
                smart_keymap::key::tap_hold::System::new([]),
            ),
        )
    };
    let mut actual_reports = DistinctReports::new();

    // Act -- tap 'a'
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert -- tap-hold key immediately resolves as 'tap'
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

mod consumer {
    use smart_keymap::input;

    #[test]
    fn consumer_key() {
        // Assemble
        let mut keymap = smart_keymap_macros::keymap!(
            r#"
            {
                keys = [
                    { usage_code = 0xCD }, // PlayPause
                ],
            }
            "#
        );

        // Act -- press 'PlayPause'
        assert!(keymap.report_output().pressed_consumer_codes().is_empty());
        keymap.handle_input(input::Event::Press { keymap_index: 0 });
        let report_output = keymap.report_output();
        let pressed_codes = report_output.pressed_consumer_codes();
        assert_eq!(&[0xCD], pressed_codes.as_slice());
        // boot report should be empty
        assert_eq!([0u8; 8], report_output.as_hid_boot_keyboard_report());

        // Act -- release 'PlayPause'
        keymap.handle_input(input::Event::Release { keymap_index: 0 });
        assert!(keymap.report_output().pressed_consumer_codes().is_empty());
    }
}

#[test]
fn basic_keymap_expression_macro() {
    // This test demonstrates using smart_keymap::keymap::Keymap directly,
    //  by using the keymap! macro.

    // Assemble
    use smart_keymap::input;
    use smart_keymap::keymap;

    use keymap::DistinctReports;

    let mut keymap = smart_keymap_macros::keymap!(
        r#"
        {
            keys = [
                { key_code = 4 },
            ],
        }
        "#
    );
    let mut actual_reports = DistinctReports::new();

    // Act -- tap 'a'
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert -- tap-hold key immediately resolves as 'tap'
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}
