use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use smart_keymap_macros::keymap;

#[test]
fn tap_chord_acts_as_chorded_tap() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                chords = [
                    { indices = [0, 1], key = K.C & K.hold K.LeftAlt, },
                ],
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B & K.hold K.LeftShift,
                ],
            }
        "#
    ));

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x06, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn hold_chord_acts_as_chorded_hold() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                chords = [
                    { indices = [0, 1], key = K.C & K.hold K.LeftAlt, },
                ],
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B & K.hold K.LeftShift,
                ],
            }
        "#
    ));

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [4, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn tap_chorded_key_acts_as_passthrough_tap() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                chords = [
                    { indices = [0, 1], key = K.C & K.hold K.LeftAlt, },
                ],
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B & K.hold K.LeftShift,
                ],
            }
        "#
    ));

    // Act
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
fn hold_chorded_key_acts_as_passthrough_hold() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                chords = [
                    { indices = [0, 1], key = K.C & K.hold K.LeftAlt, },
                ],
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B & K.hold K.LeftShift,
                ],
            }
        "#
    ));

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn tap_auxiliary_key_acts_as_passthrough_tap() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                chords = [
                    { indices = [0, 1], key = K.C & K.hold K.LeftAlt, },
                ],
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B & K.hold K.LeftShift,
                ],
            }
        "#
    ));

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x05, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn hold_auxiliary_key_acts_as_passthrough_hold() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                chords = [
                    { indices = [0, 1], key = K.C & K.hold K.LeftAlt, },
                ],
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B & K.hold K.LeftShift,
                ],
            }
        "#
    ));

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [2, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
