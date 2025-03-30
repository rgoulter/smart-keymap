use serde::Deserialize;

use crate::keymap::KeymapCallback;

/// A key for HID Keyboard usage codes.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key {
    /// The keymap callback
    pub keymap_callback: KeymapCallback,
}

impl Key {
    /// Constructs a key with the given key_code.
    pub const fn new(keymap_callback: KeymapCallback) -> Self {
        Key { keymap_callback }
    }
}
