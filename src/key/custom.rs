use serde::Deserialize;

use crate::key;

/// A key for HID Keyboard usage codes.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key {
    custom: u8,
}

impl Key {
    /// Constructs a key with the given custom key indices.
    pub const fn new(i: u8) -> Self {
        Key { custom: i }
    }

    /// Constructs a pressed key state
    pub fn new_pressed_key(&self) -> KeyState {
        KeyState(*self)
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
        let k_ks = self.new_pressed_key();
        let pks = key::PressedKeyResult::Resolved(k_ks.into());
        let pke = key::KeyEvents::no_events();
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

/// [crate::key::KeyState] for [Key]. (crate::key::keyboard pressed keys don't have state).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState(Key);

impl KeyState {
    /// Custom key always has a key_output.
    pub fn key_output(&self) -> key::KeyOutput {
        let KeyState(key) = self;
        key::KeyOutput::from_custom_code(key.custom)
    }
}
