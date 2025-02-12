use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap;
use smart_keymap::tuples;

use keymap::Keymap;

#[test]
fn test_keymap_with_layered_key_press_active_layer_when_layer_mod_held() {
    use key::{composite, keyboard, layered};
    use tuples::Keys2;

    // Assemble
    type Ctx = composite::Context;
    type Ev = composite::Event;
    type PK = composite::PressedKey;
    type MK = composite::Layered<composite::TapHold<layered::ModifierKey>>;
    type LK = composite::LayeredKey<composite::TapHold<keyboard::Key>>;
    let keys: Keys2<MK, LK, Ctx, Ev, PK> = tuples::Keys2::new((
        composite::Layered(composite::TapHold(layered::ModifierKey::Hold(0))),
        composite::LayeredKey::Layered(layered::LayeredKey::new(
            composite::TapHold(keyboard::Key::new(0x04)),
            [Some(composite::TapHold(keyboard::Key::new(0x05)))],
        )),
    ));
    let context: Ctx = composite::DEFAULT_CONTEXT;

    let mut keymap = Keymap::new(keys, context);

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0, 0, 0x05, 0, 0, 0, 0, 0];
    assert_eq!(actual_report, expected_report);
}

#[test]
fn test_keymap_with_composite_layered_key_press_base_key() {
    use key::{composite, keyboard, layered};
    use tuples::Keys2;

    // Assemble
    type Ctx = composite::Context;
    type Ev = composite::Event;
    type PK = composite::PressedKey;
    type MK = composite::Layered<composite::TapHold<layered::ModifierKey>>;
    type LK = composite::LayeredKey<composite::TapHold<keyboard::Key>>;
    let keys: Keys2<MK, LK, Ctx, Ev, PK> = tuples::Keys2::new((
        composite::Layered(composite::TapHold(layered::ModifierKey::Hold(0))),
        composite::LayeredKey::Layered(layered::LayeredKey::new(
            composite::TapHold(keyboard::Key::new(0x04)),
            [Some(composite::TapHold(keyboard::Key::new(0x05)))],
        )),
    ));
    let context: Ctx = composite::DEFAULT_CONTEXT;
    let mut keymap = Keymap::new(keys, context);

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];
    assert_eq!(actual_report, expected_report);
}

#[test]
fn test_keymap_with_composite_layered_key_press_active_layer_when_layer_mod_held() {
    use key::{composite, keyboard, layered};
    use tuples::Keys2;

    // Assemble
    type Ctx = composite::Context;
    type Ev = composite::Event;
    type PK = composite::PressedKey;
    type MK = composite::Layered<composite::TapHold<layered::ModifierKey>>;
    type LK = composite::LayeredKey<composite::TapHold<keyboard::Key>>;
    let keys: Keys2<MK, LK, Ctx, Ev, PK> = tuples::Keys2::new((
        composite::Layered(composite::TapHold(layered::ModifierKey::Hold(0))),
        composite::LayeredKey::Layered(layered::LayeredKey::new(
            composite::TapHold(keyboard::Key::new(0x04)),
            [Some(composite::TapHold(keyboard::Key::new(0x05)))],
        )),
    ));
    let context: Ctx = composite::DEFAULT_CONTEXT;

    let mut keymap = Keymap::new(keys, context);

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0, 0, 0x05, 0, 0, 0, 0, 0];
    assert_eq!(actual_report, expected_report);
}

#[test]
fn test_keymap_with_composite_layered_key_press_retained_when_layer_mod_released() {
    use key::{composite, keyboard, layered};
    use tuples::Keys2;

    // Assemble
    type Ctx = composite::Context;
    type Ev = composite::Event;
    type PK = composite::PressedKey;
    type MK = composite::Layered<composite::TapHold<layered::ModifierKey>>;
    type LK = composite::LayeredKey<composite::TapHold<keyboard::Key>>;
    let keys: Keys2<MK, LK, Ctx, Ev, PK> = tuples::Keys2::new((
        composite::Layered(composite::TapHold(layered::ModifierKey::Hold(0))),
        composite::LayeredKey::Layered(layered::LayeredKey::new(
            composite::TapHold(keyboard::Key::new(0x04)),
            [Some(composite::TapHold(keyboard::Key::new(0x05)))],
        )),
    ));
    let context: Ctx = composite::DEFAULT_CONTEXT;
    let mut keymap = Keymap::new(keys, context);

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0, 0, 0x05, 0, 0, 0, 0, 0];
    assert_eq!(actual_report, expected_report);
}

#[test]
fn test_keymap_with_composite_layered_key_uses_base_when_pressed_after_layer_mod_released() {
    use key::{composite, keyboard, layered};
    use tuples::Keys2;

    // Assemble
    type Ctx = composite::Context;
    type Ev = composite::Event;
    type PK = composite::PressedKey;
    type MK = composite::Layered<composite::TapHold<layered::ModifierKey>>;
    type LK = composite::LayeredKey<composite::TapHold<keyboard::Key>>;
    let keys: Keys2<MK, LK, Ctx, Ev, PK> = tuples::Keys2::new((
        composite::Layered(composite::TapHold(layered::ModifierKey::Hold(0))),
        composite::LayeredKey::Layered(layered::LayeredKey::new(
            composite::TapHold(keyboard::Key::new(0x04)),
            [Some(composite::TapHold(keyboard::Key::new(0x05)))],
        )),
    ));
    let context: Ctx = composite::DEFAULT_CONTEXT;
    let mut keymap = Keymap::new(keys, context);

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];
    assert_eq!(actual_report, expected_report);
}
