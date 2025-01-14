#![doc = include_str!("doc_de_simple.md")]

use serde::Deserialize;

use crate::{input, key};

/// A simple key that only has a key code.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key(pub u8);

impl Key {
    /// Gets the key code from [Key].
    pub fn key_code(&self) -> u8 {
        self.0
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

/// Unit-like struct for event. (crate::key::simple doesn't use events).
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Event();

/// Unit-like struct for [crate::key::PressedKeyState]. (crate::key::simple pressed keys don't have state).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PressedKeyState;

/// Convenience type alias for [input::PressedKey] with [Key] and [PressedKeyState].
pub type PressedKey = input::PressedKey<Key, PressedKeyState>;

impl From<Event> for () {
    fn from(_: Event) -> Self {}
}

impl key::PressedKeyState<Key> for PressedKeyState {
    type Event = Event;

    /// Simple key never emits events.
    fn handle_event_for(
        &mut self,
        _context: (),
        _keymap_index: u16,
        _key: &Key,
        _event: key::Event<Self::Event>,
    ) -> key::PressedKeyEvents<Self::Event> {
        key::PressedKeyEvents::no_events()
    }

    /// Simple key always has a key_code.
    fn key_output(&self, key: &Key) -> Option<key::KeyOutput> {
        Some(key::KeyOutput::from_key_code(key.key_code()))
    }
}
