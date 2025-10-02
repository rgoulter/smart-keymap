mod automation;
mod caps_word;
mod chorded;
mod consumer;
mod custom;
mod layered;
mod mouse;
mod sticky;
mod tap_dance;
mod tap_hold;

mod ms_per_tick;

use smart_keymap::keymap::ObservedKeymap;

#[test]
fn basic_keymap_expression() {
    // This test demonstrates using smart_keymap::keymap::Keymap directly.

    // Assemble
    use smart_keymap::input;

    let mut keymap = ObservedKeymap::new({
        use key_system::Context;
        use key_system::Ref;
        use smart_keymap::key::composite as key_system;
        const KEY_COUNT: usize = 1;
        const KEY_REFS: [Ref; KEY_COUNT] = [smart_keymap::key::composite::Ref::Keyboard(
            smart_keymap::key::keyboard::Ref::KeyCode(0x04),
        )];
        const CONTEXT: Context = Context::from_config(key_system::Config::new());

        smart_keymap::keymap::Keymap::new(
            KEY_REFS,
            CONTEXT,
            smart_keymap::key::composite::System::array_based(
                smart_keymap::key::automation::System::new([]),
                smart_keymap::key::callback::System::new([]),
                smart_keymap::key::chorded::System::new([], []),
                smart_keymap::key::keyboard::System::new([]),
                smart_keymap::key::layered::System::new([], []),
                smart_keymap::key::sticky::System::new([]),
                smart_keymap::key::tap_dance::System::new([]),
                smart_keymap::key::tap_hold::System::new([]),
            ),
        )
    });

    // Act -- tap 'a'
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

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
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
