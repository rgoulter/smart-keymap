use smart_keymap::input;
use smart_keymap::keymap::ObservedEventBasedKeymap;

use smart_keymap_macros::keymap;

#[test]
fn press_th_key_returns_time_till_ev() {
    // Assemble
    let mut keymap = ObservedEventBasedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.timeout = 200,
                keys = [
                    K.A & K.hold K.LeftCtrl
                ],
            }
        "#
    ));

    // Act
    let actual_ms = keymap.handle_input_after_time(0, input::Event::Press { keymap_index: 0 });

    // Assert
    #[rustfmt::skip]
    let expected_ms = Some(200);
    assert_eq!(expected_ms, actual_ms);
}

#[test]
fn press_th_key_returns_time_till_ev_even_after_some_time() {
    // Assemble
    let mut keymap = ObservedEventBasedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.timeout = 200,
                keys = [
                    K.A & K.hold K.LeftCtrl
                ],
            }
        "#
    ));

    // Act
    let _ = keymap.handle_input_after_time(100, input::Event::Press { keymap_index: 0 });
    let _ = keymap.handle_input_after_time(100, input::Event::Release { keymap_index: 0 });
    let actual_ms = keymap.handle_input_after_time(100, input::Event::Press { keymap_index: 0 });

    // Assert
    #[rustfmt::skip]
    let expected_ms = Some(200);
    assert_eq!(expected_ms, actual_ms);
}

#[test]
fn uninterrupted_th_tap_is_tap() {
    // Assemble
    let mut keymap = ObservedEventBasedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.timeout = 200,
                keys = [
                    K.A & K.hold K.LeftCtrl
                ],
            }
        "#
    ));

    // Act
    let _ = keymap.handle_input_after_time(0, input::Event::Press { keymap_index: 0 });
    let _ = keymap.handle_input_after_time(150, input::Event::Release { keymap_index: 0 });

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
fn held_press_resolves_as_hold() {
    // Assemble
    let mut keymap = ObservedEventBasedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.timeout = 200,
                keys = [
                    K.A & K.hold K.LeftCtrl
                ],
            }
        "#
    ));

    // Act
    // With event based interface: `tick_to_next_scheduled_event` called
    // when an event is triggered.
    let _ = keymap.handle_input_after_time(0, input::Event::Press { keymap_index: 0 });
    let _ = keymap.tick_to_next_scheduled_event();

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0x01, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
