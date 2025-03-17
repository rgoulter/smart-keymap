use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap;
use smart_keymap::tuples;

use keymap::DistinctReports;
use keymap::Keymap;

use key::{composite, keyboard, layered, tap_hold};
use tuples::Keys2;

type Ctx = composite::Context;
type Ev = composite::Event;
type PK = composite::PressedKey;

type K0 = composite::Chorded<composite::LayeredKey<composite::TapHoldKey<keyboard::Key>>>;
type K1 = composite::Chorded<composite::LayeredKey<composite::TapHoldKey<keyboard::Key>>>;

const KEYS: Keys2<K0, K1, Ctx, Ev, PK> = tuples::Keys2::new((
    composite::Chorded(composite::LayeredKey::Layered(layered::LayeredKey::new(
        composite::TapHoldKey::TapHold(tap_hold::Key {
            tap: keyboard::Key::new(0x04),
            hold: keyboard::Key::new(0x05),
        }),
        [None; 1],
    ))),
    composite::Chorded(composite::LayeredKey::Layered(layered::LayeredKey::new(
        composite::TapHoldKey::TapHold(tap_hold::Key {
            tap: keyboard::Key::new(0x06),
            hold: keyboard::Key::new(0x07),
        }),
        [None; 1],
    ))),
));
const CONTEXT: Ctx = composite::DEFAULT_CONTEXT;

#[test]
fn key_taps() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);
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
