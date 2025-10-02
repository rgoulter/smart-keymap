mod release_on_next_press;

use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use smart_keymap_macros::keymap;

#[test]
fn tap_sticky_mod_modifies_next_keyboard_key() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.sticky K.LeftShift,
                    K.A,
                    K.Slash,
                    K.sticky K.LeftCtrl,
                ],
            }
        "#
    ));

    // Act
    // Tap Sticky Modifier
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
fn tap_sticky_mod_acts_as_regular_mod_when_interrupted_by_key_slash() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.sticky K.LeftShift,
                    K.A,
                    K.Slash,
                    K.sticky K.LeftCtrl,
                ],
            }
        "#
    ));

    // Act
    // Tap Sticky Modifier
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    // Press "/"
    keymap.handle_input(input::Event::Press { keymap_index: 2 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0x02, 0, 0, 0, 0, 0, 0, 0],
        [0x02, 0, 0x38, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn tap_sticky_mod_modifies_only_next_keyboard_key() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.sticky K.LeftShift,
                    K.A,
                    K.Slash,
                    K.sticky K.LeftCtrl,
                ],
            }
        "#
    ));

    // Act
    // Tap Sticky Modifier
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    // Tap "A"
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    // Press "/"
    keymap.handle_input(input::Event::Press { keymap_index: 2 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0x02, 0, 0, 0, 0, 0, 0, 0],
        [0x02, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x38, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn tap_sticky_mod_acts_as_regular_mod_when_interrupted_by_key() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.sticky K.LeftShift,
                    K.A,
                    K.Slash,
                    K.sticky K.LeftCtrl,
                ],
            }
        "#
    ));

    // Act
    // Press Sticky Modifier
    keymap.handle_input(input::Event::Press { keymap_index: 0 });

    // Press "A"
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    // Release Sticky Modifier
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    // Release "A"
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0x02, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn tap_multiple_sticky_mod_modifies_next_keyboard_key() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.sticky K.LeftShift,
                    K.A,
                    K.Slash,
                    K.sticky K.LeftCtrl,
                ],
            }
        "#
    ));

    // Act
    // Tap Sticky Modifiers
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 3 });
    keymap.handle_input(input::Event::Release { keymap_index: 3 });

    // Tap "A"
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0x02, 0, 0, 0, 0, 0, 0, 0],
        [0x03, 0, 0, 0, 0, 0, 0, 0],
        [0x03, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn tap_sticky_mod_modifies_next_keyboard_key_until_released() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.sticky K.LeftShift,
                    K.A,
                    K.Slash,
                    K.sticky K.LeftCtrl,
                ],
            }
        "#
    ));

    // Act
    // Tap Sticky Modifier
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    // Press "A"
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    // Tap "/"
    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    keymap.handle_input(input::Event::Release { keymap_index: 2 });

    // Release "A"
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0x02, 0, 0, 0, 0, 0, 0, 0],
        [0x02, 0, 0x04, 0, 0, 0, 0, 0],
        [0x02, 0, 0x04, 0x38, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
