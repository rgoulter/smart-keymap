#![doc = include_str!("doc_de_caps_word.md")]

use serde::Deserialize;

use crate::input;
use crate::key;
use crate::keymap;

/// Caps Word context.
#[derive(Debug, Clone, Copy)]
pub struct Context {
    /// Whether caps word is active.
    pub is_active: bool,
}

/// The default [Context].
pub const DEFAULT_CONTEXT: Context = Context { is_active: false };

impl Context {
    /// Updates the context with the [LayerEvent].
    pub fn handle_event<E>(&mut self, event: key::Event<E>) -> key::KeyEvents<E>
    where
        Event: TryFrom<E>,
        E: core::fmt::Debug + core::marker::Copy,
    {
        match event {
            key::Event::Keymap(keymap::KeymapEvent::ResolvedKeyOutput(key::KeyOutput {
                key_code,
                key_modifiers:
                    key::KeyboardModifiers {
                        left_shift,
                        right_shift,
                        ..
                    },
            })) => {
                // CapsWord is deactivated for key presses other than:
                //   - A-Z
                //   - 0-9
                //   - Backspace, Delete
                //   - Minus, Underscore
                let is_shifted = left_shift || right_shift;
                let exit_caps_word = match key_code {
                    0x04..=0x1D => false,                // A-Z
                    0x1E..=0x27 if !is_shifted => false, // 0-9
                    0x2A => false,                       // Backspace
                    0x2D => false,                       // `-` minus
                    0x4C => false,                       // Delete
                    0xE1 => false,                       // Left Shift
                    0xE5 => false,                       // Right Shift
                    0x00 => false,                       // No key code (modifier)
                    _ => true,
                };

                if exit_caps_word {
                    self.is_active = false;

                    let key_code = 0xE1;
                    let vk_ev = input::Event::VirtualKeyRelease {
                        key_output: key::KeyOutput::from_key_code(key_code),
                    };
                    key::KeyEvents::event(key::Event::Input(vk_ev))
                } else {
                    key::KeyEvents::no_events()
                }
            }
            key::Event::Key { key_event, .. } => {
                if let Ok(ev) = key_event.try_into() {
                    match ev {
                        Event::EnableCapsWord => {
                            self.is_active = true;

                            let key_code = 0xE1;
                            let vk_ev = input::Event::VirtualKeyPress {
                                key_output: key::KeyOutput::from_key_code(key_code),
                            };
                            key::KeyEvents::event(key::Event::Input(vk_ev))
                        }
                        Event::DisableCapsWord => {
                            self.is_active = false;

                            let key_code = 0xE1;
                            let vk_ev = input::Event::VirtualKeyRelease {
                                key_output: key::KeyOutput::from_key_code(key_code),
                            };
                            key::KeyEvents::event(key::Event::Input(vk_ev))
                        }
                    }
                } else {
                    key::KeyEvents::no_events()
                }
            }
            _ => key::KeyEvents::no_events(),
        }
    }
}

/// Caps Word events.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// Enables Caps Word.
    EnableCapsWord,
    /// Disables Caps Word.
    DisableCapsWord,
}

/// A key for HID Keyboard usage codes.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Key {
    /// Enters/Exits CapsWord mode.
    ToggleCapsWord,
}

impl Key {
    /// Constructs a key with the given key_code.
    pub const fn new() -> Self {
        Key::ToggleCapsWord
    }

    /// Constructs a pressed key state
    pub fn new_pressed_key<E>(&self, context: Context, keymap_index: u16) -> key::KeyEvents<E>
    where
        Event: Into<E>,
        E: core::fmt::Debug + core::marker::Copy,
    {
        let key_event = match self {
            Key::ToggleCapsWord => {
                if context.is_active {
                    Event::DisableCapsWord
                } else {
                    Event::EnableCapsWord
                }
            }
        };
        key::KeyEvents::event(key::Event::key_event(keymap_index, key_event.into()))
    }
}
