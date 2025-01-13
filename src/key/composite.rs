//! This module implements the `keymap::Key` for a 'composite' key,
//!  which can be any of the other key definitions,
//!  and is the default Key for the `keymap::KeyMap` implementation.
#![doc = include_str!("doc_de_composite.md")]

use core::fmt::Debug;

use serde::Deserialize;

use crate::{input, key};
use key::{layered, simple, tap_hold};

/// Used to implement nested combinations of [Key].
pub trait NestableKey: key::Key + Sized {
    /// Constructs a [key::ModifierKeyContext] for the given Context.
    fn into_nested_context_for<C>(
        context: C,
    ) -> key::ModifierKeyContext<C, <Self as key::Key>::Context>;
    /// Constructs an [Event] for the Nestable key's event.
    fn into_event(event: key::Event<<Self as key::Key>::Event>) -> key::Event<Event>;
    /// Tries to construct the [key::Event] for the Nestable Key's event.
    fn try_event_from(
        event: key::Event<Event>,
    ) -> Result<key::Event<<Self as key::Key>::Event>, key::EventError>;
}

impl NestableKey for simple::Key {
    fn into_nested_context_for<C>(
        context: C,
    ) -> key::ModifierKeyContext<C, <Self as key::Key>::Context> {
        key::ModifierKeyContext {
            context,
            inner_context: (),
        }
    }

    fn into_event(event: key::Event<<Self as key::Key>::Event>) -> key::Event<Event> {
        match event {
            key::Event::Input(ev) => key::Event::Input(ev),
            key::Event::Key(_) => panic!("key::simple never emits events"),
        }
    }

    fn try_event_from(
        event: key::Event<Event>,
    ) -> Result<key::Event<<Self as key::Key>::Event>, key::EventError> {
        match event {
            key::Event::Input(ev) => Ok(key::Event::Input(ev)),
            key::Event::Key(_) => Err(key::EventError::UnmappableEvent),
        }
    }
}

/// Default [NestableKey] for [Key] and its associated types.
pub type DefaultNestableKey = simple::Key;

/// An aggregate of [key::Key] types.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum Key<NK: NestableKey = DefaultNestableKey, L: layered::LayerImpl = layered::ArrayImpl<0>>
where
    L::Layers<NK>: serde::de::DeserializeOwned,
{
    /// A simple key.
    Simple {
        /// The simple key.
        key: simple::Key,
    },
    /// A tap-hold key.
    TapHold {
        /// The tap-hold key.
        key: tap_hold::Key,
    },
    /// A layer modifier key.
    LayerModifier {
        /// The layer modifier key.
        key: layered::ModifierKey,
    },
    /// A layered key.
    Layered {
        /// The layered key.
        key: layered::LayeredKey<NK, L>,
    },
}

impl<NK: NestableKey, L: layered::LayerImpl> Key<NK, L>
where
    L::Layers<NK>: serde::de::DeserializeOwned,
{
    /// Constructs a [Key::Simple] from the given [simple::Key].
    pub const fn simple(key: simple::Key) -> Self {
        Self::Simple { key }
    }

    /// Constructs a [Key::TapHold] from the given [tap_hold::Key].
    pub const fn tap_hold(key: tap_hold::Key) -> Self {
        Self::TapHold { key }
    }

    /// Constructs a [Key::LayerModifier] from the given [layered::ModifierKey].
    pub const fn layer_modifier(key: layered::ModifierKey) -> Self {
        Self::LayerModifier { key }
    }

    /// Constructs a [Key::Layered] from the given [layered::LayeredKey].
    pub const fn layered(key: layered::LayeredKey<NK, L>) -> Self {
        Self::Layered { key }
    }
}

impl<NK: NestableKey, L: layered::LayerImpl> key::Key for Key<NK, L>
where
    L::Layers<NK>: serde::de::DeserializeOwned,
{
    type Context = Context<L>;
    type ContextEvent = Event;
    type Event = Event;
    type PressedKeyState = PressedKeyState<NK, L>;

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
                let (pressed_key, events) = key.new_pressed_key((), keymap_index);
                (pressed_key.into(), events.into_events())
            }
            Key::LayerModifier { key, .. } => {
                let (pressed_key, events) = key::Key::new_pressed_key(key, (), keymap_index);
                (pressed_key.into(), events.into_events())
            }
            Key::Layered { key, .. } => {
                let modifier_context = NK::into_nested_context_for(context.layer_context);
                let (pressed_key, events) = key.new_pressed_key(modifier_context, keymap_index);
                (pressed_key.into(), events.map_events(NK::into_event))
            }
        }
    }
}

/// An aggregate context for [key::Context]s.
#[derive(Debug, Clone, Copy)]
pub struct Context<L: layered::LayerImpl = layered::ArrayImpl<0>> {
    layer_context: layered::Context<L>,
}

impl<const L: usize> Context<layered::ArrayImpl<L>> {
    /// Constructs a new [Context].
    pub const fn new() -> Self {
        let layer_context = layered::Context::new();
        Self { layer_context }
    }
}

impl<const L: usize> Default for Context<layered::ArrayImpl<L>> {
    fn default() -> Self {
        Self::new()
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
pub enum PressedKeyState<NK: NestableKey, L: layered::LayerImpl = layered::ArrayImpl<0>> {
    /// A simple key's pressed state.
    Simple(simple::PressedKeyState),
    /// A tap-hold key's pressed state.
    TapHold(tap_hold::PressedKeyState),
    /// A layer modifier key's pressed state.
    LayerModifier(layered::PressedModifierKeyState),
    /// A layer modifier key's pressed state.
    Layered(layered::PressedLayeredKeyState<NK, L>),
}

/// Convenience type alias for a [key::PressedKey] with a [PressedKeyState].
pub type PressedKey<NK, L> = input::PressedKey<Key<NK, L>, PressedKeyState<NK, L>>;

impl<NK: NestableKey, L: layered::LayerImpl> From<layered::PressedModifierKey> for PressedKey<NK, L>
where
    L::Layers<NK>: serde::de::DeserializeOwned,
{
    fn from(
        input::PressedKey {
            keymap_index,
            key,
            pressed_key_state,
        }: layered::PressedModifierKey,
    ) -> Self
    where
        L::Layers<NK>: serde::de::DeserializeOwned,
    {
        input::PressedKey {
            key: Key::layer_modifier(key),
            keymap_index,
            pressed_key_state: PressedKeyState::LayerModifier(pressed_key_state),
        }
    }
}

impl<NK: NestableKey, L: layered::LayerImpl> From<layered::PressedLayeredKey<NK, L>>
    for PressedKey<NK, L>
where
    L::Layers<NK>: serde::de::DeserializeOwned,
{
    fn from(
        input::PressedKey {
            keymap_index,
            key,
            pressed_key_state,
        }: layered::PressedLayeredKey<NK, L>,
    ) -> Self
    where
        L::Layers<NK>: serde::de::DeserializeOwned,
    {
        input::PressedKey {
            key: Key::layered(key),
            keymap_index,
            pressed_key_state: PressedKeyState::Layered(pressed_key_state),
        }
    }
}

impl<NK: NestableKey, L: layered::LayerImpl> From<simple::PressedKey> for PressedKey<NK, L>
where
    L::Layers<NK>: serde::de::DeserializeOwned,
{
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

impl<NK: NestableKey, L: layered::LayerImpl> From<tap_hold::PressedKey> for PressedKey<NK, L>
where
    L::Layers<NK>: serde::de::DeserializeOwned,
{
    fn from(
        input::PressedKey {
            keymap_index,
            key,
            pressed_key_state,
        }: tap_hold::PressedKey,
    ) -> Self {
        input::PressedKey {
            key: Key::tap_hold(key),
            keymap_index,
            pressed_key_state: PressedKeyState::TapHold(pressed_key_state),
        }
    }
}

impl<NK: NestableKey, L: layered::LayerImpl> key::PressedKeyState<Key<NK, L>>
    for PressedKeyState<NK, L>
where
    L::Layers<NK>: serde::de::DeserializeOwned,
{
    type Event = Event;

    fn handle_event_for(
        &mut self,
        keymap_index: u16,
        key: &Key<NK, L>,
        event: key::Event<Self::Event>,
    ) -> key::PressedKeyEvents<Self::Event> {
        match (key, self) {
            (Key::TapHold { key, .. }, PressedKeyState::TapHold(pks)) => {
                if let Ok(ev) = key::Event::try_from(event) {
                    let events = pks.handle_event_for(keymap_index, key, ev);
                    events.into_events()
                } else {
                    key::PressedKeyEvents::no_events()
                }
            }
            (Key::LayerModifier { key, .. }, PressedKeyState::LayerModifier(pks)) => {
                if let Ok(ev) = key::Event::try_from(event) {
                    let events = pks.handle_event_for(keymap_index, key, ev);
                    events.into_events()
                } else {
                    key::PressedKeyEvents::no_events()
                }
            }
            (Key::Layered { key, .. }, PressedKeyState::Layered(pks)) => {
                if let Ok(ev) = NK::try_event_from(event) {
                    let events = pks.handle_event_for(keymap_index, key, ev);
                    events.map_events(NK::into_event)
                } else {
                    key::PressedKeyEvents::no_events()
                }
            }
            _ => key::PressedKeyEvents::no_events(),
        }
    }

    fn key_output(&self, key: &Key<NK, L>) -> Option<key::KeyOutput> {
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
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Event {
    /// A tap-hold event.
    TapHold(tap_hold::Event),
    /// A layer modification event.
    LayerModification(layered::LayerEvent),
}

impl From<key::Event<layered::LayerEvent>> for key::Event<Event> {
    fn from(ev: key::Event<layered::LayerEvent>) -> Self {
        match ev {
            key::Event::Input(ev) => key::Event::Input(ev),
            key::Event::Key(ev) => key::Event::Key(Event::LayerModification(ev)),
        }
    }
}

impl From<key::Event<simple::Event>> for key::Event<Event> {
    fn from(ev: key::Event<simple::Event>) -> Self {
        match ev {
            key::Event::Input(ev) => key::Event::Input(ev),
            key::Event::Key(_) => panic!("key::simple never emits events"),
        }
    }
}

impl From<key::Event<tap_hold::Event>> for key::Event<Event> {
    fn from(ev: key::Event<tap_hold::Event>) -> Self {
        match ev {
            key::Event::Input(ev) => key::Event::Input(ev),
            key::Event::Key(ev) => key::Event::Key(Event::TapHold(ev)),
        }
    }
}

impl TryFrom<key::Event<Event>> for key::Event<layered::LayerEvent> {
    type Error = key::EventError;

    fn try_from(ev: key::Event<Event>) -> Result<Self, Self::Error> {
        match ev {
            key::Event::Input(ev) => Ok(key::Event::Input(ev)),
            key::Event::Key(Event::LayerModification(ev)) => Ok(key::Event::Key(ev)),
            key::Event::Key(_) => Err(key::EventError::UnmappableEvent),
        }
    }
}

impl TryFrom<key::Event<Event>> for key::Event<simple::Event> {
    type Error = key::EventError;

    fn try_from(ev: key::Event<Event>) -> Result<Self, Self::Error> {
        match ev {
            key::Event::Input(ev) => Ok(key::Event::Input(ev)),
            key::Event::Key(_) => Err(key::EventError::UnmappableEvent),
        }
    }
}

impl TryFrom<key::Event<Event>> for key::Event<tap_hold::Event> {
    type Error = key::EventError;

    fn try_from(ev: key::Event<Event>) -> Result<Self, Self::Error> {
        match ev {
            key::Event::Input(ev) => Ok(key::Event::Input(ev)),
            key::Event::Key(Event::TapHold(ev)) => Ok(key::Event::Key(ev)),
            key::Event::Key(_) => Err(key::EventError::UnmappableEvent),
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
        type L = layered::ArrayImpl<1>;
        type Ctx = composite::Context<L>;
        type K = composite::Key<DefaultNestableKey, L>;
        let keymap_index: u16 = 0;
        let key = K::layer_modifier(layered::ModifierKey::Hold(0));
        let context = Ctx::new();
        let (mut pressed_lmod_key, _) = key.new_pressed_key(context, keymap_index);

        // Act
        let events = pressed_lmod_key
            .handle_event(key::Event::Input(input::Event::Release { keymap_index }));

        // Assert
        let _key_ev = match events.into_iter().next().map(|sch_ev| sch_ev.event) {
            Some(key::Event::Key(Event::LayerModification(
                layered::LayerEvent::LayerDeactivated(layer),
            ))) => {
                assert_eq!(layer, 0);
            }
            _ => panic!("Expected an Event::Key(LayerModification(LayerDeactivated(layer)))"),
        };
    }

    #[test]
    fn test_composite_context_updates_with_composite_layermodifier_press_event() {
        use key::{composite, layered, simple, Context, Key};

        // Assemble
        type L = layered::ArrayImpl<1>;
        type Ctx = composite::Context<L>;
        type K = composite::Key<DefaultNestableKey, L>;
        let keys: [K; 2] = [
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                simple::Key(0x04),
                [Some(simple::Key(0x05))],
            )),
        ];
        let mut context = Ctx::new();
        let (_pressed_key, pressed_key_events) = keys[0].new_pressed_key(context, 0);
        let maybe_ev = pressed_key_events.into_iter().next();

        // Act
        let event = match maybe_ev {
            Some(key::ScheduledEvent {
                event: key::Event::Key(ev),
                ..
            }) => ev,
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
        type L = layered::ArrayImpl<1>;
        type Ctx = composite::Context<L>;
        type K = composite::Key<DefaultNestableKey, L>;
        let keys: [K; 2] = [
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                simple::Key(0x04),
                [Some(simple::Key(0x05))],
            )),
        ];
        let mut context = Ctx::new();
        let (mut pressed_lmod_key, _) = keys[0].new_pressed_key(context, 0);
        context.layer_context.activate_layer(0);
        let events = pressed_lmod_key
            .handle_event(key::Event::Input(input::Event::Release { keymap_index: 0 }));
        let key_ev = match events.into_iter().next().map(|sch_ev| sch_ev.event) {
            Some(key::Event::Key(ev)) => ev,
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
        type L = layered::ArrayImpl<1>;
        type Ctx = composite::Context<L>;
        type K = composite::Key<DefaultNestableKey, L>;
        let keys: [K; 3] = [
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                simple::Key(0x04),
                [Some(simple::Key(0x05))],
            )),
            K::simple(simple::Key(0x06)),
        ];
        let context = Ctx::new();

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
        type L = layered::ArrayImpl<1>;
        type Ctx = composite::Context<L>;
        type K = composite::Key<DefaultNestableKey, L>;
        let keys: [K; 3] = [
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                simple::Key(0x04),
                [Some(simple::Key(0x05))],
            )),
            K::simple(simple::Key(0x06)),
        ];
        let context = Ctx::new();

        // Act
        let keymap_index: u16 = 1;
        let (pressed_key, _) = keys[keymap_index as usize].new_pressed_key(context, keymap_index);
        let actual_keycode = pressed_key.key_output();

        // Assert
        let expected_keycode = Some(key::KeyOutput::from_key_code(0x04));
        assert_eq!(actual_keycode, expected_keycode);
    }
}
