use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use smart_keymap_macros::keymap;

#[test]
fn tap_hold_resolves_as_tap_when_pressed_before_required_idle_time() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.required_idle_time = 100,
                config.tap_hold.timeout = 200,
                keys = [
                    K.A,
                    K.B & K.hold K.LeftCtrl
                ],
            }
        "#
    ));

    // Act -- tap 'a', then soon after, tap the tap-hold key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    for _ in 0..50 {
        keymap.tick();
    }

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert -- tap-hold key immediately resolves as 'tap'
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x05, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn tap_hold_resolves_as_tap_when_tapped_after_required_idle_time() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.required_idle_time = 100,
                config.tap_hold.timeout = 200,
                keys = [
                    K.A,
                    K.B & K.hold K.LeftCtrl
                ],
            }
        "#
    ));

    // Act -- tap 'a', then soon after, tap the tap-hold key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    for _ in 0..101 {
        keymap.tick();
    }

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert -- tap-hold key immediately resolves as 'tap'
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x05, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn tap_hold_resolves_as_hold_when_held_after_required_idle_time() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.required_idle_time = 100,
                config.tap_hold.timeout = 200,
                keys = [
                    K.A,
                    K.B & K.hold K.LeftCtrl
                ],
            }
        "#
    ));

    // Act -- tap 'a', then soon after, tap the tap-hold key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    for _ in 0..101 {
        keymap.tick();
    }

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    for _ in 0..201 {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert -- tap-hold key immediately resolves as 'tap'
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
