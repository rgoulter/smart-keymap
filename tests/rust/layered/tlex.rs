use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

#[test]
fn tlex_acts_as_base_and_deactivates_toggle_layer() {
    // Assemble -- toggle layer 1, tlex over base A
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.toggle 1, K.A],
                    [K.TTTT, K.tlex],
                ],
            }
        "#
    ));

    // Act -- enable layer 1, press tlex key
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    // Assert -- this press is base A, and layer is off afterward
    let expected_report: [u8; 8] = [0, 0, KC_A, 0, 0, 0, 0, 0];
    assert_eq!(expected_report, keymap.boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    // Second press of the same key still base (layer exited)
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    assert_eq!(expected_report, keymap.boot_keyboard_report());
}

#[test]
fn tlex_differs_from_full_transparency() {
    // Assemble -- without tlex, layer stays on after a transparent-adjacent key
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.toggle 1, K.A, K.C],
                    [K.TTTT, K.tlex, K.B],
                ],
            }
        "#
    ));

    // Act -- enable layer 1
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    // Layer-1 key B works
    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    let expected_b: [u8; 8] = [0, 0, KC_B, 0, 0, 0, 0, 0];
    assert_eq!(expected_b, keymap.boot_keyboard_report());
    keymap.handle_input(input::Event::Release { keymap_index: 2 });

    // tlex on index 1 → base A, deactivates layer 1
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    let expected_a: [u8; 8] = [0, 0, KC_A, 0, 0, 0, 0, 0];
    assert_eq!(expected_a, keymap.boot_keyboard_report());
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    // Assert -- same physical key that was B is now base C
    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    let expected_c: [u8; 8] = [0, 0, KC_C, 0, 0, 0, 0, 0];
    assert_eq!(expected_c, keymap.boot_keyboard_report());
}

#[test]
fn exit_on_transparent_maps_null_cells_to_tlex() {
    // Assemble -- layer row piped through K.exit_on_transparent
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.toggle 1, K.A, K.C],
                    [K.TTTT, K.TTTT, K.B] |> K.exit_on_transparent,
                ],
            }
        "#
    ));

    // Act -- enable layer 1, press a hole that was only TTTT
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    // Assert -- base A and layer exits
    let expected_a: [u8; 8] = [0, 0, KC_A, 0, 0, 0, 0, 0];
    assert_eq!(expected_a, keymap.boot_keyboard_report());
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    // Layer-1 key B would have been on layer; after hole-exit, base C
    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    let expected_c: [u8; 8] = [0, 0, KC_C, 0, 0, 0, 0, 0];
    assert_eq!(expected_c, keymap.boot_keyboard_report());
}

#[test]
fn hold_trans_projects_hold_from_below() {
    // Assemble -- layer 1: J on tap, hold.trans → Shift from base
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.hold 1, K.A & K.hold K.LeftShift],
                    [K.TTTT, K.J & K.hold K.trans],
                ],
            }
        "#
    ));

    // Act -- hold layer, hold the TH long enough for hold
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.tick_until_no_scheduled_events();

    // Assert -- Shift held (layer still on; hold.trans does not exit)
    let expected: [u8; 8] = [0x02, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(expected, keymap.boot_keyboard_report());
}

#[test]
fn hold_tlex_exits_layer_and_registers_hold_from_below() {
    // Assemble -- FAK-shaped hold.tlex: tap Z, hold exits + Shift
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.toggle 1, K.A & K.hold K.LeftShift],
                    [K.TTTT, K.Z & K.hold K.tlex],
                ],
            }
        "#
    ));

    // Act -- toggle layer 1, hold the TH to hold
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.tick_until_no_scheduled_events();

    // Assert -- Shift held after layer exit
    let expected_shift: [u8; 8] = [0x02, 0, 0, 0, 0, 0, 0, 0];
    assert_eq!(expected_shift, keymap.boot_keyboard_report());

    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    // After exit, tap of base TH is A
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.tick_until_no_scheduled_events();
    // Tap already released — check distinct reports include A
    let reports = keymap.distinct_reports();
    assert!(
        reports.reports().iter().any(|r| r[2] == KC_A),
        "expected a tap of A after layer exit, reports={:?}",
        reports.reports()
    );
}

#[test]
fn tlex_under_hold_layer_still_active_resolves_base_for_this_press() {
    // Assemble -- hold-layer still held; tlex continue-evals for this press
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.hold 1, K.A],
                    [K.TTTT, K.tlex],
                ],
            }
        "#
    ));

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    // Assert
    let expected_a: [u8; 8] = [0, 0, KC_A, 0, 0, 0, 0, 0];
    assert_eq!(expected_a, keymap.boot_keyboard_report());
}
