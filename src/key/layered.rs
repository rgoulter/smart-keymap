use core::fmt::Debug;
use core::marker::Copy;

use serde::Deserialize;

use crate::input;
use crate::key;

/// The type used for layer index.
pub type LayerIndex = usize;

/// Implementation of associated [Layers] and [LayerState].
pub trait LayerImpl: Copy + Debug + PartialEq {
    /// The associated [LayerState] type.
    type LayerState: LayerState;
    /// The associated [Layers] type.
    type Layers<K: key::Key>: Layers<K>;
}

/// Tuple struct indicating array-based layer implementation.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub struct ArrayImpl<const L: usize>;

impl<const L: usize> LayerImpl for ArrayImpl<L> {
    type LayerState = [bool; L];
    type Layers<K: key::Key> = [Option<K>; L];
}

/// Modifier layer key affects what layers are active.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum ModifierKey {
    /// Activates the given layer when the held.
    Hold(LayerIndex),
}

impl ModifierKey {
    /// Create a new [input::PressedKey] and [key::ScheduledEvent] for the given keymap index.
    ///
    /// Pressing a [ModifierKey::Hold] emits a [LayerEvent::LayerActivated] event.
    pub fn new_pressed_key(
        &self,
        keymap_index: u16,
    ) -> (input::PressedKey<Self, PressedModifierKeyState>, LayerEvent) {
        match self {
            ModifierKey::Hold(layer) => {
                let event = LayerEvent::LayerActivated(*layer);
                (
                    input::PressedKey {
                        keymap_index,
                        key: *self,
                        pressed_key_state: PressedModifierKeyState,
                    },
                    event,
                )
            }
        }
    }
}

impl From<LayerEvent> for () {
    fn from(_: LayerEvent) -> Self {}
}

impl key::Key for ModifierKey {
    type Context = ();
    type ContextEvent = ();
    type Event = LayerEvent;
    type PressedKeyState = PressedModifierKeyState;

    fn new_pressed_key(
        &self,
        _context: &Self::Context,
        keymap_index: u16,
    ) -> (
        input::PressedKey<Self, Self::PressedKeyState>,
        key::PressedKeyEvents<Self::Event>,
    ) {
        let (pk, ev) = ModifierKey::new_pressed_key(self, keymap_index);
        (pk, key::PressedKeyEvents::key_event(ev))
    }
}

/// Tracks state of active layers.
pub trait LayerState: Copy + Debug {
    /// Activate the given layer.
    fn activate(&mut self, layer: LayerIndex);
    /// Deactivate the given layer.
    fn deactivate(&mut self, layer: LayerIndex);
    /// Get the active layers, from highest active layer to lowest.
    fn active_layers(&self) -> impl Iterator<Item = LayerIndex>;
}

impl<const L: usize> LayerState for [bool; L] {
    fn activate(&mut self, layer: LayerIndex) {
        debug_assert!(layer < L, "layer must be less than array length of {}", L);
        self[layer] = true;
    }

    fn deactivate(&mut self, layer: LayerIndex) {
        debug_assert!(layer < L, "layer must be less than array length of {}", L);
        self[layer] = false;
    }

    fn active_layers(&self) -> impl Iterator<Item = LayerIndex> {
        self.iter()
            .enumerate()
            .rev()
            .filter_map(|(i, &active)| if active { Some(i) } else { None })
    }
}

/// [crate::key::Context] for [LayeredKey] that tracks active layers.
#[derive(Debug, Clone, Copy)]
pub struct Context<C: key::Context, L: LayerImpl> {
    active_layers: L::LayerState,
    inner_context: C,
}

impl<C: key::Context, const L: usize> Context<C, ArrayImpl<L>> {
    /// Create a new [Context].
    pub const fn new(inner_context: C) -> Self {
        Self {
            active_layers: [false; L],
            inner_context,
        }
    }
}

impl<C: key::Context, L: LayerImpl> Context<C, L> {
    /// Activate the given layer.
    pub fn activate_layer(&mut self, layer: LayerIndex) {
        self.active_layers.activate(layer);
    }

    /// Get the active layers.
    pub fn layer_state(&self) -> &L::LayerState {
        &self.active_layers
    }
}

impl<C: key::Context, L: LayerImpl> key::Context for Context<C, L> {
    type Event = LayerEvent;

    fn handle_event(&mut self, event: Self::Event) {
        match event {
            LayerEvent::LayerActivated(layer) => {
                self.active_layers.activate(layer);
            }
            LayerEvent::LayerDeactivated(layer) => {
                self.active_layers.deactivate(layer);
            }
        }
    }
}

/// Trait for layers of [LayeredKey].
pub trait Layers<K: key::Key>: Copy + Debug + PartialEq {
    /// Get the highest active key, if any, for the given [LayerState].
    fn highest_active_key<LS: LayerState>(&self, layer_state: &LS) -> Option<K>;
}

impl<K: key::Key, const L: usize> Layers<K> for [Option<K>; L] {
    fn highest_active_key<LS: LayerState>(&self, layer_state: &LS) -> Option<K> {
        for layer in layer_state.active_layers() {
            if let Some(key) = self[layer] {
                return Some(key);
            }
        }

        None
    }
}

/// A key whose behavior depends on which layer is active.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub struct LayeredKey<K: key::Key, L: LayerImpl> {
    base: K,
    #[serde(bound(deserialize = "L::Layers<K>: serde::de::DeserializeOwned"))]
    layered: L::Layers<K>,
}

impl<const L: LayerIndex, K: key::Key> LayeredKey<K, ArrayImpl<L>> {
    /// Constructs a new [LayeredKey].
    pub fn new(base: K, layered: [Option<K>; L]) -> Self {
        Self { base, layered }
    }
}

impl<L: LayerImpl, K: key::Key> LayeredKey<K, L> {
    /// Create a new [input::PressedKey], depending on the active layers in [Context].
    pub fn new_pressed_key(
        &self,
        context: &Context<K::Context, L>,
        keymap_index: u16,
    ) -> (
        input::PressedKey<K, K::PressedKeyState>,
        key::PressedKeyEvents<K::Event>,
    ) {
        if let Some(key) = self.layered.highest_active_key(context.layer_state()) {
            return key.new_pressed_key(&context.inner_context, keymap_index);
        }

        self.base
            .new_pressed_key(&context.inner_context, keymap_index)
    }
}

impl<L: LayerImpl, K: key::Key> key::Key<K> for LayeredKey<K, L>
where
    LayerEvent: From<<K as key::Key>::Event>,
{
    type Context = Context<K::Context, L>;
    type ContextEvent = LayerEvent;
    type Event = K::Event;
    type PressedKeyState = K::PressedKeyState;

    fn new_pressed_key(
        &self,
        context: &Self::Context,
        keymap_index: u16,
    ) -> (
        input::PressedKey<K, Self::PressedKeyState>,
        key::PressedKeyEvents<Self::Event>,
    ) {
        self.new_pressed_key(context, keymap_index)
    }
}

/// Events from [ModifierKey] which affect [Context].
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum LayerEvent {
    /// Activates the given layer.
    LayerActivated(LayerIndex),
    /// Deactivates the given layer.
    LayerDeactivated(LayerIndex),
}

/// Unit-like struct, for [crate::key::PressedKeyState] of [ModifierKey].
#[derive(Debug, Clone, Copy)]
pub struct PressedModifierKeyState;

/// Type alias for [crate::input::PressedKey] of [ModifierKey].
pub type PressedModifierKey = input::PressedKey<ModifierKey, PressedModifierKeyState>;

impl key::PressedKeyState<ModifierKey> for PressedModifierKeyState {
    type Event = LayerEvent;

    fn handle_event_for(
        &mut self,
        keymap_index: u16,
        key: &ModifierKey,
        event: key::Event<Self::Event>,
    ) -> impl IntoIterator<Item = key::Event<Self::Event>> {
        match key {
            ModifierKey::Hold(layer) => match event {
                key::Event::Input(input::Event::Release { keymap_index: ki }) => {
                    if keymap_index == ki {
                        Some(key::Event::Key(LayerEvent::LayerDeactivated(*layer)))
                    } else {
                        None
                    }
                }
                _ => None,
            },
        }
    }

    fn key_output(&self, _key: &ModifierKey) -> Option<key::KeyOutput> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use key::{simple, Context as _, Key, PressedKey as _};

    #[test]
    fn test_pressing_hold_modifier_key_emits_event_activate_layer() {
        let layer = 0;
        let key = ModifierKey::Hold(layer);

        let keymap_index = 9; // arbitrary
        let (_pressed_key, layer_event) = key.new_pressed_key(keymap_index);

        assert_eq!(layer_event, LayerEvent::LayerActivated(layer));
    }

    #[test]
    fn test_releasing_hold_modifier_key_emits_event_deactivate_layer() {
        // Assemble: press a Hold layer modifier key
        let layer = 0;
        let key = ModifierKey::Hold(layer);
        let keymap_index = 9; // arbitrary
        let (mut pressed_key, _) = key.new_pressed_key(keymap_index);

        // Act: the modifier key handles "release key" input event
        let actual_events = pressed_key
            .handle_event(key::Event::Input(input::Event::Release { keymap_index }))
            .into_iter()
            .next();

        // Assert: the pressed key should have emitted a layer deactivation event
        if let Some(key::Event::Key(actual_layer_event)) = actual_events {
            let expected_layer_event = LayerEvent::LayerDeactivated(layer);
            assert_eq!(actual_layer_event, expected_layer_event);
        } else {
            panic!("Expected Some LayerDeactivated event");
        }
    }

    #[test]
    fn test_releasing_different_hold_modifier_key_does_not_emit_event() {
        // Assemble: press a Hold layer modifier key
        let layer = 0;
        let key = ModifierKey::Hold(layer);
        let keymap_index = 9; // arbitrary
        let (mut pressed_key, _) = key.new_pressed_key(keymap_index);

        // Act: the modifier key handles "release key" input event for a different key
        let different_keymap_index = keymap_index + 1;
        let different_key_released_ev = key::Event::Input(input::Event::Release {
            keymap_index: different_keymap_index,
        });
        let actual_events = pressed_key
            .handle_event(different_key_released_ev)
            .into_iter()
            .next();

        // Assert: the pressed key should not emit an event
        if actual_events.is_some() {
            panic!("Expected no event emitted");
        }
    }

    #[test]
    fn test_context_handling_event_adjusts_active_layers() {
        type L = ArrayImpl<3>;
        let mut context: Context<(), L> = Context::new(());

        context.handle_event(LayerEvent::LayerActivated(1));

        let actual_active_layers = &context.active_layers;
        assert_eq!(&[false, true, false], actual_active_layers);
    }

    #[test]
    fn test_pressing_layered_key_acts_as_base_key_when_no_layers_active() {
        // Assemble
        type L = ArrayImpl<3>;
        let context: Context<(), L> = Context::new(());
        let expected_key = simple::Key(0x04);
        let layered_key = LayeredKey {
            base: expected_key,
            layered: [
                Some(simple::Key(0x05)),
                Some(simple::Key(0x06)),
                Some(simple::Key(0x07)),
            ],
        };

        // Act: without activating a layer, press the layered key
        let keymap_index = 9; // arbitrary
        let (actual_pressed_key, actual_event) =
            layered_key.new_pressed_key(&context, keymap_index);

        // Assert
        let (expected_pressed_key, expected_event) =
            expected_key.new_pressed_key(&(), keymap_index);
        assert_eq!(actual_pressed_key, expected_pressed_key);
        assert_eq!(actual_event, expected_event);
    }

    // Terminology:
    //   "defined layer" = LayeredKey.layered[] is Some for that layer;
    //   "active layer" = Context.active_layers[] = true for that layer.

    #[test]
    fn test_pressing_layered_key_falls_through_undefined_active_layers() {
        // Assemble: layered key (with no layered definitions)
        type L = ArrayImpl<3>;
        let mut context: Context<(), L> = Context::new(());
        let expected_key = simple::Key(0x04);
        let layered_key = LayeredKey {
            base: expected_key,
            layered: [None, None, None],
        };

        // Act: activate all layers, press layered key
        context.handle_event(LayerEvent::LayerActivated(0));
        context.handle_event(LayerEvent::LayerActivated(1));
        context.handle_event(LayerEvent::LayerActivated(2));
        let keymap_index = 9; // arbitrary
        let (actual_pressed_key, actual_event) =
            layered_key.new_pressed_key(&context, keymap_index);

        // Assert
        let (expected_pressed_key, expected_event) =
            expected_key.new_pressed_key(&(), keymap_index);
        assert_eq!(actual_pressed_key, expected_pressed_key);
        assert_eq!(actual_event, expected_event);
    }

    #[test]
    fn test_pressing_layered_key_acts_as_highest_defined_active_layer() {
        // Assemble: layered key (with no layered definitions)
        type L = ArrayImpl<3>;
        let mut context: Context<(), L> = Context::new(());
        let expected_key = simple::Key(0x09);
        let layered_key = LayeredKey {
            base: simple::Key(0x04),
            layered: [
                Some(simple::Key(0x05)),
                Some(simple::Key(0x06)),
                Some(expected_key),
            ],
        };

        // Act: activate all layers, press layered key
        context.handle_event(LayerEvent::LayerActivated(0));
        context.handle_event(LayerEvent::LayerActivated(1));
        context.handle_event(LayerEvent::LayerActivated(2));
        let keymap_index = 9; // arbitrary
        let (actual_pressed_key, actual_event) =
            layered_key.new_pressed_key(&context, keymap_index);

        // Assert
        let (expected_pressed_key, expected_event) =
            expected_key.new_pressed_key(&(), keymap_index);
        assert_eq!(actual_pressed_key, expected_pressed_key);
        assert_eq!(actual_event, expected_event);
    }

    #[test]
    fn test_pressing_layered_key_with_some_transparency_acts_as_highest_defined_active_layer() {
        // Assemble: layered key (with no layered definitions)
        type L = ArrayImpl<3>;
        let mut context: Context<(), L> = Context::new(());
        let expected_key = simple::Key(0x09);
        let layered_key = LayeredKey {
            base: simple::Key(0x04),
            layered: [Some(expected_key), Some(simple::Key(0x06)), None],
        };

        // Act: activate all layers, press layered key
        context.handle_event(LayerEvent::LayerActivated(0));
        context.handle_event(LayerEvent::LayerActivated(2));
        let keymap_index = 9; // arbitrary
        let (actual_pressed_key, actual_event) =
            layered_key.new_pressed_key(&context, keymap_index);

        // Assert
        let (expected_pressed_key, expected_event) =
            expected_key.new_pressed_key(&(), keymap_index);
        assert_eq!(actual_pressed_key, expected_pressed_key);
        assert_eq!(actual_event, expected_event);
    }

    #[test]
    fn test_deserialize_ron_simple() {
        use key::simple;

        let actual_key: key::simple::Key = ron::from_str("Key(0x04)").unwrap();
        let expected_key: key::simple::Key = simple::Key(0x04);
        assert_eq!(actual_key, expected_key);
    }

    #[test]
    fn test_deserialize_ron_option_simple() {
        use key::simple;

        let actual_key: Option<key::simple::Key> = ron::from_str("Some(Key(0x04))").unwrap();
        let expected_key: Option<key::simple::Key> = Some(simple::Key(0x04));
        assert_eq!(actual_key, expected_key);
    }

    #[test]
    fn test_deserialize_ron_array1_u8() {
        let actual: [u8; 1] = ron::from_str("(5)").unwrap();
        let expected: [u8; 1] = [5];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialize_ron_array1_option_simple() {
        let actual: [Option<key::simple::Key>; 1] = ron::from_str("(Some(Key(0x04)))").unwrap();
        let expected: [Option<key::simple::Key>; 1] = [Some(simple::Key(0x04))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialize_json_option_simple() {
        let actual: Option<key::simple::Key> = serde_json::from_str(r#"4"#).unwrap();
        let expected: Option<key::simple::Key> = Some(simple::Key(0x04));
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialize_json_vec1_option_simple() {
        let actual: heapless::Vec<Option<key::simple::Key>, 1> =
            serde_json::from_str(r#"[4]"#).unwrap();
        let mut expected: heapless::Vec<Option<key::simple::Key>, 1> = heapless::Vec::new();
        expected.push(Some(simple::Key(0x04))).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialize_json_array1_option_simple() {
        let actual: [Option<key::simple::Key>; 1] = serde_json::from_str("[4]").unwrap();
        let expected: [Option<key::simple::Key>; 1] = [Some(simple::Key(0x04))];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_deserialize_ron_layered_key_simple_0layer() {
        type L = ArrayImpl<0>;
        let actual_key: LayeredKey<key::simple::Key, L> =
            ron::from_str("(base: (0x04), layered: ())").unwrap();
        let expected_key: LayeredKey<key::simple::Key, L> = LayeredKey {
            base: key::simple::Key(0x04),
            layered: [],
        };
        assert_eq!(actual_key, expected_key);
    }

    #[test]
    fn test_deserialize_json_layered_key_simple_0layer() {
        type L = ArrayImpl<0>;
        let actual_key: LayeredKey<key::simple::Key, L> =
            serde_json::from_str(r#"{"base": 4, "layered": []}"#).unwrap();
        let expected_key: LayeredKey<key::simple::Key, L> = LayeredKey {
            base: key::simple::Key(0x04),
            layered: [],
        };
        assert_eq!(actual_key, expected_key);
    }

    #[test]
    fn test_deserialize_ron_layered_key_simple_1layer_none() {
        type L = ArrayImpl<1>;
        let actual_key: LayeredKey<key::simple::Key, L> =
            ron::from_str("LayeredKey(base: Key(0x04), layered: (None))").unwrap();
        let expected_key: LayeredKey<key::simple::Key, L> = LayeredKey {
            base: key::simple::Key(0x04),
            layered: [None],
        };
        assert_eq!(actual_key, expected_key);
    }

    #[test]
    fn test_layer_state_array_active_layers() {
        let mut layer_state: [bool; 5] = [false; 5];
        layer_state.activate(0);
        layer_state.activate(1);
        layer_state.activate(3);
        let actual_active_layers: Vec<LayerIndex> = layer_state.active_layers().collect();
        let expected_active_layers: Vec<LayerIndex> = vec![3, 1, 0];

        assert_eq!(actual_active_layers, expected_active_layers);
    }
}
