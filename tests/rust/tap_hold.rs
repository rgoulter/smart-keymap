mod interrupt_ignore;
mod layered;

use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap;
use smart_keymap::tuples;

use keymap::Keymap;

use key::composite::{Context, Event, PressedKey};
use key::{composite, keyboard, tap_hold};
use tuples::Keys1;

type K = composite::Layered<composite::TapHoldKey<keyboard::Key>>;
const KEYS: Keys1<K, Context, Event, PressedKey> = Keys1::new((composite::Layered(
    composite::TapHoldKey::TapHold(tap_hold::Key {
        tap: keyboard::Key::new(0x04),
        hold: keyboard::Key::new(0xE0),
    }),
),));
const CONTEXT: Context = composite::DEFAULT_CONTEXT;

#[test]
fn key_tapped() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];
    assert_eq!(actual_report, expected_report);
}

#[test]
fn key_unaffected_by_prev_key_release() {
    // When a tap-hold key is pressed,
    //  it schedules a Timeout event after 200 ticks.
    // In case of releasing, then pressing the key a second time within 200 ticks,
    //  we do not want the first Timeout to affect the second key press.

    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);

    // Act
    // Press key (starting a 200 tick timeout),
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    // Release, then press key a second time before 200 ticks.
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    for _ in 0..150 {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    // Tick a few more times, until the first timeout would be scheduled,
    // (but before the second timeout is scheduled)
    for _ in 0..100 {
        // 250
        keymap.tick();
    }
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    // Second timeout not invoked, key is still "Pending" state.
    let expected_report: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(actual_report, expected_report);
}
