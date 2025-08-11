use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap;
use smart_keymap::tuples;

use keymap::DistinctReports;
use keymap::Keymap;

use key::{composite, keyboard, layered};
use tuples::Keys4;

type Ctx = composite::Context;
type Ev = composite::Event;
type PKS = composite::PendingKeyState;
type KS = composite::KeyState;
type MK = composite::Chorded<composite::Layered<composite::TapHold<layered::ModifierKey>>>;
type LK = composite::Chorded<composite::LayeredKey<composite::TapHold<keyboard::Key>>>;

const KEYS: Keys4<MK, MK, LK, LK, Ctx, Ev, PKS, KS> = tuples::Keys4::new((
    composite::Chorded(composite::Layered(composite::TapHold(
        layered::ModifierKey::set_active_layers(&[1]),
    ))),
    composite::Chorded(composite::Layered(composite::TapHold(
        layered::ModifierKey::set_active_layers(&[0]),
    ))),
    composite::Chorded(composite::LayeredKey::Layered(layered::LayeredKey::new(
        composite::TapHold(keyboard::Key::new(0x04)),
        [Some(composite::TapHold(keyboard::Key::new(0x05)))],
    ))),
    composite::Chorded(composite::LayeredKey::Layered(layered::LayeredKey::new(
        composite::TapHold(keyboard::Key::new(0x07)),
        [Some(composite::TapHold(keyboard::Key::new(0x06)))],
    ))),
));

const CONTEXT: Ctx = composite::DEFAULT_CONTEXT;

#[test]
fn tap_set_active_layers_activates_layers() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    #[rustfmt::skip]
    let tap_indices = &[
        2, // tap the third key, (should be default layer)
        0, // set layers to [1]
        2, // tap the two layered keys
        3,
        1, // set layers to [0]
        3, // tap the second layered keys again
    ];

    for &keymap_index in tap_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

        keymap.handle_input(input::Event::Release { keymap_index });
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x05, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x06, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x07, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn press_set_active_layers_activates_layers() {
    // Assemble
    type K = composite::Chorded<composite::LayeredKey<composite::TapHold<composite::BaseKey>>>;
    let keys: tuples::Keys2<K, K, Ctx, Ev, PKS, KS> = tuples::Keys2::new((
        composite::Chorded(composite::LayeredKey::Layered(layered::LayeredKey::new(
            composite::TapHold(composite::BaseKey::LayerModifier(
                layered::ModifierKey::set_active_layers(&[1]),
            )),
            [Some(composite::TapHold(composite::BaseKey::LayerModifier(
                layered::ModifierKey::set_active_layers(&[0]),
            )))],
        ))),
        composite::Chorded(composite::LayeredKey::Layered(layered::LayeredKey::new(
            composite::TapHold(composite::BaseKey::Keyboard(keyboard::Key::new(0x04))),
            [Some(composite::TapHold(composite::BaseKey::Keyboard(
                keyboard::Key::new(0x05),
            )))],
        ))),
    ));
    let mut keymap = Keymap::new(keys, CONTEXT);
    let mut actual_reports = DistinctReports::new();

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.tick();
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.tick();
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x05, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}
