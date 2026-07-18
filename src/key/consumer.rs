use core::fmt::Debug;
use core::marker::PhantomData;
use core::ops::Index;

use serde::Deserialize;

use crate::key;

/// Reference for a consumer key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Ref {
    /// A usage code without keyboard modifiers. (Value is the HID usage code).
    UsageCode(u8),
    /// Index into the key data array of [System] for a [Key] (usage + modifiers).
    Key(u8),
}

/// A consumer key: HID usage code with optional keyboard modifiers.
///
/// A modifiers value of zero is equivalent to no modifiers.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key {
    /// HID consumer usage code.
    pub usage_code: u8,
    /// Keyboard modifiers.
    #[serde(default)]
    pub modifiers: key::KeyboardModifiers,
}

impl Key {
    /// Constructs a key with the given usage code and no modifiers.
    pub const fn new(usage_code: u8) -> Self {
        Self {
            usage_code,
            modifiers: key::KeyboardModifiers::new(),
        }
    }

    /// Constructs a key with the given usage code and modifiers.
    pub const fn new_with_modifiers(usage_code: u8, modifiers: key::KeyboardModifiers) -> Self {
        Self {
            usage_code,
            modifiers,
        }
    }
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
pub struct System<R: Debug, Keys: Index<usize, Output = Key>> {
    keys: Keys,
    marker: PhantomData<R>,
}

impl<R: Debug, Keys: Index<usize, Output = Key>> System<R, Keys> {
    /// Constructs a new [System] with the given key data.
    ///
    /// The key data is for consumer keys that include keyboard modifiers.
    pub const fn new(keys: Keys) -> Self {
        Self {
            keys,
            marker: PhantomData,
        }
    }
}

impl<R: Debug, Keys: Debug + Index<usize, Output = Key>> key::System<R> for System<R, Keys> {
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
            Ref::Key(idx) => {
                let Key {
                    usage_code,
                    modifiers,
                } = self.keys[*idx as usize];
                Some(key::KeyOutput::from_usage_with_modifiers(
                    key::KeyUsage::Consumer(usage_code),
                    modifiers,
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sizeof_ref() {
        assert_eq!(2, core::mem::size_of::<Ref>());
    }

    #[test]
    fn test_sizeof_event() {
        assert_eq!(0, core::mem::size_of::<Event>());
    }
}
