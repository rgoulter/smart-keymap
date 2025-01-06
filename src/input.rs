use serde::Deserialize;

/// Input events for [crate::keymap::Keymap].
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Event {
    /// A physical key press for a given `keymap_index`.
    Press {
        /// The index of the key in the keymap.
        keymap_index: u16,
    },
    /// A physical key release for a given `keymap_index`.
    Release {
        /// The index of the key in the keymap.
        keymap_index: u16,
    },
    /// A virtual key press for a given `key_code`.
    VirtualKeyPress {
        /// The virtual key code.
        key_code: u8,
    },
    /// A virtual key release for a given `key_code`.
    VirtualKeyRelease {
        /// The virtual key code.
        key_code: u8,
    },
}

/// A struct for associating a [crate::key::Key] with a [crate::key::PressedKeyState].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PressedKey<K, S> {
    /// The index of the pressed key in some keymap.
    pub keymap_index: u16,
    /// The pressed key.
    pub key: K,
    /// The pressed key state.
    pub pressed_key_state: S,
}

impl<K: crate::key::Key, S: crate::key::PressedKeyState<K, Event = K::Event>> crate::key::PressedKey
    for PressedKey<K, S>
{
    type Event = K::Event;

    fn handle_event(
        &mut self,
        event: crate::key::Event<Self::Event>,
    ) -> impl IntoIterator<Item = crate::key::Event<Self::Event>> {
        self.pressed_key_state
            .handle_event_for(self.keymap_index, &self.key, event)
    }

    fn key_output(&self) -> Option<crate::key::KeyOutput> {
        self.pressed_key_state.key_output(&self.key)
    }
}

/// State resulting from [Event].
#[derive(Debug, Clone, Copy)]
pub enum PressedInput {
    /// Physically pressed key.
    Key {
        /// The index of the pressed key in the keymap.
        keymap_index: u16,
    },
    /// Virtually pressed key.
    Virtual {
        /// The pressed key code.
        key_code: u8,
    },
}

impl PressedInput {
    /// Constructor for a [PressedInput::Key].
    pub fn new_pressed_key(keymap_index: u16) -> Self {
        Self::Key { keymap_index }
    }
}
