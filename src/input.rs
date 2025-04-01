use serde::{Deserialize, Serialize};

use crate::key;

/// Input events for [crate::keymap::Keymap].
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
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
}

/// A struct for associating a [crate::key::Key] with a [crate::key::KeyState].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PressedKey<S> {
    /// The index of the pressed key in some keymap.
    pub keymap_index: u16,
    /// The pressed key state.
    pub key_state: S,
    /// Whether the key is currently pressed.
    /// (Persistent key state can be retained even when key is not pressed).
    pub key_pressed: bool,
}

impl<Ctx, Ev, S: crate::key::KeyState<Context = Ctx, Event = Ev>> PressedKey<S> {
    /// Convenience passthrough to key_state handle_event.
    pub fn handle_event(
        &mut self,
        context: Ctx,
        event: crate::key::Event<Ev>,
    ) -> crate::key::PressedKeyEvents<Ev> {
        self.key_state
            .handle_event(context, self.keymap_index, event)
    }

    /// Convenience passthrough to key_state key_output.
    pub fn key_output(&self) -> Option<key::KeyOutput> {
        self.key_state.key_output()
    }
}
