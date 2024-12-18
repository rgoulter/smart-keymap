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
pub enum PressedKeyState {
    Simple { keymap_index: u16 },
    TapHold { keymap_index: u16, state: TapHoldState },
    Virtual { keycode: u8, },
}

impl PressedKeyState {
    pub fn keymap_index(self: &Self) -> Option<u16> {
        match self {
            PressedKeyState::Simple { keymap_index } => Some(*keymap_index),
            PressedKeyState::TapHold { keymap_index, .. } => Some(*keymap_index),
            _ => None,
        }
    }

    pub fn key_code<const N: usize>(self: &Self, key_definitions: [KeyDefinition; N]) -> Option<u8> {
        match self {
            PressedKeyState::Simple { keymap_index } => {
                let key_definition = key_definitions[*keymap_index as usize];
                match key_definition {
                    KeyDefinition::Simple(key_code) => Some(key_code),
                    _ => None,
                }
            },
            PressedKeyState::TapHold { keymap_index, state } => {
                let key_definition = key_definitions[*keymap_index as usize];
                match key_definition {
                    KeyDefinition::TapHold { tap, hold } => {
                        match state {
                            TapHoldState::Tap => Some(tap),
                            TapHoldState::Hold => Some(hold),
                            _ => None,
                        }
                    },
                    _ => None,
                }
            },
            PressedKeyState::Virtual { keycode } => Some(*keycode),
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
    pending_events: heapless::spsc::Queue<input::Event, 256>,
}

impl<const N: usize> Keymap<N> {
    pub const fn new(key_definitions: [KeyDefinition; N]) -> Self {
        Self {
            key_definitions,
            pressed_keys: heapless::Vec::new(),
            pending_events: heapless::spsc::Queue::new(),
        }
    }

    pub fn init(self: &mut Self) {
        self.pressed_keys.clear();
        while let Some(_) = self.pending_events.dequeue() {}
    }

    pub fn handle_input(self: &mut Self, ev: input::Event) {
        match ev {
            input::Event::Press { keymap_index } => {
                // TapHold: any interruption resolves pending TapHold as Hold.
                self.pressed_keys.iter_mut()
                                 .filter(|pk| {
                                     match pk {
                                         PressedKeyState::TapHold { state: TapHoldState::Pending, .. } => true,
                                         _ => false,
                                     }
                                 })
                                 .for_each(|pk| {
                                     match pk {
                                         PressedKeyState::TapHold { state, .. } => {
                                             *state = TapHoldState::Hold;
                                         },
                                         _ => {}
                                     }
                                 });

                let key_definition = self.key_definitions[keymap_index as usize];
                match key_definition {
                    KeyDefinition::Simple(_) => {
                        let pressed_key = PressedKeyState::Simple {
                            keymap_index,
                        };
                        self.pressed_keys.push(pressed_key).unwrap();
                    },
                    KeyDefinition::TapHold { tap: _, hold: _ } => {
                        let pressed_key = PressedKeyState::TapHold {
                            keymap_index,
                            state: TapHoldState::Pending,
                        };
                        self.pressed_keys.push(pressed_key).unwrap();
                    },
                    _ => {}
                }
            },
            input::Event::Release { keymap_index } => {
                // TapHold: resolved as tap (unless it's already resolved as a Hold).
                // But, since the key is released,
                //  we need to enque the tap keycode in the pending events.
                let key_definition = self.key_definitions[keymap_index as usize];
                match key_definition {
                    KeyDefinition::TapHold { tap, hold: _ } => {
                        if let Some(pressed_key) = self.pressed_keys.iter_mut().find(|pk| {
                            pk.keymap_index() == Some(keymap_index)
                        }) {
                           match pressed_key {
                               PressedKeyState::TapHold { state: TapHoldState::Pending, .. } => {
                                   self.pending_events.enqueue(input::Event::VirtualKeyPress { keycode: tap }).unwrap();
                                   self.pending_events.enqueue(input::Event::VirtualKeyRelease { keycode: tap }).unwrap();
                               },
                               _ => {}
                           }
                        }
                    },
                    _ => {}
                }

                self.pressed_keys.iter().position(|&k| k.keymap_index() == Some(keymap_index))
                                        .map(|i| self.pressed_keys.remove(i));
            }
            _ => {}
        }
    }

    pub fn tick(self: &mut Self) {
        // take from pending
        if let Some(ev) = self.pending_events.dequeue() {
            match ev {
                input::Event::VirtualKeyPress { keycode } => {
                    // Add keycode to pressed keys.
                    let pressed_key = PressedKeyState::Virtual {
                        keycode,
                    };
                    self.pressed_keys.push(pressed_key).unwrap();
                },
                input::Event::VirtualKeyRelease { keycode } => {
                    // Remove keycode from pressed keys.
                    self.pressed_keys.iter().position(|&k| {
                        match k {
                            PressedKeyState::Virtual { keycode: kc } => keycode == kc,
                            _ => false,
                        }
                    }).map(|i| self.pressed_keys.remove(i));
                },
                _ => {}
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
