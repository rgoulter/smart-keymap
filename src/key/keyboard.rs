use core::fmt::Debug;
use core::ops::Index;

use serde::Deserialize;

use crate::key;

/// Reference for a keyboard key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Ref {
    /// A key code without modifiers. (Value is the HID usage code).
    KeyCode(u8),
    /// A modifiers. (Value is a bitfield of `key::KeyboardModifiers`).
    Modifiers(u8),
    /// A key code with modifiers. (Value is the index into the key data array of [System]).
    KeyCodeAndModifier(u8),
}

/// A key for HID Keyboard usage codes.
#[derive(Deserialize, Clone, Copy, PartialEq, Default)]
pub struct Key {
    /// HID usage code.
    #[serde(default)]
    pub key_code: u8,
    /// Modifiers.
    #[serde(default)]
    pub modifiers: key::KeyboardModifiers,
}

impl core::fmt::Debug for Key {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match (
            self.key_code != 0x00,
            self.modifiers != key::KeyboardModifiers::new(),
        ) {
            (true, true) => f
                .debug_struct("Key")
                .field("key_code", &self.key_code)
                .field("modifiers", &self.modifiers)
                .finish(),
            (false, true) => f
                .debug_struct("Key")
                .field("modifiers", &self.modifiers)
                .finish(),
            _ => f
                .debug_struct("Key")
                .field("key_code", &self.key_code)
                .finish(),
        }
    }
}

/// Config for keyboard keys. (No config).
pub struct Config;

/// Default config for keyboard keys. (No config).
pub const DEFAULT_CONFIG: Config = Config {};

/// Context for keyboard keys. (No context).
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

/// The event type for keyboard keys. (No events).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Event;

/// The pending key state type for keyboard keys. (No pending state).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PendingKeyState;

/// Key state used by [System].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState;

/// The [key::System] implementation for keyboard keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct System<Keys: Index<usize, Output = Key>> {
    keys: Keys,
}

impl<Keys: Index<usize, Output = Key>> System<Keys> {
    /// Constructs a new [System] with the given key data.
    ///
    /// The key data is for keys with both key codes and modifiers.
    pub const fn new(key_data: Keys) -> Self {
        Self { keys: key_data }
    }
}

impl<R, Keys: Debug + Index<usize, Output = Key>> key::System<R> for System<Keys> {
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
            Ref::KeyCode(kc) => Some(key::KeyOutput::from_key_code(*kc)),
            Ref::Modifiers(m) => Some(key::KeyOutput::from_key_modifiers(
                key::KeyboardModifiers::from_byte(*m),
            )),
            Ref::KeyCodeAndModifier(idx) => {
                let Key {
                    key_code,
                    modifiers,
                } = self.keys[*idx as usize];
                Some(key::KeyOutput::from_key_code_with_modifiers(
                    key_code, modifiers,
                ))
            }
        }
    }
}
