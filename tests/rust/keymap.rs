use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap;
use smart_keymap::tuples;

use keymap::Keymap;

#[test]
fn test_keymap_with_tap_hold_key_with_composite_context_key_tapped() {
    use key::composite::{Context, Event, PressedKey};
    use key::{composite, keyboard, tap_hold};
    use tuples::Keys1;

    // Assemble
    type K = composite::Layered<composite::TapHoldKey<keyboard::Key>>;
    let keys: Keys1<K, Context, Event, PressedKey> = Keys1::new((composite::Layered(
        composite::TapHoldKey::TapHold(tap_hold::Key {
            tap: keyboard::Key::new(0x04),
            hold: keyboard::Key::new(0xE0),
        }),
    ),));
    let context: Context = composite::DEFAULT_CONTEXT;
    let mut keymap = Keymap::new(keys, context);

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];
    assert_eq!(actual_report, expected_report);
}

#[test]
fn test_keymap_with_tap_hold_key_with_composite_context_key_unaffected_by_prev_key_release() {
    use key::composite::{Context, Event, PressedKey};
    use key::{composite, keyboard, tap_hold};
    use tuples::Keys1;

    // When a tap-hold key is pressed,
    //  it schedules a Timeout event after 200 ticks.
    // In case of releasing, then pressing the key a second time within 200 ticks,
    //  we do not want the first Timeout to affect the second key press.

    // Assemble
    type K = composite::Layered<composite::TapHoldKey<keyboard::Key>>;
    let keys: Keys1<K, Context, Event, PressedKey> = Keys1::new((composite::Layered(
        composite::TapHoldKey::TapHold(tap_hold::Key {
            tap: keyboard::Key::new(0x04),
            hold: keyboard::Key::new(0xE0),
        }),
    ),));
    let context: Context = composite::DEFAULT_CONTEXT;
    let mut keymap = Keymap::new(keys, context);

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

#[test]
fn test_keymap_with_tap_keyboard_hold_layermod_press_active_layer_when_held() {
    // Check TapHold { tap: Keyboard, hold: LayerModifier } works

    use key::{composite, keyboard, layered, tap_hold};
    use tuples::Keys2;

    // Assemble
    // - In order to have { tap: Keyboard, hold: LayerMod },
    //    we need to use the aggregate composite::Key
    //    as the nested key type.
    type Ctx = composite::Context;
    type Ev = composite::Event;
    type PK = composite::PressedKey;

    type K0 = composite::Layered<composite::TapHoldKey<composite::BaseKey>>;
    type K1 = composite::LayeredKey<composite::TapHold<keyboard::Key>>;

    let keys: Keys2<K0, K1, Ctx, Ev, PK> = tuples::Keys2::new((
        composite::Layered(composite::TapHoldKey::TapHold(tap_hold::Key {
            tap: composite::BaseKey::Keyboard(keyboard::Key::new(0x04)),
            hold: composite::BaseKey::LayerModifier(layered::ModifierKey::Hold(0)),
        })),
        composite::LayeredKey::Layered(layered::LayeredKey::new(
            composite::TapHold(keyboard::Key::new(0x04)),
            [Some(composite::TapHold(keyboard::Key::new(0x05)))],
        )),
    ));
    let context: Ctx = composite::DEFAULT_CONTEXT;

    let mut keymap = Keymap::new(keys, context);

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
    assert_eq!(actual_report, expected_report);
}

#[test]
fn test_keymap_with_tap_keyboard_hold_layermod_uses_base_when_pressed_after_layer_mod_released() {
    use key::{composite, keyboard, layered, tap_hold};
    use tuples::Keys2;

    // Assemble
    type Ctx = composite::Context;
    type Ev = composite::Event;
    type PK = composite::PressedKey;

    type K0 = composite::Layered<composite::TapHoldKey<composite::BaseKey>>;
    type K1 = composite::LayeredKey<composite::TapHold<keyboard::Key>>;

    let keys: Keys2<K0, K1, Ctx, Ev, PK> = tuples::Keys2::new((
        composite::Layered(composite::TapHoldKey::TapHold(tap_hold::Key {
            tap: composite::BaseKey::Keyboard(keyboard::Key::new(0x04)),
            hold: composite::BaseKey::LayerModifier(layered::ModifierKey::Hold(0)),
        })),
        composite::LayeredKey::Layered(layered::LayeredKey::new(
            composite::TapHold(keyboard::Key::new(0x04)),
            [Some(composite::TapHold(keyboard::Key::new(0x05)))],
        )),
    ));
    let context: Ctx = composite::DEFAULT_CONTEXT;
    let mut keymap = Keymap::new(keys, context);

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
    assert_eq!(actual_report, expected_report);
}

#[test]
fn test_keymap_with_tap_hold_ignoring_interrupts_rolled_presses_output() {
    use key::{composite, keyboard, tap_hold};
    use tuples::Keys2;

    // Assemble
    type Ctx = composite::Context;
    type Ev = composite::Event;
    type PK = composite::PressedKey;

    type K0 = composite::Layered<composite::TapHoldKey<keyboard::Key>>;
    type K1 = composite::Layered<composite::TapHold<keyboard::Key>>;

    let keys: Keys2<K0, K1, Ctx, Ev, PK> = tuples::Keys2::new((
        composite::Layered(composite::TapHoldKey::TapHold(tap_hold::Key {
            tap: keyboard::Key::new(0x04),
            hold: keyboard::Key::new(0xE0),
        })),
        composite::Layered(composite::TapHold(keyboard::Key::new(0x05))),
    ));
    let context: Ctx = composite::DEFAULT_CONTEXT;
    let mut keymap = Keymap::new(keys, context);

    // Act
    // Roll the keys: press 0, press 1, release 0,
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    let actual_report = keymap.boot_keyboard_report();

    // Assert
    let expected_report: [u8; 8] = [0, 0, 0x04, 0x05, 0, 0, 0, 0];
    assert_eq!(actual_report, expected_report);
}

#[test]
fn test_keymap_with_tap_hold_ignoring_interrupts_rolled_presses_output_desc_keycodes() {
    use key::{composite, keyboard, tap_hold};
    use tuples::Keys2;

    // Assemble
    type Ctx = composite::Context;
    type Ev = composite::Event;
    type PK = composite::PressedKey;

    type K0 = composite::Layered<composite::TapHoldKey<keyboard::Key>>;
    type K1 = composite::Layered<composite::TapHold<keyboard::Key>>;

    const K_G: u8 = 0x0A;
    const K_O: u8 = 0x12;

    let keys: Keys2<K0, K1, Ctx, Ev, PK> = tuples::Keys2::new((
        composite::Layered::tap_hold(tap_hold::Key {
            tap: keyboard::Key::new(K_O),
            hold: keyboard::Key::new(0xE0),
        }),
        composite::Layered::keyboard(keyboard::Key::new(K_G)),
    ));
    let context: Ctx = composite::DEFAULT_CONTEXT;
    let mut keymap = Keymap::new(keys, context);

    // Roll the keys: press 0, press 1, release 0,
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    {
        keymap.tick();
        let actual_report = keymap.report_output().as_hid_boot_keyboard_report();
        let expected_report: [u8; 8] = [0, 0, K_O, 0, 0, 0, 0, 0];
        assert_eq!(actual_report, expected_report);
    }
    {
        keymap.tick();
        let actual_report = keymap.report_output().as_hid_boot_keyboard_report();
        let expected_report: [u8; 8] = [0, 0, K_O, K_G, 0, 0, 0, 0];
        assert_eq!(actual_report, expected_report);
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
        assert_eq!(actual_report, expected_report);
    }
    {
        keymap.tick();
        let actual_report = keymap.report_output().as_hid_boot_keyboard_report();
        let expected_report: [u8; 8] = [0, 0, K_O, K_G, 0, 0, 0, 0];
        assert_eq!(actual_report, expected_report);
    }
}
