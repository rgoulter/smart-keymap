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
    /// A virtual key press for a given `key_code`.
    VirtualKeyPress {
        /// The virtual key code.
        key_output: key::KeyOutput,
    },
    /// A virtual key release for a given `key_code`.
    VirtualKeyRelease {
        /// The virtual key code.
        key_output: key::KeyOutput,
    },
}

/// A struct for associating a key ref with a [crate::key::KeyState].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PressedKey<R, S> {
    /// The index of the pressed key in some keymap.
    pub keymap_index: u16,
    /// The Ref to the key data of the key state.
    pub key_ref: R,
    /// The pressed key state.
    pub key_state: S,
}

/// State resulting from [Event].
#[derive(Debug, Clone, Copy)]
pub enum PressedInput<KR, KS> {
    /// Physically pressed key.
    Key(PressedKey<KR, KS>),
    /// Virtually pressed key, and its keycode.
    Virtual(key::KeyOutput),
}

impl<KR, KS> PressedInput<KR, KS> {
    /// Constructor for a [PressedInput::Key].
    pub fn pressed_key(keymap_index: u16, key_ref: KR, key_state: KS) -> Self {
        Self::Key(PressedKey {
            keymap_index,
            key_ref,
            key_state,
        })
    }
}
