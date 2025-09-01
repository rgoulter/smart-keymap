use smart_keymap::input;
use smart_keymap::keymap;

use smart_keymap_macros::keymap;

use keymap::DistinctReports;

#[test]
fn press_active_layer_when_hold_layer_mod_held() {
    // Check TapHold { tap: Keyboard, hold: LayerModifier } works

    // Assemble
    // - In order to have { tap: Keyboard, hold: LayerMod },
    //    we need to use the aggregate composite::Key
    //    as the nested key type.
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [
                        K.F & K.hold(K.layer_mod.hold(1)),
                        K.A
                    ],
                    [
                        K.TTTT,
                        K.B
                    ],
                ],
            }
        "#
    );
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
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [
                        K.F & K.hold(K.layer_mod.hold(1)),
                        K.A
                    ],
                    [
                        K.TTTT,
                        K.B
                    ],
                ],
            }
        "#
    );
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
