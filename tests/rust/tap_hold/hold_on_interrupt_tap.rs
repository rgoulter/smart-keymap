use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap;
use smart_keymap::tuples;

use keymap::Keymap;

use key::{composite, keyboard, tap_hold};
use tuples::Keys2;

type Ctx = composite::Context;
type Ev = composite::Event;
type PK = composite::PressedKey;

type K0 = composite::Layered<composite::TapHoldKey<keyboard::Key>>;
type K1 = composite::Layered<composite::TapHold<keyboard::Key>>;

const KEYS: Keys2<K0, K1, Ctx, Ev, PK> = tuples::Keys2::new((
    composite::Layered(composite::TapHoldKey::TapHold(tap_hold::Key {
        tap: keyboard::Key::new(0x04),
        hold: keyboard::Key::new(0xE0),
    })),
    composite::Layered(composite::TapHold(keyboard::Key::new(0x05))),
));

const CONTEXT: Ctx = Ctx {
    tap_hold_context: tap_hold::Context::from_config(tap_hold::Config {
        interrupt_response: tap_hold::InterruptResponse::HoldOnKeyTap,
        ..tap_hold::DEFAULT_CONFIG
    }),
    ..composite::DEFAULT_CONTEXT
};

#[test]
fn rolled_presses_resolves_tap() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);

    // Act
    // Roll the keys: press 0, press 1, release 0,
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0, 0, 0x04, 0x05, 0, 0, 0, 0];
    assert_eq!(expected_report, actual_report);
}

#[test]
fn interrupting_tap_resolves_hold() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);

    // Act
    // Press the TH key, then interrupt it with a press.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0x01, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, actual_report);
}
