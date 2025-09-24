use smart_keymap::input;

#[test]
fn consumer_key() {
    // Assemble
    let mut keymap = smart_keymap_macros::keymap!(
        r#"
        {
            keys = [
                { usage_code = 0xCD }, # PlayPause
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
