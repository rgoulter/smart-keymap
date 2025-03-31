#![doc = include_str!("doc_de_caps_word.md")]

use serde::Deserialize;

use crate::key;
use crate::keymap;

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
    pub fn new_pressed_key(&self) -> KeyState {
        KeyState::new()
    }
}

/// Caps Word [crate::key::KeyState].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState {
    is_active: bool,
}

impl KeyState {
    /// Constructs a new key state.
    pub const fn new() -> Self {
        Self { is_active: true }
    }
}

impl key::KeyState for KeyState {
    type Context = key::composite::Context;
    type Event = key::composite::Event;

    fn handle_event(
        &mut self,
        _context: Self::Context,
        _keymap_index: u16,
        event: key::Event<Self::Event>,
    ) -> key::PressedKeyEvents<Self::Event> {
        if let key::Event::Keymap(keymap::KeymapEvent::ResolvedKeyOutput(key::KeyOutput {
            key_code,
            key_modifiers:
                key::KeyboardModifiers {
                    left_shift,
                    right_shift,
                    ..
                },
        })) = event
        {
            // CapsWord is deactivated for key presses other than:
            //   - A-Z
            //   - 0-9
            //   - Backspace, Delete
            //   - Minus, Underscore
            let is_shifted = left_shift || right_shift;
            match key_code {
                0x04..=0x1D => {}                // A-Z
                0x1E..=0x27 if !is_shifted => {} // 0-9
                0x2A => {}                       // Backspace
                0x2D => {}                       // `-` minus
                0x4C => {}                       // Delete
                0xE1 => {}                       // Left Shift
                0xE5 => {}                       // Right Shift
                0x00 => {}                       // No key code (modifier)
                _ => {
                    self.is_active = false;
                }
            }
        }
        key::PressedKeyEvents::no_events()
    }

    fn key_output(&self) -> Option<key::KeyOutput> {
        let lsft = key::KeyboardModifiers::LEFT_SHIFT;
        Some(key::KeyOutput::from_key_modifiers(lsft))
    }

    fn is_persistent(&self) -> bool {
        self.is_active
    }
}
