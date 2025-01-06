use serde::Deserialize;

use crate::{input, key};

/// Chorded key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Context {}

impl key::Context for Context {
    type Event = Event;

    fn handle_event(&mut self, _event: Self::Event) {}
}

/// Chorded key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key(pub u8);

impl key::Key for Key {
    type Context = Context;
    type ContextEvent = Event;
    type Event = Event;
    type PressedKeyState = PressedKeyState;

    fn new_pressed_key(
        &self,
        _context: &Self::Context,
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

/// Events for chorded keys.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Event {}

/// State for pressed keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PressedKeyState;

/// Convenience type alias.
pub type PressedKey = input::PressedKey<Key, PressedKeyState>;

impl key::PressedKeyState<Key> for PressedKeyState {
    type Event = Event;

    fn handle_event_for(
        &mut self,
        _keymap_index: u16,
        _key: &Key,
        _event: key::Event<Self::Event>,
    ) -> impl IntoIterator<Item = key::Event<Self::Event>> {
        None
    }

    fn key_output(&self, _key: &Key) -> Option<key::KeyOutput> {
        None
    }
}
