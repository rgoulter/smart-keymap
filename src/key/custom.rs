use core::fmt::Debug;
use core::marker::PhantomData;

use serde::Deserialize;

use crate::key;

/// Reference for a custom key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Ref(pub u8);

/// The context type for keymap callback keys. (No events).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context;

/// The event type for keymap callback keys. (No events).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Event;

/// The pending key state type for keymap callback keys. (No pending state).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PendingKeyState;

/// Key state used by [System]. (No state).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState;

/// The [key::System] implementation for custom keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct System<R>(PhantomData<R>);

impl<R> System<R> {
    /// Constructs a new [System] with the given key data.
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<R> Default for System<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: Debug> key::System<R> for System<R> {
    type Ref = Ref;
    type Context = Context;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        _keymap_index: u16,
        _context: &Self::Context,
        _key_ref: Ref,
    ) -> (
        key::PressedKeyResult<R, Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        let pkr = key::PressedKeyResult::Resolved(KeyState);
        let pke = key::KeyEvents::no_events();
        (pkr, pke)
    }

    fn update_pending_state(
        &self,
        _pending_state: &mut Self::PendingKeyState,
        _keymap_index: u16,
        _context: &Self::Context,
        _key_ref: Ref,
        _event: key::Event<Self::Event>,
    ) -> (Option<key::NewPressedKey<R>>, key::KeyEvents<Self::Event>) {
        panic!()
    }

    fn update_state(
        &self,
        _key_state: &mut Self::KeyState,
        _ref: &Self::Ref,
        _context: &Self::Context,
        _keymap_index: u16,
        _event: key::Event<Self::Event>,
    ) -> key::KeyEvents<Self::Event> {
        panic!()
    }

    fn key_output(
        &self,
        Ref(custom_code): &Self::Ref,
        _key_state: &Self::KeyState,
    ) -> Option<key::KeyOutput> {
        Some(key::KeyOutput::from_custom_code(*custom_code))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sizeof_ref() {
        assert_eq!(1, core::mem::size_of::<Ref>());
    }
}
