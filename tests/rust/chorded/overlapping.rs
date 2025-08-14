use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap;
use smart_keymap::slice::Slice;
use smart_keymap::tuples;

use keymap::DistinctReports;
use keymap::Keymap;

use key::{chorded, composite, keyboard};
use tuples::Keys4;

type Ctx = composite::Context;
type Ev = composite::Event;
type PKS = composite::PendingKeyState;
type KS = composite::KeyState;
type CK = composite::ChordedKey<composite::Layered<composite::TapHold<keyboard::Key>>>;
type AK = composite::ChordedKey<composite::Layered<composite::TapHold<keyboard::Key>>>;

// 4-key keymap
//   A B C D
// chords:
// 0: X X - - => M=0x10
// 1: X X X - => N=0x11
// 2: - X X - => O=0x12
// 3: - - X X => P=0x13

const KEYS: Keys4<CK, CK, CK, AK, Ctx, Ev, PKS, KS> = tuples::Keys4::new((
    composite::ChordedKey::Chorded(chorded::Key::new(
        &[
            (
                0,
                composite::Layered(composite::TapHold(keyboard::Key::new(0x10))),
            ),
            (
                1,
                composite::Layered(composite::TapHold(keyboard::Key::new(0x11))),
            ),
        ],
        composite::Layered(composite::TapHold(keyboard::Key::new(0x04))),
    )),
    composite::ChordedKey::Chorded(chorded::Key::new(
        &[(
            2,
            composite::Layered(composite::TapHold(keyboard::Key::new(0x12))),
        )],
        composite::Layered(composite::TapHold(keyboard::Key::new(0x05))),
    )),
    composite::ChordedKey::Chorded(chorded::Key::new(
        &[(
            3,
            composite::Layered(composite::TapHold(keyboard::Key::new(0x13))),
        )],
        composite::Layered(composite::TapHold(keyboard::Key::new(0x06))),
    )),
    composite::ChordedKey::Auxiliary(chorded::AuxiliaryKey::new(composite::Layered(
        composite::TapHold(keyboard::Key::new(0x07)),
    ))),
));

const CONTEXT: Ctx = key::composite::Context::from_config(composite::Config {
    chorded: chorded::Config {
        chords: Slice::from_slice(&[
            chorded::ChordIndices::from_slice(&[0, 1]),
            chorded::ChordIndices::from_slice(&[0, 1, 2]),
            chorded::ChordIndices::from_slice(&[1, 2]),
            chorded::ChordIndices::from_slice(&[2, 3]),
        ]),
        ..chorded::DEFAULT_CONFIG
    },
    ..composite::DEFAULT_CONFIG
});

#[test]
fn overlap_tap_key_acts_as_passthrough() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    let tap_indices = &[1];

    for &keymap_index in tap_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
        keymap.handle_input(input::Event::Release { keymap_index });
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x05, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn overlap_partial_press_cd_acts_as_chord() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    let press_indices = &[2, 3];

    for &keymap_index in press_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    let expected_reports: &[[u8; 8]] = &[[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0x13, 0, 0, 0, 0, 0]];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn overlap_partial_press_dc_acts_as_chord() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    let press_indices = &[3, 2];

    for &keymap_index in press_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    let expected_reports: &[[u8; 8]] = &[[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0x13, 0, 0, 0, 0, 0]];
    assert_eq!(expected_reports, actual_reports.reports());
}
