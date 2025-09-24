use core::fmt::Debug;

use serde::Deserialize;

use crate::key;

/// Reference for a consumer key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Ref {
    /// A usage code. (Value is the HID usage code).
    UsageCode(u16),
}

/// A key for HID Consumer usage codes.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Default)]
pub struct Key {
    /// HID usage code.
    #[serde(default)]
    pub usage_code: u16,
}

/// Config for consumer keys. (No config).
pub struct Config;

/// Default config for consumer keys. (No config).
pub const DEFAULT_CONFIG: Config = Config {};

/// Context for consumer keys. (No context).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context;

impl Context {
    /// Constructs a context from the given config.
    pub const fn from_config(_config: Config) -> Context {
        Context {}
    }
}

impl key::Context for Context {
    type Event = Event;

    /// Used to update the [Context]'s state.
    fn handle_event(&mut self, _event: key::Event<Self::Event>) -> key::KeyEvents<Self::Event> {
        key::KeyEvents::no_events()
    }
}

impl crate::keymap::SetKeymapContext for Context {
    fn set_keymap_context(&mut self, _context: crate::keymap::KeymapContext) {}
}

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
        let k_ks = KeyState;
        let pks = key::PressedKeyResult::Resolved(k_ks);
        let pke = key::KeyEvents::no_events();
        (pks, pke)
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
