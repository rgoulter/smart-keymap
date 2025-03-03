use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap;
use smart_keymap::tuples;

use keymap::Keymap;

use key::{composite, keyboard, layered};
use tuples::Keys2;

type Ctx = composite::Context;
type Ev = composite::Event;
type PK = composite::PressedKey;
type MK = composite::Layered<composite::TapHold<layered::ModifierKey>>;
type LK = composite::LayeredKey<composite::TapHold<keyboard::Key>>;

const KEYS: Keys2<MK, LK, Ctx, Ev, PK> = tuples::Keys2::new((
    composite::Layered(composite::TapHold(layered::ModifierKey::Hold(0))),
    composite::LayeredKey::Layered(layered::LayeredKey::new(
        composite::TapHold(keyboard::Key::new(0x04)),
        [Some(composite::TapHold(keyboard::Key::new(0x05)))],
    )),
));

const CONTEXT: Ctx = composite::DEFAULT_CONTEXT;

#[test]
fn press_base_key_when_no_layers_active() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, actual_report,);
}

#[test]
fn press_active_layer_when_layer_mod_held() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..smart_keymap::keymap::INPUT_QUEUE_TICK_DELAY {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0, 0, 0x05, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, actual_report);
}

#[test]
fn press_retained_when_layer_mod_released() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..smart_keymap::keymap::INPUT_QUEUE_TICK_DELAY {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    for _ in 0..smart_keymap::keymap::INPUT_QUEUE_TICK_DELAY {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0, 0, 0x05, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, actual_report);
}

#[test]
fn uses_base_when_pressed_after_layer_mod_released() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    for _ in 0..smart_keymap::keymap::INPUT_QUEUE_TICK_DELAY {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    for _ in 0..smart_keymap::keymap::INPUT_QUEUE_TICK_DELAY {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, actual_report);
}
