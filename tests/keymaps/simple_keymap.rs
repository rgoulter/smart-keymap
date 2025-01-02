type KeyDefinitionsType = tuples::Keys4<
    Key,
    Key,
    Key,
    Key,
>;
pub const KEY_DEFINITIONS: KeyDefinitionsType = tuples::Keys4::new((
    Key::TapHold(tap_hold::Key {
        tap: 0x06,
        hold: 0xE0,
    }), // Tap C, Hold LCtrl
    Key::TapHold(tap_hold::Key {
        tap: 0x07,
        hold: 0xE1,
    }), // Tap D, Hold LShift
    Key::Simple(simple::Key(0x04)), // A
    Key::Simple(simple::Key(0x05)), // B
));
