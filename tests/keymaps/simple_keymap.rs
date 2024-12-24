pub const KEY_DEFINITIONS: [Key; 4] = [
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
];
