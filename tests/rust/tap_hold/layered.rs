use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use smart_keymap_macros::keymap;

#[test]
fn press_active_layer_when_hold_layer_mod_held() {
    // Check TapHold { tap: Keyboard, hold: LayerModifier } works

    // Assemble
    // - In order to have { tap: Keyboard, hold: LayerMod },
    //    we need to use the aggregate composite::Key
    //    as the nested key type.
    let mut keymap = ObservedKeymap::new(keymap!(
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
    ));

    // Act
    // - Press the tap-hold key.
    // - Resolve the tap-hold as hold (Time the tap-hold key out)
    // - Press the layered key.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });

    // Tick until the tap-hold's timeout event occurs.
    keymap.tick_until_no_scheduled_events();

    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    // Assert
    // - Check the keycode from the layer is used.
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x05, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn uses_base_when_pressed_after_hold_layer_mod_released() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
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
    ));

    // 1. Press the tap-hold key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });

    // 2. Resolve the tap-hold as hold (Time the tap-hold key out)
    // Tick until the tap-hold's timeout event occurs.
    keymap.tick_until_no_scheduled_events();

    // 3. Release the tap-hold key (release layered::LayerModifier::Hold)
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    // Act
    // Press the layered key; it should be the base key.
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x04, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
