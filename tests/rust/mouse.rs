use smart_keymap::{input, key};

#[test]
fn mouse_key() {
    // Assemble
    let mut keymap = smart_keymap_macros::keymap!(
        r#"
        let K = import "keys.ncl" in
        {
            keys = [
                K.MouseBtn1,
            ],
        }
        "#
    );

    // Act -- press 'Btn1'
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.tick();
    let report_output = keymap.report_output();
    let actual_pressed_output = report_output.pressed_mouse_output();

    // Assert
    assert_eq!(
        key::MouseOutput {
            pressed_buttons: 1,
            ..key::MouseOutput::NO_OUTPUT
        },
        actual_pressed_output
    );
    // boot report should be empty
    assert_eq!([0u8; 8], report_output.as_hid_boot_keyboard_report());
}

#[test]
fn mouse_key_cleared_when_released() {
    // Assemble
    let mut keymap = smart_keymap_macros::keymap!(
        r#"
        let K = import "keys.ncl" in
        {
            keys = [
                K.MouseBtn1,
            ],
        }
        "#
    );

    // Act -- press and release 'Btn1'
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.tick();
    let _ = keymap.report_output();
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.tick();
    let actual_mouse_output = keymap.report_output().pressed_mouse_output();

    // Assert -- mouse output is cleared
    assert_eq!(key::MouseOutput::NO_OUTPUT, actual_mouse_output);
}

#[test]
fn multiple_mouse_keys() {
    // Assemble
    let mut keymap = smart_keymap_macros::keymap!(
        r#"
        let K = import "keys.ncl" in
        {
            keys = [
                K.MouseBtn1,
                K.MouseLeft,
                K.MouseWheelUp,
            ],
        }
        "#
    );

    // Act -- press keys
    assert_eq!(
        key::MouseOutput::NO_OUTPUT,
        keymap.report_output().pressed_mouse_output()
    );
    keymap.handle_input(input::Event::Press { keymap_index: 0 }); // MouseBtn1
    keymap.tick();
    let _ = keymap.report_output();
    keymap.handle_input(input::Event::Press { keymap_index: 1 }); // MouseLeft
    keymap.tick();
    let _ = keymap.report_output();
    keymap.handle_input(input::Event::Press { keymap_index: 2 }); // MouseWheelUp
    keymap.tick();

    let report_output = keymap.report_output();
    let actual_pressed_output = report_output.pressed_mouse_output();

    // Assert -- multiple mouse outputs are combined
    assert_eq!(
        key::MouseOutput {
            pressed_buttons: 1,
            x: -5,
            y: 0,
            vertical_scroll: 1,
            horizontal_scroll: 0,
        },
        actual_pressed_output
    );
}
