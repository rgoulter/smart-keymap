use core::fmt::Debug;

#[cfg(feature = "std")]
use serde::Deserialize;

use crate::key;
use key::keyboard;
use key::layered;

use super::{Context, Event, KeyState, PendingKeyState, PressedKeyResult};

/// An aggregate of [key::Key] types.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "std", derive(Deserialize))]
#[cfg_attr(feature = "std", serde(untagged))]
pub enum BaseKey {
    /// A layer modifier key.
    LayerModifier(layered::ModifierKey),
    /// A keyboard key.
    Keyboard(keyboard::Key),
}

impl key::Key for layered::ModifierKey {
    type Context = Context;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        _context: Self::Context,
        key_path: key::KeyPath,
    ) -> (PressedKeyResult, key::PressedKeyEvents<Self::Event>) {
        let keymap_index: u16 = key_path[0];
        let (m_ks, lmod_ev) = self.new_pressed_key();
        let pks = key::PressedKeyResult::Resolved(KeyState::LayerModifier(m_ks));
        let pke = key::PressedKeyEvents::event(key::Event::key_event(
            keymap_index,
            Event::LayerModification(lmod_ev),
        ));
        (pks, pke)
    }

    fn handle_event(
        &self,
        _pending_state: &mut Self::PendingKeyState,
        _context: Self::Context,
        _key_path: key::KeyPath,
        _event: key::Event<Self::Event>,
    ) -> (Option<Self::KeyState>, key::PressedKeyEvents<Self::Event>) {
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

impl key::Key for keyboard::Key {
    type Context = Context;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        _context: Self::Context,
        _key_path: key::KeyPath,
    ) -> (PressedKeyResult, key::PressedKeyEvents<Self::Event>) {
        let k_ks = self.new_pressed_key();
        let pks = key::PressedKeyResult::Resolved(KeyState::Keyboard(k_ks));
        let pke = key::PressedKeyEvents::no_events();
        (pks, pke)
    }

    fn handle_event(
        &self,
        _pending_state: &mut Self::PendingKeyState,
        _context: Self::Context,
        _key_path: key::KeyPath,
        _event: key::Event<Self::Event>,
    ) -> (Option<Self::KeyState>, key::PressedKeyEvents<Self::Event>) {
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

impl key::Key for BaseKey {
    type Context = Context;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        key_path: key::KeyPath,
    ) -> (PressedKeyResult, key::PressedKeyEvents<Self::Event>) {
        match self {
            BaseKey::Keyboard(key) => key::Key::new_pressed_key(key, context, key_path),
            BaseKey::LayerModifier(key) => key::Key::new_pressed_key(key, context, key_path),
        }
    }

    fn handle_event(
        &self,
        _pending_state: &mut Self::PendingKeyState,
        _context: Self::Context,
        _key_path: key::KeyPath,
        _event: key::Event<Self::Event>,
    ) -> (Option<Self::KeyState>, key::PressedKeyEvents<Self::Event>) {
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

impl From<keyboard::Key> for BaseKey {
    fn from(key: keyboard::Key) -> Self {
        BaseKey::Keyboard(key)
    }
}

impl From<layered::ModifierKey> for BaseKey {
    fn from(key: layered::ModifierKey) -> Self {
        BaseKey::LayerModifier(key)
    }
}

impl BaseKey {
    /// Constructs a [BaseKey::Keyboard] from the given [keyboard::Key].
    pub const fn keyboard(key: keyboard::Key) -> Self {
        Self::Keyboard(key)
    }

    /// Constructs a [BaseKey::LayerModifier] from the given [layered::ModifierKey].
    pub const fn layer_modifier(key: layered::ModifierKey) -> Self {
        Self::LayerModifier(key)
    }
}
