mod overlapping;
mod overlapping_aux;
mod overlapping_simultaneous;
mod tap_hold;

use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap;
use smart_keymap::slice::Slice;
use smart_keymap::tuples;

use keymap::DistinctReports;
use keymap::Keymap;

use key::{chorded, composite, keyboard};
use tuples::Keys2;

type Ctx = composite::Context;
type Ev = composite::Event;
type PKS = composite::PendingKeyState;
type KS = composite::KeyState;
type CK = composite::ChordedKey<composite::Layered<composite::TapHold<keyboard::Key>>>;
type AK = composite::ChordedKey<composite::Layered<composite::TapHold<keyboard::Key>>>;

const KEYS: Keys2<CK, AK, Ctx, Ev, PKS, KS> = tuples::Keys2::new((
    composite::ChordedKey::Chorded(chorded::Key::new(
        &[(
            0,
            composite::Layered(composite::TapHold(keyboard::Key::new(0x06))),
        )],
        composite::Layered(composite::TapHold(keyboard::Key::new(0x04))),
    )),
    composite::ChordedKey::Auxiliary(chorded::AuxiliaryKey::new(composite::Layered(
        composite::TapHold(keyboard::Key::new(0x05)),
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
fn tap_auxiliary_key_acts_as_passthrough() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

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
fn tap_chorded_key_acts_as_passthrough() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn press_chord_acts_as_chord() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x06, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn press_chord_alt_order_acts_as_chord() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    // (press the auxiliary key first)
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x06, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn nested_tap_chord_acts_as_chord() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 1 });
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
        [0, 0, 0x06, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn rolling_tap_chord_acts_as_chord() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x06, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn press_chord_4_acts_as_chord() {
    // Assemble
    let keys: tuples::Keys4<CK, AK, AK, AK, Ctx, Ev, PKS, KS> = tuples::Keys4::new((
        composite::ChordedKey::Chorded(chorded::Key::new(
            &[(
                0,
                composite::Layered(composite::TapHold(keyboard::Key::new(0x08))),
            )],
            composite::Layered(composite::TapHold(keyboard::Key::new(0x04))),
        )),
        composite::ChordedKey::Auxiliary(chorded::AuxiliaryKey::new(composite::Layered(
            composite::TapHold(keyboard::Key::new(0x05)),
        ))),
        composite::ChordedKey::Auxiliary(chorded::AuxiliaryKey::new(composite::Layered(
            composite::TapHold(keyboard::Key::new(0x06)),
        ))),
        composite::ChordedKey::Auxiliary(chorded::AuxiliaryKey::new(composite::Layered(
            composite::TapHold(keyboard::Key::new(0x07)),
        ))),
    ));
    let context: Ctx = key::composite::Context::from_config(composite::Config {
        chorded: chorded::Config {
            chords: Slice::from_slice(&[chorded::ChordIndices::from_slice(&[0, 1, 2, 3])]),
            ..chorded::DEFAULT_CONFIG
        },
        ..composite::DEFAULT_CONFIG
    });
    let mut keymap = Keymap::new(keys, context);
    let mut actual_reports = DistinctReports::new();

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 3 });
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
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn release_and_repress_chorded_after_hold_chord_acts_as_passthrough() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x06, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn release_and_repress_aux_after_hold_chord_acts_as_passthrough() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x06, 0, 0, 0, 0, 0],
        [0, 0, 0x06, 0x05, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}
