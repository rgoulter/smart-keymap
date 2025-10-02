use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use smart_keymap_macros::keymap;

#[test]
fn rolled_presses_resolves_hold() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.interrupt_response = "HoldOnKeyPress",
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B,
                ],
            }
        "#
    ));

    // Act
    // Roll the keys: press 0, press 1, release 0, release 1
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0x01, 0, 0, 0, 0, 0, 0, 0],
        [0x01, 0, 0x05, 0, 0, 0, 0, 0],
        [0, 0, 0x05, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn interrupting_press_resolves_hold() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.interrupt_response = "HoldOnKeyPress",
                keys = [
                    K.A & K.hold K.LeftCtrl,
                    K.B,
                ],
            }
        "#
    ));

    // Act
    // Press the TH key, then interrupt it with a press.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0x01, 0, 0, 0, 0, 0, 0, 0],
        [0x01, 0, 0x05, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
