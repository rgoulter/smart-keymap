use core::fmt::Debug;

#[cfg(feature = "std")]
use serde::Deserialize;

use crate::{input, key};

use super::BaseKey;
use super::{Context, Event};
use super::{TapHold, TapHoldKey, TapHoldNestable, TapHoldPressedKey};

use key::PressedKey as _;

/// Trait for types which can be nested in [LayeredKey] variants.
pub trait LayeredNestable:
    key::Key<Context = Context, Event = Event, PressedKey = TapHoldPressedKey<BaseKey>>
    + Copy
    + PartialEq
{
    /// Construct a 'full representation' of the nestable key.
    fn as_fat_key(self) -> TapHoldKey<BaseKey>;
}

impl LayeredNestable for TapHold<key::layered::ModifierKey> {
    fn as_fat_key(self) -> TapHoldKey<BaseKey> {
        TapHold::as_fat_key(self)
    }
}
impl LayeredNestable for TapHold<key::keyboard::Key> {
    fn as_fat_key(self) -> TapHoldKey<BaseKey> {
        TapHold::as_fat_key(self)
    }
}
impl LayeredNestable for TapHold<BaseKey> {
    fn as_fat_key(self) -> TapHoldKey<BaseKey> {
        TapHold::as_fat_key(self)
    }
}
impl<K: TapHoldNestable> LayeredNestable for TapHoldKey<K> {
    fn as_fat_key(self) -> TapHoldKey<BaseKey> {
        TapHoldKey::as_fat_key(self)
    }
}

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

impl<K: LayeredNestable> LayeredKey<K> {
    /// Constructs a 'fat' key value from the given tap hold key.
    pub fn as_fat_key(self) -> LayeredKey<TapHoldKey<BaseKey>> {
        match self {
            LayeredKey::Layered(key) => LayeredKey::Layered(key.map_key(|k| k.as_fat_key())),
            LayeredKey::Pass(key) => LayeredKey::Pass(key.as_fat_key()),
        }
    }
}

/// Newtype for [LayeredNestable] keys so they can implement [key::Key] for [LayeredPressedKey].
#[derive(Debug, Clone, Copy)]
pub struct Layered<K: LayeredNestable>(pub K);

impl<K: LayeredNestable> key::Key for LayeredKey<K> {
    type Context = Context;
    type Event = Event;
    type PressedKey = LayeredPressedKey<TapHoldKey<BaseKey>>;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        keymap_index: u16,
    ) -> (Self::PressedKey, key::PressedKeyEvents<Self::Event>) {
        match self {
            LayeredKey::Layered(key) => {
                let (layered_pk, pke) = key.new_pressed_key(context.into(), keymap_index);
                let pk = input::PressedKey {
                    key: LayeredKey::Pass(layered_pk.key),
                    keymap_index,
                    pressed_key_state: LayeredPressedKeyState::<TapHoldKey<BaseKey>>(layered_pk),
                };
                (pk, pke)
            }
            LayeredKey::Pass(key) => {
                let (passthrough_pk, pke) = key.new_pressed_key(context.into(), keymap_index);
                let pk = input::PressedKey {
                    key: LayeredKey::Pass(passthrough_pk.key),
                    keymap_index,
                    pressed_key_state: LayeredPressedKeyState::<TapHoldKey<BaseKey>>(
                        passthrough_pk,
                    ),
                };
                (pk, pke)
            }
        }
    }
}

impl<K: LayeredNestable> key::Key for Layered<K> {
    type Context = Context;
    type Event = Event;
    type PressedKey = LayeredPressedKey<TapHoldKey<BaseKey>>;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        keymap_index: u16,
    ) -> (Self::PressedKey, key::PressedKeyEvents<Self::Event>) {
        let Layered(key) = self;
        let (passthrough_pk, pke) = <K as key::Key>::new_pressed_key(key, context, keymap_index);
        let pk = input::PressedKey {
            key: LayeredKey::Pass(passthrough_pk.key),
            keymap_index,
            pressed_key_state: LayeredPressedKeyState::<TapHoldKey<BaseKey>>(passthrough_pk),
        };
        (pk, pke)
    }
}

impl LayeredKey<TapHoldKey<BaseKey>> {
    /// Constructs a [Key] from the given [key::keyboard::Key].
    pub const fn keyboard(key: key::keyboard::Key) -> Self {
        Self::Pass(TapHoldKey::keyboard(key))
    }

    /// Constructs a [Key] from the given [key::tap_hold::Key].
    pub const fn tap_hold(key: key::tap_hold::Key<BaseKey>) -> Self {
        Self::Pass(TapHoldKey::tap_hold(key))
    }

    /// Constructs a [Key] from the given [key::layered::ModifierKey].
    pub const fn layer_modifier(key: key::layered::ModifierKey) -> Self {
        Self::Pass(TapHoldKey::layer_modifier(key))
    }
}

impl<K: LayeredNestable> LayeredKey<K> {
    /// Constructs a [Key] from the given [key::layered::LayeredKey].
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
    /// Constructs a [Key] from the given [key::tap_hold::Key].
    pub const fn tap_hold(key: key::tap_hold::Key<key::keyboard::Key>) -> Self {
        Self(TapHoldKey::TapHold(key))
    }
}

/// Aggregates the [key::PressedKeyState] types.
#[derive(Debug, PartialEq)]
pub struct LayeredPressedKeyState<K: LayeredNestable>(K::PressedKey);

/// Convenience type alias for a [key::PressedKey] with a layered key.
pub type LayeredPressedKey<K> = input::PressedKey<LayeredKey<K>, LayeredPressedKeyState<K>>;

impl<K: Copy + Into<LayeredKey<NK>>, NK: LayeredNestable> key::PressedKeyState<K>
    for LayeredPressedKeyState<NK>
{
    type Context = Context;
    type Event = Event;

    fn handle_event_for(
        &mut self,
        context: Context,
        _keymap_index: u16,
        _key: &K,
        event: key::Event<Event>,
    ) -> key::PressedKeyEvents<Event> {
        let LayeredPressedKeyState(pk) = self;
        pk.handle_event(context, event)
    }

    fn key_output(&self, _key: &K) -> key::KeyOutputState {
        let LayeredPressedKeyState(pk) = self;
        pk.key_output()
    }
}
