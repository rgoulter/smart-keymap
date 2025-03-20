//! This module implements the `keymap::Key` for a 'composite' key,
//!  which can be any of the other key definitions,
//!  and is the default Key for the `keymap::KeyMap` implementation.
#![doc = include_str!("doc_de_composite.md")]

use core::fmt::Debug;

#[cfg(feature = "std")]
use serde::Deserialize;

use crate::key;

mod base;
mod chorded;
mod layered;
mod tap_hold;

pub use base::{BaseKey, BasePressedKey, BasePressedKeyState};
pub use chorded::{
    Chorded, ChordedKey, ChordedNestable, ChordedPressedKey, ChordedPressedKeyState,
};
pub use layered::{
    Layered, LayeredKey, LayeredNestable, LayeredPressedKey, LayeredPressedKeyState,
};
pub use tap_hold::{
    TapHold, TapHoldKey, TapHoldNestable, TapHoldPressedKey, TapHoldPressedKeyState,
};

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
///
///   Chorded := Chorded<Layered> | AuxChorded<Layered> | Layered
///   ```
pub type Key = ChordedKey<LayeredKey<TapHoldKey<BaseKey>>>;

/// Config used for constructing initial context
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "std", derive(Deserialize))]
pub struct Config {
    /// The chorded configuration.
    #[cfg_attr(feature = "std", serde(default))]
    pub chorded: key::chorded::Config,
    /// The tap hold configuration.
    #[cfg_attr(feature = "std", serde(default))]
    pub tap_hold: key::tap_hold::Config,
}

/// The default config.
pub const DEFAULT_CONFIG: Config = Config {
    chorded: key::chorded::DEFAULT_CONFIG,
    tap_hold: key::tap_hold::DEFAULT_CONFIG,
};

/// An aggregate context for [key::Context]s.
#[derive(Debug, Clone, Copy)]
pub struct Context {
    /// The chorded key context.
    pub chorded_context: key::chorded::Context,
    /// The layered key context.
    pub layer_context: key::layered::Context,
    /// The tap hold key context.
    pub tap_hold_context: key::tap_hold::Context,
}

/// The default context.
pub const DEFAULT_CONTEXT: Context = Context {
    chorded_context: key::chorded::DEFAULT_CONTEXT,
    layer_context: key::layered::DEFAULT_CONTEXT,
    tap_hold_context: key::tap_hold::DEFAULT_CONTEXT,
};

impl Context {
    /// Constructs a [Context] from the given [Config].
    pub const fn from_config(config: Config) -> Self {
        Self {
            chorded_context: key::chorded::Context::from_config(config.chorded),
            layer_context: key::layered::DEFAULT_CONTEXT,
            tap_hold_context: key::tap_hold::Context::from_config(config.tap_hold),
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
    fn handle_event(&mut self, event: key::Event<Self::Event>) {
        if let Ok(e) = event.try_into_key_event(|e| e.try_into()) {
            self.chorded_context.handle_event(e);
        }

        if let key::Event::Key {
            key_event: Event::LayerModification(ev),
            ..
        } = event
        {
            self.layer_context.handle_event(ev);
        }
    }
}

/// keyboard::Context from composite::Context
impl From<Context> for () {
    fn from(_: Context) -> Self {}
}

impl From<Context> for key::chorded::Context {
    fn from(ctx: Context) -> Self {
        ctx.chorded_context
    }
}

impl From<Context> for key::layered::Context {
    fn from(ctx: Context) -> Self {
        ctx.layer_context
    }
}

impl From<Context> for key::tap_hold::Context {
    fn from(ctx: Context) -> Self {
        ctx.tap_hold_context
    }
}

/// Convenience type alias for the 'highest' composite key.
pub type PressedKey = ChordedPressedKey<LayeredKey<TapHoldKey<BaseKey>>>;

/// Sum type aggregating the [key::Event] types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// A chorded event.
    Chorded(key::chorded::Event),
    /// A tap-hold event.
    TapHold(key::tap_hold::Event),
    /// A layer modification event.
    LayerModification(key::layered::LayerEvent),
}

impl From<key::chorded::Event> for Event {
    fn from(ev: key::chorded::Event) -> Self {
        Event::Chorded(ev)
    }
}

impl From<key::layered::LayerEvent> for Event {
    fn from(ev: key::layered::LayerEvent) -> Self {
        Event::LayerModification(ev)
    }
}

impl From<key::tap_hold::Event> for Event {
    fn from(ev: key::tap_hold::Event) -> Self {
        Event::TapHold(ev)
    }
}

impl TryFrom<Event> for key::chorded::Event {
    type Error = key::EventError;

    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        match ev {
            Event::Chorded(ev) => Ok(ev),
            _ => Err(key::EventError::UnmappableEvent),
        }
    }
}

impl TryFrom<Event> for key::layered::LayerEvent {
    type Error = key::EventError;

    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        match ev {
            Event::LayerModification(ev) => Ok(ev),
            _ => Err(key::EventError::UnmappableEvent),
        }
    }
}

impl TryFrom<Event> for key::tap_hold::Event {
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
        use key::{composite, Key, PressedKey};

        // Assemble
        type Ctx = composite::Context;
        type K = composite::Key;
        let keymap_index: u16 = 0;
        let key = K::layer_modifier(key::layered::ModifierKey::Hold(0));
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
                key_event:
                    Event::LayerModification(key::layered::LayerEvent::LayerDeactivated(layer)),
                ..
            }) => {
                assert_eq!(0, layer);
            }
            _ => panic!("Expected an Event::Key(LayerModification(LayerDeactivated(layer)))"),
        };
    }

    #[test]
    fn test_composite_context_updates_with_composite_layermodifier_press_event() {
        use key::{composite, Context, Key};

        // Assemble
        type Ctx = composite::Context;
        type K = composite::Key;
        let keys: [K; 2] = [
            K::layer_modifier(key::layered::ModifierKey::Hold(0)),
            K::layered(key::layered::LayeredKey::new(
                key::keyboard::Key::new(0x04).into(),
                [Some(key::keyboard::Key::new(0x05).into())],
            )),
        ];
        let mut context: Ctx = DEFAULT_CONTEXT;
        let (_pressed_key, pressed_key_events) = keys[0].new_pressed_key(context, 0);
        let maybe_ev = pressed_key_events.into_iter().next();

        // Act
        let event = match maybe_ev {
            Some(key::ScheduledEvent { event, .. }) => event,
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
        use key::{composite, Context, Key, PressedKey};

        // Assemble
        type Ctx = composite::Context;
        type K = composite::Key;
        let keys: [K; 2] = [
            K::layer_modifier(key::layered::ModifierKey::Hold(0)),
            K::layered(key::layered::LayeredKey::new(
                key::keyboard::Key::new(0x04).into(),
                [Some(key::keyboard::Key::new(0x05).into())],
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
            Some(key_event) => key_event,
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
        use key::{composite, Key, PressedKey};

        // Assemble
        type Ctx = composite::Context;
        type K = composite::Key;
        let keys: [K; 3] = [
            K::layer_modifier(key::layered::ModifierKey::Hold(0)),
            K::layered(key::layered::LayeredKey::new(
                key::keyboard::Key::new(0x04).into(),
                [Some(key::keyboard::Key::new(0x05).into())],
            )),
            K::keyboard(key::keyboard::Key::new(0x06)),
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
        use key::{composite, Key, PressedKey};

        // Assemble
        type Ctx = composite::Context;
        type K = composite::Key;
        let keys: [K; 3] = [
            K::layer_modifier(key::layered::ModifierKey::Hold(0)),
            K::layered(key::layered::LayeredKey::new(
                key::keyboard::Key::new(0x04).into(),
                [Some(key::keyboard::Key::new(0x05).into())],
            )),
            K::keyboard(key::keyboard::Key::new(0x06)),
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
