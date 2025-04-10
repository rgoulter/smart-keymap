use core::fmt::Debug;

#[cfg(feature = "std")]
use serde::Deserialize;

use crate::key;

use super::BaseKey;
use super::{Context, Event, KeyState, PendingKeyState, PressedKeyResult};
use super::{TapHold, TapHoldKey, TapHoldNestable};

/// Trait for types which can be nested in [LayeredKey] variants.
pub trait LayeredNestable:
    key::Key<
        Context = Context,
        Event = Event,
        KeyState = KeyState,
        PendingKeyState = PendingKeyState,
    > + Copy
    + PartialEq
{
}

impl<K: TapHoldNestable> LayeredNestable for TapHold<K> {}
impl<K: TapHoldNestable> LayeredNestable for TapHoldKey<K> {}

/// An aggregate of [key::Key] types.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "std", derive(Deserialize))]
#[cfg_attr(feature = "std", serde(untagged))]
pub enum LayeredKey<K: LayeredNestable> {
    /// A layered key.
    Layered(key::layered::LayeredKey<K>),
    /// Non-layered,
    Pass(K),
}

/// Newtype for [LayeredNestable] keys so they can implement [key::Key].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Layered<K: LayeredNestable>(pub K);

impl<K: LayeredNestable> key::Key for LayeredKey<K> {
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
            LayeredKey::Layered(key) => key.new_pressed_key(context, key_path),
            LayeredKey::Pass(key) => key.new_pressed_key(context, key_path),
        }
    }

    fn handle_event(
        &self,
        pending_state: &mut Self::PendingKeyState,
        context: Self::Context,
        key_path: key::KeyPath,
        event: key::Event<Self::Event>,
    ) -> (
        Option<key::PressedKeyResult<Self::PendingKeyState, Self::KeyState>>,
        key::KeyEvents<Self::Event>,
    ) {
        match self {
            LayeredKey::Layered(key) => key.handle_event(pending_state, context, key_path, event),
            LayeredKey::Pass(key) => key.handle_event(pending_state, context, key_path, event),
        }
    }

    fn lookup(
        &self,
        path: &[u16],
    ) -> &dyn key::Key<
        Context = Self::Context,
        Event = Self::Event,
        PendingKeyState = Self::PendingKeyState,
        KeyState = Self::KeyState,
    > {
        match self {
            LayeredKey::Layered(key) => key.lookup(path),
            LayeredKey::Pass(key) => key.lookup(path),
        }
    }
}

impl<K: LayeredNestable> key::Key for Layered<K> {
    type Context = Context;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        key_path: key::KeyPath,
    ) -> (PressedKeyResult, key::KeyEvents<Self::Event>) {
        let Layered(key) = self;
        key.new_pressed_key(context, key_path)
    }

    fn handle_event(
        &self,
        pending_state: &mut Self::PendingKeyState,
        context: Self::Context,
        key_path: key::KeyPath,
        event: key::Event<Self::Event>,
    ) -> (
        Option<key::PressedKeyResult<Self::PendingKeyState, Self::KeyState>>,
        key::KeyEvents<Self::Event>,
    ) {
        let Layered(key) = self;
        key.handle_event(pending_state, context, key_path, event)
    }

    fn lookup(
        &self,
        path: &[u16],
    ) -> &dyn key::Key<
        Context = Self::Context,
        Event = Self::Event,
        PendingKeyState = Self::PendingKeyState,
        KeyState = Self::KeyState,
    > {
        let Layered(key) = self;
        key.lookup(path)
    }
}

impl LayeredKey<TapHoldKey<BaseKey>> {
    /// Constructs a [LayeredKey] from the given [key::keyboard::Key].
    pub const fn keyboard(key: key::keyboard::Key) -> Self {
        Self::Pass(TapHoldKey::keyboard(key))
    }

    /// Constructs a [LayeredKey] from the given [key::tap_hold::Key].
    pub const fn tap_hold(key: key::tap_hold::Key<BaseKey>) -> Self {
        Self::Pass(TapHoldKey::tap_hold(key))
    }

    /// Constructs a [LayeredKey] from the given [key::layered::ModifierKey].
    pub const fn layer_modifier(key: key::layered::ModifierKey) -> Self {
        Self::Pass(TapHoldKey::layer_modifier(key))
    }
}

impl<K: LayeredNestable> LayeredKey<K> {
    /// Constructs a [LayeredKey] from the given [key::layered::LayeredKey].
    pub const fn layered(key: key::layered::LayeredKey<K>) -> Self {
        Self::Layered(key)
    }
}

impl Layered<TapHold<key::keyboard::Key>> {
    /// Constructs a [Layered] newtype from the given key.
    pub const fn keyboard(key: key::keyboard::Key) -> Self {
        Self(TapHold(key))
    }
}

impl Layered<TapHoldKey<key::keyboard::Key>> {
    /// Constructs a [Layered] newtype from the given [key::tap_hold::Key].
    pub const fn tap_hold(key: key::tap_hold::Key<key::keyboard::Key>) -> Self {
        Self(TapHoldKey::TapHold(key))
    }
}
