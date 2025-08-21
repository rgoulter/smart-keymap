use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap;
use smart_keymap::slice::Slice;
use smart_keymap::tuples;

use keymap::DistinctReports;
use keymap::Keymap;

use key::{chorded, composite, keyboard, layered, tap_hold};
use tuples::Keys4;

type Ctx = composite::Context;
type Ev = composite::Event;
type PKS = composite::PendingKeyState;
type KS = composite::KeyState;
type CK = composite::ChordedKey<composite::LayeredKey<composite::TapHoldKey<keyboard::Key>>>;
type AK = composite::ChordedKey<composite::Layered<composite::TapHold<keyboard::Key>>>;
type LK = composite::Chorded<composite::LayeredKey<composite::TapHold<keyboard::Key>>>;
type MK = composite::Chorded<composite::Layered<composite::TapHold<layered::ModifierKey>>>;

// 4 keys:
//   0: Layers: [{ tap: A, hold: lctrl }, { tap: F, hold: lshift }],
//   1: B
//   2: Layers: [C, D]
//   3: Set Default (Layer 1)
// chord [01] = { E }
const KEYS: Keys4<CK, AK, LK, MK, Ctx, Ev, PKS, KS> = tuples::Keys4::new((
    composite::ChordedKey::Chorded(chorded::Key::new(
        &[(
            0,
            composite::LayeredKey::Pass(composite::TapHoldKey::Pass(keyboard::Key::new(0x08))),
        )],
        composite::LayeredKey::Layered(layered::LayeredKey::new(
            composite::TapHoldKey::TapHold(tap_hold::Key::new(
                keyboard::Key::new(0x04),
                keyboard::Key::new(0xE2),
            )),
            [Some(composite::TapHoldKey::TapHold(tap_hold::Key::new(
                keyboard::Key::new(0x09),
                keyboard::Key::new(0xE2),
            )))],
        )),
    )),
    composite::ChordedKey::Auxiliary(chorded::AuxiliaryKey::new(composite::Layered(
        composite::TapHold(keyboard::Key::new(0x05)),
    ))),
    composite::Chorded(composite::LayeredKey::Layered(layered::LayeredKey::new(
        composite::TapHold(keyboard::Key::new(0x06)),
        [Some(composite::TapHold(keyboard::Key::new(0x07)))],
    ))),
    composite::Chorded(composite::Layered(composite::TapHold(
        layered::ModifierKey::Default(1),
    ))),
));

const CONTEXT: Ctx = key::composite::Context::from_config(composite::Config {
    chorded: chorded::Config {
        chords: Slice::from_slice(&[chorded::ChordIndices::from_slice(&[0, 1])]),
        ..chorded::DEFAULT_CONFIG
    },
    ..composite::DEFAULT_CONFIG
});

#[test]
fn tap_key_after_tapping_chord_on_default_layer() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    // - Default layer,
    // - Press chord (01), release chord.
    // - Press letter.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 2 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x08, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x06, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn tap_key_after_tapping_chord_on_layer_1() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    // - Set default layer to 1
    // - Press chord (01), release chord.
    // - Press letter.
    keymap.handle_input(input::Event::Press { keymap_index: 3 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 3 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 2 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x08, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x07, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn tap_chorded_key_passes_through_as_tap() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    // - Default layer,
    // - Press chord (01), release chord.
    // - Press letter.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

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
