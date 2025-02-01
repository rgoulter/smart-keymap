use serde::Deserialize;

use crate::key;

/// Input events for [crate::keymap::Keymap].
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
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
        /// Inserts the virtual key before the pressed key with this keymap index.
        pressed_keymap_index: u16,
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

impl<K, S> PressedKey<K, S> {
    /// Transforms the PressedKey to a new type.
    pub fn into_pressed_key<IK, IS>(self) -> PressedKey<IK, IS>
    where
        K: Into<IK>,
        S: Into<IS>,
    {
        PressedKey {
            keymap_index: self.keymap_index,
            key: self.key.into(),
            pressed_key_state: self.pressed_key_state.into(),
        }
    }
}

impl<
        K: crate::key::Key,
        S: crate::key::PressedKeyState<K, Context = K::Context, Event = K::Event>,
    > crate::key::PressedKey for PressedKey<K, S>
{
    type Context = K::Context;
    type Event = K::Event;

    fn handle_event(
        &mut self,
        context: Self::Context,
        event: crate::key::Event<Self::Event>,
    ) -> crate::key::PressedKeyEvents<Self::Event> {
        self.pressed_key_state
            .handle_event_for(context, self.keymap_index, &self.key, event)
    }

    fn key_output(&self) -> key::KeyOutputState {
        self.pressed_key_state.key_output(&self.key)
    }
}

/// State resulting from [Event].
#[derive(Debug, Clone, Copy)]
pub enum PressedInput<PK> {
    /// Physically pressed key.
    Key {
        /// The pressed key.
        pressed_key: PK,
    },
    /// Virtually pressed key.
    Virtual {
        /// The pressed key code.
        key_code: u8,
    },
}

impl<PK> PressedInput<PK> {
    /// Constructor for a [PressedInput::Key].
    pub fn new_pressed_key(pressed_key: PK) -> Self {
        Self::Key { pressed_key }
    }
}
