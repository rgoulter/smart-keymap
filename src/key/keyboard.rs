// #![doc = include_str!("doc_de_keyboard.md")]

use serde::Deserialize;

use crate::key;

/// Reference for a keyboard key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Ref {
    /// A key code without modifiers.
    KeyCode(u8),
    /// A modifiers.
    Modifiers(u8),
    /// A key code with modifiers.
    KeyCodeAndModifier(u8),
}

/// A key for HID Keyboard usage codes.
#[derive(Deserialize, Clone, Copy, PartialEq, Default)]
pub struct Key {
    #[serde(default)]
    key_code: u8,
    #[serde(default)]
    modifiers: key::KeyboardModifiers,
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
    /// Constructs a context from the given config
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

/// The [key::System] implementation for keyboard keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct System<const DATA_LEN: usize> {
    key_data: [Key; DATA_LEN],
}

impl<const DATA_LEN: usize> System<DATA_LEN> {
    /// Constructs a new [System] with the given key data.
    ///
    /// The key data is for keys with both key codes and modifiers.
    pub const fn new(key_data: [Key; DATA_LEN]) -> Self {
        Self { key_data }
    }
}

impl<const DATA_LEN: usize> key::System for System<DATA_LEN> {
    type Ref = Ref;
    type Context = Context;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        _context: &Self::Context,
        key_ref: Ref,
    ) -> (
        key::PressedKeyResult<Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        let k_ks = KeyState;
        let pks = key::PressedKeyResult::Resolved(k_ks.into());
        let pke = key::KeyEvents::no_events();
        (pks, pke)
    }

    fn update_pending_state(
        &self,
        _pending_state: &mut Self::PendingKeyState,
        _context: &Self::Context,
        _key_ref: Ref,
        _event: key::Event<Self::Event>,
    ) -> (Option<key::NewPressedKey>, key::KeyEvents<Self::Event>) {
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
                } = self.key_data[*idx as usize];
                Some(key::KeyOutput::from_key_code_with_modifiers(
                    key_code, modifiers,
                ))
            }
        }
    }
}

/// [crate::key::KeyState] for [Key]. (crate::key::keyboard pressed keys don't have state).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState;
