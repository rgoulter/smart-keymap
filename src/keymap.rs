use crate::input;

#[derive(Debug, Clone, Copy)]
pub enum KeyDefinition {
    Simple(u8),
    TapHold { tap: u8, hold: u8 },
}

#[derive(Debug, Clone, Copy)]
pub enum TapHoldState {
    Pending,
    Tap,
    Hold,
}

#[derive(Debug, Clone, Copy)]
pub enum PressedKeyStateKeyData {
    Simple,
    TapHold(TapHoldState),
}

#[derive(Debug, Clone, Copy)]
pub struct PressedKeyState {
    pub keymap_index: u16,
    pub key_data: PressedKeyStateKeyData,
}

impl PressedKeyState {
    pub fn key_code<const N: usize>(self: &Self, key_definitions: [KeyDefinition; N]) -> Option<u8> {
        let key_definition = key_definitions[self.keymap_index as usize];
        match key_definition {
            KeyDefinition::Simple(key_code) => Some(key_code),
            KeyDefinition::TapHold { tap, hold } => {
                match self.key_data {
                    PressedKeyStateKeyData::TapHold(TapHoldState::Tap) => Some(tap),
                    PressedKeyStateKeyData::TapHold(TapHoldState::Hold) => Some(hold),
                    _ => None,
                }
            }
            _ => None,
        }
    }
}

pub const KEY_DEFINITIONS: [KeyDefinition; 4] = [
    KeyDefinition::TapHold { tap: 0x06, hold: 0xE0 }, // Tap C, Hold LCtrl
    KeyDefinition::TapHold { tap: 0x07, hold: 0xE1 }, // Tap D, Hold LShift
    KeyDefinition::Simple(0x04),  // A
    KeyDefinition::Simple(0x05),  // B
];

/// The engine (set of key definition systems),
///  and key definitions.
pub struct Keymap<const N: usize> {
    key_definitions: [KeyDefinition; N],
    pressed_keys: heapless::Vec<PressedKeyState, N>,
}

impl<const N: usize> Keymap<N> {
    pub const fn new(key_definitions: [KeyDefinition; N]) -> Self {
        Self {
            key_definitions,
            pressed_keys: heapless::Vec::new(),
        }
    }

    pub fn init(self: &mut Self) {
        self.pressed_keys.clear();
    }

    pub fn handle_input(self: &mut Self, ev: input::Event) {
        match ev {
            input::Event::Press(keymap_index) => {
                // TapHold: any interruption resolves pending TapHold as Hold.
                self.pressed_keys.iter_mut()
                                 .filter(|pk| {
                                     match pk.key_data {
                                         PressedKeyStateKeyData::TapHold(TapHoldState::Pending) => true,
                                         _ => false,
                                     }
                                 })
                                 .for_each(|pk| {
                                     pk.key_data = PressedKeyStateKeyData::TapHold(TapHoldState::Hold);
                                 });

                let key_definition = self.key_definitions[keymap_index as usize];
                match key_definition {
                    KeyDefinition::Simple(_) => {
                        let pressed_key = PressedKeyState {
                            keymap_index,
                            key_data: PressedKeyStateKeyData::Simple,
                        };
                        self.pressed_keys.push(pressed_key).unwrap();
                    },
                    KeyDefinition::TapHold { tap: _, hold: _ } => {
                        let pressed_key = PressedKeyState {
                            keymap_index,
                            key_data: PressedKeyStateKeyData::TapHold(TapHoldState::Pending),
                        };
                        self.pressed_keys.push(pressed_key).unwrap();
                    },
                    _ => {}
                }
            },
            input::Event::Release(keymap_index) => {
                self.pressed_keys.iter().position(|&k| k.keymap_index == keymap_index)
                                        .map(|i| self.pressed_keys.remove(i));
            }
        }
    }

    pub fn boot_keyboard_report(self: &Self) -> [u8; 8] {
        let mut report = [0u8; 8];

        let pressed_keys = self.pressed_keys.iter()
                                            .filter_map(|&pk| pk.key_code(self.key_definitions));

        let (modifier_keys, key_codes): (heapless::Vec<u8, N>, heapless::Vec<u8, N>) =
            pressed_keys.partition(|&kc| kc >= 0xE0 && kc <= 0xE7);

        let modifier = modifier_keys.iter().fold(0u8, |acc, &kc| acc | (1 << (kc - 0xE0)));
        report[0] = modifier;

        for (i, key_code) in key_codes.iter().take(6).enumerate() {
            report[i + 2] = *key_code;
        }
        report
    }
}
