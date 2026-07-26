use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

#[test]
fn sequence_start_then_two_steps_emits_bound_key() {
    // Assemble: SEQ_START, A, B — sequence [1, 2] → C
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                sequences = [
                    { indices = [1, 2], key = K.C },
                ],
                config.sequence.timeout = 500,
                keys = [
                    K.sequence_start,
                    K.A,
                    K.B,
                ],
            }
        "#
    ));

    // Act: start, then A, then B
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    keymap.handle_input(input::Event::Release { keymap_index: 2 });

    keymap.tick_until_no_scheduled_events();

    // Assert: bound C, no A/B passthrough
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_C, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn sequence_member_passthrough_when_mode_inactive() {
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                sequences = [
                    { indices = [1, 2], key = K.C },
                ],
                keys = [
                    K.sequence_start,
                    K.A,
                    K.B,
                ],
            }
        "#
    ));

    // Tap A without arming
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.tick_until_no_scheduled_events();

    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_A, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn sequence_unknown_key_aborts_without_sequence_output() {
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                sequences = [
                    { indices = [1, 2], key = K.C },
                ],
                keys = [
                    K.sequence_start,
                    K.A,
                    K.B,
                    K.X,
                ],
            }
        "#
    ));

    // Arm, press A (valid first step), then X (unknown) — abort, no C
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.handle_input(input::Event::Press { keymap_index: 3 });
    keymap.handle_input(input::Event::Release { keymap_index: 3 });
    keymap.tick_until_no_scheduled_events();

    let reports = keymap.distinct_reports().reports();
    // No C from sequence
    assert!(
        !reports.iter().any(|r| r[2] == KC_C),
        "unexpected sequence output: {:?}",
        reports
    );
}

#[test]
fn sequence_timeout_aborts_incomplete() {
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                sequences = [
                    { indices = [1, 2], key = K.C },
                ],
                config.sequence.timeout = 50,
                keys = [
                    K.sequence_start,
                    K.A,
                    K.B,
                ],
            }
        "#
    ));

    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    // Wait out timeout without completing
    for _ in 0..100 {
        keymap.tick();
    }
    keymap.tick_until_no_scheduled_events();

    let reports = keymap.distinct_reports().reports();
    assert!(
        !reports.iter().any(|r| r[2] == KC_C),
        "timeout should not emit C: {:?}",
        reports
    );
}
