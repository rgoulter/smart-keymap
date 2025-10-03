use smart_keymap::keymap::ObservedKeymap;

#[test]
fn test_simple_1char_string_macro() {
    // Assemble
    use smart_keymap::input;

    let mut keymap = ObservedKeymap::new(smart_keymap_macros::keymap!(
        r#"
        let K = import "keys.ncl" in

        let MY_MACRO = K.string_macro "a" in
        {
            keys = [
                MY_MACRO,
            ],
        }
        "#
    ));

    // Act -- tap macro key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn test_simple_string_macro() {
    // This test demonstrates using smart_keymap::keymap::Keymap directly,
    //  by using the keymap! macro.

    // Assemble
    use smart_keymap::input;

    let mut keymap = ObservedKeymap::new(smart_keymap_macros::keymap!(
        r#"
        let K = import "keys.ncl" in

        let MY_MACRO = K.string_macro "abc" in
        {
            keys = [
                MY_MACRO,
            ],
        }
        "#
    ));

    // Act -- tap macro key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    keymap.tick_until_no_scheduled_events();

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
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
