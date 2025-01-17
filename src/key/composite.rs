//! This module implements the `keymap::Key` for a 'composite' key,
//!  which can be any of the other key definitions,
//!  and is the default Key for the `keymap::KeyMap` implementation.
#![doc = include_str!("doc_de_composite.md")]

use core::fmt::Debug;
use core::marker::PhantomData;

use serde::Deserialize;

use crate::{input, key};
use key::{layered, simple, tap_hold};

/// Used to implement nested combinations of [Key].
pub trait NestableKey: key::Key + Sized {
    /// Get the context for the nestable key from the given Context
    fn pluck_context<L: layered::LayerImpl>(context: Context<L>) -> <Self as key::Key>::Context;
    /// Constructs an [Event] for the Nestable key's event.
    fn into_event(event: <Self as key::Key>::Event) -> Event;
    /// Tries to construct the [key::Event] for the Nestable Key's event.
    fn try_event_from(event: Event) -> Result<<Self as key::Key>::Event, key::EventError>;
}

macro_rules! impl_nestable_key {
    ($key_type:path) => {
        impl NestableKey for $key_type {
            fn pluck_context<L: layered::LayerImpl>(
                context: Context<L>,
            ) -> <Self as key::Key>::Context {
                context.into()
            }

            fn into_event(event: <Self as crate::key::Key>::Event) -> Event {
                event.into()
            }

            fn try_event_from(
                event: Event,
            ) -> Result<<Self as crate::key::Key>::Event, crate::key::EventError> {
                event.try_into()
            }
        }
    };
}

impl_nestable_key!(simple::Key);
impl_nestable_key!(tap_hold::Key<simple::Key>);
impl_nestable_key!(layered::ModifierKey);

/// Related types used by [Key], [Context] and [Event].
pub trait CompositeTypes: Copy + Debug + PartialEq
where
    // The LayerImpl must be deserializable (and not contain references).
    <<Self as CompositeTypes>::L as layered::LayerImpl>::Layers<Self::NK>:
        serde::de::DeserializeOwned,
    // The NestedKey must be deserializable (and not contain references).
    <Self as CompositeTypes>::NK: serde::de::DeserializeOwned,
{
    /// The nested key type used within composite keys.
    type NK: NestableKey;
    /// The layer impl. used within composite keys.
    type L: layered::LayerImpl;
}

/// Struct to use as an impl of [CompositeImpl].
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct CompositeImpl<
    NK: NestableKey = DefaultNestableKey,
    L: layered::LayerImpl = layered::ArrayImpl<0>,
>(PhantomData<(NK, L)>);

impl<NK: NestableKey, L: layered::LayerImpl> CompositeTypes for CompositeImpl<NK, L>
where
    <L as layered::LayerImpl>::Layers<NK>: serde::de::DeserializeOwned,
    NK: serde::de::DeserializeOwned,
    // <NK as key::Key>::Context: From<Context<Self>>,
{
    type NK = NK;
    type L = L;
}

/// Default [NestableKey] for [Key] and its associated types.
pub type DefaultNestableKey = simple::Key;

/// An aggregate of [key::Key] types.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum Key<T: CompositeTypes = CompositeImpl> {
    /// A simple key.
    Simple {
        /// The simple key.
        key: simple::Key,
    },
    /// A tap-hold key.
    TapHold {
        /// The tap-hold key.
        key: tap_hold::Key<T::NK>,
    },
    /// A layer modifier key.
    LayerModifier {
        /// The layer modifier key.
        key: layered::ModifierKey,
    },
    /// A layered key.
    Layered {
        /// The layered key.
        key: layered::LayeredKey<T::NK, T::L>,
    },
}

impl<T: CompositeTypes> Key<T> {
    /// Constructs a [Key::Simple] from the given [simple::Key].
    pub const fn simple(key: simple::Key) -> Self {
        Self::Simple { key }
    }

    /// Constructs a [Key::TapHold] from the given [tap_hold::Key].
    pub const fn tap_hold(key: tap_hold::Key<T::NK>) -> Self {
        Self::TapHold { key }
    }

    /// Constructs a [Key::LayerModifier] from the given [layered::ModifierKey].
    pub const fn layer_modifier(key: layered::ModifierKey) -> Self {
        Self::LayerModifier { key }
    }

    /// Constructs a [Key::Layered] from the given [layered::LayeredKey].
    pub const fn layered(key: layered::LayeredKey<T::NK, T::L>) -> Self {
        Self::Layered { key }
    }
}

impl<T: CompositeTypes> key::Key for Key<T>
where
    Event: From<<T::NK as key::Key>::Event>,
    <T::NK as key::Key>::Event: TryFrom<Event, Error = key::EventError>,
{
    type Context = Context<T::L>;
    type ContextEvent = Event;
    type Event = Event;
    type PressedKeyState = PressedKeyState<T>;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        keymap_index: u16,
    ) -> (
        input::PressedKey<Self, Self::PressedKeyState>,
        key::PressedKeyEvents<Self::Event>,
    ) {
        match self {
            Key::Simple { key, .. } => {
                let (pressed_key, events) = key.new_pressed_key((), keymap_index);
                (pressed_key.into(), events.into_events())
            }
            Key::TapHold { key, .. } => {
                let taphold_context = context.into();
                let inner_context = T::NK::pluck_context(context);
                let modifier_key_context = key::ModifierKeyContext {
                    context: taphold_context,
                    inner_context,
                };
                let (pressed_key, events) = key.new_pressed_key(modifier_key_context, keymap_index);
                (
                    pressed_key.into(),
                    events.map_events(|mke| {
                        mke.map_events(|th_e| th_e.into(), |nk_e| T::NK::into_event(nk_e))
                    }),
                )
            }
            Key::LayerModifier { key, .. } => {
                let (pressed_key, events) = key::Key::new_pressed_key(key, (), keymap_index);
                (pressed_key.into(), events.into_events())
            }
            Key::Layered { key, .. } => {
                let layered_context = context.into();
                let inner_context = T::NK::pluck_context(context);
                let modifier_key_context = key::ModifierKeyContext {
                    context: layered_context,
                    inner_context,
                };
                let (pressed_key, events) = key.new_pressed_key(modifier_key_context, keymap_index);
                (pressed_key.into(), events.map_events(T::NK::into_event))
            }
        }
    }
}

/// An aggregate context for [key::Context]s.
#[derive(Debug, Clone, Copy)]
pub struct Context<L: layered::LayerImpl = layered::ArrayImpl<0>> {
    /// The layered key context.
    pub layer_context: layered::Context<L>,
}

impl<L: layered::LayerImpl> Context<L> {
    /// Constructs a new [Context].
    pub const fn new(layer_context: layered::Context<L>) -> Self {
        Self { layer_context }
    }
}

impl<L: layered::LayerImpl> key::Context for Context<L> {
    type Event = Event;
    fn handle_event(&mut self, event: Self::Event) {
        if let Event::LayerModification(ev) = event {
            self.layer_context.handle_event(ev);
        }
    }
}

/// simple::Context from composite::Context
impl<L: layered::LayerImpl> From<Context<L>> for () {
    fn from(_: Context<L>) -> Self {}
}

impl<L: layered::LayerImpl> From<Context<L>> for layered::Context<L> {
    fn from(ctx: Context<L>) -> Self {
        ctx.layer_context
    }
}

impl<L: layered::LayerImpl, MC, IC> From<Context<L>> for key::ModifierKeyContext<MC, IC>
where
    MC: From<Context<L>>,
    IC: From<Context<L>>,
{
    fn from(ctx: Context<L>) -> Self {
        key::ModifierKeyContext {
            context: ctx.into(),
            inner_context: ctx.into(),
        }
    }
}

/// Aggregates the [key::PressedKeyState] types.
#[derive(Debug)]
pub enum PressedKeyState<T: CompositeTypes> {
    /// A simple key's pressed state.
    Simple(simple::PressedKeyState),
    /// A tap-hold key's pressed state.
    TapHold(tap_hold::PressedKeyState<T::NK>),
    /// A layer modifier key's pressed state.
    LayerModifier(layered::PressedModifierKeyState),
    /// A layer modifier key's pressed state.
    Layered(layered::PressedLayeredKeyState<T::NK, T::L>),
}

/// Convenience type alias for a [key::PressedKey] with a [PressedKeyState].
pub type PressedKey<T> = input::PressedKey<Key<T>, PressedKeyState<T>>;

impl<T: CompositeTypes> From<layered::PressedModifierKey> for PressedKey<T> {
    fn from(
        input::PressedKey {
            keymap_index,
            key,
            pressed_key_state,
        }: layered::PressedModifierKey,
    ) -> Self {
        input::PressedKey {
            key: Key::layer_modifier(key),
            keymap_index,
            pressed_key_state: PressedKeyState::LayerModifier(pressed_key_state),
        }
    }
}

impl<T: CompositeTypes> From<layered::PressedLayeredKey<T::NK, T::L>> for PressedKey<T> {
    fn from(
        input::PressedKey {
            keymap_index,
            key,
            pressed_key_state,
        }: layered::PressedLayeredKey<T::NK, T::L>,
    ) -> Self {
        input::PressedKey {
            key: Key::layered(key),
            keymap_index,
            pressed_key_state: PressedKeyState::Layered(pressed_key_state),
        }
    }
}

impl<T: CompositeTypes> From<simple::PressedKey> for PressedKey<T> {
    fn from(
        input::PressedKey {
            keymap_index,
            key,
            pressed_key_state,
        }: simple::PressedKey,
    ) -> Self {
        input::PressedKey {
            key: Key::simple(key),
            keymap_index,
            pressed_key_state: PressedKeyState::Simple(pressed_key_state),
        }
    }
}

impl<T: CompositeTypes> From<tap_hold::PressedKey<T::NK>> for PressedKey<T> {
    fn from(
        input::PressedKey {
            keymap_index,
            key,
            pressed_key_state,
        }: tap_hold::PressedKey<T::NK>,
    ) -> Self {
        input::PressedKey {
            key: Key::tap_hold(key),
            keymap_index,
            pressed_key_state: PressedKeyState::TapHold(pressed_key_state),
        }
    }
}

impl<T: CompositeTypes> key::PressedKeyState<Key<T>> for PressedKeyState<T>
where
    Event: From<<T::NK as key::Key>::Event>,
    <T::NK as key::Key>::Event: TryFrom<Event, Error = key::EventError>,
{
    type Event = Event;

    fn handle_event_for(
        &mut self,
        context: Context<T::L>,
        keymap_index: u16,
        key: &Key<T>,
        event: key::Event<Self::Event>,
    ) -> key::PressedKeyEvents<Self::Event> {
        match (key, self) {
            (Key::TapHold { key, .. }, PressedKeyState::TapHold(pks)) => {
                if let Ok(ev) = event.try_into_key_event(|e| e.try_into()) {
                    let tap_hold_context = context.into();
                    let inner_context = T::NK::pluck_context(context);
                    let modifier_key_context = key::ModifierKeyContext {
                        context: tap_hold_context,
                        inner_context,
                    };
                    let events = pks.handle_event_for(modifier_key_context, keymap_index, key, ev);
                    events.into_events()
                } else {
                    key::PressedKeyEvents::no_events()
                }
            }
            (Key::LayerModifier { key, .. }, PressedKeyState::LayerModifier(pks)) => {
                if let Ok(ev) = event.try_into_key_event(|e| e.try_into()) {
                    let events = pks.handle_event_for(context.into(), keymap_index, key, ev);
                    events.into_events()
                } else {
                    key::PressedKeyEvents::no_events()
                }
            }
            (Key::Layered { key, .. }, PressedKeyState::Layered(pks)) => {
                if let Ok(ev) = event.try_into_key_event(T::NK::try_event_from) {
                    let layered_context = context.into();
                    let inner_context = T::NK::pluck_context(context);
                    let modifier_key_context = key::ModifierKeyContext {
                        context: layered_context,
                        inner_context,
                    };
                    let events = pks.handle_event_for(modifier_key_context, keymap_index, key, ev);
                    events.map_events(T::NK::into_event)
                } else {
                    key::PressedKeyEvents::no_events()
                }
            }
            _ => key::PressedKeyEvents::no_events(),
        }
    }

    fn key_output(&self, key: &Key<T>) -> Option<key::KeyOutput> {
        match (key, self) {
            (Key::LayerModifier { key, .. }, PressedKeyState::LayerModifier(pk)) => {
                pk.key_output(key)
            }
            (Key::Layered { key, .. }, PressedKeyState::Layered(pk)) => pk.key_output(key),
            (Key::Simple { key, .. }, PressedKeyState::Simple(pk)) => pk.key_output(key),
            (Key::TapHold { key, .. }, PressedKeyState::TapHold(pk)) => pk.key_output(key),
            _ => None,
        }
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

impl From<simple::Event> for Event {
    fn from(_ev: simple::Event) -> Self {
        panic!("key::simple never emits events")
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

impl TryFrom<Event> for simple::Event {
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
    ME: TryFrom<Event, Error = key::EventError>,
    IE: TryFrom<Event, Error = key::EventError>,
{
    type Error = key::EventError;

    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        let res: Result<ME, key::EventError> = ev.try_into();
        if let Ok(key_event) = res {
            Ok(key::ModifierKeyEvent::Modifier(key_event))
        } else {
            let res: Result<IE, key::EventError> = ev.try_into();
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
        const NUM_LAYERS: usize = 1;
        type NK = DefaultNestableKey;
        type L = layered::ArrayImpl<NUM_LAYERS>;
        type T = CompositeImpl<NK, L>;
        type Ctx = composite::Context<L>;
        type K = composite::Key<T>;
        let keymap_index: u16 = 0;
        let key = K::layer_modifier(layered::ModifierKey::Hold(0));
        let context: Ctx = Ctx {
            layer_context: layered::Context {
                active_layers: [false; NUM_LAYERS],
            },
        };
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
        use key::{composite, layered, simple, Context, Key};

        // Assemble
        const NUM_LAYERS: usize = 1;
        type NK = DefaultNestableKey;
        type L = layered::ArrayImpl<NUM_LAYERS>;
        type T = CompositeImpl<NK, L>;
        type Ctx = composite::Context<L>;
        type K = composite::Key<T>;
        let keys: [K; 2] = [
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                simple::Key(0x04),
                [Some(simple::Key(0x05))],
            )),
        ];
        let mut context: Ctx = Ctx {
            layer_context: layered::Context {
                active_layers: [false; NUM_LAYERS],
            },
        };
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
        assert_eq!(actual_active_layers, expected_active_layers);
    }

    #[test]
    fn test_composite_context_updates_with_composite_layerpressedmodifier_release_event() {
        use crate::input;
        use key::{composite, layered, simple, Context, Key, PressedKey};

        // Assemble
        const NUM_LAYERS: usize = 1;
        type NK = DefaultNestableKey;
        type L = layered::ArrayImpl<NUM_LAYERS>;
        type T = CompositeImpl<NK, L>;
        type Ctx = composite::Context<L>;
        type K = composite::Key<T>;
        let keys: [K; 2] = [
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                simple::Key(0x04),
                [Some(simple::Key(0x05))],
            )),
        ];
        let mut context: Ctx = Ctx {
            layer_context: layered::Context {
                active_layers: [false; NUM_LAYERS],
            },
        };
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
        assert_eq!(actual_active_layers, expected_active_layers);
    }

    #[test]
    fn test_composite_simple_pressed_key_has_key_code_for_composite_simple_key_def() {
        use key::{composite, layered, simple, Key, PressedKey};

        // Assemble
        const NUM_LAYERS: usize = 1;
        type NK = DefaultNestableKey;
        type L = layered::ArrayImpl<NUM_LAYERS>;
        type T = CompositeImpl<NK, L>;
        type Ctx = composite::Context<L>;
        type K = composite::Key<T>;
        let keys: [K; 3] = [
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                simple::Key(0x04),
                [Some(simple::Key(0x05))],
            )),
            K::simple(simple::Key(0x06)),
        ];
        let context: Ctx = Ctx {
            layer_context: layered::Context {
                active_layers: [false; NUM_LAYERS],
            },
        };

        // Act
        let keymap_index: u16 = 2;
        let (pressed_key, _) = keys[keymap_index as usize].new_pressed_key(context, keymap_index);
        let actual_keycode = pressed_key.key_output();

        // Assert
        let expected_keycode = Some(key::KeyOutput::from_key_code(0x06));
        assert_eq!(actual_keycode, expected_keycode);
    }

    #[test]
    fn test_composite_simple_pressed_key_has_key_code_for_composite_layered_key_def() {
        use key::{composite, layered, simple, Key, PressedKey};

        // Assemble
        const NUM_LAYERS: usize = 1;
        type NK = DefaultNestableKey;
        type L = layered::ArrayImpl<NUM_LAYERS>;
        type T = CompositeImpl<NK, L>;
        type Ctx = composite::Context<L>;
        type K = composite::Key<T>;
        let keys: [K; 3] = [
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                simple::Key(0x04),
                [Some(simple::Key(0x05))],
            )),
            K::simple(simple::Key(0x06)),
        ];
        let context: Ctx = Ctx {
            layer_context: layered::Context {
                active_layers: [false; NUM_LAYERS],
            },
        };

        // Act
        let keymap_index: u16 = 1;
        let (pressed_key, _) = keys[keymap_index as usize].new_pressed_key(context, keymap_index);
        let actual_keycode = pressed_key.key_output();

        // Assert
        let expected_keycode = Some(key::KeyOutput::from_key_code(0x04));
        assert_eq!(actual_keycode, expected_keycode);
    }
}
