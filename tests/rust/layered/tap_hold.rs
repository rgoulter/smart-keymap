use smart_keymap::input;
use smart_keymap::keymap::ObservedKeymap;

use smart_keymap_macros::keymap;

#[test]
fn key_taps() {
    // Assemble
    let mut keymap = ObservedKeymap::new(keymap!(
        r#"
            let K = import "keys.ncl" in
            {
                layers = [
                    [K.A & K.hold K.B, K.C & K.hold K.D],
                    [K.TTTT, K.TTTT],
                ],
            }
        "#
    ));

    // Act
    // Tap (press, release) the layered key of the tap-hold key
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
