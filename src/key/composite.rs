//! This module implements the `keymap::Key` for a 'composite' key,
//!  which can be any of the other key definitions,
//!  and is the default Key for the `keymap::KeyMap` implementation.

use core::fmt::Debug;
use core::marker::PhantomData;

use serde::Deserialize;

use crate::{input, key};
use key::{layered, simple, tap_hold};

use super::layered::LayerState;

/// Used to implement nested combinations of [Key].
pub trait NestableKey: key::Key + Sized {}

impl NestableKey for simple::Key {}

/// Default [NestableKey] for [Key] and its associated types.
pub type DefaultNestableKey = simple::Key;

/// An aggregate of [key::Key] types.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum Key<
    const L: layered::LayerIndex = 0,
    K: NestableKey = DefaultNestableKey,
    LS: layered::LayerState = [bool; 0],
> where
    [Option<K>; L]: serde::de::DeserializeOwned,
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
        key: layered::ModifierKey<L>,
    },
    /// A layered key.
    Layered {
        /// The layered key.
        key: layered::LayeredKey<L, K>,
        /// Marker for the LayerState, for LayeredKey new_pressed_key.
        #[serde(skip)]
        _phantom: PhantomData<LS>,
    },
}

impl<const L: layered::LayerIndex, LS: layered::LayerState + Debug> Key<L, DefaultNestableKey, LS>
where
    [Option<DefaultNestableKey>; L]: serde::de::DeserializeOwned,
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
    pub const fn layer_modifier(key: layered::ModifierKey<L>) -> Self {
        Self::LayerModifier { key }
    }

    /// Constructs a [Key::Layered] from the given [layered::LayeredKey].
    pub const fn layered(key: layered::LayeredKey<L, DefaultNestableKey>) -> Self {
        Self::Layered {
            key,
            _phantom: PhantomData,
        }
    }
}

impl<const L: layered::LayerIndex, LS: layered::LayerState + Debug> key::Key
    for Key<L, DefaultNestableKey, LS>
where
    [Option<DefaultNestableKey>; L]: serde::de::DeserializeOwned,
{
    type Context = Context<DefaultNestableKey, LS>;
    type ContextEvent = Event;
    type Event = Event;
    type PressedKeyState = PressedKeyState<L>;

    fn new_pressed_key(
        &self,
        context: &Self::Context,
        keymap_index: u16,
    ) -> (
        input::PressedKey<Self, Self::PressedKeyState>,
        Option<key::ScheduledEvent<Event>>,
    ) {
        match self {
            Key::Simple { key, .. } => {
                let (pressed_key, _new_event) = key.new_pressed_key(&(), keymap_index);
                (pressed_key.into(), None)
            }
            Key::TapHold { key, .. } => {
                let (pressed_key, new_event) = key.new_pressed_key(&(), keymap_index);
                (pressed_key.into(), new_event.map(|ev| ev.into()))
            }
            Key::LayerModifier { key, .. } => {
                let (pressed_key, new_event) = key.new_pressed_key(keymap_index);
                (pressed_key.into(), Some(new_event.into()))
            }
            Key::Layered { key, .. } => {
                let Context { layer_context } = context;
                let (pressed_key, new_event) = key.new_pressed_key(layer_context, keymap_index);
                (pressed_key.into(), new_event.map(|ev| ev.into()))
            }
        }
    }
}

/// An aggregate context for [key::Context]s.
#[derive(Debug, Clone, Copy)]
pub struct Context<K: key::Key, LS: layered::LayerState = [bool; 0]> {
    layer_context: layered::Context<K::Context, LS>,
}

impl<const L: usize> Context<DefaultNestableKey, [bool; L]> {
    /// Constructs a new [Context].
    pub const fn new() -> Self {
        let layer_context = layered::Context::new(());
        Self { layer_context }
    }
}

impl<const L: usize> Default for Context<DefaultNestableKey, [bool; L]> {
    fn default() -> Self {
        Self::new()
    }
}

impl<LS: LayerState> key::Context for Context<DefaultNestableKey, LS> {
    type Event = Event;
    fn handle_event(&mut self, event: Self::Event) {
        if let Event::LayerModification(ev) = event {
            self.layer_context.handle_event(ev);
        }
    }
}

/// simple::Context from composite::Context
impl<LS: layered::LayerState> From<&Context<DefaultNestableKey, LS>> for &() {
    fn from(_: &Context<DefaultNestableKey, LS>) -> Self {
        &()
    }
}

/// Aggregates the [key::PressedKeyState] types.
#[derive(Debug, Clone, Copy)]
pub enum PressedKeyState<const L: layered::LayerIndex = 0> {
    /// A simple key's pressed state.
    Simple(simple::PressedKeyState),
    /// A tap-hold key's pressed state.
    TapHold(tap_hold::PressedKeyState),
    /// A layer modifier key's pressed state.
    LayerModifier(layered::PressedModifierKeyState),
}

/// Convenience type alias for a [key::PressedKey] with a [PressedKeyState].
pub type PressedKey<const L: layered::LayerIndex, LS> =
    input::PressedKey<Key<L, DefaultNestableKey, LS>, PressedKeyState<L>>;

impl<const L: layered::LayerIndex, LS: layered::LayerState + Debug>
    From<layered::PressedModifierKey<L>> for PressedKey<L, LS>
where
    [Option<DefaultNestableKey>; L]: serde::de::DeserializeOwned,
{
    fn from(
        input::PressedKey {
            keymap_index,
            key,
            pressed_key_state,
        }: layered::PressedModifierKey<L>,
    ) -> Self {
        input::PressedKey {
            key: Key::layer_modifier(key),
            keymap_index,
            pressed_key_state: PressedKeyState::LayerModifier(pressed_key_state),
        }
    }
}

impl<const L: layered::LayerIndex, LS: layered::LayerState + Debug> From<simple::PressedKey>
    for PressedKey<L, LS>
where
    [Option<DefaultNestableKey>; L]: serde::de::DeserializeOwned,
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

impl<const L: layered::LayerIndex, LS: layered::LayerState + Debug> From<tap_hold::PressedKey>
    for PressedKey<L, LS>
where
    [Option<DefaultNestableKey>; L]: serde::de::DeserializeOwned,
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

impl<const L: layered::LayerIndex, LS: layered::LayerState + Debug>
    key::PressedKeyState<Key<L, DefaultNestableKey, LS>> for PressedKeyState<L>
where
    [Option<DefaultNestableKey>; L]: serde::de::DeserializeOwned,
{
    type Event = Event;

    fn handle_event_for(
        &mut self,
        keymap_index: u16,
        key: &Key<L, DefaultNestableKey, LS>,
        event: key::Event<Self::Event>,
    ) -> impl IntoIterator<Item = key::Event<Self::Event>> {
        match (key, self) {
            (Key::TapHold { key, .. }, PressedKeyState::TapHold(pks)) => {
                if let Ok(ev) = key::Event::try_from(event) {
                    let events = pks.handle_event_for(keymap_index, key, ev);
                    events.into_iter().map(|ev| ev.into()).collect()
                } else {
                    heapless::Vec::<key::Event<Self::Event>, 2>::new()
                }
            }
            (Key::LayerModifier { key, .. }, PressedKeyState::LayerModifier(pks)) => {
                if let Ok(ev) = key::Event::try_from(event) {
                    let events = pks.handle_event_for(keymap_index, key, ev);
                    events.into_iter().map(|ev| ev.into()).collect()
                } else {
                    heapless::Vec::<key::Event<Self::Event>, 2>::new()
                }
            }
            _ => heapless::Vec::new(),
        }
    }

    fn key_code(&self, key: &Key<L, DefaultNestableKey, LS>) -> Option<u8> {
        match (key, self) {
            (Key::LayerModifier { key, .. }, PressedKeyState::LayerModifier(pk)) => {
                pk.key_code(key)
            }
            (Key::Simple { key, .. }, PressedKeyState::Simple(pk)) => pk.key_code(key),
            (Key::TapHold { key, .. }, PressedKeyState::TapHold(pk)) => pk.key_code(key),
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

impl From<key::ScheduledEvent<layered::LayerEvent>> for key::ScheduledEvent<Event> {
    fn from(ev: key::ScheduledEvent<layered::LayerEvent>) -> Self {
        Self {
            schedule: ev.schedule,
            event: ev.event.into(),
        }
    }
}

impl From<key::ScheduledEvent<simple::Event>> for key::ScheduledEvent<Event> {
    fn from(ev: key::ScheduledEvent<simple::Event>) -> Self {
        Self {
            schedule: ev.schedule,
            event: ev.event.into(),
        }
    }
}

impl From<key::ScheduledEvent<tap_hold::Event>> for key::ScheduledEvent<Event> {
    fn from(ev: key::ScheduledEvent<tap_hold::Event>) -> Self {
        Self {
            schedule: ev.schedule,
            event: ev.event.into(),
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
        const L: layered::LayerIndex = 1;
        type LS = [bool; L];
        type Ctx = composite::Context<DefaultNestableKey, LS>;
        type K = composite::Key<L, DefaultNestableKey, LS>;
        let keymap_index: u16 = 0;
        let key = K::layer_modifier(layered::ModifierKey::Hold(0));
        let context = Ctx::new();
        let (mut pressed_lmod_key, _) = key.new_pressed_key(&context, keymap_index);

        // Act
        let events = pressed_lmod_key
            .handle_event(key::Event::Input(input::Event::Release { keymap_index }));

        // Assert
        let _key_ev = match events.into_iter().next() {
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
        const L: layered::LayerIndex = 1;
        type LS = [bool; L];
        type Ctx = composite::Context<DefaultNestableKey, LS>;
        type K = composite::Key<L, DefaultNestableKey, LS>;
        let keys: [K; 2] = [
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                simple::Key(0x04),
                [Some(simple::Key(0x05))],
            )),
        ];
        let mut context = Ctx::new();
        let (_pressed_key, maybe_ev) = keys[0].new_pressed_key(&context, 0);

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
        const L: layered::LayerIndex = 1;
        type LS = [bool; L];
        type Ctx = composite::Context<DefaultNestableKey, LS>;
        type K = composite::Key<L, DefaultNestableKey, LS>;
        let keys: [K; 2] = [
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                simple::Key(0x04),
                [Some(simple::Key(0x05))],
            )),
        ];
        let mut context = Ctx::new();
        let (mut pressed_lmod_key, _) = keys[0].new_pressed_key(&context, 0);
        context.layer_context.activate_layer(0);
        let events = pressed_lmod_key
            .handle_event(key::Event::Input(input::Event::Release { keymap_index: 0 }));
        let key_ev = match events.into_iter().next() {
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
        const L: layered::LayerIndex = 1;
        type LS = [bool; L];
        type Ctx = composite::Context<DefaultNestableKey, LS>;
        type K = composite::Key<L, DefaultNestableKey, LS>;
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
        let (pressed_key, _) = keys[keymap_index as usize].new_pressed_key(&context, keymap_index);
        let actual_keycode = pressed_key.key_code();

        // Assert
        let expected_keycode = Some(0x06);
        assert_eq!(actual_keycode, expected_keycode);
    }

    #[test]
    fn test_composite_simple_pressed_key_has_key_code_for_composite_layered_key_def() {
        use key::{composite, layered, simple, Key, PressedKey};

        // Assemble
        const L: layered::LayerIndex = 1;
        type LS = [bool; L];
        type Ctx = composite::Context<DefaultNestableKey, LS>;
        type K = composite::Key<L, DefaultNestableKey, LS>;
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
        let (pressed_key, _) = keys[keymap_index as usize].new_pressed_key(&context, keymap_index);
        let actual_keycode = pressed_key.key_code();

        // Assert
        let expected_keycode = Some(0x04);
        assert_eq!(actual_keycode, expected_keycode);
    }
}
