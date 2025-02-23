use core::fmt::Debug;

#[cfg(feature = "std")]
use serde::Deserialize;

use crate::{input, key};
use key::keyboard;
use key::layered;

use super::{Context, Event};

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
    type PressedKey = BasePressedKey;

    fn new_pressed_key(
        &self,
        _context: Self::Context,
        keymap_index: u16,
    ) -> (Self::PressedKey, key::PressedKeyEvents<Self::Event>) {
        let (pks, lmod_ev) = self.new_pressed_key();
        let pk = BasePressedKey {
            key: (*self).into(),
            keymap_index,
            pressed_key_state: pks.into(),
        };
        let pke = key::PressedKeyEvents::event(key::Event::key_event(
            keymap_index,
            Event::LayerModification(lmod_ev),
        ));
        (pk.into_pressed_key(), pke)
    }
}

impl key::Key for keyboard::Key {
    type Context = Context;
    type Event = Event;
    type PressedKey = BasePressedKey;

    fn new_pressed_key(
        &self,
        _context: Self::Context,
        keymap_index: u16,
    ) -> (Self::PressedKey, key::PressedKeyEvents<Self::Event>) {
        let pks = self.new_pressed_key();
        let pk = BasePressedKey {
            key: (*self).into(),
            keymap_index,
            pressed_key_state: pks.into(),
        };
        let pke = key::PressedKeyEvents::no_events();
        (pk.into_pressed_key(), pke)
    }
}

impl key::Key for BaseKey {
    type Context = Context;
    type Event = Event;
    type PressedKey = BasePressedKey;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        keymap_index: u16,
    ) -> (Self::PressedKey, key::PressedKeyEvents<Self::Event>) {
        match self {
            BaseKey::Keyboard(key) => key::Key::new_pressed_key(key, context, keymap_index),
            BaseKey::LayerModifier(key) => key::Key::new_pressed_key(key, context, keymap_index),
        }
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

/// Aggregates the [key::PressedKeyState] types.
#[derive(Debug, PartialEq)]
pub enum BasePressedKeyState {
    /// A keyboard key's pressed state.
    Keyboard(keyboard::PressedKeyState),
    /// A layer modifier key's pressed state.
    LayerModifier(layered::PressedModifierKeyState),
}

/// Convenience type alias for a [key::PressedKey] with a base key.
pub type BasePressedKey = input::PressedKey<BaseKey, BasePressedKeyState>;

impl<K: Copy + Into<BaseKey>> key::PressedKeyState<K> for BasePressedKeyState {
    type Context = Context;
    type Event = Event;

    fn handle_event_for(
        &mut self,
        _context: Context,
        keymap_index: u16,
        key: &K,
        event: key::Event<Event>,
    ) -> key::PressedKeyEvents<Event> {
        let bk: BaseKey = (*key).into();

        match (bk, self) {
            (BaseKey::LayerModifier(key), BasePressedKeyState::LayerModifier(pks)) => {
                if let Ok(ev) = event.try_into_key_event(|e| e.try_into()) {
                    let events = pks.handle_event_for(keymap_index, &key, ev);
                    match events {
                        Some(ev) => key::PressedKeyEvents::event(key::Event::key_event(
                            keymap_index,
                            Event::LayerModification(ev),
                        )),
                        None => key::PressedKeyEvents::no_events(),
                    }
                } else {
                    key::PressedKeyEvents::no_events()
                }
            }
            _ => key::PressedKeyEvents::no_events(),
        }
    }

    fn key_output(&self, key: &K) -> key::KeyOutputState {
        let bk: BaseKey = (*key).into();

        match (bk, self) {
            (BaseKey::Keyboard(key), BasePressedKeyState::Keyboard(pks)) => pks.key_output(&key),
            _ => key::KeyOutputState::no_output(),
        }
    }
}

impl From<keyboard::PressedKeyState> for BasePressedKeyState {
    fn from(pks: keyboard::PressedKeyState) -> Self {
        BasePressedKeyState::Keyboard(pks)
    }
}

impl From<layered::PressedModifierKeyState> for BasePressedKeyState {
    fn from(pks: layered::PressedModifierKeyState) -> Self {
        BasePressedKeyState::LayerModifier(pks)
    }
}
