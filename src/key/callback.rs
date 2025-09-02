#![doc = include_str!("doc_de_callback.md")]

use serde::Deserialize;

use crate::key;

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

impl key::Key for Key {
    type Context = crate::init::Context;
    type Event = crate::init::Event;
    type PendingKeyState = crate::init::PendingKeyState;
    type KeyState = crate::init::KeyState;

    fn new_pressed_key(
        &self,
        _context: &Self::Context,
        _key_path: key::KeyPath,
    ) -> (
        key::PressedKeyResult<Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        let &Key { keymap_callback } = self;
        let pks = key::PressedKeyResult::Resolved(key::NoOpKeyState::new().into());
        let km_ev = crate::keymap::KeymapEvent::Callback(keymap_callback);
        let pke = key::KeyEvents::event(key::Event::Keymap(km_ev));
        (pks, pke)
    }

    fn handle_event(
        &self,
        _pending_state: &mut Self::PendingKeyState,
        _context: &Self::Context,
        _key_path: key::KeyPath,
        _event: key::Event<Self::Event>,
    ) -> (Option<key::NewPressedKey>, key::KeyEvents<Self::Event>) {
        panic!()
    }

    fn lookup(
        &self,
        _path: &[u16],
    ) -> &dyn key::Key<
        Context = Self::Context,
        Event = Self::Event,
        PendingKeyState = Self::PendingKeyState,
        KeyState = Self::KeyState,
    > {
        self
    }
}
