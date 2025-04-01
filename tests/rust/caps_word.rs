use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap;
use smart_keymap::tuples;

use keymap::DistinctReports;
use keymap::Keymap;

use key::{caps_word, composite, keyboard};
use tuples::Keys4;

type Ctx = composite::Context;
type Ev = composite::Event;
type PKS = composite::PendingKeyState;
type KS = composite::KeyState;
type CK = composite::Chorded<composite::Layered<composite::TapHold<caps_word::Key>>>;
type K = composite::Chorded<composite::Layered<composite::TapHold<keyboard::Key>>>;

const KEYS: Keys4<CK, K, K, K, Ctx, Ev, PKS, KS> = tuples::Keys4::new((
    composite::Chorded(composite::Layered(
        composite::TapHold(caps_word::Key::new()),
    )),
    composite::Chorded(
        composite::Layered(composite::TapHold(keyboard::Key::new(0x04))), // A
    ),
    composite::Chorded(
        composite::Layered(composite::TapHold(keyboard::Key::new(0x05))), // B
    ),
    composite::Chorded(
        composite::Layered(composite::TapHold(keyboard::Key::new(0x2C))), // Space
    ),
));

const CONTEXT: Ctx = composite::DEFAULT_CONTEXT;

#[test]
fn tap_caps_word_shifts_keyboard_keys() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    // Tap CapsWord
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    // Press "A"
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0x02, 0, 0, 0, 0, 0, 0, 0],
        [0x02, 0, 0x04, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn tap_caps_word_spc_deactivates_caps_word() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    // Tap CapsWord
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    // Tap "A"
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    // Tap "Spc"
    keymap.handle_input(input::Event::Press { keymap_index: 3 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    keymap.handle_input(input::Event::Release { keymap_index: 3 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    // Press "A"
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0x02, 0, 0, 0, 0, 0, 0, 0],
        [0x02, 0, 0x04, 0, 0, 0, 0, 0],
        [0x02, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x2C, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}
