use core::fmt::Debug;

#[cfg(feature = "std")]
use serde::Deserialize;

use crate::key;

use key::callback;
use key::caps_word;
use key::keyboard;
use key::layered;
use key::sticky;

use super::{Context, Event, KeyState, PendingKeyState, PressedKeyResult};

/// An aggregate of [key::Key] types.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "std", derive(Deserialize))]
#[cfg_attr(feature = "std", serde(untagged))]
pub enum BaseKey {
    /// A layer modifier key.
    LayerModifier(layered::ModifierKey),
    /// A callback key.
    Callback(callback::Key),
    /// Caps Word key
    CapsWord(caps_word::Key),
    /// Sticky Modifier key
    Sticky(sticky::Key),
    /// A keyboard key.
    Keyboard(keyboard::Key),
}

impl key::Key for callback::Key {
    type Context = Context;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        _context: Self::Context,
        _key_path: key::KeyPath,
    ) -> (PressedKeyResult, key::KeyEvents<Self::Event>) {
        let &callback::Key { keymap_callback } = self;
        let pks = key::PressedKeyResult::Resolved(KeyState::NoOp);
        let km_ev = crate::keymap::KeymapEvent::Callback(keymap_callback);
        let pke = key::KeyEvents::event(key::Event::Keymap(km_ev));
        (pks, pke)
    }

    fn handle_event(
        &self,
        _pending_state: &mut Self::PendingKeyState,
        _context: Self::Context,
        _key_path: key::KeyPath,
        _event: key::Event<Self::Event>,
    ) -> (Option<Self::KeyState>, key::KeyEvents<Self::Event>) {
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

impl key::Key for caps_word::Key {
    type Context = Context;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        key_path: key::KeyPath,
    ) -> (PressedKeyResult, key::KeyEvents<Self::Event>) {
        let caps_word_context = context.into();
        let keymap_index: u16 = key_path[0];
        let pke = self.new_pressed_key(caps_word_context, keymap_index);
        let pks = key::PressedKeyResult::Resolved(KeyState::NoOp);
        (pks, pke.into_events())
    }

    fn handle_event(
        &self,
        _pending_state: &mut Self::PendingKeyState,
        _context: Self::Context,
        _key_path: key::KeyPath,
        _event: key::Event<Self::Event>,
    ) -> (Option<Self::KeyState>, key::KeyEvents<Self::Event>) {
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

impl key::Key for sticky::Key {
    type Context = Context;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        _context: Self::Context,
        _key_path: key::KeyPath,
    ) -> (PressedKeyResult, key::KeyEvents<Self::Event>) {
        let ks = self.new_pressed_key();
        let pks = key::PressedKeyResult::Resolved(ks.into());
        let pke = key::KeyEvents::no_events();
        (pks, pke)
    }

    fn handle_event(
        &self,
        _pending_state: &mut Self::PendingKeyState,
        _context: Self::Context,
        _key_path: key::KeyPath,
        _event: key::Event<Self::Event>,
    ) -> (Option<Self::KeyState>, key::KeyEvents<Self::Event>) {
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
    ) -> (PressedKeyResult, key::KeyEvents<Self::Event>) {
        match self {
            BaseKey::Keyboard(key) => key::Key::new_pressed_key(key, context, key_path),
            BaseKey::LayerModifier(key) => key::Key::new_pressed_key(key, context, key_path),
            BaseKey::Callback(key) => key::Key::new_pressed_key(key, context, key_path),
            BaseKey::CapsWord(key) => key::Key::new_pressed_key(key, context, key_path),
            BaseKey::Sticky(key) => key::Key::new_pressed_key(key, context, key_path),
        }
    }

    fn handle_event(
        &self,
        _pending_state: &mut Self::PendingKeyState,
        _context: Self::Context,
        _key_path: key::KeyPath,
        _event: key::Event<Self::Event>,
    ) -> (Option<Self::KeyState>, key::KeyEvents<Self::Event>) {
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

impl From<callback::Key> for BaseKey {
    fn from(key: callback::Key) -> Self {
        BaseKey::Callback(key)
    }
}

impl From<caps_word::Key> for BaseKey {
    fn from(key: caps_word::Key) -> Self {
        BaseKey::CapsWord(key)
    }
}

impl From<sticky::Key> for BaseKey {
    fn from(key: sticky::Key) -> Self {
        BaseKey::Sticky(key)
    }
}
