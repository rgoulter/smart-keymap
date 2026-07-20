mod automation;
mod caps_word;
mod chorded;
mod consumer;
mod custom;
mod hid_keycodes;
mod layered;
mod mouse;
mod sticky;
mod tap_dance;
mod tap_hold;

mod ms_per_tick;

mod event_based;

use hid_keycodes::*;
use smart_keymap::keymap::ObservedKeymap;

#[test]
fn basic_keymap_expression_macro() {
    // This test demonstrates using smart_keymap::keymap::Keymap directly,
    //  by using the keymap! macro.

    // Assemble
    use smart_keymap::input;

    let mut keymap = ObservedKeymap::new(smart_keymap_macros::keymap!(
        r#"
        {
            keys = [
                { key_code = 4 },
            ],
        }
        "#
    ));

    // Act -- tap 'a'
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_A, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
