use smart_keymap::input;
use smart_keymap::key;
use smart_keymap::keymap;
use smart_keymap::tuples;

use keymap::Keymap;

use key::composite::{Context, Event, KeyState, PendingKeyState};
use key::{composite, custom};
use tuples::Keys1;

type K = composite::Chorded<composite::Layered<composite::TapHold<custom::Key>>>;
const KEYS: Keys1<K, Context, Event, PendingKeyState, KeyState> = Keys1::new((composite::Chorded(
    composite::Layered(composite::TapHold(custom::Key::new(255))),
),));
const CONTEXT: Context = composite::Context::from_config(composite::Config {
    ..composite::DEFAULT_CONFIG
});

#[test]
fn key_press_once_and_hold_resolves_as_first_definition() {
    // Assemble
    let mut keymap = Keymap::new(KEYS, CONTEXT);

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });

    // Assert
    let expected_custom_codes: &[u8] = &[255];
    let actual_custom_codes = keymap.report_output().pressed_custom_codes();
    assert_eq!(expected_custom_codes, actual_custom_codes);
}
