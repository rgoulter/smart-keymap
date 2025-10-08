use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use smart_keymap_macros::keymap;

#[test]
fn press_chord_resolves_as_passthrough_quickly_following_key_press() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.chorded.required_idle_time = 100,
                chords = [
                    { indices = [0, 1], key = K.C, },
                ],
                keys = [
                    K.A, K.B, K.D,
                ],
            }
        "#
    ));

    // Act -- press 'd', then soon after, press the chord
    keymap.handle_input(input::Event::Press { keymap_index: 2 });

    for _ in 0..50 {
        keymap.tick();
    }

    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x07, 0, 0, 0, 0, 0],
        [0, 0, 0x07, 0x04, 0, 0, 0, 0],
        [0, 0, 0x07, 0x04, 0x05, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn press_chord_resolves_as_passthrough_quickly_alt_following_key_press() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.chorded.required_idle_time = 100,
                chords = [
                    { indices = [0, 1], key = K.C, },
                ],
                keys = [
                    K.A, K.B, K.D,
                ],
            }
        "#
    ));

    // Act -- press 'd', then soon after, press the chord
    keymap.handle_input(input::Event::Press { keymap_index: 2 });

    for _ in 0..50 {
        keymap.tick();
    }

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Press { keymap_index: 0 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x07, 0, 0, 0, 0, 0],
        [0, 0, 0x07, 0x05, 0, 0, 0, 0],
        [0, 0, 0x07, 0x05, 0x04, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn press_chord_resolves_as_passthrough_when_pressed_quickly() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.chorded.required_idle_time = 100,
                chords = [
                    { indices = [0, 1], key = K.C, },
                ],
                keys = [
                    K.A, K.B, K.D,
                ],
            }
        "#
    ));

    // Act -- tap 'd', then soon after, press the chord
    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    keymap.handle_input(input::Event::Release { keymap_index: 2 });

    for _ in 0..50 {
        keymap.tick();
    }

    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x07, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0x05, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn press_chord_resolves_as_chord_when_pressed_after_required_idle_time() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.chorded.required_idle_time = 100,
                chords = [
                    { indices = [0, 1], key = K.C, },
                ],
                keys = [
                    K.A, K.B, K.D,
                ],
            }
        "#
    ));

    // Act -- tap 'd', then soon after, press the chord
    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    keymap.handle_input(input::Event::Release { keymap_index: 2 });

    for _ in 0..150 {
        keymap.tick();
    }

    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x07, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x06, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn press_chord_resolves_as_chord_following_key_press_after_required_idle_time() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.chorded.required_idle_time = 100,
                chords = [
                    { indices = [0, 1], key = K.C, },
                ],
                keys = [
                    K.A, K.B, K.D,
                ],
            }
        "#
    ));

    // Act -- press 'd', then after required idle time, press the chord
    keymap.handle_input(input::Event::Press { keymap_index: 2 });

    for _ in 0..150 {
        keymap.tick();
    }

    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x07, 0, 0, 0, 0, 0],
        [0, 0, 0x07, 0x06, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn press_chord_resolves_as_chord_following_key_press_after_required_idle_time_alt() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.chorded.required_idle_time = 100,
                chords = [
                    { indices = [0, 1], key = K.C, },
                ],
                keys = [
                    K.A, K.B, K.D,
                ],
            }
        "#
    ));

    // Act -- press 'd', then after required idle time, press the chord
    keymap.handle_input(input::Event::Press { keymap_index: 2 });

    for _ in 0..150 {
        keymap.tick();
    }

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Press { keymap_index: 0 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x07, 0, 0, 0, 0, 0],
        [0, 0, 0x07, 0x06, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn quick_press_chord_resolves_as_chord_following_tap_chord() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.chorded.required_idle_time = 100,
                chords = [
                    { indices = [0, 1], key = K.C, },
                ],
                keys = [
                    K.A, K.B, K.D,
                ],
            }
        "#
    ));

    // Act -- tap 'd', then after required idle time, press the chord
    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    keymap.handle_input(input::Event::Release { keymap_index: 2 });

    for _ in 0..150 {
        keymap.tick();
    }

    // Tap the chord
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    for _ in 0..50 {
        keymap.tick();
    }

    // Quickly re-press the chord
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x07, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x06, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x06, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
