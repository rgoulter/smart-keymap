#![doc = include_str!("doc_de_keyboard.md")]

use serde::Deserialize;

use crate::key;

/// Reference for a keyboard key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Ref {
    /// A key code without modifiers.
    KeyCode(u8),
    /// A modifiers.
    Modifier(u8),
    /// A key code with modifiers.
    KeyCodeAndModifier(u8),
}

/// A key for HID Keyboard usage codes.
#[derive(Deserialize, Clone, Copy, PartialEq)]
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

impl Key {
    /// Constructs a key with the given key_code.
    pub const fn new(key_code: u8) -> Self {
        let modifiers = key::KeyboardModifiers::new();
        Key {
            key_code,
            modifiers,
        }
    }

    /// Constructs a key with the given key_code and modifiers.
    pub const fn new_with_modifiers(key_code: u8, modifiers: key::KeyboardModifiers) -> Self {
        Key {
            key_code,
            modifiers,
        }
    }

    /// Constructs a key with the given modifiers.
    pub const fn from_modifiers(modifiers: key::KeyboardModifiers) -> Self {
        Key {
            key_code: 0x00,
            modifiers,
        }
    }

    /// Gets the key code from [Key].
    pub fn key_code(&self) -> u8 {
        self.key_code
    }

    /// Constructs a pressed key state
    pub fn new_pressed_key(&self) -> KeyState {
        KeyState(*self)
    }
}

/// Config for keyboard keys. (No config).
pub struct Config;

/// Default config for keyboard keys. (No config).
pub const DEFAULT_CONFIG: Config = Config {};

/// Context for keyboard keys. (No context).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context {
}

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
pub struct System;

impl key::System for System {
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
        match key_ref {
            Ref::KeyCode(kc) => {
                let k = Key::new(kc);
                let k_ks = k.new_pressed_key();
                let pks = key::PressedKeyResult::Resolved(k_ks.into());
                let pke = key::KeyEvents::no_events();
                (pks, pke)
            }
            Ref::Modifier(m) => {
                todo!()
            }
            Ref::KeyCodeAndModifier(kc) => {
                todo!()
            }
        }
    }

    fn handle_event(
        &self,
        _pending_state: &mut Self::PendingKeyState,
        _context: &Self::Context,
        _key_ref: Ref,
        _event: key::Event<Self::Event>,
    ) -> (Option<key::NewPressedKey>, key::KeyEvents<Self::Event>) {
        panic!()
    }
}

/// [crate::key::KeyState] for [Key]. (crate::key::keyboard pressed keys don't have state).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState(Key);

impl KeyState {
    /// Keyboard key always has a key_output.
    pub fn key_output(&self) -> key::KeyOutput {
        let KeyState(key) = self;
        key::KeyOutput::from_key_code_with_modifiers(key.key_code, key.modifiers)
    }
}

impl key::KeyState for KeyState {
    type Context = Context;
    type Event = Event;

    fn key_output(&self) -> Option<key::KeyOutput> {
        Some(self.key_output())
    }
}
