use crate::key;

#[derive(Debug, Clone, Copy)]
pub struct Key(pub u8);

impl Key {
    pub fn key_code(&self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct Event();

#[derive(Debug, Clone, Copy)]
pub struct PressedKey {}

impl PressedKey {
    pub fn new() -> Self {
        Self {}
    }

    pub fn key_code(&self, key_def: &Key) -> u8 {
        key_def.key_code()
    }
}

impl Default for PressedKey {
    fn default() -> Self {
        Self::new()
    }
}

impl key::Key for Key {
    type Event = Event;
    type PressedKey = PressedKey;

    fn new_pressed_key(
        _keymap_index: u16,
        _key_definition: &Self,
    ) -> (Self::PressedKey, Option<key::ScheduledEvent<Self::Event>>) {
        (PressedKey::new(), None)
    }
}

impl key::PressedKey for PressedKey {
    type Event = Event;
    type Key = Key;

    /// Simple key never emits events.
    fn handle_event(
        &mut self,
        _key_definition: &Self::Key,
        _event: key::Event<Self::Event>,
    ) -> impl IntoIterator<Item = key::Event<Self::Event>> {
        None
    }

    /// Simple key always has a key_code.
    fn key_code(&self, key_definition: &Self::Key) -> Option<u8> {
        Some(self.key_code(key_definition))
    }
}
