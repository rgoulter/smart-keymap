use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap;
use smart_keymap::tuples;

use keymap::DistinctReports;
use keymap::Keymap;

use key::{composite, keyboard, layered, tap_hold};
use tuples::Keys2;

type Ctx = composite::Context;
type Ev = composite::Event;
type PKS = composite::PendingKeyState;
type KS = composite::KeyState;

type K0 = composite::Chorded<composite::Layered<composite::TapHoldKey<composite::BaseKey>>>;
type K1 = composite::Chorded<composite::LayeredKey<composite::TapHold<keyboard::Key>>>;

const KEYS: Keys2<K0, K1, Ctx, Ev, PKS, KS> = tuples::Keys2::new((
    composite::Chorded(composite::Layered(composite::TapHoldKey::TapHold(
        tap_hold::Key {
            tap: composite::BaseKey::Keyboard(keyboard::Key::new(0x09)),
            hold: composite::BaseKey::LayerModifier(layered::ModifierKey::Hold(0)),
        },
    ))),
    composite::Chorded(composite::LayeredKey::Layered(layered::LayeredKey::new([
        Some(composite::TapHold(keyboard::Key::new(0x04))),
        Some(composite::TapHold(keyboard::Key::new(0x05))),
    ]))),
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
    let mut actual_reports = DistinctReports::new();

    // Act
    // - Press the tap-hold key.
    // - Resolve the tap-hold as hold (Time the tap-hold key out)
    // - Press the layered key.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    // Tick until the tap-hold's timeout event occurs.
    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    // Assert
    // - Check the keycode from the layer is used.
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x05, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn uses_base_when_pressed_after_hold_layer_mod_released() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // 1. Press the tap-hold key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    // 2. Resolve the tap-hold as hold (Time the tap-hold key out)
    // Tick until the tap-hold's timeout event occurs.
    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // 3. Release the tap-hold key (release layered::LayerModifier::Hold)
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    for _ in 0..smart_keymap::keymap::INPUT_QUEUE_TICK_DELAY {
        keymap.tick();
    }

    // Act
    // Press the layered key; it should be the base key.
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}
