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

#[test]
fn test_shifted_string_macro() {
    // Assemble
    use smart_keymap::input;

    let mut keymap = ObservedKeymap::new(smart_keymap_macros::keymap!(
        r#"
        let K = import "keys.ncl" in

        let MY_MACRO = K.string_macro "Ab Cd" in
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
        [2, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x05, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x2C, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [2, 0, 0x06, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x07, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn test_macro_while_pressed() {
    // Assemble
    use smart_keymap::input;

    let mut keymap = ObservedKeymap::new(smart_keymap_macros::keymap!(
        r#"
        let K = import "keys.ncl" in

        let MY_MACRO = {
            automation_instructions = {
                while_pressed = [
                    { Press = { key_code = { Keyboard = 0x04 } } },
                    { Release = { key_code = { Keyboard = 0x04 } } },
                ],
            }
        }
        in
        {
            keys = [
                MY_MACRO,
            ],
        }
        "#
    ));

    // Act -- press macro key; wait 3500
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..50 {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    keymap.tick_until_no_scheduled_events();

    // Assert -- the macro should have repeated 3 times
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn test_macro_on_release_not_fired_on_press() {
    // Assemble
    use smart_keymap::input;

    let mut keymap = ObservedKeymap::new(smart_keymap_macros::keymap!(
        r#"
        let K = import "keys.ncl" in

        let MY_MACRO = {
            automation_instructions = {
                on_release = [
                    { Press = { key_code = { Keyboard = 0x04 } } },
                    { Release = { key_code = { Keyboard = 0x04 } } },
                ],
            }
        }
        in
        {
            keys = [
                MY_MACRO,
            ],
        }
        "#
    ));

    // Act -- press macro key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });

    keymap.tick_until_no_scheduled_events();

    // Assert -- the macro should have repeated 3 times
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn test_macro_press_while_pressed_release() {
    // Assemble
    use smart_keymap::input;

    let mut keymap = ObservedKeymap::new(smart_keymap_macros::keymap!(
        r#"
        let K = import "keys.ncl" in

        let MY_MACRO = {
            automation_instructions = {
                on_press = [
                    { Press = { key_code = { Keyboard = 0x04 } } },
                    { Release = { key_code = { Keyboard = 0x04 } } },
                ],
                while_pressed = [
                    { Press = { key_code = { Keyboard = 0x05 } } },
                    { Release = { key_code = { Keyboard = 0x05 } } },
                    { Wait = 1000 },
                ],
                on_release = [
                    { Press = { key_code = { Keyboard = 0x06 } } },
                    { Release = { key_code = { Keyboard = 0x06 } } },
                ],
            }
        }
        in
        {
            keys = [
                MY_MACRO,
            ],
        }
        "#
    ));

    // Act -- press macro key; wait 3500
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..3050 {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    keymap.tick_until_no_scheduled_events();

    // Assert -- the macro should have repeated 3 times
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x05, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x05, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x05, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x06, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
