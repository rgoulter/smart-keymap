mod caps_word;
mod chorded;
mod custom;
mod layered;
mod sticky;
mod tap_dance;
mod tap_hold;

mod ms_per_tick;

#[test]
fn basic_keymap_expression() {
    // This test demonstrates using smart_keymap::keymap::Keymap directly.

    // Assemble
    use smart_keymap::input;
    use smart_keymap::keymap;

    use keymap::DistinctReports;

    let mut keymap = {
        use smart_keymap::key::composite::Context;
        use smart_keymap::key::composite::Event;
        use smart_keymap::key::composite::KeyState;
        use smart_keymap::key::composite::PendingKeyState;
        smart_keymap::tuples::define_keys!(1);
        type KeyDefinitionsType = Keys1<
            smart_keymap::key::composite::Chorded<
                smart_keymap::key::composite::Layered<
                    smart_keymap::key::composite::TapHold<smart_keymap::key::keyboard::Key>,
                >,
            >,
            Context,
            Event,
            PendingKeyState,
            KeyState,
        >;
        type Keymap = smart_keymap::keymap::Keymap<
            Context,
            Event,
            PendingKeyState,
            KeyState,
            KeyDefinitionsType,
        >;
        const KEY_DEFINITIONS: KeyDefinitionsType = Keys1::new((
            smart_keymap::key::composite::Chorded(smart_keymap::key::composite::Layered(
                smart_keymap::key::composite::TapHold(smart_keymap::key::keyboard::Key::new(4)),
            )),
        ));
        const CONTEXT: Context = smart_keymap::key::composite::Context::from_config(
            smart_keymap::key::composite::Config {
                chorded: smart_keymap::key::chorded::DEFAULT_CONFIG,
                sticky: smart_keymap::key::sticky::DEFAULT_CONFIG,
                tap_dance: smart_keymap::key::tap_dance::DEFAULT_CONFIG,
                tap_hold: smart_keymap::key::tap_hold::DEFAULT_CONFIG,
                ..smart_keymap::key::composite::DEFAULT_CONFIG
            },
        );

        Keymap::new(KEY_DEFINITIONS, CONTEXT)
    };
    let mut actual_reports = DistinctReports::new();

    // Act -- tap 'a'
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert -- tap-hold key immediately resolves as 'tap'
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}
