mod hold_on_interrupt_press;
mod hold_on_interrupt_tap;
mod interrupt_ignore;
mod layered;
mod required_idle_time;

use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use smart_keymap_macros::keymap;

#[test]
fn key_tapped() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.A & K.hold K.LeftCtrl
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
fn key_uninterrupted_tap_is_reported() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.A & K.hold K.LeftCtrl
                ],
            }
        "#
    ));

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn key_unaffected_by_prev_key_release() {
    // When a tap-hold key is pressed,
    //  it schedules a Timeout event after 200 ticks.
    // In case of releasing, then pressing the key a second time within 200 ticks,
    //  we do not want the first Timeout to affect the second key press.

    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                keys = [
                    K.A & K.hold K.LeftCtrl
                ],
            }
        "#
    ));

    // Act
    // Press key (starting a 200 tick timeout),
    keymap.handle_input(input::Event::Press { keymap_index: 0 });

    // Release, then press key a second time before 200 ticks.
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    for _ in 0..150 {
        keymap.tick();
    }

    keymap.handle_input(input::Event::Press { keymap_index: 0 });

    // Tick a few more times, until the first timeout would be scheduled,
    // (but before the second timeout is scheduled)
    for _ in 0..100 {
        // 150 + 100 = 250
        keymap.tick();
    }

    // Assert
    // Second timeout not invoked, key is still "Pending" state.
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
