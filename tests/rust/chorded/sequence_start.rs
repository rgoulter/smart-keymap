use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use crate::hid_keycodes::*;
use smart_keymap_macros::keymap;

/// `q;` — aux (1) then primary (0) — then the sequence step.
#[test]
fn aux_then_primary_then_sequence_step() {
    // Assemble: `;` = index 0 (primary), `Q` = index 1 (aux),
    //  sequence member = index 2 → C.
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.chorded.required_idle_time = 125,
                config.sequence.timeout = 2500,
                chords = [
                    { indices = [0, 1], key = K.sequence_start },
                ],
                sequences = [
                    { indices = [2], key = K.C },
                ],
                keys = [
                    K.A,
                    K.B,
                    K.X,
                ],
            }
        "#
    ));

    // Act: idle, aux then primary, release, then the sequence step.
    for _ in 0..150 {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });

    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    keymap.handle_input(input::Event::Release { keymap_index: 2 });
    keymap.tick_until_no_scheduled_events();

    // Assert: sequence output C, no passthrough.
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_C, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, keymap.distinct_reports().reports());
}

/// `;q` — primary (0) then aux (1) — then the sequence step.
///
/// This is the userspace failure: chord appears not to activate.
#[test]
fn primary_then_aux_then_sequence_step() {
    // Assemble: `;` = index 0 (primary), `Q` = index 1 (aux),
    //  sequence member = index 2 → C.
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.chorded.required_idle_time = 125,
                config.sequence.timeout = 2500,
                chords = [
                    { indices = [0, 1], key = K.sequence_start },
                ],
                sequences = [
                    { indices = [2], key = K.C },
                ],
                keys = [
                    K.A,
                    K.B,
                    K.X,
                ],
            }
        "#
    ));

    // Act: idle, primary then aux, release, then the sequence step.
    for _ in 0..150 {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });

    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    keymap.handle_input(input::Event::Release { keymap_index: 2 });
    keymap.tick_until_no_scheduled_events();

    // Assert: sequence output C, no passthrough.
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_C, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, keymap.distinct_reports().reports());
}

/// Same chord with a letter output (like DESK_LEFT) works both ways.
/// Isolates the failure to SequenceStart, not chord idle / order.
#[test]
fn letter_chord_both_orders_still_work() {
    // Assemble: letter chord on [0, 1], no sequence.
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                config.chorded.required_idle_time = 125,
                chords = [
                    { indices = [0, 1], key = K.C },
                ],
                keys = [
                    K.A,
                    K.B,
                ],
            }
        "#
    ));

    // Act: both press orders, with idle between.
    for _ in 0..150 {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.tick_until_no_scheduled_events();

    for _ in 0..150 {
        keymap.tick();
    }
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.tick_until_no_scheduled_events();

    // Assert: C from each order, no leftover output.
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_C, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, KC_C, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    assert_eq!(expected_reports, keymap.distinct_reports().reports());
}
