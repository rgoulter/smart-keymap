type KeyDefinitionsType = tuples::Keys4<
    Key,
    Key,
    Key,
    Key,
>;
pub const KEY_DEFINITIONS: KeyDefinitionsType = tuples::Keys4::new((
    Key::tap_hold(tap_hold::Key {
        tap: 0x06,
        hold: 0xE0,
    }), // Tap C, Hold LCtrl
    Key::tap_hold(tap_hold::Key {
        tap: 0x07,
        hold: 0xE1,
    }), // Tap D, Hold LShift
    Key::simple(simple::Key(0x04)), // A
    Key::simple(simple::Key(0x05)), // B
));
