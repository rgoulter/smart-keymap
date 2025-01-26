//! This module implements the `keymap::Key` for a 'composite' key,
//!  which can be any of the other key definitions,
//!  and is the default Key for the `keymap::KeyMap` implementation.
#![doc = include_str!("doc_de_composite.md")]

use core::fmt::Debug;

#[cfg(feature = "std")]
use serde::Deserialize;

use crate::{input, key};
use key::{keyboard, layered, tap_hold};

/// Default [NestableKey] for [Key] and its associated types.
pub type DefaultNestableKey = keyboard::Key;

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
pub trait TapHoldNestable: key::Key {}

impl TapHoldNestable for layered::ModifierKey {}
impl TapHoldNestable for keyboard::Key {}
impl TapHoldNestable for BaseKey {}

/// An aggregate of [key::Key] types.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "std", derive(Deserialize))]
#[cfg_attr(feature = "std", serde(untagged))]
pub enum TapHoldKey<K: TapHoldNestable = BaseKey> {
    /// A tap-hold key.
    TapHold(tap_hold::Key<K>),
    /// A non-tap-hold key.
    Pass(K),
}

/// Trait for types which can be nested in [LayeredKey] variants.
pub trait LayeredNestable: key::Key {}

impl LayeredNestable for layered::ModifierKey {}
impl LayeredNestable for keyboard::Key {}
impl LayeredNestable for BaseKey {}
impl<K: TapHoldNestable> LayeredNestable for TapHoldKey<K>
where
    <K as key::Key>::Context: From<Context>,
    <K as key::Key>::Event: TryFrom<Event>,
    Event: From<<K as key::Key>::Event>,
    TapHoldKey<K>: From<K>,
    PressedTapHoldKeyState<K>: From<<K as key::Key>::PressedKeyState>,
{
}

/// An aggregate of [key::Key] types.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "std", derive(Deserialize))]
#[cfg_attr(feature = "std", serde(untagged))]
pub enum LayeredKey<K: LayeredNestable = TapHoldKey> {
    /// A layered key.
    Layered(layered::LayeredKey<K>),
    /// Non-layered,
    Pass(K),
}

impl key::Key for BaseKey {
    type Context = Context;
    type ContextEvent = Event;
    type Event = Event;
    type PressedKeyState = PressedBaseKeyState;

    fn new_pressed_key(
        &self,
        _context: Self::Context,
        keymap_index: u16,
    ) -> (
        input::PressedKey<Self, Self::PressedKeyState>,
        key::PressedKeyEvents<Self::Event>,
    ) {
        match self {
            BaseKey::Keyboard(key) => {
                let (pressed_key, events) = key.new_pressed_key((), keymap_index);
                (pressed_key.into_pressed_key(), events.into_events())
            }
            BaseKey::LayerModifier(key) => {
                let (pressed_key, events) = key::Key::new_pressed_key(key, (), keymap_index);
                (pressed_key.into_pressed_key(), events.into_events())
            }
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

impl<K: TapHoldNestable> key::Key for TapHoldKey<K>
where
    <K as key::Key>::Context: From<Context>,
    <K as key::Key>::Event: TryFrom<Event>,
    Event: From<<K as key::Key>::Event>,
    TapHoldKey<K>: From<K>,
    PressedTapHoldKeyState<K>: From<<K as key::Key>::PressedKeyState>,
{
    type Context = Context;
    type ContextEvent = Event;
    type Event = Event;
    type PressedKeyState = PressedTapHoldKeyState<K>;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        keymap_index: u16,
    ) -> (
        input::PressedKey<Self, Self::PressedKeyState>,
        key::PressedKeyEvents<Self::Event>,
    ) {
        match self {
            TapHoldKey::TapHold(key) => {
                let (pressed_key, events) = key.new_pressed_key(context.into(), keymap_index);
                (pressed_key.into_pressed_key(), events.into_events())
            }
            TapHoldKey::Pass(key) => {
                let (pressed_key, events) = key.new_pressed_key(context.into(), keymap_index);
                (pressed_key.into_pressed_key(), events.into_events())
            }
        }
    }
}

impl<K: TapHoldNestable> From<tap_hold::Key<K>> for TapHoldKey<K> {
    fn from(key: tap_hold::Key<K>) -> Self {
        TapHoldKey::TapHold(key)
    }
}

impl<K: Into<BaseKey>> From<K> for TapHoldKey {
    fn from(key: K) -> Self {
        TapHoldKey::Pass(key.into())
    }
}

impl<K: LayeredNestable> key::Key for LayeredKey<K>
where
    <K as key::Key>::Context: From<Context>,
    <K as key::Key>::Event: TryFrom<Event>,
    Event: From<<K as key::Key>::Event>,
    LayeredKey<K>: From<K>,
    PressedLayeredKeyState<K>: From<<K as key::Key>::PressedKeyState>,
{
    type Context = Context;
    type ContextEvent = Event;
    type Event = Event;
    type PressedKeyState = PressedLayeredKeyState<K>;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        keymap_index: u16,
    ) -> (
        input::PressedKey<Self, Self::PressedKeyState>,
        key::PressedKeyEvents<Self::Event>,
    ) {
        match self {
            LayeredKey::Layered(key) => {
                let (pressed_key, events) = key.new_pressed_key(context.into(), keymap_index);
                (pressed_key.into_pressed_key(), events.into_events())
            }
            LayeredKey::Pass(key) => {
                let (pressed_key, events) = key.new_pressed_key(context.into(), keymap_index);
                (pressed_key.into_pressed_key(), events.into_events())
            }
        }
    }
}

impl<K: LayeredNestable> From<layered::LayeredKey<K>> for LayeredKey<K> {
    fn from(key: layered::LayeredKey<K>) -> Self {
        LayeredKey::Layered(key)
    }
}

impl From<layered::ModifierKey> for LayeredKey<layered::ModifierKey> {
    fn from(key: layered::ModifierKey) -> Self {
        LayeredKey::Pass(key.into())
    }
}

impl From<keyboard::Key> for LayeredKey<keyboard::Key> {
    fn from(key: keyboard::Key) -> Self {
        LayeredKey::Pass(key.into())
    }
}

impl<K: Into<BaseKey>> From<K> for LayeredKey<BaseKey> {
    fn from(key: K) -> Self {
        LayeredKey::Pass(key.into())
    }
}

impl<K: Into<TapHoldKey>> From<K> for LayeredKey<TapHoldKey> {
    fn from(key: K) -> Self {
        LayeredKey::Pass(key.into())
    }
}

/// Type alias for composite key types.
///
/// Composite key is defined as a tree of key nodes:
///   Base    := LayerModifier | Keyboard
///   TapHold := TapHold<Base> | Base
///   Layered := Layered<TapHold> | TapHold
pub type Key = LayeredKey<TapHoldKey<BaseKey>>;

impl BaseKey {
    /// Constructs a [Key::Keyboard] from the given [keyboard::Key].
    pub const fn keyboard(key: keyboard::Key) -> Self {
        Self::Keyboard(key)
    }

    /// Constructs a [Key::LayerModifier] from the given [layered::ModifierKey].
    pub const fn layer_modifier(key: layered::ModifierKey) -> Self {
        Self::LayerModifier(key)
    }
}

impl TapHoldKey {
    /// Constructs a [Key::Keyboard] from the given [keyboard::Key].
    pub const fn keyboard(key: keyboard::Key) -> Self {
        Self::Pass(BaseKey::keyboard(key))
    }

    /// Constructs a [Key::TapHold] from the given [tap_hold::Key].
    pub const fn tap_hold(key: tap_hold::Key<BaseKey>) -> Self {
        Self::TapHold(key)
    }

    /// Constructs a [Key::LayerModifier] from the given [layered::ModifierKey].
    pub const fn layer_modifier(key: layered::ModifierKey) -> Self {
        Self::Pass(BaseKey::layer_modifier(key))
    }
}

impl Key {
    /// Constructs a [Key::Keyboard] from the given [keyboard::Key].
    pub const fn keyboard(key: keyboard::Key) -> Self {
        Self::Pass(TapHoldKey::keyboard(key))
    }

    /// Constructs a [Key::TapHold] from the given [tap_hold::Key].
    pub const fn tap_hold(key: tap_hold::Key<BaseKey>) -> Self {
        Self::Pass(TapHoldKey::tap_hold(key))
    }

    /// Constructs a [Key::LayerModifier] from the given [layered::ModifierKey].
    pub const fn layer_modifier(key: layered::ModifierKey) -> Self {
        Self::Pass(TapHoldKey::layer_modifier(key))
    }

    /// Constructs a [Key::Layered] from the given [layered::LayeredKey].
    pub const fn layered(key: layered::LayeredKey<TapHoldKey>) -> Self {
        Self::Layered(key)
    }
}

/// An aggregate context for [key::Context]s.
#[derive(Debug, Clone, Copy)]
pub struct Context {
    /// The layered key context.
    pub layer_context: layered::Context,
}

/// The default context.
pub const DEFAULT_CONTEXT: Context = Context {
    layer_context: layered::DEFAULT_CONTEXT,
};

impl Context {
    /// Constructs a new [Context].
    pub const fn new(layer_context: layered::Context) -> Self {
        Self { layer_context }
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

impl<MC, IC> From<Context> for key::ModifierKeyContext<MC, IC>
where
    MC: Copy + From<Context>,
    IC: Copy + From<Context>,
{
    fn from(ctx: Context) -> Self {
        key::ModifierKeyContext {
            context: ctx.into(),
            inner_context: ctx.into(),
        }
    }
}

/// Aggregates the [key::PressedKeyState] types.
#[derive(Debug)]
pub enum PressedBaseKeyState {
    /// A keyboard key's pressed state.
    Keyboard(keyboard::PressedKeyState),
    /// A layer modifier key's pressed state.
    LayerModifier(layered::PressedModifierKeyState),
}

/// Convenience type alias for a [key::PressedKey] with a base key.
pub type PressedBaseKey = input::PressedKey<BaseKey, PressedBaseKeyState>;

impl key::PressedKeyState<BaseKey> for PressedBaseKeyState {
    type Event = Event;

    fn handle_event_for(
        &mut self,
        context: Context,
        keymap_index: u16,
        key: &BaseKey,
        event: key::Event<Self::Event>,
    ) -> key::PressedKeyEvents<Self::Event> {
        match (key, self) {
            (BaseKey::LayerModifier(key), PressedBaseKeyState::LayerModifier(pks)) => {
                if let Ok(ev) = event.try_into_key_event(|e| e.try_into()) {
                    let events = pks.handle_event_for(context.into(), keymap_index, key, ev);
                    events.into_events()
                } else {
                    key::PressedKeyEvents::no_events()
                }
            }
            _ => key::PressedKeyEvents::no_events(),
        }
    }

    fn key_output(&self, key: &BaseKey) -> key::KeyOutputState {
        match (key, self) {
            (BaseKey::LayerModifier(key), PressedBaseKeyState::LayerModifier(pks)) => {
                pks.key_output(key)
            }
            (BaseKey::Keyboard(key), PressedBaseKeyState::Keyboard(pks)) => pks.key_output(key),
            _ => key::KeyOutputState::no_output(),
        }
    }
}

impl From<keyboard::PressedKeyState> for PressedBaseKeyState {
    fn from(pks: keyboard::PressedKeyState) -> Self {
        PressedBaseKeyState::Keyboard(pks)
    }
}

impl From<layered::PressedModifierKeyState> for PressedBaseKeyState {
    fn from(pks: layered::PressedModifierKeyState) -> Self {
        PressedBaseKeyState::LayerModifier(pks)
    }
}

/// Aggregates the [key::PressedKeyState] types.
#[derive(Debug)]
pub enum PressedTapHoldKeyState<K: TapHoldNestable = BaseKey> {
    /// A tap-hold key's pressed state.
    TapHold(tap_hold::PressedKeyState<K>),
    /// Passthrough state.
    Pass(<K as key::Key>::PressedKeyState),
}

/// Convenience type alias for a [key::PressedKey] with a taphold key.
pub type PressedTapHoldKey<K> = input::PressedKey<TapHoldKey, PressedTapHoldKeyState<K>>;

impl<K: TapHoldNestable> key::PressedKeyState<TapHoldKey<K>> for PressedTapHoldKeyState<K>
where
    <K as key::Key>::Context: From<Context>,
    <K as key::Key>::Event: TryFrom<Event>,
    Event: From<<K as key::Key>::Event>,
    TapHoldKey<K>: From<K>,
    PressedTapHoldKeyState<K>: From<<K as key::Key>::PressedKeyState>,
{
    type Event = Event;

    fn handle_event_for(
        &mut self,
        context: Context,
        keymap_index: u16,
        key: &TapHoldKey<K>,
        event: key::Event<Self::Event>,
    ) -> key::PressedKeyEvents<Self::Event> {
        match (key, self) {
            (TapHoldKey::TapHold(key), PressedTapHoldKeyState::TapHold(pks)) => {
                if let Ok(ev) = event.try_into_key_event(|event| event.try_into()) {
                    let events = pks.handle_event_for(context.into(), keymap_index, key, ev);
                    events.into_events()
                } else {
                    key::PressedKeyEvents::no_events()
                }
            }
            (TapHoldKey::Pass(key), PressedTapHoldKeyState::Pass(pks)) => {
                if let Ok(ev) = event.try_into_key_event(|event| {
                    event
                        .try_into()
                        .map_err(|_| key::EventError::UnmappableEvent)
                }) {
                    let events = pks.handle_event_for(context.into(), keymap_index, key, ev);
                    events.into_events()
                } else {
                    key::PressedKeyEvents::no_events()
                }
            }
            _ => key::PressedKeyEvents::no_events(),
        }
    }

    fn key_output(&self, key: &TapHoldKey<K>) -> key::KeyOutputState {
        match (key, self) {
            (TapHoldKey::TapHold(key), PressedTapHoldKeyState::TapHold(pks)) => pks.key_output(key),
            (TapHoldKey::Pass(key), PressedTapHoldKeyState::Pass(pks)) => pks.key_output(key),
            _ => key::KeyOutputState::no_output(),
        }
    }
}

impl<K: TapHoldNestable> From<tap_hold::PressedKeyState<K>> for PressedTapHoldKeyState<K> {
    fn from(pks: tap_hold::PressedKeyState<K>) -> Self {
        PressedTapHoldKeyState::TapHold(pks)
    }
}

impl<PKS: Into<PressedBaseKeyState>> From<PKS> for PressedTapHoldKeyState {
    fn from(pks: PKS) -> Self {
        PressedTapHoldKeyState::Pass(pks.into())
    }
}

/// Aggregates the [key::PressedKeyState] types.
#[derive(Debug)]
pub enum PressedLayeredKeyState<K: LayeredNestable = TapHoldKey> {
    /// A layer modifier key's pressed state.
    Layered(layered::PressedLayeredKeyState<K>),
    /// Passthrough state.
    Pass(<K as key::Key>::PressedKeyState),
}

/// Convenience type alias for a [key::PressedKey] with a layered key.
pub type PressedLayeredKey<K> = input::PressedKey<LayeredKey<K>, PressedLayeredKeyState<K>>;

impl<K: LayeredNestable> key::PressedKeyState<LayeredKey<K>> for PressedLayeredKeyState<K>
where
    <K as key::Key>::Context: From<Context>,
    <K as key::Key>::Event: TryFrom<Event>,
    Event: From<<K as key::Key>::Event>,
    LayeredKey<K>: From<K>,
    PressedLayeredKeyState<K>: From<<K as key::Key>::PressedKeyState>,
{
    type Event = Event;

    fn handle_event_for(
        &mut self,
        context: Context,
        keymap_index: u16,
        key: &LayeredKey<K>,
        event: key::Event<Self::Event>,
    ) -> key::PressedKeyEvents<Self::Event> {
        match (key, self) {
            (LayeredKey::Layered(key), PressedLayeredKeyState::Layered(pks)) => {
                if let Ok(ev) = event.try_into_key_event(|e| {
                    e.try_into().map_err(|_| key::EventError::UnmappableEvent)
                }) {
                    let events = pks.handle_event_for(context.into(), keymap_index, key, ev);
                    events.into_events()
                } else {
                    key::PressedKeyEvents::no_events()
                }
            }
            (LayeredKey::Pass(key), PressedLayeredKeyState::Pass(pks)) => {
                if let Ok(ev) = event.try_into_key_event(|event| {
                    event
                        .try_into()
                        .map_err(|_| key::EventError::UnmappableEvent)
                }) {
                    let events = pks.handle_event_for(context.into(), keymap_index, key, ev);
                    events.into_events()
                } else {
                    key::PressedKeyEvents::no_events()
                }
            }
            _ => key::PressedKeyEvents::no_events(),
        }
    }

    fn key_output(&self, key: &LayeredKey<K>) -> key::KeyOutputState {
        match (key, self) {
            (LayeredKey::Layered(key), PressedLayeredKeyState::Layered(pks)) => pks.key_output(key),
            (LayeredKey::Pass(key), PressedLayeredKeyState::Pass(pks)) => pks.key_output(key),
            _ => key::KeyOutputState::no_output(),
        }
    }
}

impl<K: LayeredNestable> From<layered::PressedLayeredKeyState<K>> for PressedLayeredKeyState<K> {
    fn from(pks: layered::PressedLayeredKeyState<K>) -> Self {
        PressedLayeredKeyState::Layered(pks)
    }
}

impl From<layered::PressedModifierKeyState> for PressedLayeredKeyState<layered::ModifierKey> {
    fn from(pks: layered::PressedModifierKeyState) -> Self {
        PressedLayeredKeyState::Pass(pks.into())
    }
}

impl From<keyboard::PressedKeyState> for PressedLayeredKeyState<keyboard::Key> {
    fn from(pks: keyboard::PressedKeyState) -> Self {
        PressedLayeredKeyState::Pass(pks.into())
    }
}

impl<PKS: Into<PressedBaseKeyState>> From<PKS> for PressedLayeredKeyState<BaseKey> {
    fn from(pks: PKS) -> Self {
        PressedLayeredKeyState::Pass(pks.into())
    }
}

impl<PKS: Into<PressedTapHoldKeyState>> From<PKS> for PressedLayeredKeyState<TapHoldKey> {
    fn from(pks: PKS) -> Self {
        PressedLayeredKeyState::Pass(pks.into())
    }
}

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

impl From<keyboard::Event> for Event {
    fn from(_ev: keyboard::Event) -> Self {
        panic!("key::keyboard never emits events")
    }
}

impl From<tap_hold::Event> for Event {
    fn from(ev: tap_hold::Event) -> Self {
        Event::TapHold(ev)
    }
}

impl<ME: Copy, IE: Copy> From<key::ModifierKeyEvent<ME, IE>> for Event
where
    Event: From<ME>,
    Event: From<IE>,
{
    fn from(ev: key::ModifierKeyEvent<ME, IE>) -> Self {
        match ev {
            key::ModifierKeyEvent::Modifier(key_event) => key_event.into(),
            key::ModifierKeyEvent::Inner(key_event) => key_event.into(),
        }
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

impl TryFrom<Event> for keyboard::Event {
    type Error = key::EventError;

    fn try_from(_ev: Event) -> Result<Self, Self::Error> {
        Err(key::EventError::UnmappableEvent)
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

impl<ME: Copy, IE: Copy> TryFrom<Event> for key::ModifierKeyEvent<ME, IE>
where
    ME: TryFrom<Event>,
    IE: TryFrom<Event>,
{
    type Error = key::EventError;

    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        let res: Result<ME, _> = ev.try_into();
        if let Ok(key_event) = res {
            Ok(key::ModifierKeyEvent::Modifier(key_event))
        } else {
            let res: Result<IE, _> = ev.try_into();
            if let Ok(key_event) = res {
                Ok(key::ModifierKeyEvent::Inner(key_event))
            } else {
                Err(key::EventError::UnmappableEvent)
            }
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
                assert_eq!(layer, 0);
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
        assert_eq!(actual_active_layers[0..1], expected_active_layers[0..1]);
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
        assert_eq!(actual_active_layers[0..1], expected_active_layers[0..1]);
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
        assert_eq!(actual_keycode.to_option(), expected_keycode);
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
        assert_eq!(actual_keycode.to_option(), expected_keycode);
    }
}
