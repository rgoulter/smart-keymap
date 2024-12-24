pub const KEY_DEFINITIONS: [KeyDefinition; 4] = [
    KeyDefinition::TapHold(tap_hold::KeyDefinition {
        tap: 0x06,
        hold: 0xE0,
    }), // Tap C, Hold LCtrl
    KeyDefinition::TapHold(tap_hold::KeyDefinition {
        tap: 0x07,
        hold: 0xE1,
    }), // Tap D, Hold LShift
    KeyDefinition::Simple(simple::KeyDefinition(0x04)), // A
    KeyDefinition::Simple(simple::KeyDefinition(0x05)), // B
];
