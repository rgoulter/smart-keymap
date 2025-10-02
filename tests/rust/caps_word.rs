use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use smart_keymap_macros::keymap;

#[test]
fn tap_caps_word_shifts_keyboard_keys() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
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
    ));

    // Act
    // Tap CapsWord
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    // Press "A"
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0x02, 0, 0, 0, 0, 0, 0, 0],
        [0x02, 0, 0x04, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn tap_caps_word_spc_deactivates_caps_word() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
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
    ));

    // Act
    // Tap CapsWord
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    // Tap "A"
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    // Tap "Spc"
    keymap.handle_input(input::Event::Press { keymap_index: 3 });
    keymap.handle_input(input::Event::Release { keymap_index: 3 });

    // Press "A"
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

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
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
