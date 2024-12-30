use serde::Deserialize;

use crate::{input, key};

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key(pub u8);

impl Key {
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
        _context: &Self::Context,
        keymap_index: u16,
    ) -> (
        input::PressedKey<Self, Self::PressedKeyState>,
        Option<key::ScheduledEvent<Self::Event>>,
    ) {
        (
            input::PressedKey {
                keymap_index,
                key: *self,
                pressed_key_state: PressedKeyState,
            },
            None,
        )
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Event();

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PressedKeyState;

pub type PressedKey = input::PressedKey<Key, PressedKeyState>;

impl From<Event> for () {
    fn from(_: Event) -> Self {}
}

impl key::PressedKeyState<Key> for PressedKeyState {
    type Event = Event;

    /// Simple key never emits events.
    fn handle_event_for(
        &mut self,
        _keymap_index: u16,
        _key: &Key,
        _event: key::Event<Self::Event>,
    ) -> impl IntoIterator<Item = key::Event<Self::Event>> {
        None
    }

    /// Simple key always has a key_code.
    fn key_code(&self, key: &Key) -> Option<u8> {
        Some(key.key_code())
    }
}
