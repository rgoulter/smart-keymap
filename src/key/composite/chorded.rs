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

impl ChordedNestable for Layered<TapHold<key::layered::ModifierKey>> {
    fn as_fat_key(self) -> LayeredKey<TapHoldKey<BaseKey>> {
        todo!() // TODO
    }
}
impl ChordedNestable for Layered<TapHold<key::keyboard::Key>> {
    fn as_fat_key(self) -> LayeredKey<TapHoldKey<BaseKey>> {
        todo!() // TODO
    }
}
impl ChordedNestable for Layered<TapHold<BaseKey>> {
    fn as_fat_key(self) -> LayeredKey<TapHoldKey<BaseKey>> {
        todo!() // TODO
    }
}
impl<K: TapHoldNestable> ChordedNestable for Layered<TapHoldKey<K>> {
    fn as_fat_key(self) -> LayeredKey<TapHoldKey<BaseKey>> {
        todo!() // TODO
    }
}
impl<K: LayeredNestable> ChordedNestable for LayeredKey<K> {
    fn as_fat_key(self) -> LayeredKey<TapHoldKey<BaseKey>> {
        todo!() // TODO
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
    Auxiliary(key::chorded::AuxiliaryKey<K>),
    /// Non-chorded,
    Pass(K),
}

/// Newtype for [ChordedNestable] keys so they can implement [key::Key] for [ChordedPressedKey].
#[derive(Debug, Clone, Copy)]
pub struct Chorded<K: ChordedNestable>(pub K);

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
            _ => todo!(), // TODO
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
        todo!() // TODO
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
        _keymap_index: u16,
        _key: &K,
        event: key::Event<Event>,
    ) -> key::PressedKeyEvents<Event> {
        todo!() // TODO
    }

    fn key_output(&self, _key: &K) -> key::KeyOutputState {
        todo!() // TODO
    }
}
