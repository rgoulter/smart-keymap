//! This module implements the `keymap::Key` for a 'composite' key,
//!  which can be any of the other key definitions,
//!  and is the default Key for the `keymap::KeyMap` implementation.
#![doc = include_str!("doc_de_composite.md")]

use core::fmt::Debug;

#[cfg(feature = "std")]
use serde::Deserialize;

use crate::{input, key};
use key::keyboard;
use key::layered;
use key::tap_hold;

use key::PressedKey as _;

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

/// Trait for types which can be nested in [TapHoldKey] variants.
pub trait TapHoldNestable:
    key::Key<Context = Context, Event = Event, PressedKey = BasePressedKey>
    + Copy
    + PartialEq
    + Into<BaseKey>
{
}

impl TapHoldNestable for layered::ModifierKey {}
impl TapHoldNestable for keyboard::Key {}
impl TapHoldNestable for BaseKey {}

/// An aggregate of [key::Key] types.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "std", derive(Deserialize))]
#[cfg_attr(feature = "std", serde(untagged))]
pub enum TapHoldKey<K: TapHoldNestable> {
    /// A tap-hold key.
    TapHold(tap_hold::Key<K>),
    /// A non-tap-hold key.
    Pass(K),
}

impl<K: TapHoldNestable> TapHoldKey<K> {
    /// Maps K to BaseKey.
    fn as_fat_key(self) -> TapHoldKey<BaseKey> {
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
    /// Maps K to BaseKey.
    fn as_fat_key(self) -> TapHoldKey<BaseKey> {
        let TapHold(k) = self;
        TapHoldKey::Pass(k.into())
    }
}

/// Trait for types which can be nested in [LayeredKey] variants.
pub trait LayeredNestable:
    key::Key<Context = Context, Event = Event, PressedKey = TapHoldPressedKey<BaseKey>>
    + Copy
    + PartialEq
{
    /// Construct a 'full representation' of the nestable key.
    fn as_fat_key(self) -> TapHoldKey<BaseKey>;
}

impl LayeredNestable for TapHold<layered::ModifierKey> {
    fn as_fat_key(self) -> TapHoldKey<BaseKey> {
        TapHold::as_fat_key(self)
    }
}
impl LayeredNestable for TapHold<keyboard::Key> {
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
    Layered(layered::LayeredKey<K>),
    /// Non-layered,
    Pass(K),
}

/// Newtype for [LayeredNestable] keys so they can implement [key::Key] for [LayeredPressedKey].
#[derive(Debug, Clone, Copy)]
pub struct Layered<K: LayeredNestable>(pub K);

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

impl<K: TapHoldNestable> key::Key for tap_hold::Key<K> {
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
                let (pressed_key, pke) =
                    <tap_hold::Key<K> as key::Key>::new_pressed_key(key, context, keymap_index);
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
        let (pressed_key, events) = <K as key::Key>::new_pressed_key(key, context, keymap_index);
        (pressed_key.into_pressed_key(), events)
    }
}

impl<K: TapHoldNestable> From<tap_hold::Key<K>> for TapHoldKey<K> {
    fn from(key: tap_hold::Key<K>) -> Self {
        TapHoldKey::TapHold(key)
    }
}

impl<K: Into<BaseKey>> From<K> for TapHoldKey<BaseKey> {
    fn from(key: K) -> Self {
        TapHoldKey::Pass(key.into())
    }
}

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
                let (pk, events) = key.new_pressed_key(context.into(), keymap_index);
                let lpk = input::PressedKey {
                    key: LayeredKey::Pass(pk.key),
                    keymap_index,
                    pressed_key_state: LayeredPressedKeyState::<TapHoldKey<BaseKey>>(pk),
                };
                (lpk, events)
            }
            LayeredKey::Pass(key) => {
                let (pk, events) = key.new_pressed_key(context.into(), keymap_index);
                let lpk = input::PressedKey {
                    key: LayeredKey::Pass(pk.key),
                    keymap_index,
                    pressed_key_state: LayeredPressedKeyState::<TapHoldKey<BaseKey>>(pk),
                };
                (lpk, events)
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
        let (pk, events) = <K as key::Key>::new_pressed_key(key, context, keymap_index);
        let lpk = input::PressedKey {
            key: LayeredKey::Pass(pk.key),
            keymap_index,
            pressed_key_state: LayeredPressedKeyState::<TapHoldKey<BaseKey>>(pk),
        };
        (lpk, events)
    }
}

/// Type alias for composite key types.
///
/// Composite key is defined as a tree of key nodes:
///
///   ```text
///   Base    := LayerModifier | Keyboard
///
///   TapHold := TapHold<Base> | Base
///
///   Layered := Layered<TapHold> | TapHold
///   ```
pub type Key = LayeredKey<TapHoldKey<BaseKey>>;

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

impl TapHoldKey<BaseKey> {
    /// Constructs a [TapHoldKey] from the given [keyboard::Key].
    pub const fn keyboard(key: keyboard::Key) -> Self {
        Self::Pass(BaseKey::keyboard(key))
    }

    /// Constructs a [TapHoldKey] from the given [tap_hold::Key].
    pub const fn tap_hold(key: tap_hold::Key<BaseKey>) -> Self {
        Self::TapHold(key)
    }

    /// Constructs a [TapHoldKey] from the given [layered::ModifierKey].
    pub const fn layer_modifier(key: layered::ModifierKey) -> Self {
        Self::Pass(BaseKey::layer_modifier(key))
    }
}

impl TapHold<keyboard::Key> {
    /// Constructs a [TapHold] newtype from the given key.
    pub const fn keyboard(key: keyboard::Key) -> Self {
        Self(key)
    }
}

impl TapHold<layered::ModifierKey> {
    /// Constructs a [TapHold] newtypefrom the given key.
    pub const fn layer_modifier(key: layered::ModifierKey) -> Self {
        Self(key)
    }
}

impl TapHold<BaseKey> {
    /// Constructs a [TapHold] newtype from the given key.
    pub const fn base_key(key: BaseKey) -> Self {
        Self(key)
    }
}

impl LayeredKey<TapHoldKey<BaseKey>> {
    /// Constructs a [Key] from the given [keyboard::Key].
    pub const fn keyboard(key: keyboard::Key) -> Self {
        Self::Pass(TapHoldKey::keyboard(key))
    }

    /// Constructs a [Key] from the given [tap_hold::Key].
    pub const fn tap_hold(key: tap_hold::Key<BaseKey>) -> Self {
        Self::Pass(TapHoldKey::tap_hold(key))
    }

    /// Constructs a [Key] from the given [layered::ModifierKey].
    pub const fn layer_modifier(key: layered::ModifierKey) -> Self {
        Self::Pass(TapHoldKey::layer_modifier(key))
    }
}

impl<K: LayeredNestable> LayeredKey<K> {
    /// Constructs a [Key] from the given [layered::LayeredKey].
    pub const fn layered(key: layered::LayeredKey<K>) -> Self {
        Self::Layered(key)
    }
}

impl Layered<TapHold<keyboard::Key>> {
    /// Constructs a [Layered] newtype from the given key.
    pub const fn keyboard(key: keyboard::Key) -> Self {
        Self(TapHold(key))
    }
}

impl Layered<TapHoldKey<keyboard::Key>> {
    /// Constructs a [Key] from the given [tap_hold::Key].
    pub const fn tap_hold(key: tap_hold::Key<keyboard::Key>) -> Self {
        Self(TapHoldKey::TapHold(key))
    }
}

/// Config used for constructing initial context
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "std", derive(Deserialize))]
pub struct Config {
    /// The tap hold configuration.
    #[cfg_attr(feature = "std", serde(default))]
    pub tap_hold: tap_hold::Config,
}

/// The default config.
pub const DEFAULT_CONFIG: Config = Config {
    tap_hold: tap_hold::DEFAULT_CONFIG,
};

/// An aggregate context for [key::Context]s.
#[derive(Debug, Clone, Copy)]
pub struct Context {
    /// The layered key context.
    pub layer_context: layered::Context,
    /// The tap hold key context.
    pub tap_hold_context: tap_hold::Context,
}

/// The default context.
pub const DEFAULT_CONTEXT: Context = Context {
    layer_context: layered::DEFAULT_CONTEXT,
    tap_hold_context: tap_hold::DEFAULT_CONTEXT,
};

impl Context {
    /// Constructs a [Context] from the given [Config].
    pub const fn from_config(config: Config) -> Self {
        Self {
            layer_context: layered::DEFAULT_CONTEXT,
            tap_hold_context: tap_hold::Context::from_config(config.tap_hold),
        }
    }
}

impl Default for Context {
    /// Returns the default context.
    fn default() -> Self {
        DEFAULT_CONTEXT
    }
}

impl key::Context for Context {
    type Event = Event;
    fn handle_event(&mut self, event: Self::Event) {
        if let Event::LayerModification(ev) = event {
            self.layer_context.handle_event(ev);
        }
    }
}

/// keyboard::Context from composite::Context
impl From<Context> for () {
    fn from(_: Context) -> Self {}
}

impl From<Context> for layered::Context {
    fn from(ctx: Context) -> Self {
        ctx.layer_context
    }
}

impl From<Context> for tap_hold::Context {
    fn from(ctx: Context) -> Self {
        ctx.tap_hold_context
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

/// Aggregates the [key::PressedKeyState] types.
#[derive(Debug, PartialEq)]
pub enum TapHoldPressedKeyState<K: TapHoldNestable> {
    /// A tap-hold key's pressed state.
    TapHold(tap_hold::PressedKeyState<K>),
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
            TapHoldPressedKeyState::Pass(pks) => TapHoldPressedKeyState::Pass(pks.into()),
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
                if let Ok(ev) = event.try_into_key_event(|event| event.try_into()) {
                    let events = pks.handle_event_for(context.into(), keymap_index, &key, ev);
                    events.into_events()
                } else {
                    key::PressedKeyEvents::no_events()
                }
            }
            (TapHoldKey::Pass(key), TapHoldPressedKeyState::Pass(pks)) => {
                let k: BaseKey = key.into();

                if let Ok(ev) = event.try_into_key_event(|event| {
                    event
                        .try_into()
                        .map_err(|_| key::EventError::UnmappableEvent)
                }) {
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
        let k: TapHoldKey<BaseKey> = (*key).into().as_fat_key();

        match (k, self) {
            (TapHoldKey::TapHold(_), TapHoldPressedKeyState::TapHold(pks)) => pks.key_output(),
            (TapHoldKey::Pass(key), TapHoldPressedKeyState::Pass(pks)) => pks.key_output(&key),
            _ => key::KeyOutputState::no_output(),
        }
    }
}

impl<K: TapHoldNestable> From<tap_hold::PressedKeyState<K>> for TapHoldPressedKeyState<K> {
    fn from(pks: tap_hold::PressedKeyState<K>) -> Self {
        TapHoldPressedKeyState::TapHold(pks)
    }
}

impl<PKS: Into<BasePressedKeyState>> From<PKS> for TapHoldPressedKeyState<BaseKey> {
    fn from(pks: PKS) -> Self {
        TapHoldPressedKeyState::Pass(pks.into())
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

/// Convenience type alias for the 'highest' composite key.
pub type PressedKey = LayeredPressedKey<TapHoldKey<BaseKey>>;

/// Sum type aggregating the [key::Event] types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// A tap-hold event.
    TapHold(tap_hold::Event),
    /// A layer modification event.
    LayerModification(layered::LayerEvent),
}

impl From<layered::LayerEvent> for Event {
    fn from(ev: layered::LayerEvent) -> Self {
        Event::LayerModification(ev)
    }
}

impl From<tap_hold::Event> for Event {
    fn from(ev: tap_hold::Event) -> Self {
        Event::TapHold(ev)
    }
}

impl TryFrom<Event> for layered::LayerEvent {
    type Error = key::EventError;

    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        match ev {
            Event::LayerModification(ev) => Ok(ev),
            _ => Err(key::EventError::UnmappableEvent),
        }
    }
}

impl TryFrom<Event> for tap_hold::Event {
    type Error = key::EventError;

    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        match ev {
            Event::TapHold(ev) => Ok(ev),
            _ => Err(key::EventError::UnmappableEvent),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_composite_pressedkey_layerpressedmodifier_handles_release_event() {
        use crate::input;
        use key::{composite, layered, Key, PressedKey};

        // Assemble
        type Ctx = composite::Context;
        type K = composite::Key;
        let keymap_index: u16 = 0;
        let key = K::layer_modifier(layered::ModifierKey::Hold(0));
        let context: Ctx = DEFAULT_CONTEXT;
        let (mut pressed_lmod_key, _) = key.new_pressed_key(context, keymap_index);

        // Act
        let events = pressed_lmod_key.handle_event(
            context,
            key::Event::Input(input::Event::Release { keymap_index }),
        );

        // Assert
        let _key_ev = match events.into_iter().next().map(|sch_ev| sch_ev.event) {
            Some(key::Event::Key {
                key_event: Event::LayerModification(layered::LayerEvent::LayerDeactivated(layer)),
                ..
            }) => {
                assert_eq!(0, layer);
            }
            _ => panic!("Expected an Event::Key(LayerModification(LayerDeactivated(layer)))"),
        };
    }

    #[test]
    fn test_composite_context_updates_with_composite_layermodifier_press_event() {
        use key::{composite, keyboard, layered, Context, Key};

        // Assemble
        type Ctx = composite::Context;
        type K = composite::Key;
        let keys: [K; 2] = [
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                keyboard::Key::new(0x04).into(),
                [Some(keyboard::Key::new(0x05).into())],
            )),
        ];
        let mut context: Ctx = DEFAULT_CONTEXT;
        let (_pressed_key, pressed_key_events) = keys[0].new_pressed_key(context, 0);
        let maybe_ev = pressed_key_events.into_iter().next();

        // Act
        let event = match maybe_ev {
            Some(key::ScheduledEvent {
                event: key::Event::Key { key_event, .. },
                ..
            }) => key_event,
            _ => panic!("Expected Some(ScheduledEvent(Event::Key(_)))"),
        };
        context.handle_event(event);
        let actual_active_layers = context.layer_context.layer_state();

        // Assert
        let expected_active_layers = &[true];
        assert_eq!(expected_active_layers[0..1], actual_active_layers[0..1]);
    }

    #[test]
    fn test_composite_context_updates_with_composite_layerpressedmodifier_release_event() {
        use crate::input;
        use key::{composite, keyboard, layered, Context, Key, PressedKey};

        // Assemble
        type Ctx = composite::Context;
        type K = composite::Key;
        let keys: [K; 2] = [
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                keyboard::Key::new(0x04).into(),
                [Some(keyboard::Key::new(0x05).into())],
            )),
        ];
        let mut context: Ctx = DEFAULT_CONTEXT;
        let (mut pressed_lmod_key, _) = keys[0].new_pressed_key(context, 0);
        context.layer_context.activate_layer(0);
        let events = pressed_lmod_key.handle_event(
            context,
            key::Event::Input(input::Event::Release { keymap_index: 0 }),
        );
        let key_ev = match events.into_iter().next().map(|sch_ev| sch_ev.event) {
            Some(key::Event::Key { key_event, .. }) => key_event,
            _ => panic!("Expected an Event::Key(_)"),
        };

        // Act
        context.handle_event(key_ev);
        let actual_active_layers = context.layer_context.layer_state();

        // Assert
        let expected_active_layers = &[false];
        assert_eq!(expected_active_layers[0..1], actual_active_layers[0..1]);
    }

    #[test]
    fn test_composite_keyboard_pressed_key_has_key_code_for_composite_keyboard_key_def() {
        use key::{composite, keyboard, layered, Key, PressedKey};

        // Assemble
        type Ctx = composite::Context;
        type K = composite::Key;
        let keys: [K; 3] = [
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                keyboard::Key::new(0x04).into(),
                [Some(keyboard::Key::new(0x05).into())],
            )),
            K::keyboard(keyboard::Key::new(0x06)),
        ];
        let context: Ctx = DEFAULT_CONTEXT;

        // Act
        let keymap_index: u16 = 2;
        let (pressed_key, _) = keys[keymap_index as usize].new_pressed_key(context, keymap_index);
        let actual_keycode = pressed_key.key_output();

        // Assert
        let expected_keycode = Some(key::KeyOutput::from_key_code(0x06));
        assert_eq!(expected_keycode, actual_keycode.to_option());
    }

    #[test]
    fn test_composite_keyboard_pressed_key_has_key_code_for_composite_layered_key_def() {
        use key::{composite, keyboard, layered, Key, PressedKey};

        // Assemble
        type Ctx = composite::Context;
        type K = composite::Key;
        let keys: [K; 3] = [
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                keyboard::Key::new(0x04).into(),
                [Some(keyboard::Key::new(0x05).into())],
            )),
            K::keyboard(keyboard::Key::new(0x06)),
        ];
        let context: Ctx = DEFAULT_CONTEXT;

        // Act
        let keymap_index: u16 = 1;
        let (pressed_key, _) = keys[keymap_index as usize].new_pressed_key(context, keymap_index);
        let actual_keycode = pressed_key.key_output();

        // Assert
        let expected_keycode = Some(key::KeyOutput::from_key_code(0x04));
        assert_eq!(expected_keycode, actual_keycode.to_option());
    }
}
