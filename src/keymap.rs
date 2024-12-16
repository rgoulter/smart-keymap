use crate::input;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum KeyDefinition {
    Simple(u8),
    TapHold { tap: u8, hold: u8 },
}

pub const KEY_DEFINITIONS: [KeyDefinition; 3] = [
    KeyDefinition::Simple(0x04),
    KeyDefinition::Simple(0x04),
    KeyDefinition::Simple(0x04),
];

/// The engine (set of key definition systems),
///  and key definitions.
pub struct Keymap<const N: usize> {
    key_definitions: [KeyDefinition; N],
    pressed_keys: heapless::Vec<u8, N>,
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
            input::Event::Press(idx) => {
                let key_definition = self.key_definitions[idx as usize];
                match key_definition {
                    KeyDefinition::Simple(key_code) => {
                        self.pressed_keys.push(key_code).unwrap();
                    },
                    _ => {}
                }
            },
            input::Event::Release(idx) => {
                let key_definition = self.key_definitions[idx as usize];
                match key_definition {
                    KeyDefinition::Simple(key_code) => {
                        self.pressed_keys.iter().position(|&k| k == key_code)
                                                .map(|i| self.pressed_keys.remove(i));
                    },
                    _ => {}
                }
            }
        }
    }

    pub fn boot_keyboard_report(self: &Self) -> [u8; 8] {
        let mut report = [0u8; 8];
        for (i, key_code) in self.pressed_keys.iter().take(6).enumerate() {
            report[i + 2] = *key_code;
        }
        report
    }
}
