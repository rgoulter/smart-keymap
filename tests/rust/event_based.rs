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

#[test]
fn tick_to_next_scheduled_event_does_not_invoke_later_events() {
    // Assemble
    let mut keymap = ObservedEventBasedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.timeout = 200,
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B & K.hold K.LeftShift,
                ],
            }
        "#
    ));

    // Act
    // Event based interface 'fires' tick_to_next_scheduled_event
    //  after the time from each handle_input_ call.

    // Pressing TapHold key: A timeout is scheduled "at time 200"
    // press   @ time   0 (schedule timeout after time 200)
    let _ = keymap.handle_input_after_time(0, input::Event::Press { keymap_index: 0 });
    // release @ time 100
    let _ = keymap.handle_input_after_time(100, input::Event::Release { keymap_index: 0 });

    // Press a different TapHold key before the timeout from the first key has fired.
    // press   @ time 150 (schedule another timeout after time 200 = time 350)
    let _ = keymap.handle_input_after_time(50, input::Event::Press { keymap_index: 1 });
    //         @ time 200, first key press's timeout fires
    let _ = keymap.tick_to_next_scheduled_event();
    // release @ time 300 (= time 100 after last event),
    let _ = keymap.handle_input_after_time(100, input::Event::Release { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x4, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x5, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn tick_to_next_scheduled_event_advances_time() {
    // Assemble
    let mut keymap = ObservedEventBasedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.timeout = 200,
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B & K.hold K.LeftShift,
                ],
            }
        "#
    ));

    // Act
    // Event based interface 'fires' tick_to_next_scheduled_event
    //  after the time from each handle_input_ call.

    // Pressing TapHold key: A timeout is scheduled "at time 200"
    // press   @ time   0 (schedule timeout after time 200)
    let _ = keymap.handle_input_after_time(0, input::Event::Press { keymap_index: 0 });
    // release @ time 100
    let _ = keymap.handle_input_after_time(100, input::Event::Release { keymap_index: 0 });

    // Press a different TapHold key before the timeout from the first key has fired.
    // press   @ time 150 (schedule another timeout after time 200 = time 350)
    let _ = keymap.handle_input_after_time(50, input::Event::Press { keymap_index: 1 });
    //         @ time 200, first key press's timeout fires
    let _ = keymap.tick_to_next_scheduled_event();
    //         @ time 350, second key press's timeout fires
    let _ = keymap.tick_to_next_scheduled_event();

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x4, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0x2, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
