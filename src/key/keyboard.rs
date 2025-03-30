#![doc = include_str!("doc_de_keyboard.md")]

use serde::Deserialize;

use crate::key;

/// A key for HID Keyboard usage codes.
#[derive(Deserialize, Clone, Copy, PartialEq)]
pub struct Key {
    #[serde(default)]
    key_code: u8,
    #[serde(default)]
    modifiers: key::KeyboardModifiers,
}

impl core::fmt::Debug for Key {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match (
            self.key_code != 0x00,
            self.modifiers != key::KeyboardModifiers::new(),
        ) {
            (true, true) => f
                .debug_struct("Key")
                .field("key_code", &self.key_code)
                .field("modifiers", &self.modifiers)
                .finish(),
            (false, true) => f
                .debug_struct("Key")
                .field("modifiers", &self.modifiers)
                .finish(),
            _ => f
                .debug_struct("Key")
                .field("key_code", &self.key_code)
                .finish(),
        }
    }
}

impl Key {
    /// Constructs a key with the given key_code.
    pub const fn new(key_code: u8) -> Self {
        let modifiers = key::KeyboardModifiers::new();
        Key {
            key_code,
            modifiers,
        }
    }

    /// Constructs a key with the given key_code and modifiers.
    pub const fn new_with_modifiers(key_code: u8, modifiers: key::KeyboardModifiers) -> Self {
        Key {
            key_code,
            modifiers,
        }
    }

    /// Constructs a key with the given modifiers.
    pub const fn from_modifiers(modifiers: key::KeyboardModifiers) -> Self {
        Key {
            key_code: 0x00,
            modifiers,
        }
    }

    /// Gets the key code from [Key].
    pub fn key_code(&self) -> u8 {
        self.key_code
    }

    /// Constructs a pressed key state
    pub fn new_pressed_key(&self) -> KeyState {
        KeyState(*self)
    }
}

/// Unit-like struct for [crate::key::KeyState]. (crate::key::keyboard pressed keys don't have state).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState(Key);

impl KeyState {
    /// Keyboard key always has a key_output.
    pub fn key_output(&self) -> key::KeyOutput {
        let KeyState(key) = self;
        key::KeyOutput::from_key_code_with_modifiers(key.key_code, key.modifiers)
    }
}

impl key::KeyState for KeyState {
    type Context = key::composite::Context;
    type Event = key::composite::Event;

    fn handle_event(
        &mut self,
        _context: Self::Context,
        _keymap_index: u16,
        _event: key::Event<Self::Event>,
    ) -> key::PressedKeyEvents<Self::Event> {
        key::PressedKeyEvents::no_events()
    }

    fn key_output(&self) -> Option<key::KeyOutput> {
        Some(self.key_output())
    }

    fn is_persistent(&self) -> bool {
        false
    }
}
