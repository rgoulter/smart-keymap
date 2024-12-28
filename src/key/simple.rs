use serde::Deserialize;

use crate::key;

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key(pub u8);

impl Key {
    pub fn key_code(&self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Event();

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PressedKey {
    key_code: u8,
}

impl PressedKey {
    pub fn new(key_code: u8) -> Self {
        Self { key_code }
    }

    pub fn key_code(&self, key_def: &Key) -> u8 {
        key_def.key_code()
    }
}

impl From<Event> for () {
    fn from(_: Event) -> Self {}
}

impl key::Key for Key {
    type Context = ();
    type ContextEvent = ();
    type Event = Event;
    type PressedKey = PressedKey;

    fn new_pressed_key(
        &self,
        _context: &Self::Context,
        _keymap_index: u16,
    ) -> (Self::PressedKey, Option<key::ScheduledEvent<Self::Event>>) {
        (PressedKey::new(self.0), None)
    }
}

impl key::PressedKey<Key> for PressedKey {
    type Event = Event;

    /// Simple key never emits events.
    fn handle_event(
        &mut self,
        _key_definition: &Key,
        _event: key::Event<Self::Event>,
    ) -> impl IntoIterator<Item = key::Event<Self::Event>> {
        None
    }

    /// Simple key always has a key_code.
    fn key_code(&self, key_definition: &Key) -> Option<u8> {
        Some(self.key_code(key_definition))
    }
}
