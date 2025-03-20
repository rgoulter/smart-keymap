use core::fmt::Debug;

#[cfg(feature = "std")]
use serde::Deserialize;

use crate::{input, key};

use super::{BaseKey, BasePressedKey, BasePressedKeyState, Context, Event};

/// Trait for types which can be nested in [TapHoldKey] variants.
pub trait TapHoldNestable:
    key::Key<Context = Context, Event = Event, PressedKey = BasePressedKey>
    + Copy
    + PartialEq
    + Into<BaseKey>
{
}

impl TapHoldNestable for key::layered::ModifierKey {}
impl TapHoldNestable for key::keyboard::Key {}
impl TapHoldNestable for BaseKey {}

/// An aggregate of [key::Key] types.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "std", derive(Deserialize))]
#[cfg_attr(feature = "std", serde(untagged))]
pub enum TapHoldKey<K: TapHoldNestable> {
    /// A tap-hold key.
    TapHold(key::tap_hold::Key<K>),
    /// A non-tap-hold key.
    Pass(K),
}

impl<K: TapHoldNestable> TapHoldKey<K> {
    /// Constructs a 'fat' key value from the given tap hold key.
    pub fn as_fat_key(self) -> TapHoldKey<BaseKey> {
        match self {
            TapHoldKey::TapHold(key) => TapHoldKey::TapHold(key.map_key(|k| k.into())),
            TapHoldKey::Pass(key) => TapHoldKey::Pass(key.into()),
        }
    }
}

/// Newtype for [TapHoldNestable] keys so they can implement [key::Key] for [TapHoldPressedKey].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TapHold<K: TapHoldNestable>(pub K);

impl<K: TapHoldNestable> TapHold<K> {
    /// Constructs a 'fat' key value from the given tap hold key.
    pub fn as_fat_key(self) -> TapHoldKey<BaseKey> {
        let TapHold(k) = self;
        TapHoldKey::Pass(k.into())
    }
}

impl<K: TapHoldNestable> key::Key for key::tap_hold::Key<K> {
    type Context = Context;
    type Event = Event;
    type PressedKey = TapHoldPressedKey<BaseKey>;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        keymap_index: u16,
    ) -> (Self::PressedKey, key::PressedKeyEvents<Self::Event>) {
        let (pks, sch_ev) = self.new_pressed_key(context.into(), keymap_index);
        let pk = TapHoldPressedKey {
            key: TapHoldKey::TapHold((*self).into_key()),
            keymap_index,
            pressed_key_state: TapHoldPressedKeyState::TapHold(pks).as_fat_key_state(),
        };
        let pke = key::PressedKeyEvents::scheduled_event(sch_ev.into_scheduled_event());
        (pk, pke)
    }
}

impl<K: TapHoldNestable> key::Key for TapHoldKey<K> {
    type Context = Context;
    type Event = Event;
    type PressedKey = TapHoldPressedKey<BaseKey>;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        keymap_index: u16,
    ) -> (Self::PressedKey, key::PressedKeyEvents<Self::Event>) {
        match self {
            TapHoldKey::TapHold(key) => {
                let (pressed_key, pke) = <key::tap_hold::Key<K> as key::Key>::new_pressed_key(
                    key,
                    context,
                    keymap_index,
                );
                (pressed_key.map_pressed_key(|k| k, |pks| pks), pke)
            }
            TapHoldKey::Pass(key) => {
                let (pk, pke) = <K as key::Key>::new_pressed_key(key, context, keymap_index);
                (pk.into_pressed_key(), pke.into_events())
            }
        }
    }
}

impl<K: TapHoldNestable> key::Key for TapHold<K> {
    type Context = Context;
    type Event = Event;
    type PressedKey = TapHoldPressedKey<BaseKey>;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        keymap_index: u16,
    ) -> (Self::PressedKey, key::PressedKeyEvents<Self::Event>) {
        let TapHold(key) = self;
        let (pk, pke) = <K as key::Key>::new_pressed_key(key, context, keymap_index);
        (pk.into_pressed_key(), pke)
    }
}

impl<K: TapHoldNestable> From<key::tap_hold::Key<K>> for TapHoldKey<K> {
    fn from(key: key::tap_hold::Key<K>) -> Self {
        TapHoldKey::TapHold(key)
    }
}

impl<K: Into<BaseKey>> From<K> for TapHoldKey<BaseKey> {
    fn from(key: K) -> Self {
        TapHoldKey::Pass(key.into())
    }
}

impl TapHoldKey<BaseKey> {
    /// Constructs a [TapHoldKey] from the given [key::keyboard::Key].
    pub const fn keyboard(key: key::keyboard::Key) -> Self {
        Self::Pass(BaseKey::keyboard(key))
    }

    /// Constructs a [TapHoldKey] from the given [key::tap_hold::Key].
    pub const fn tap_hold(key: key::tap_hold::Key<BaseKey>) -> Self {
        Self::TapHold(key)
    }

    /// Constructs a [TapHoldKey] from the given [key::layered::ModifierKey].
    pub const fn layer_modifier(key: key::layered::ModifierKey) -> Self {
        Self::Pass(BaseKey::layer_modifier(key))
    }
}

impl TapHold<key::keyboard::Key> {
    /// Constructs a [TapHold] newtype from the given key.
    pub const fn keyboard(key: key::keyboard::Key) -> Self {
        Self(key)
    }
}

impl TapHold<key::layered::ModifierKey> {
    /// Constructs a [TapHold] newtypefrom the given key.
    pub const fn layer_modifier(key: key::layered::ModifierKey) -> Self {
        Self(key)
    }
}

impl TapHold<BaseKey> {
    /// Constructs a [TapHold] newtype from the given key.
    pub const fn base_key(key: BaseKey) -> Self {
        Self(key)
    }
}

/// Aggregates the [key::PressedKeyState] types.
#[derive(Debug, PartialEq)]
pub enum TapHoldPressedKeyState<K: TapHoldNestable> {
    /// A tap-hold key's pressed state.
    TapHold(key::tap_hold::PressedKeyState<K>),
    /// Passthrough state.
    Pass(BasePressedKeyState),
}

impl<K: TapHoldNestable> TapHoldPressedKeyState<K> {
    /// Maps K to BaseKey.
    fn as_fat_key_state(self) -> TapHoldPressedKeyState<BaseKey> {
        match self {
            TapHoldPressedKeyState::TapHold(pks) => {
                TapHoldPressedKeyState::TapHold(pks.into_pressed_key())
            }
            TapHoldPressedKeyState::Pass(pks) => TapHoldPressedKeyState::Pass(pks),
        }
    }
}

/// Convenience type alias for a [key::PressedKey] with a taphold key.
pub type TapHoldPressedKey<K> = input::PressedKey<TapHoldKey<K>, TapHoldPressedKeyState<K>>;

impl<K: Copy + Into<TapHoldKey<NK>>, NK: TapHoldNestable> key::PressedKeyState<K>
    for TapHoldPressedKeyState<NK>
{
    type Context = Context;
    type Event = Event;

    fn handle_event_for(
        &mut self,
        context: Context,
        keymap_index: u16,
        key: &K,
        event: key::Event<Event>,
    ) -> key::PressedKeyEvents<Event> {
        let k: TapHoldKey<NK> = (*key).into();

        match (k, self) {
            (TapHoldKey::TapHold(key), TapHoldPressedKeyState::TapHold(pks)) => {
                let events = pks.handle_event_for(context, keymap_index, &key, event);
                events.into_events()
            }
            (TapHoldKey::Pass(key), TapHoldPressedKeyState::Pass(pks)) => {
                let k: BaseKey = key.into();
                let events = pks.handle_event_for(context, keymap_index, &k, event);
                events.into_events()
            }
            _ => key::PressedKeyEvents::no_events(),
        }
    }

    fn key_output(&self, key: &K) -> key::KeyOutputState {
        let k: TapHoldKey<BaseKey> = (*key).into().as_fat_key();

        match (k, self) {
            (TapHoldKey::TapHold(_), TapHoldPressedKeyState::TapHold(pks)) => pks.key_output(),
            (TapHoldKey::Pass(key), TapHoldPressedKeyState::Pass(pks)) => pks.key_output(&key),
            _ => key::KeyOutputState::no_output(),
        }
    }
}

impl<K: TapHoldNestable> From<key::tap_hold::PressedKeyState<K>> for TapHoldPressedKeyState<K> {
    fn from(pks: key::tap_hold::PressedKeyState<K>) -> Self {
        TapHoldPressedKeyState::TapHold(pks)
    }
}

impl<PKS: Into<BasePressedKeyState>> From<PKS> for TapHoldPressedKeyState<BaseKey> {
    fn from(pks: PKS) -> Self {
        TapHoldPressedKeyState::Pass(pks.into())
    }
}
