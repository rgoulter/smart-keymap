use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap;
use smart_keymap::tuples;

use keymap::DistinctReports;
use keymap::Keymap;

use key::composite::{Context, Event, KeyState, PendingKeyState};
use key::{composite, keyboard, tap_hold};
use tuples::Keys1;

type K = composite::Chorded<composite::Layered<composite::TapHoldKey<keyboard::Key>>>;
const KEYS: Keys1<K, Context, Event, PendingKeyState, KeyState> = Keys1::new((composite::Chorded(
    composite::Layered(composite::TapHoldKey::TapHold(tap_hold::Key {
        tap: keyboard::Key::new(0x04),
        hold: keyboard::Key::new(0xE0),
    })),
),));
const CONTEXT: Context = composite::Context::from_config(composite::Config {
    tap_hold: tap_hold::Config {
        timeout: 200,
        ..tap_hold::DEFAULT_CONFIG
    },
    ..composite::DEFAULT_CONFIG
});

#[test]
fn keymap_ms_per_tick_affects_tap_hold_timeout() {
    // Assemble -- set ms_per_tick to 100
    let mut keymap = Keymap::new(KEYS, CONTEXT);
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
