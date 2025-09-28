use smart_keymap::input;

use smart_keymap_macros::keymap;

#[test]
fn press_active_layer_when_layer_mod_toggle_pressed() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.toggle 1, K.A],
                    [K.TTTT, K.B],
                ],
            }
        "#
    );

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.tick();
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.tick();
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0, 0, 0x05, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, actual_report);
}

#[test]
fn press_active_layer_when_layer_mod_toggle_tapped() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.toggle 1, K.A],
                    [K.TTTT, K.B],
                ],
            }
        "#
    );

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.tick();
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.tick();
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.tick();
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0, 0, 0x05, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, actual_report);
}

#[test]
fn toggle_tapped_twice_deactivates_layer() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.toggle 1, K.A],
                    [K.TTTT, K.B],
                ],
            }
        "#
    );

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.tick();
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.tick();
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.tick();
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.tick();
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.tick();
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, actual_report);
}
