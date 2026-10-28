use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

#[test]
fn tap_set_active_layers_activates_layers() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.set_active_layers_to [1], K.layer_mod.set_active_layers_to [0], K.A, K.D],
                    [K.TTTT, K.TTTT, K.B, K.C],
                ],
            }
        "#
    ));

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
        keymap.handle_input(input::Event::Release { keymap_index });
    }

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_A, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_B, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_C, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_D, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn press_set_active_layers_activates_layers() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.layer_mod.set_active_layers_to [1], K.A],
                    [K.layer_mod.set_active_layers_to [0], K.B],
                ],
            }
        "#
    ));

    // Act
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_B, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn masked_set_active_layers_preserves_layers_outside_mask() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [
                        K.layer_mod.set_active_layers_to [1],
                        K.layer_mod.set_active_layers_amongst {
                            set_active_layers_to = [3],
                            affected_layers = [3, 4],
                        },
                        K.A,
                        K.G,
                        K.D,
                    ],
                    [K.TTTT, K.TTTT, K.TTTT, K.B, K.E],
                    [K.TTTT, K.TTTT, K.TTTT, K.TTTT, K.TTTT],
                    [K.TTTT, K.TTTT, K.TTTT, K.C, K.TTTT],
                    [K.TTTT, K.TTTT, K.TTTT, K.F, K.TTTT],
                ],
            }
        "#
    ));

    // Act
    #[rustfmt::skip]
    let tap_indices = &[
        0, // set active layers to [1]
        4, // key only defined on layer 1
        1, // masked set active layers to [3]
        3, // key defined on layer 3
        4, // key only defined on layer 1 (base is K.D)
    ];

    for &keymap_index in tap_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
        keymap.handle_input(input::Event::Release { keymap_index });
    }

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_E, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_C, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_E, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn masked_set_active_layers_switches_within_mask() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [
                        K.layer_mod.set_active_layers_amongst {
                            set_active_layers_to = [3],
                            affected_layers = [3, 4],
                        },
                        K.layer_mod.set_active_layers_amongst {
                            set_active_layers_to = [4],
                            affected_layers = [3, 4],
                        },
                        K.A,
                    ],
                    [K.TTTT, K.TTTT, K.TTTT],
                    [K.TTTT, K.TTTT, K.TTTT],
                    [K.TTTT, K.TTTT, K.C],
                    [K.TTTT, K.TTTT, K.F],
                ],
            }
        "#
    ));

    // Act
    #[rustfmt::skip]
    let tap_indices = &[
        0, // masked set active layers to [3]
        2,
        1, // masked set active layers to [4]
        2,
    ];

    for &keymap_index in tap_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
        keymap.handle_input(input::Event::Release { keymap_index });
    }

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_C, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_F, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn unmasked_set_active_layers_clears_layers_outside_previous_mask() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [
                        K.layer_mod.set_active_layers_to [1],
                        K.layer_mod.set_active_layers_amongst {
                            set_active_layers_to = [3],
                            affected_layers = [3, 4],
                        },
                        K.A,
                    ],
                    [K.TTTT, K.TTTT, K.B],
                    [K.TTTT, K.TTTT, K.TTTT],
                    [K.TTTT, K.TTTT, K.C],
                    [K.TTTT, K.TTTT, K.TTTT],
                ],
            }
        "#
    ));

    // Act
    #[rustfmt::skip]
    let tap_indices = &[
        1, // masked set active layers to [3]
        2, // key on layer 3
        0, // unmasked set active layers to [1]
        2, // key on layer 1
    ];

    for &keymap_index in tap_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
        keymap.handle_input(input::Event::Release { keymap_index });
    }

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_C, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_B, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
