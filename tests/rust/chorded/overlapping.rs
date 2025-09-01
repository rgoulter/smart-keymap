use smart_keymap::input;
use smart_keymap::keymap;

use smart_keymap_macros::keymap;

use keymap::DistinctReports;

#[test]
fn overlap_tap_key_acts_as_passthrough() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            let CH = import "chording.ncl" in
            {
                chords = [
                    { indices = "X X _ _" |> CH.indices, key = K.M, },
                    { indices = "X X X _" |> CH.indices, key = K.N, },
                    { indices = "_ X X _" |> CH.indices, key = K.O, },
                    { indices = "_ _ X X" |> CH.indices, key = K.P, },
                ],
                keys = [
                    K.A, K.B, K.C, K.D,
                ],
            }
        "#
    );
    let mut actual_reports = DistinctReports::new();

    // Act
    let tap_indices = &[1];

    for &keymap_index in tap_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
        keymap.handle_input(input::Event::Release { keymap_index });
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x05, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn overlap_press_d_bc_results_in_passthrough_followed_by_chord() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            let CH = import "chording.ncl" in
            {
                chords = [
                    { indices = "X X _ _" |> CH.indices, key = K.M, },
                    { indices = "X X X _" |> CH.indices, key = K.N, },
                    { indices = "_ X X _" |> CH.indices, key = K.O, },
                    { indices = "_ _ X X" |> CH.indices, key = K.P, },
                ],
                keys = [
                    K.A, K.B, K.C, K.D,
                ],
            }
        "#
    );
    let mut actual_reports = DistinctReports::new();

    // Act
    // Press D, then B and C.
    let press_indices = &[3, 1, 2];

    for &keymap_index in press_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x07, 0, 0, 0, 0, 0],
        [0, 0, 0x07, 0x12, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn overlap_partial_press_cd_acts_as_chord() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            let CH = import "chording.ncl" in
            {
                chords = [
                    { indices = "X X _ _" |> CH.indices, key = K.M, },
                    { indices = "X X X _" |> CH.indices, key = K.N, },
                    { indices = "_ X X _" |> CH.indices, key = K.O, },
                    { indices = "_ _ X X" |> CH.indices, key = K.P, },
                ],
                keys = [
                    K.A, K.B, K.C, K.D,
                ],
            }
        "#
    );
    let mut actual_reports = DistinctReports::new();

    // Act
    let press_indices = &[2, 3];

    for &keymap_index in press_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    let expected_reports: &[[u8; 8]] = &[[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0x13, 0, 0, 0, 0, 0]];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn overlap_partial_press_dc_acts_as_chord() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            let CH = import "chording.ncl" in
            {
                chords = [
                    { indices = "X X _ _" |> CH.indices, key = K.M, },
                    { indices = "X X X _" |> CH.indices, key = K.N, },
                    { indices = "_ X X _" |> CH.indices, key = K.O, },
                    { indices = "_ _ X X" |> CH.indices, key = K.P, },
                ],
                keys = [
                    K.A, K.B, K.C, K.D,
                ],
            }
        "#
    );
    let mut actual_reports = DistinctReports::new();

    // Act
    let press_indices = &[3, 2];

    for &keymap_index in press_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    let expected_reports: &[[u8; 8]] = &[[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0x13, 0, 0, 0, 0, 0]];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn overlap_press_bc_acts_as_chord() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            let CH = import "chording.ncl" in
            {
                chords = [
                    { indices = "X X _ _" |> CH.indices, key = K.M, },
                    { indices = "X X X _" |> CH.indices, key = K.N, },
                    { indices = "_ X X _" |> CH.indices, key = K.O, },
                    { indices = "_ _ X X" |> CH.indices, key = K.P, },
                ],
                keys = [
                    K.A, K.B, K.C, K.D,
                ],
            }
        "#
    );
    let mut actual_reports = DistinctReports::new();

    // Act
    let press_indices = &[1, 2];

    for &keymap_index in press_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    let expected_reports: &[[u8; 8]] = &[[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0x12, 0, 0, 0, 0, 0]];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn overlap_press_cb_acts_as_chord() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            let CH = import "chording.ncl" in
            {
                chords = [
                    { indices = "X X _ _" |> CH.indices, key = K.M, },
                    { indices = "X X X _" |> CH.indices, key = K.N, },
                    { indices = "_ X X _" |> CH.indices, key = K.O, },
                    { indices = "_ _ X X" |> CH.indices, key = K.P, },
                ],
                keys = [
                    K.A, K.B, K.C, K.D,
                ],
            }
        "#
    );
    let mut actual_reports = DistinctReports::new();

    // Act
    let press_indices = &[2, 1];

    for &keymap_index in press_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    let expected_reports: &[[u8; 8]] = &[[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0x12, 0, 0, 0, 0, 0]];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn overlap_press_ab_acts_as_chord() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            let CH = import "chording.ncl" in
            {
                chords = [
                    { indices = "X X _ _" |> CH.indices, key = K.M, },
                    { indices = "X X X _" |> CH.indices, key = K.N, },
                    { indices = "_ X X _" |> CH.indices, key = K.O, },
                    { indices = "_ _ X X" |> CH.indices, key = K.P, },
                ],
                keys = [
                    K.A, K.B, K.C, K.D,
                ],
            }
        "#
    );
    let mut actual_reports = DistinctReports::new();

    // Act
    let press_indices = &[0, 1];

    for &keymap_index in press_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    let expected_reports: &[[u8; 8]] = &[[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0x10, 0, 0, 0, 0, 0]];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn overlap_press_ba_acts_as_chord() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            let CH = import "chording.ncl" in
            {
                chords = [
                    { indices = "X X _ _" |> CH.indices, key = K.M, },
                    { indices = "X X X _" |> CH.indices, key = K.N, },
                    { indices = "_ X X _" |> CH.indices, key = K.O, },
                    { indices = "_ _ X X" |> CH.indices, key = K.P, },
                ],
                keys = [
                    K.A, K.B, K.C, K.D,
                ],
            }
        "#
    );
    let mut actual_reports = DistinctReports::new();

    // Act
    let press_indices = &[1, 0];

    for &keymap_index in press_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    let expected_reports: &[[u8; 8]] = &[[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0x10, 0, 0, 0, 0, 0]];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn overlap_press_abc_acts_as_chord() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            let CH = import "chording.ncl" in
            {
                chords = [
                    { indices = "X X _ _" |> CH.indices, key = K.M, },
                    { indices = "X X X _" |> CH.indices, key = K.N, },
                    { indices = "_ X X _" |> CH.indices, key = K.O, },
                    { indices = "_ _ X X" |> CH.indices, key = K.P, },
                ],
                keys = [
                    K.A, K.B, K.C, K.D,
                ],
            }
        "#
    );
    let mut actual_reports = DistinctReports::new();

    // Act
    let press_indices = &[0, 1, 2];

    for &keymap_index in press_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    let expected_reports: &[[u8; 8]] = &[[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0x11, 0, 0, 0, 0, 0]];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn overlap_press_cba_acts_as_chord() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            let CH = import "chording.ncl" in
            {
                chords = [
                    { indices = "X X _ _" |> CH.indices, key = K.M, },
                    { indices = "X X X _" |> CH.indices, key = K.N, },
                    { indices = "_ X X _" |> CH.indices, key = K.O, },
                    { indices = "_ _ X X" |> CH.indices, key = K.P, },
                ],
                keys = [
                    K.A, K.B, K.C, K.D,
                ],
            }
        "#
    );
    let mut actual_reports = DistinctReports::new();

    // Act
    let press_indices = &[2, 1, 0];

    for &keymap_index in press_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    let expected_reports: &[[u8; 8]] = &[[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0x11, 0, 0, 0, 0, 0]];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn overlap_press_cab_acts_as_chord() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            let CH = import "chording.ncl" in
            {
                chords = [
                    { indices = "X X _ _" |> CH.indices, key = K.M, },
                    { indices = "X X X _" |> CH.indices, key = K.N, },
                    { indices = "_ X X _" |> CH.indices, key = K.O, },
                    { indices = "_ _ X X" |> CH.indices, key = K.P, },
                ],
                keys = [
                    K.A, K.B, K.C, K.D,
                ],
            }
        "#
    );
    let mut actual_reports = DistinctReports::new();

    // Act
    let press_indices = &[2, 0, 1];

    for &keymap_index in press_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    let expected_reports: &[[u8; 8]] = &[[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0x11, 0, 0, 0, 0, 0]];
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn interrupting_satisfied_overlapped_chord_resolves_as_chord() {
    // Assemble
    let mut keymap = keymap!(
        r#"
            let K = import "keys.ncl" in
            let CH = import "chording.ncl" in
            {
                chords = [
                    { indices = "X X _ _" |> CH.indices, key = K.M, },
                    { indices = "X X X _" |> CH.indices, key = K.N, },
                    { indices = "_ X X _" |> CH.indices, key = K.O, },
                    { indices = "_ _ X X" |> CH.indices, key = K.P, },
                ],
                keys = [
                    K.A, K.B, K.C, K.D,
                ],
            }
        "#
    );
    let mut actual_reports = DistinctReports::new();

    // Act
    // Press BC then D.
    let press_indices = &[1, 2, 3];

    for &keymap_index in press_indices {
        keymap.handle_input(input::Event::Press { keymap_index });
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    while keymap.has_scheduled_events() {
        keymap.tick();
        actual_reports.update(keymap.report_output().as_hid_boot_keyboard_report());
    }

    // Assert
    // Should chord BC then press D.
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x12, 0, 0, 0, 0, 0],
        [0, 0, 0x12, 0x07, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, actual_reports.reports());
}
