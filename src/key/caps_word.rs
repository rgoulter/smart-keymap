#![doc = include_str!("doc_de_caps_word.md")]

use serde::Deserialize;

use crate::input;
use crate::key;
use crate::keymap;

/// Caps Word context.
#[derive(Debug, Clone, Copy)]
pub struct Context {
    is_active: bool,
}

/// The default [Context].
pub const DEFAULT_CONTEXT: Context = Context { is_active: false };

impl Context {
    /// Updates the context with the given event.
    pub fn handle_event<E>(&mut self, event: key::Event<E>) -> key::KeyEvents<E>
    where
        Event: TryFrom<E>,
        E: core::fmt::Debug + core::marker::Copy,
    {
        match event {
            key::Event::Keymap(keymap::KeymapEvent::ResolvedKeyOutput {
                key_output:
                    key::KeyOutput {
                        key_code,
                        key_modifiers,
                    },
                ..
            }) if self.is_active => {
                // CapsWord is deactivated for key presses other than:
                //   - A-Z
                //   - 0-9
                //   - Backspace, Delete
                //   - Minus, Underscore
                let is_shifted = key_modifiers.has_modifiers(
                    &key::KeyboardModifiers::LEFT_SHIFT.union(&key::KeyboardModifiers::RIGHT_SHIFT),
                );
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
    pub fn new_pressed_key(&self, context: Context, keymap_index: u16) -> key::KeyEvents<Event> {
        let key_event = match self {
            Key::ToggleCapsWord => {
                if context.is_active {
                    Event::DisableCapsWord
                } else {
                    Event::EnableCapsWord
                }
            }
        };
        key::KeyEvents::event(key::Event::key_event(keymap_index, key_event))
    }
}

impl key::Key for Key {
    type Context = crate::init::Context;
    type Event = crate::init::Event;
    type PendingKeyState = crate::init::PendingKeyState;
    type KeyState = crate::init::KeyState;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        key_path: key::KeyPath,
    ) -> (
        key::PressedKeyResult<Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        let caps_word_context = context.into();
        let keymap_index: u16 = key_path[0];
        let pke = self.new_pressed_key(caps_word_context, keymap_index);
        let pks = key::PressedKeyResult::Resolved(key::NoOpKeyState::new().into());
        (pks, pke.into_events())
    }

    fn handle_event(
        &self,
        _pending_state: &mut Self::PendingKeyState,
        _context: Self::Context,
        _key_path: key::KeyPath,
        _event: key::Event<Self::Event>,
    ) -> (Option<Self::KeyState>, key::KeyEvents<Self::Event>) {
        panic!()
    }

    fn lookup(
        &self,
        _path: &[u16],
    ) -> &dyn key::Key<
        Context = Self::Context,
        Event = Self::Event,
        PendingKeyState = Self::PendingKeyState,
        KeyState = Self::KeyState,
    > {
        self
    }
}
