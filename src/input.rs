use core::fmt::Debug;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Event {
    Press { keymap_index: u16 },
    Release { keymap_index: u16 },
    VirtualKeyPress { key_code: u8 },
    VirtualKeyRelease { key_code: u8 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PressedKey<K, S> {
    pub keymap_index: u16,
    pub key: K,
    pub pressed_key_state: S,
}

impl<K, Ev, S> crate::key::PressedKey for PressedKey<K, S>
where
    K: crate::key::Key<Ev>,
    Ev: Copy + Debug + Ord,
    S: crate::key::PressedKeyState<K, Event = Ev>,
{
    type Event = Ev;

    fn handle_event(
        &mut self,
        event: crate::key::Event<Self::Event>,
    ) -> impl IntoIterator<Item = crate::key::Event<Self::Event>> {
        self.pressed_key_state
            .handle_event_for(self.keymap_index, &self.key, event)
    }

    fn key_code(&self) -> Option<u8> {
        self.pressed_key_state.key_code(&self.key)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PressedInput<K, S> {
    Key(PressedKey<K, S>),
    Virtual { key_code: u8 },
}

impl<K, S> PressedInput<K, S> {
    pub fn new_pressed_key(keymap_index: u16, key: K, pressed_key_state: S) -> Self {
        Self::Key(PressedKey {
            keymap_index,
            key,
            pressed_key_state,
        })
    }
}
