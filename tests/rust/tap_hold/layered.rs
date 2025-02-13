use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap;
use smart_keymap::tuples;

use keymap::Keymap;

use key::{composite, keyboard, layered, tap_hold};
use tuples::Keys2;

type Ctx = composite::Context;
type Ev = composite::Event;
type PK = composite::PressedKey;

type K0 = composite::Layered<composite::TapHoldKey<composite::BaseKey>>;
type K1 = composite::LayeredKey<composite::TapHold<keyboard::Key>>;

const KEYS: Keys2<K0, K1, Ctx, Ev, PK> = tuples::Keys2::new((
    composite::Layered(composite::TapHoldKey::TapHold(tap_hold::Key {
        tap: composite::BaseKey::Keyboard(keyboard::Key::new(0x04)),
        hold: composite::BaseKey::LayerModifier(layered::ModifierKey::Hold(0)),
    })),
    composite::LayeredKey::Layered(layered::LayeredKey::new(
        composite::TapHold(keyboard::Key::new(0x04)),
        [Some(composite::TapHold(keyboard::Key::new(0x05)))],
    )),
));
const CONTEXT: Ctx = composite::DEFAULT_CONTEXT;

#[test]
fn press_active_layer_when_hold_layer_mod_held() {
    // Check TapHold { tap: Keyboard, hold: LayerModifier } works

    // Assemble
    // - In order to have { tap: Keyboard, hold: LayerMod },
    //    we need to use the aggregate composite::Key
    //    as the nested key type.
    let mut keymap = Keymap::new(KEYS, CONTEXT);

    // Act
    // - Press the tap-hold key.
    // - Resolve the tap-hold as hold (Time the tap-hold key out)
    // - Press the layered key.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    // Tick until the tap-hold's timeout event occurs.
    for _ in 0..200 {
        keymap.tick();
        let _ = keymap.pressed_keys();
    }

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    // - Check the keycode from the layer is used.
    let expected_report: [u8; 8] = [0, 0, 0x05, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, actual_report);
}

#[test]
fn uses_base_when_pressed_after_hold_layer_mod_released() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);

    // 1. Press the tap-hold key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    // 2. Resolve the tap-hold as hold (Time the tap-hold key out)
    // Tick until the tap-hold's timeout event occurs.
    for _ in 0..200 {
        keymap.tick();
        let _ = keymap.pressed_keys();
    }
    keymap.tick();
    // 3. Release the tap-hold key (release layered::LayerModifier::Hold)
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    // Act
    // Press the layered key; it should be the base key.
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, actual_report);
}
