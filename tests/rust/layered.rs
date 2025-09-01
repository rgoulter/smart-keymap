mod set_active_layers;
mod tap_hold;

use smart_keymap::input;

use smart_keymap_macros::keymap;

#[test]
fn press_base_key_when_no_layers_active() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.hold 1, K.A],
                    [K.TTTT, K.B],
                ],
            }
        "#
    );

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, actual_report,);
}

#[test]
fn press_active_layer_when_layer_mod_held() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.hold 1, K.A],
                    [K.TTTT, K.B],
                ],
            }
        "#
    );

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..smart_keymap::keymap::INPUT_QUEUE_TICK_DELAY {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0, 0, 0x05, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, actual_report);
}

#[test]
fn press_retained_when_layer_mod_released() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.hold 1, K.A],
                    [K.TTTT, K.B],
                ],
            }
        "#
    );

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..smart_keymap::keymap::INPUT_QUEUE_TICK_DELAY {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    for _ in 0..smart_keymap::keymap::INPUT_QUEUE_TICK_DELAY {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0, 0, 0x05, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, actual_report);
}

#[test]
fn uses_base_when_pressed_after_layer_mod_released() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.hold 1, K.A],
                    [K.TTTT, K.B],
                ],
            }
        "#
    );

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..smart_keymap::keymap::INPUT_QUEUE_TICK_DELAY {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    for _ in 0..smart_keymap::keymap::INPUT_QUEUE_TICK_DELAY {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, actual_report);
}
