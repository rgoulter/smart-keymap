#![doc = include_str!("doc_de_keyboard.md")]

use serde::Deserialize;

use crate::{input, key};

/// A key for HID Keyboard usage codes.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key {
    key_code: u8,
    #[serde(default)]
    modifiers: key::KeyboardModifiers,
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
}

impl key::Key for Key {
    type Context = ();
    type ContextEvent = ();
    type Event = Event;
    type PressedKeyState = PressedKeyState;

    fn new_pressed_key(
        &self,
        _context: Self::Context,
        keymap_index: u16,
    ) -> (
        input::PressedKey<Self, Self::PressedKeyState>,
        key::PressedKeyEvents<Self::Event>,
    ) {
        (
            input::PressedKey {
                keymap_index,
                key: *self,
                pressed_key_state: PressedKeyState,
            },
            key::PressedKeyEvents::no_events(),
        )
    }
}

/// Unit-like struct for event. (crate::key::keyboard doesn't use events).
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Event();

/// Unit-like struct for [crate::key::PressedKeyState]. (crate::key::keyboard pressed keys don't have state).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PressedKeyState;

/// Convenience type alias for [input::PressedKey] with [Key] and [PressedKeyState].
pub type PressedKey = input::PressedKey<Key, PressedKeyState>;

impl From<Event> for () {
    fn from(_: Event) -> Self {}
}

impl key::PressedKeyState<Key> for PressedKeyState {
    type Event = Event;

    /// Keyboard key never emits events.
    fn handle_event_for(
        &mut self,
        _context: (),
        _keymap_index: u16,
        _key: &Key,
        _event: key::Event<Self::Event>,
    ) -> key::PressedKeyEvents<Self::Event> {
        key::PressedKeyEvents::no_events()
    }

    /// Keyboard key always has a key_output.
    fn key_output(&self, key: &Key) -> Option<key::KeyOutput> {
        Some(key::KeyOutput::from_key_code(key.key_code()))
    }
}
