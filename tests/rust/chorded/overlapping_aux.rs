use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use smart_keymap_macros::keymap;

#[test]
fn overlap_aux_press_ad_results_chord() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            let CH = import "chording.ncl" in
            {
                chords = [
                    { indices = "X _ _ X" |> CH.indices, key = K.M, },
                    { indices = "X X _ X" |> CH.indices, key = K.N, },
                    { indices = "_ _ X X" |> CH.indices, key = K.O, },
                ],
                keys = [
                    K.A, K.B, K.C, K.D,
                ],
            }
        "#
    ));

    // Act
    // Press AD
    let press_indices = &[0, 3];

    for &keymap_index in press_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
    }

    keymap.tick_until_no_scheduled_events();

    // Assert
    let expected_reports: &[[u8; 8]] = &[[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0x10, 0, 0, 0, 0, 0]];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn overlap_aux_press_cd_results_chord() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            let CH = import "chording.ncl" in
            {
                chords = [
                    { indices = "X _ _ X" |> CH.indices, key = K.M, },
                    { indices = "X X _ X" |> CH.indices, key = K.N, },
                    { indices = "_ _ X X" |> CH.indices, key = K.O, },
                ],
                keys = [
                    K.A, K.B, K.C, K.D,
                ],
            }
        "#
    ));

    // Act
    // Press CD
    let press_indices = &[2, 3];

    for &keymap_index in press_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
    }

    keymap.tick_until_no_scheduled_events();

    // Assert
    let expected_reports: &[[u8; 8]] = &[[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0x12, 0, 0, 0, 0, 0]];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn overlap_aux_press_abd_results_chord() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            let CH = import "chording.ncl" in
            {
                chords = [
                    { indices = "X _ _ X" |> CH.indices, key = K.M, },
                    { indices = "X X _ X" |> CH.indices, key = K.N, },
                    { indices = "_ _ X X" |> CH.indices, key = K.O, },
                ],
                keys = [
                    K.A, K.B, K.C, K.D,
                ],
            }
        "#
    ));

    // Act
    // Press AD
    let press_indices = &[0, 1, 3];

    for &keymap_index in press_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
    }

    keymap.tick_until_no_scheduled_events();

    // Assert
    let expected_reports: &[[u8; 8]] = &[[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0x11, 0, 0, 0, 0, 0]];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
