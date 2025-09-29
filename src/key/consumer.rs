use core::fmt::Debug;

use serde::Deserialize;

use crate::key;

/// Reference for a consumer key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Ref {
    /// A usage code. (Value is the HID usage code).
    UsageCode(u8),
}

/// Context for consumer keys. (No context).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context;

/// The event type for consumer keys. (No events).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Event;

/// The pending key state type for consumer keys. (No pending state).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PendingKeyState;

/// Key state used by [System].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState;

/// The [key::System] implementation for consumer keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct System<R: Debug> {
    _marker: core::marker::PhantomData<R>,
}

impl<R: Debug> System<R> {
    /// Constructs a new [System].
    pub const fn new() -> Self {
        Self {
            _marker: core::marker::PhantomData,
        }
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
        (
            key::PressedKeyResult::Resolved(KeyState),
            key::KeyEvents::no_events(),
        )
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
        key::KeyEvents::no_events()
    }

    fn key_output(
        &self,
        key_ref: &Self::Ref,
        _key_state: &Self::KeyState,
    ) -> Option<key::KeyOutput> {
        match key_ref {
            Ref::UsageCode(uc) => Some(key::KeyOutput::from_consumer_code(*uc)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sizeof_ref() {
        assert_eq!(1, core::mem::size_of::<Ref>());
    }

    #[test]
    fn test_sizeof_event() {
        assert_eq!(0, core::mem::size_of::<Event>());
    }
}
