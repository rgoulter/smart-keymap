use core::fmt::Debug;

#[cfg(feature = "std")]
use serde::Deserialize;

use crate::{input, key};

use super::BaseKey;
use super::{Context, Event};
use super::{Layered, LayeredKey, LayeredNestable, LayeredPressedKey, LayeredPressedKeyState};
use super::{TapHold, TapHoldKey, TapHoldNestable};

use key::PressedKey as _;

/// Trait for types which can be nested in [ChordedKey] variants.
pub trait ChordedNestable:
    key::Key<Context = Context, Event = Event, PressedKey = LayeredPressedKey<TapHoldKey<BaseKey>>>
    + Copy
    + PartialEq
{
    /// Construct a 'full representation' of the nestable key.
    fn as_fat_key(self) -> LayeredKey<TapHoldKey<BaseKey>>;
}

impl<K: LayeredNestable> ChordedNestable for Layered<K> {
    fn as_fat_key(self) -> LayeredKey<TapHoldKey<BaseKey>> {
        self.as_fat_key()
    }
}
impl<K: LayeredNestable> ChordedNestable for LayeredKey<K> {
    fn as_fat_key(self) -> LayeredKey<TapHoldKey<BaseKey>> {
        self.as_fat_key()
    }
}

/// An aggregate of [key::Key] types.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "std", derive(Deserialize))]
#[cfg_attr(feature = "std", serde(untagged))]
pub enum ChordedKey<K: ChordedNestable> {
    /// A chorded key.
    Chorded(key::chorded::Key<K>),
    /// A chorded key.
    Auxiliary {
        /// The auxiliary chorded key.
        chorded: key::chorded::AuxiliaryKey<K>,
    },
    /// Non-chorded,
    Pass(K),
}

/// Newtype for [ChordedNestable] keys so they can implement [key::Key] for [ChordedPressedKey].
#[derive(Debug, Clone, Copy)]
pub struct Chorded<K: ChordedNestable>(pub K);

impl<K: ChordedNestable> key::Key for key::chorded::Key<K> {
    type Context = Context;
    type Event = Event;
    type PressedKey = ChordedPressedKey<LayeredKey<TapHoldKey<BaseKey>>>;

    fn new_pressed_key(
        &self,
        _context: Self::Context,
        keymap_index: u16,
    ) -> (Self::PressedKey, key::PressedKeyEvents<Self::Event>) {
        let fat_key = (*self).map_key(|k| k.as_fat_key());
        let pk = fat_key.new_pressed_key(keymap_index);
        let pke = key::PressedKeyEvents::no_events();
        (
            pk.map_pressed_key(
                |k| ChordedKey::Chorded(k),
                |pks| ChordedPressedKeyState::Chorded(pks),
            ),
            pke,
        )
    }
}

impl<K: ChordedNestable> key::Key for key::chorded::AuxiliaryKey<K> {
    type Context = Context;
    type Event = Event;
    type PressedKey = ChordedPressedKey<LayeredKey<TapHoldKey<BaseKey>>>;

    fn new_pressed_key(
        &self,
        _context: Self::Context,
        keymap_index: u16,
    ) -> (Self::PressedKey, key::PressedKeyEvents<Self::Event>) {
        let fat_key = (*self).map_key(|k| k.as_fat_key());
        let pk = fat_key.new_pressed_key(keymap_index);
        let pke = key::PressedKeyEvents::no_events();
        (
            pk.map_pressed_key(
                |k| ChordedKey::Auxiliary { chorded: k },
                |pks| ChordedPressedKeyState::Auxiliary(pks),
            ),
            pke,
        )
    }
}

impl<K: ChordedNestable> key::Key for ChordedKey<K> {
    type Context = Context;
    type Event = Event;
    type PressedKey = ChordedPressedKey<LayeredKey<TapHoldKey<BaseKey>>>;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        keymap_index: u16,
    ) -> (Self::PressedKey, key::PressedKeyEvents<Self::Event>) {
        match self {
            ChordedKey::Chorded(key) => key::Key::new_pressed_key(key, context, keymap_index),
            ChordedKey::Auxiliary { chorded } => {
                key::Key::new_pressed_key(chorded, context, keymap_index)
            }
            ChordedKey::Pass(key) => {
                let (passthrough_pk, pke) = key.new_pressed_key(context.into(), keymap_index);
                let pk = input::PressedKey {
                    key: ChordedKey::Pass(passthrough_pk.key),
                    keymap_index,
                    pressed_key_state:
                        ChordedPressedKeyState::<LayeredKey<TapHoldKey<BaseKey>>>::Pass(
                            passthrough_pk.pressed_key_state,
                        ),
                };
                (pk, pke)
            }
        }
    }
}

impl<K: ChordedNestable> ChordedKey<K> {
    /// Constructs a 'fat' key value from the given chorded key.
    pub fn as_fat_key(self) -> ChordedKey<LayeredKey<TapHoldKey<BaseKey>>> {
        match self {
            ChordedKey::Chorded(key) => ChordedKey::Chorded(key.map_key(|k| k.as_fat_key())),
            ChordedKey::Auxiliary { chorded } => ChordedKey::Auxiliary {
                chorded: chorded.map_key(|k| k.as_fat_key()),
            },
            ChordedKey::Pass(key) => ChordedKey::Pass(key.as_fat_key()),
        }
    }
}

impl<K: ChordedNestable> key::Key for Chorded<K> {
    type Context = Context;
    type Event = Event;
    type PressedKey = ChordedPressedKey<LayeredKey<TapHoldKey<BaseKey>>>;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        keymap_index: u16,
    ) -> (Self::PressedKey, key::PressedKeyEvents<Self::Event>) {
        let Chorded(key) = self;
        let (pk, pke) = <K as key::Key>::new_pressed_key(key, context, keymap_index);
        (
            pk.map_pressed_key(
                |k| ChordedKey::Pass(k.as_fat_key()),
                |pks| ChordedPressedKeyState::Pass(pks),
            ),
            pke,
        )
    }
}

impl ChordedKey<LayeredKey<TapHoldKey<BaseKey>>> {
    /// Constructs a [Key] from the given [key::keyboard::Key].
    pub const fn keyboard(key: key::keyboard::Key) -> Self {
        ChordedKey::Pass(LayeredKey::Pass(TapHoldKey::keyboard(key)))
    }

    /// Constructs a [Key] from the given [key::tap_hold::Key].
    pub const fn tap_hold(key: key::tap_hold::Key<BaseKey>) -> Self {
        ChordedKey::Pass(LayeredKey::Pass(TapHoldKey::tap_hold(key)))
    }

    /// Constructs a [Key] from the given [key::layered::ModifierKey].
    pub const fn layer_modifier(key: key::layered::ModifierKey) -> Self {
        ChordedKey::Pass(LayeredKey::Pass(TapHoldKey::layer_modifier(key)))
    }
}

impl<K: LayeredNestable> ChordedKey<LayeredKey<K>> {
    /// Constructs a [Key] from the given [key::layered::LayeredKey].
    pub const fn layered(key: key::layered::LayeredKey<K>) -> Self {
        ChordedKey::Pass(LayeredKey::Layered(key))
    }
}

/// Aggregates the [key::PressedKeyState] types.
#[derive(Debug, PartialEq)]
pub enum ChordedPressedKeyState<K: ChordedNestable> {
    /// A chorded key's pressed state.
    Chorded(key::chorded::PressedKeyState<K>),
    /// An auxiliary chorded key's pressed state.
    Auxiliary(key::chorded::PressedKeyState<K>),
    /// Passthrough state.
    Pass(LayeredPressedKeyState<TapHoldKey<BaseKey>>),
}

/// Convenience type alias for a [key::PressedKey] with a layered key.
pub type ChordedPressedKey<K> = input::PressedKey<ChordedKey<K>, ChordedPressedKeyState<K>>;

impl<K: Copy + Into<ChordedKey<NK>>, NK: ChordedNestable> key::PressedKeyState<K>
    for ChordedPressedKeyState<NK>
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
        let k: ChordedKey<NK> = (*key).into();

        match (k, self) {
            (ChordedKey::Chorded(key), ChordedPressedKeyState::Chorded(pks)) => {
                pks.handle_event_for(context, keymap_index, &key, event)
            }
            (ChordedKey::Auxiliary { chorded }, ChordedPressedKeyState::Auxiliary(pks)) => {
                pks.handle_event_for(context, keymap_index, &chorded, event)
            }
            (ChordedKey::Pass(key), ChordedPressedKeyState::Pass(pks)) => {
                if let Ok(ev) = event.try_into_key_event(|event| event.try_into()) {
                    let k: LayeredKey<TapHoldKey<BaseKey>> = key.as_fat_key();
                    let events = pks.handle_event_for(context.into(), keymap_index, &k, ev);
                    events.into_events()
                } else {
                    key::PressedKeyEvents::no_events()
                }
            }
            _ => key::PressedKeyEvents::no_events(),
        }
    }

    fn key_output(&self, key: &K) -> key::KeyOutputState {
        let k: ChordedKey<LayeredKey<TapHoldKey<BaseKey>>> = (*key).into().as_fat_key();

        match (k, self) {
            (ChordedKey::Chorded(_), ChordedPressedKeyState::Chorded(pks)) => pks.key_output(),
            (ChordedKey::Auxiliary { chorded: _ }, ChordedPressedKeyState::Auxiliary(pks)) => {
                pks.key_output()
            }
            (ChordedKey::Pass(key), ChordedPressedKeyState::Pass(pks)) => pks.key_output(&key),
            _ => key::KeyOutputState::no_output(),
        }
    }
}
