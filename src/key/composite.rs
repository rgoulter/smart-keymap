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
    /// Constructs an [Event] for the Nestable key's event.
    fn into_event<T: CompositeTypes>(
        event: key::Event<<Self as key::Key>::Event>,
    ) -> key::Event<Event<T>>;
    /// Tries to construct the [key::Event] for the Nestable Key's event.
    fn try_event_from<T: CompositeTypes>(
        event: key::Event<Event<T>>,
    ) -> Result<key::Event<<Self as key::Key>::Event>, key::EventError>;
}

impl NestableKey for simple::Key {
    fn into_event<T: CompositeTypes>(
        event: key::Event<<Self as key::Key>::Event>,
    ) -> key::Event<Event<T>> {
        event.into()
    }

    fn try_event_from<T: CompositeTypes>(
        event: key::Event<Event<T>>,
    ) -> Result<key::Event<simple::Event>, key::EventError> {
        key::Event::try_from(event)
    }
}

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
    <T::NK as key::Key>::Context: From<Context<T>>,
{
    type Context = Context<T>;
    type ContextEvent = Event<T>;
    type Event = Event<T>;
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
                let (pressed_key, events) = key.new_pressed_key(context.into(), keymap_index);
                (pressed_key.into(), events.into_events())
            }
            Key::LayerModifier { key, .. } => {
                let (pressed_key, events) = key::Key::new_pressed_key(key, (), keymap_index);
                (pressed_key.into(), events.into_events())
            }
            Key::Layered { key, .. } => {
                let (pressed_key, events) = key.new_pressed_key(context.into(), keymap_index);
                (pressed_key.into(), events.map_events(T::NK::into_event))
            }
        }
    }
}

/// An aggregate context for [key::Context]s.
#[derive(Debug, Clone, Copy)]
pub struct Context<T: CompositeTypes = CompositeImpl> {
    /// The layered key context.
    pub layer_context: layered::Context<T::L>,
}

impl<T: CompositeTypes> Context<T> {
    /// Constructs a new [Context].
    pub const fn new(layer_context: layered::Context<T::L>) -> Self {
        Self { layer_context }
    }
}

impl<NK: NestableKey, const L: usize> Default for Context<CompositeImpl<NK, layered::ArrayImpl<L>>>
where
    <layered::ArrayImpl<L> as layered::LayerImpl>::Layers<NK>: serde::de::DeserializeOwned,
    NK: serde::de::DeserializeOwned,
    <NK as key::Key>::Context: From<Context<CompositeImpl<NK, layered::ArrayImpl<L>>>>,
{
    fn default() -> Self {
        let layer_context = layered::Context::default();
        Self { layer_context }
    }
}

impl<T: CompositeTypes> key::Context for Context<T> {
    type Event = Event<T>;
    fn handle_event(&mut self, event: Self::Event) {
        if let Event::LayerModification(ev) = event {
            self.layer_context.handle_event(ev);
        }
    }
}

/// simple::Context from composite::Context
impl<T: CompositeTypes> From<Context<T>> for () {
    fn from(_: Context<T>) -> Self {}
}

impl<T: CompositeTypes> From<Context<T>> for layered::Context<T::L> {
    fn from(ctx: Context<T>) -> Self {
        ctx.layer_context
    }
}

impl<T: CompositeTypes, MC, IC> From<Context<T>> for key::ModifierKeyContext<MC, IC>
where
    MC: From<Context<T>>,
    IC: From<Context<T>>,
{
    fn from(ctx: Context<T>) -> Self {
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
    <T::NK as key::Key>::Context: From<Context<T>>,
{
    type Event = Event<T>;

    fn handle_event_for(
        &mut self,
        context: Context<T>,
        keymap_index: u16,
        key: &Key<T>,
        event: key::Event<Self::Event>,
    ) -> key::PressedKeyEvents<Self::Event> {
        match (key, self) {
            (Key::TapHold { key, .. }, PressedKeyState::TapHold(pks)) => {
                if let Ok(ev) = key::Event::try_from(event) {
                    let events = pks.handle_event_for(context.into(), keymap_index, key, ev);
                    events.into_events()
                } else {
                    key::PressedKeyEvents::no_events()
                }
            }
            (Key::LayerModifier { key, .. }, PressedKeyState::LayerModifier(pks)) => {
                if let Ok(ev) = key::Event::try_from(event) {
                    let events = pks.handle_event_for(context.into(), keymap_index, key, ev);
                    events.into_events()
                } else {
                    key::PressedKeyEvents::no_events()
                }
            }
            (Key::Layered { key, .. }, PressedKeyState::Layered(pks)) => {
                if let Ok(ev) = T::NK::try_event_from(event) {
                    let events = pks.handle_event_for(context.into(), keymap_index, key, ev);
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

/// Aggregates the [key::Event] types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event<T: CompositeTypes = CompositeImpl> {
    /// A tap-hold event.
    TapHold(tap_hold::Event<<T::NK as key::Key>::Event>),
    /// A layer modification event.
    LayerModification(layered::LayerEvent),
}

impl<T: CompositeTypes> From<key::Event<layered::LayerEvent>> for key::Event<Event<T>> {
    fn from(ev: key::Event<layered::LayerEvent>) -> Self {
        ev.map_key_event(Event::LayerModification)
    }
}

impl<T: CompositeTypes> From<key::Event<simple::Event>> for key::Event<Event<T>> {
    fn from(ev: key::Event<simple::Event>) -> Self {
        ev.map_key_event(|_| panic!("key::simple never emits events"))
    }
}

impl<T: CompositeTypes> From<key::Event<tap_hold::Event<<T::NK as key::Key>::Event>>>
    for key::Event<Event<T>>
{
    fn from(ev: key::Event<tap_hold::Event<<T::NK as key::Key>::Event>>) -> Self {
        ev.map_key_event(Event::TapHold)
    }
}

impl<T: CompositeTypes> TryFrom<key::Event<Event<T>>> for key::Event<layered::LayerEvent> {
    type Error = key::EventError;

    fn try_from(ev: key::Event<Event<T>>) -> Result<Self, Self::Error> {
        ev.try_map_key_event(|ev| match ev {
            Event::LayerModification(ev) => Ok(ev),
            _ => Err(key::EventError::UnmappableEvent),
        })
    }
}

impl<T: CompositeTypes> TryFrom<key::Event<Event<T>>> for key::Event<simple::Event> {
    type Error = key::EventError;

    fn try_from(ev: key::Event<Event<T>>) -> Result<Self, Self::Error> {
        ev.try_map_key_event(|_| Err(key::EventError::UnmappableEvent))
    }
}

impl<T: CompositeTypes> TryFrom<key::Event<Event<T>>>
    for key::Event<tap_hold::Event<<T::NK as key::Key>::Event>>
{
    type Error = key::EventError;

    fn try_from(ev: key::Event<Event<T>>) -> Result<Self, Self::Error> {
        ev.try_map_key_event(|ev| match ev {
            Event::TapHold(ev) => Ok(ev),
            _ => Err(key::EventError::UnmappableEvent),
        })
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
        type NK = DefaultNestableKey;
        type L = layered::ArrayImpl<1>;
        type T = CompositeImpl<NK, L>;
        type Ctx = composite::Context<T>;
        type K = composite::Key<T>;
        let keymap_index: u16 = 0;
        let key = K::layer_modifier(layered::ModifierKey::Hold(0));
        let context = Ctx::default();
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
        type NK = DefaultNestableKey;
        type L = layered::ArrayImpl<1>;
        type T = CompositeImpl<NK, L>;
        type Ctx = composite::Context<T>;
        type K = composite::Key<T>;
        let keys: [K; 2] = [
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                simple::Key(0x04),
                [Some(simple::Key(0x05))],
            )),
        ];
        let mut context = Ctx::default();
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
        type NK = DefaultNestableKey;
        type L = layered::ArrayImpl<1>;
        type T = CompositeImpl<NK, L>;
        type Ctx = composite::Context<T>;
        type K = composite::Key<T>;
        let keys: [K; 2] = [
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                simple::Key(0x04),
                [Some(simple::Key(0x05))],
            )),
        ];
        let mut context = Ctx::default();
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
        type NK = DefaultNestableKey;
        type L = layered::ArrayImpl<1>;
        type T = CompositeImpl<NK, L>;
        type Ctx = composite::Context<T>;
        type K = composite::Key<T>;
        let keys: [K; 3] = [
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                simple::Key(0x04),
                [Some(simple::Key(0x05))],
            )),
            K::simple(simple::Key(0x06)),
        ];
        let context = Ctx::default();

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
        type NK = DefaultNestableKey;
        type L = layered::ArrayImpl<1>;
        type T = CompositeImpl<NK, L>;
        type Ctx = composite::Context<T>;
        type K = composite::Key<T>;
        let keys: [K; 3] = [
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                simple::Key(0x04),
                [Some(simple::Key(0x05))],
            )),
            K::simple(simple::Key(0x06)),
        ];
        let context = Ctx::default();

        // Act
        let keymap_index: u16 = 1;
        let (pressed_key, _) = keys[keymap_index as usize].new_pressed_key(context, keymap_index);
        let actual_keycode = pressed_key.key_output();

        // Assert
        let expected_keycode = Some(key::KeyOutput::from_key_code(0x04));
        assert_eq!(actual_keycode, expected_keycode);
    }
}
