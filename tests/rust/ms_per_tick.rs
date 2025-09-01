use smart_keymap::input;
use smart_keymap::keymap;

use smart_keymap_macros::keymap;

use keymap::DistinctReports;

#[test]
fn keymap_ms_per_tick_affects_tap_hold_timeout() {
    // Assemble -- set ms_per_tick to 100
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.tap_hold.timeout = 200,
                keys = [
                    K.A & K.hold K.LeftCtrl
                ],
            }
        "#
    );
    keymap.set_ms_per_tick(100);

    let mut actual_reports = DistinctReports::new();

    // Act -- tick 21 times
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    for _ in 0..21 {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert -- tap-hold key should have resolved as 'hold'
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [1, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}
