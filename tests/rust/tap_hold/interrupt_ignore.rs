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
const CONTEXT: Ctx = composite::DEFAULT_CONTEXT;

#[test]
fn rolled_presses() {
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
fn rolled_presses_desc_keycodes() {
    // Assemble
    const K_G: u8 = 0x0A;
    const K_O: u8 = 0x12;

    let keys: Keys2<K0, K1, Ctx, Ev, PK> = tuples::Keys2::new((
        composite::Layered::tap_hold(tap_hold::Key {
            tap: keyboard::Key::new(K_O),
            hold: keyboard::Key::new(0xE0),
        }),
        composite::Layered::keyboard(keyboard::Key::new(K_G)),
    ));
    let mut keymap = Keymap::new(keys, CONTEXT);

    // Roll the keys: press 0, press 1, release 0,
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    {
        keymap.tick();
        let actual_report = keymap.report_output().as_hid_boot_keyboard_report();
        let expected_report: [u8; 8] = [0, 0, K_O, 0, 0, 0, 0, 0];
        assert_eq!(expected_report, actual_report);
    }
    {
        keymap.tick();
        let actual_report = keymap.report_output().as_hid_boot_keyboard_report();
        let expected_report: [u8; 8] = [0, 0, K_O, K_G, 0, 0, 0, 0];
        assert_eq!(expected_report, actual_report);
    }

    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    // Tick until the "tap" virtual key from tap-hold
    //  has been released
    for _ in 0..200 {
        keymap.tick();
        let _ = keymap.pressed_keys();
    }

    let _ = keymap.report_output();

    // Act
    // Roll a second time
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    // Assert
    // Only one pressed key reported in each new report,
    //  even for multiple rolls.
    {
        keymap.tick();
        let actual_report = keymap.report_output().as_hid_boot_keyboard_report();
        let expected_report: [u8; 8] = [0, 0, K_O, 0, 0, 0, 0, 0];
        assert_eq!(expected_report, actual_report);
    }
    {
        keymap.tick();
        let actual_report = keymap.report_output().as_hid_boot_keyboard_report();
        let expected_report: [u8; 8] = [0, 0, K_O, K_G, 0, 0, 0, 0];
        assert_eq!(expected_report, actual_report);
    }
}
