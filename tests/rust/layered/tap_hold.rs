use smart_keymap::input;
use smart_keymap::keymap;

use smart_keymap_macros::keymap;

use keymap::DistinctReports;

#[test]
fn key_taps() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.A & K.hold K.B, K.C & K.hold K.D],
                    [K.TTTT, K.TTTT],
                ],
            }
        "#
    );
    let mut actual_reports = DistinctReports::new();

    // Act
    // Tap (press, release) the layered key of the tap-hold key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    for _ in 0..smart_keymap::keymap::INPUT_QUEUE_TICK_DELAY {
        keymap.tick();
    }

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}
