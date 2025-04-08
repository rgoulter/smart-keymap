use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap;
use smart_keymap::tuples;

use keymap::DistinctReports;
use keymap::Keymap;

use key::{composite, keyboard, tap_hold};
use tuples::{Keys2, Keys4};

type Ctx = composite::Context;
type Ev = composite::Event;
type PKS = composite::PendingKeyState;
type KS = composite::KeyState;

type K0 = composite::Chorded<composite::Layered<composite::TapHoldKey<keyboard::Key>>>;
type K1 = composite::Chorded<composite::Layered<composite::TapHold<keyboard::Key>>>;

const KEYS: Keys2<K0, K1, Ctx, Ev, PKS, KS> = tuples::Keys2::new((
    composite::Chorded(composite::Layered(composite::TapHoldKey::TapHold(
        tap_hold::Key {
            tap: keyboard::Key::new(0x04),
            hold: keyboard::Key::new(0xE0),
        },
    ))),
    composite::Chorded(composite::Layered(composite::TapHold(keyboard::Key::new(
        0x05,
    )))),
));

const CONTEXT: Ctx = key::composite::Context::from_config(composite::Config {
    tap_hold: tap_hold::Config {
        interrupt_response: tap_hold::InterruptResponse::HoldOnKeyTap,
        ..tap_hold::DEFAULT_CONFIG
    },
    ..composite::DEFAULT_CONFIG
});

#[test]
fn rolled_presses_resolves_tap() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    // Roll the keys: press 0, press 1, release 0, release 1
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
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0x05, 0, 0, 0, 0],
        [0, 0, 0x05, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn interrupting_tap_resolves_hold() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    // Press the TH key, then interrupt it with a tap.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
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
        [0x01, 0, 0, 0, 0, 0, 0, 0],
        [0x01, 0, 0x05, 0, 0, 0, 0, 0],
        [0x01, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn rolling_nested_tap_th_tap_th_tap_kbd() {
    // Observed buggy behaviour with,
    //  (in the case where the releases occur quickly enough,
    //   quicker than the keymap's INPUT_QUEUE_TICK_DELAY):
    // - Press TH(A)
    // - Press TH(B)
    // - Press C
    // - Release TH(A)
    // - Release TH(B)
    // - Release C

    // Assemble
    let keys: Keys4<K0, K0, K1, K1, Ctx, Ev, PKS, KS> = tuples::Keys4::new((
        composite::Chorded(composite::Layered(composite::TapHoldKey::TapHold(
            tap_hold::Key {
                tap: keyboard::Key::new(0x04),
                hold: keyboard::Key::new(0xE0),
            },
        ))),
        composite::Chorded(composite::Layered(composite::TapHoldKey::TapHold(
            tap_hold::Key {
                tap: keyboard::Key::new(0x05),
                hold: keyboard::Key::new(0xE0),
            },
        ))),
        composite::Chorded(composite::Layered(composite::TapHold(keyboard::Key::new(
            0x06,
        )))),
        composite::Chorded(composite::Layered(composite::TapHold(keyboard::Key::new(
            0x07,
        )))),
    ));
    let mut keymap = Keymap::new(keys, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    // Press the TH key, then interrupt it with a tap.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 1 });
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
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0x05, 0, 0, 0, 0],
        [0, 0, 0x04, 0x05, 0x06, 0, 0, 0],
        [0, 0, 0x05, 0x06, 0, 0, 0, 0],
        [0, 0, 0x06, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn rolling_nested_tap_th_tap_th_tap_th() {
    // Observed buggy behaviour with,
    //  (in the case where the releases occur quickly enough,
    //   quicker than the keymap's INPUT_QUEUE_TICK_DELAY):
    // - Press TH(A)
    // - Press TH(B)
    // - Press TH(C)
    // - Release TH(A)
    // - Release TH(B)
    // - Release TH(C)

    // Assemble
    let keys: Keys4<K0, K0, K0, K1, Ctx, Ev, PKS, KS> = tuples::Keys4::new((
        composite::Chorded(composite::Layered(composite::TapHoldKey::TapHold(
            tap_hold::Key {
                tap: keyboard::Key::new(0x04),
                hold: keyboard::Key::new(0xE0),
            },
        ))),
        composite::Chorded(composite::Layered(composite::TapHoldKey::TapHold(
            tap_hold::Key {
                tap: keyboard::Key::new(0x05),
                hold: keyboard::Key::new(0xE0),
            },
        ))),
        composite::Chorded(composite::Layered(composite::TapHoldKey::TapHold(
            tap_hold::Key {
                tap: keyboard::Key::new(0x06),
                hold: keyboard::Key::new(0xE0),
            },
        ))),
        composite::Chorded(composite::Layered(composite::TapHold(keyboard::Key::new(
            0x07,
        )))),
    ));
    let mut keymap = Keymap::new(keys, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    // Press the TH key, then interrupt it with a tap.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 1 });
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
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0x05, 0, 0, 0, 0],
        [0, 0, 0x04, 0x05, 0x06, 0, 0, 0],
        [0, 0, 0x05, 0x06, 0, 0, 0, 0],
        [0, 0, 0x06, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn tap_th_after_rolling_th_kbd() {
    // Observed buggy behaviour with,
    // - Press TH(A)
    // - Press B
    // - Release TH(A)
    // - Release B
    // - Press TH(C)
    // - Release TH(C)

    // Assemble
    let keys: Keys4<K0, K1, K0, K1, Ctx, Ev, PKS, KS> = tuples::Keys4::new((
        composite::Chorded(composite::Layered(composite::TapHoldKey::TapHold(
            tap_hold::Key {
                tap: keyboard::Key::new(0x04),
                hold: keyboard::Key::new(0xE0),
            },
        ))),
        composite::Chorded(composite::Layered(composite::TapHold(keyboard::Key::new(
            0x05,
        )))),
        composite::Chorded(composite::Layered(composite::TapHoldKey::TapHold(
            tap_hold::Key {
                tap: keyboard::Key::new(0x06),
                hold: keyboard::Key::new(0xE0),
            },
        ))),
        composite::Chorded(composite::Layered(composite::TapHold(keyboard::Key::new(
            0x07,
        )))),
    ));
    let mut keymap = Keymap::new(keys, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    // Press the TH key, then interrupt it with a tap.
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
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0x05, 0, 0, 0, 0],
        [0, 0, 0x05, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x06, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn tap_th_then_tap_th() {
    // Observed buggy behaviour with,
    // - Press TH(A)
    // - Release TH(A)
    // - Press TH(C)
    // - Release TH(C)

    // Assemble
    let keys: Keys4<K0, K1, K0, K1, Ctx, Ev, PKS, KS> = tuples::Keys4::new((
        composite::Chorded(composite::Layered(composite::TapHoldKey::TapHold(
            tap_hold::Key {
                tap: keyboard::Key::new(0x04),
                hold: keyboard::Key::new(0xE0),
            },
        ))),
        composite::Chorded(composite::Layered(composite::TapHold(keyboard::Key::new(
            0x05,
        )))),
        composite::Chorded(composite::Layered(composite::TapHoldKey::TapHold(
            tap_hold::Key {
                tap: keyboard::Key::new(0x06),
                hold: keyboard::Key::new(0xE0),
            },
        ))),
        composite::Chorded(composite::Layered(composite::TapHold(keyboard::Key::new(
            0x07,
        )))),
    ));
    let mut keymap = Keymap::new(keys, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 0 });
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
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x06, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}
