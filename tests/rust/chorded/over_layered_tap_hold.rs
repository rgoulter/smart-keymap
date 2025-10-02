use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use smart_keymap_macros::keymap;

#[test]
fn tap_key_after_tapping_chord_on_default_layer() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            let CH = import "chording.ncl" in
            {
                chords = [
                    { indices = "X X _ _" |> CH.indices, key = K.E, },
                ],
                layers = [
                    [K.A & K.hold K.LeftCtrl, K.B, K.C, K.layer_mod.set_default 1],
                    [K.F & K.hold K.LeftShift, K.TTTT, K.D, K.TTTT],
                ],
            }
        "#
    ));

    // Act
    // - Default layer,
    // - Press chord (01), release chord.
    // - Press letter.
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    keymap.handle_input(input::Event::Release { keymap_index: 2 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x08, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x06, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn tap_key_after_tapping_chord_on_layer_1() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            let CH = import "chording.ncl" in
            {
                chords = [
                    { indices = "X X _ _" |> CH.indices, key = K.E, },
                ],
                layers = [
                    [K.A & K.hold K.LeftCtrl, K.B, K.C, K.layer_mod.set_default 1],
                    [K.F & K.hold K.LeftShift, K.TTTT, K.D, K.TTTT],
                ],
            }
        "#
    ));

    // Act
    // - Set default layer to 1
    // - Press chord (01), release chord.
    // - Press letter.
    keymap.handle_input(input::Event::Press { keymap_index: 3 });
    keymap.handle_input(input::Event::Press { keymap_index: 3 });
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 1 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 1 });
    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    keymap.handle_input(input::Event::Release { keymap_index: 2 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x08, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x07, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}

#[test]
fn tap_chorded_key_passes_through_as_tap() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            let CH = import "chording.ncl" in
            {
                chords = [
                    { indices = "X X _ _" |> CH.indices, key = K.E, },
                ],
                layers = [
                    [K.A & K.hold K.LeftCtrl, K.B, K.C, K.layer_mod.set_default 1],
                    [K.F & K.hold K.LeftShift, K.TTTT, K.D, K.TTTT],
                ],
            }
        "#
    ));

    // Act
    // - Default layer,
    // - Press chord (01), release chord.
    // - Press letter.
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
fn tap_key_after_tapping_chorded_key_on_layer_1() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            let CH = import "chording.ncl" in
            {
                chords = [
                    { indices = "X X _ _" |> CH.indices, key = K.E, },
                ],
                layers = [
                    [K.A & K.hold K.LeftCtrl, K.B, K.C, K.layer_mod.set_default 1],
                    [K.F & K.hold K.LeftShift, K.TTTT, K.D, K.TTTT],
                ],
            }
        "#
    ));

    // Act
    // - Set default layer to 1
    // - Press chord (01), release chord.
    // - Press letter.
    keymap.handle_input(input::Event::Press { keymap_index: 3 });
    keymap.handle_input(input::Event::Press { keymap_index: 3 });
    keymap.handle_input(input::Event::Press { keymap_index: 0 });
    keymap.handle_input(input::Event::Release { keymap_index: 0 });
    keymap.handle_input(input::Event::Press { keymap_index: 2 });
    keymap.handle_input(input::Event::Release { keymap_index: 2 });

    keymap.tick_until_no_scheduled_events();

    // Assert
    #[rustfmt::skip]
    let expected_reports: &[[u8; 8]] = &[
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x09, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
        [0, 0, 0x07, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0, 0, 0],
    ];
    let actual_reports = keymap.distinct_reports();
    assert_eq!(expected_reports, actual_reports.reports());
}
