#![doc = include_str!("doc_de_layered.md")]

use core::fmt::Debug;
use core::marker::Copy;

use serde::Deserialize;

use crate::input;
use crate::key;

pub use crate::init::LAYER_COUNT;

/// The type used for layer index.
pub type LayerIndex = usize;

/// Modifier layer key affects what layers are active.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum ModifierKey {
    /// Activates the given layer when the held.
    Hold(LayerIndex),
    /// Sets the default layer.
    Default(LayerIndex),
}

impl ModifierKey {
    /// Create a new [input::PressedKey] and [key::ScheduledEvent] for the given keymap index.
    ///
    /// Pressing a [ModifierKey::Hold] emits a [LayerEvent::LayerActivated] event.
    pub fn new_pressed_key(&self) -> (ModifierKeyState, LayerEvent) {
        match self {
            ModifierKey::Hold(layer) => {
                let pks = ModifierKeyState(*self);
                let event = LayerEvent::LayerActivated(*layer);
                (pks, event)
            }
            ModifierKey::Default(layer) => {
                let pks = ModifierKeyState(*self);
                let event = LayerEvent::DefaultLayerSet(*layer);
                (pks, event)
            }
        }
    }
}

impl From<LayerEvent> for () {
    fn from(_: LayerEvent) -> Self {}
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
    fn activate(&mut self, layer_index: LayerIndex) {
        debug_assert!(
            layer_index < L,
            "layer must be less than array length of {}",
            L
        );
        self[layer_index - 1] = true;
    }

    fn deactivate(&mut self, layer_index: LayerIndex) {
        debug_assert!(
            layer_index < L,
            "layer must be less than array length of {}",
            L
        );
        self[layer_index - 1] = false;
    }

    fn active_layers(&self) -> impl Iterator<Item = LayerIndex> {
        self.iter()
            .enumerate()
            .rev()
            .filter_map(|(i, &active)| if active { Some(i + 1) } else { None })
    }
}

/// [crate::key::Context] for [LayeredKey] that tracks active layers.
#[derive(Debug, Clone, Copy)]
pub struct Context {
    /// The default layer.
    pub default_layer: Option<LayerIndex>,
    /// The active layers.
    pub active_layers: [bool; LAYER_COUNT],
}

/// The default [Context] with no active layers.
pub const DEFAULT_CONTEXT: Context = Context {
    default_layer: None,
    active_layers: [false; LAYER_COUNT],
};

impl Context {
    /// Create a new [Context].
    pub const fn new() -> Self {
        DEFAULT_CONTEXT
    }
}

impl Default for Context {
    fn default() -> Self {
        DEFAULT_CONTEXT
    }
}

impl Context {
    /// Activate the given layer.
    pub fn activate_layer(&mut self, layer: LayerIndex) {
        self.active_layers.activate(layer);
    }

    /// Get the active layers.
    pub fn layer_state(&self) -> &[bool; LAYER_COUNT] {
        &self.active_layers
    }

    /// Updates the context with the [LayerEvent].
    pub fn handle_event(&mut self, event: LayerEvent) {
        match event {
            LayerEvent::LayerActivated(layer) => {
                self.active_layers.activate(layer);
            }
            LayerEvent::LayerDeactivated(layer) => {
                self.active_layers.deactivate(layer);
            }
            LayerEvent::DefaultLayerSet(0) => self.default_layer = None,
            LayerEvent::DefaultLayerSet(layer) => self.default_layer = Some(layer),
        }
    }
}

/// Errors when constructing Layers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayersError {
    /// Trying to construct more layers than the Layers can store.
    Overflow,
}

impl core::fmt::Display for LayersError {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "LayersError::Overflow")
    }
}

/// Trait for layers of [LayeredKey].
pub trait Layers<K: key::Key>: Copy + Debug {
    /// Get the highest active key, if any, for the given [LayerState].
    fn highest_active_key<LS: LayerState>(
        &self,
        layer_state: &LS,
        default_layer: Option<LayerIndex>,
    ) -> Option<(LayerIndex, &K)>;
    /// Constructs layers; return Err if the iterable has more keys than Layers can store.
    fn from_iterable<I: IntoIterator<Item = Option<K>>>(keys: I) -> Result<Self, LayersError>;
}

impl<K: key::Key + Copy, const L: usize> Layers<K> for [Option<K>; L] {
    fn highest_active_key<LS: LayerState>(
        &self,
        layer_state: &LS,
        default_layer: Option<LayerIndex>,
    ) -> Option<(LayerIndex, &K)> {
        for layer_index in layer_state.active_layers() {
            if self[layer_index - 1].is_some() {
                return self[layer_index - 1].as_ref().map(|k| (layer_index, k));
            }
        }

        match default_layer {
            Some(layer_index) if self[layer_index - 1].is_some() => {
                self[layer_index - 1].as_ref().map(|k| (layer_index, k))
            }
            _ => None,
        }
    }

    fn from_iterable<I: IntoIterator<Item = Option<K>>>(keys: I) -> Result<Self, LayersError> {
        let mut layered: [Option<K>; L] = [None; L];
        for (i, maybe_key) in keys.into_iter().enumerate() {
            if i < L {
                layered[i] = maybe_key;
            } else {
                return Err(LayersError::Overflow);
            }
        }
        Ok(layered)
    }
}

/// Constructs an array of keys for the given array.
pub const fn layered_keys<K: Copy, const L: usize>(
    keys: [Option<K>; L],
) -> [Option<K>; LAYER_COUNT] {
    let mut layered: [Option<K>; LAYER_COUNT] = [None; LAYER_COUNT];

    if L > LAYER_COUNT {
        panic!("Too many layers for layered_keys");
    }

    let mut i = 0;

    while i < L {
        layered[i] = keys[i];
        i += 1;
    }

    layered
}

/// A key whose behavior depends on which layer is active.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub struct LayeredKey<K: key::Key + Copy + PartialEq> {
    /// The base key, used when no layers are active.
    pub base: K,
    /// The layered keys, used when the corresponding layer is active.
    #[serde(deserialize_with = "deserialize_layered")]
    #[serde(bound(deserialize = "K: Deserialize<'de>"))]
    pub layered: [Option<K>; LAYER_COUNT],
}

/// Deserialize a [Layers].
fn deserialize_layered<'de, K, L: Layers<K>, D>(deserializer: D) -> Result<L, D::Error>
where
    K: key::Key + Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    let keys_vec: heapless::Vec<Option<K>, 64> = Deserialize::deserialize(deserializer)?;

    L::from_iterable(keys_vec).map_err(serde::de::Error::custom)
}

impl<K: key::Key + Copy + PartialEq> LayeredKey<K> {
    /// Constructs a new [LayeredKey].
    pub const fn new<const L: usize>(base: K, layered: [Option<K>; L]) -> Self {
        let layered = layered_keys(layered);
        Self { base, layered }
    }

    /// Maps the Key of the Key into a new type.
    pub fn map_key<T: key::Key + Copy + PartialEq>(self, f: fn(K) -> T) -> LayeredKey<T> {
        let LayeredKey { base, layered } = self;

        LayeredKey {
            base: f(base),
            layered: layered.map(|k| k.map(f)),
        }
    }

    /// Maps the Key of the Key into a new type.
    pub fn into_key<T: key::Key + Copy + PartialEq>(self) -> LayeredKey<T>
    where
        K: Into<T>,
    {
        self.map_key(|k| k.into())
    }
}

impl<K: key::Key + Copy + PartialEq> LayeredKey<K>
where
    K::Context: Into<Context>,
{
    /// Presses the key, using the highest active key, if any.
    pub fn new_pressed_key(
        &self,
        context: K::Context,
        key_path: key::KeyPath,
    ) -> (
        key::PressedKeyResult<K::PendingKeyState, K::KeyState>,
        key::PressedKeyEvents<K::Event>,
    ) {
        let layer_context: Context = context.into();
        let (layer, passthrough_key) = self
            .layered
            .highest_active_key(layer_context.layer_state(), layer_context.default_layer)
            .map(|(layer_index, key)| (layer_index, key))
            .unwrap_or((0, &self.base));

        // PRESSED KEY PATH: add Layer (0 = base, n = layer_index)
        let (pkr, pke) = passthrough_key.new_pressed_key(context, key_path);
        (pkr.add_path_item(layer as u16), pke)
    }
}

/// Events from [ModifierKey] which affect [Context].
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LayerEvent {
    /// Activates the given layer.
    LayerActivated(LayerIndex),
    /// Deactivates the given layer.
    LayerDeactivated(LayerIndex),
    /// Changes the default layer.
    DefaultLayerSet(LayerIndex),
}

/// Unit-like struct, for [crate::key::KeyState] of [ModifierKey].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModifierKeyState(ModifierKey);

impl ModifierKeyState {
    /// Handle the given event for the given key.
    pub fn handle_event(
        &mut self,
        keymap_index: u16,
        event: key::Event<LayerEvent>,
    ) -> Option<LayerEvent> {
        let ModifierKeyState(key) = self;
        match key {
            ModifierKey::Hold(layer) => match event {
                key::Event::Input(input::Event::Release { keymap_index: ki }) => {
                    if keymap_index == ki {
                        Some(LayerEvent::LayerDeactivated(*layer))
                    } else {
                        None
                    }
                }
                _ => None,
            },
            ModifierKey::Default(layer) => match event {
                key::Event::Input(input::Event::Release { keymap_index: ki }) => {
                    if keymap_index == ki {
                        Some(LayerEvent::DefaultLayerSet(*layer))
                    } else {
                        None
                    }
                }
                _ => None,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use key::keyboard;

    use key::KeyOutput;

    use key::composite::KeyState;

    #[test]
    fn test_pressing_hold_modifier_key_emits_event_activate_layer() {
        let layer = 1;
        let key = ModifierKey::Hold(layer);

        let (_pressed_key, layer_event) = key.new_pressed_key();

        assert_eq!(LayerEvent::LayerActivated(layer), layer_event);
    }

    #[test]
    fn test_releasing_hold_modifier_key_emits_event_deactivate_layer() {
        // Assemble: press a Hold layer modifier key
        let layer = 1;
        let key = ModifierKey::Hold(layer);
        let keymap_index = 9; // arbitrary
        let (mut pressed_key_state, _) = key.new_pressed_key();

        // Act: the modifier key handles "release key" input event
        let actual_events = pressed_key_state
            .handle_event(
                keymap_index,
                key::Event::Input(input::Event::Release { keymap_index }),
            )
            .into_iter()
            .next();

        // Assert: the pressed key should have emitted a layer deactivation event
        let first_ev = actual_events.into_iter().next();
        if let Some(actual_layer_event) = first_ev {
            let expected_layer_event = LayerEvent::LayerDeactivated(layer);
            assert_eq!(expected_layer_event, actual_layer_event);
        } else {
            panic!("Expected Some LayerDeactivated event");
        }
    }

    #[test]
    fn test_releasing_different_hold_modifier_key_does_not_emit_event() {
        // Assemble: press a Hold layer modifier key
        let layer = 1;
        let key = ModifierKey::Hold(layer);
        let keymap_index = 9; // arbitrary
        let (mut pressed_key_state, _) = key.new_pressed_key();

        // Act: the modifier key handles "release key" input event for a different key
        let different_keymap_index = keymap_index + 1;
        let different_key_released_ev = key::Event::Input(input::Event::Release {
            keymap_index: different_keymap_index,
        });
        let actual_events = pressed_key_state
            .handle_event(keymap_index, different_key_released_ev)
            .into_iter()
            .next();

        // Assert: the pressed key should not emit an event
        if actual_events.is_some() {
            panic!("Expected no event emitted");
        }
    }

    #[test]
    fn test_context_handling_event_adjusts_active_layers() {
        let mut context: Context = Context::default();

        context.handle_event(LayerEvent::LayerActivated(2));

        let actual_active_layers = &context.active_layers[0..3];
        assert_eq!(&[false, true, false], actual_active_layers);
    }

    #[test]
    fn test_pressing_layered_key_acts_as_base_key_when_no_layers_active() {
        // Assemble
        let context = key::composite::Context::default();
        let expected_key = keyboard::Key::new(0x04);
        let layered_key = LayeredKey::new(
            expected_key,
            [
                Some(keyboard::Key::new(0x05)),
                Some(keyboard::Key::new(0x06)),
                Some(keyboard::Key::new(0x07)),
            ],
        );

        // Act: without activating a layer, press the layered key
        let keymap_index = 9; // arbitrary
        let key_path = key::key_path(keymap_index);
        let (actual_key_state, _actual_event) = layered_key.new_pressed_key(context, key_path);

        // Assert
        let expected_key_state: KeyState = KeyState::Keyboard(expected_key.new_pressed_key());
        assert_eq!(expected_key_state, actual_key_state.unwrap_resolved(),);
    }

    #[test]
    fn test_pressing_layered_key_when_no_layers_active_has_key_code() {
        use key::KeyState as _;

        // Assemble
        let context = key::composite::Context::default();
        let expected_key = keyboard::Key::new(0x04);
        let layered_key = LayeredKey::new(
            expected_key,
            [
                Some(keyboard::Key::new(0x05)),
                Some(keyboard::Key::new(0x06)),
                Some(keyboard::Key::new(0x07)),
            ],
        );

        // Act: without activating a layer, press the layered key
        let keymap_index = 9; // arbitrary
        let key_path = key::key_path(keymap_index);
        let (actual_pressed_key, _event) = layered_key.new_pressed_key(context, key_path);

        let actual_key_output = actual_pressed_key.unwrap_resolved().key_output();

        // Assert
        let expected_pressed_key_state = expected_key.new_pressed_key();
        let expected_key_output = Some(expected_pressed_key_state.key_output());
        assert_eq!(expected_key_output, actual_key_output);
        assert_eq!(Some(KeyOutput::from_key_code(0x04)), actual_key_output,);
    }

    // Terminology:
    //   "defined layer" = LayeredKey.layered[] is Some for that layer;
    //   "active layer" = Context.active_layers[] = true for that layer.

    #[test]
    fn test_pressing_layered_key_falls_through_undefined_active_layers() {
        use key::Context as _;

        // Assemble: layered key (with no layered definitions)
        let mut context = key::composite::Context::default();
        let expected_key = keyboard::Key::new(0x04);
        let layered_key = LayeredKey::new(expected_key, [None, None, None]);

        // Act: activate all layers, press layered key
        context.handle_event(key::Event::key_event(
            0,
            LayerEvent::LayerActivated(1).into(),
        ));
        context.handle_event(key::Event::key_event(
            0,
            LayerEvent::LayerActivated(2).into(),
        ));
        context.handle_event(key::Event::key_event(
            0,
            LayerEvent::LayerActivated(3).into(),
        ));
        let keymap_index = 9; // arbitrary
        let key_path = key::key_path(keymap_index);
        let (actual_pressed_key, _actual_event) = layered_key.new_pressed_key(context, key_path);

        // Assert
        let expected_pressed_key = KeyState::Keyboard(expected_key.new_pressed_key());
        assert_eq!(expected_pressed_key, actual_pressed_key.unwrap_resolved(),);
    }

    #[test]
    fn test_pressing_layered_key_acts_as_highest_defined_active_layer() {
        use key::Context as _;

        // Assemble: layered key (with no layered definitions)
        let mut context = key::composite::Context::default();
        let expected_key = keyboard::Key::new(0x09);
        let layered_key = LayeredKey::new(
            keyboard::Key::new(0x04),
            [
                Some(keyboard::Key::new(0x05)),
                Some(keyboard::Key::new(0x06)),
                Some(expected_key),
            ],
        );

        // Act: activate all layers, press layered key
        context.handle_event(key::Event::key_event(
            0,
            LayerEvent::LayerActivated(1).into(),
        ));
        context.handle_event(key::Event::key_event(
            0,
            LayerEvent::LayerActivated(2).into(),
        ));
        context.handle_event(key::Event::key_event(
            0,
            LayerEvent::LayerActivated(3).into(),
        ));
        let keymap_index = 9; // arbitrary
        let key_path = key::key_path(keymap_index);
        let (actual_pressed_key, _actual_event) = layered_key.new_pressed_key(context, key_path);

        // Assert
        let expected_pressed_key = KeyState::Keyboard(expected_key.new_pressed_key());
        assert_eq!(expected_pressed_key, actual_pressed_key.unwrap_resolved(),);
    }

    #[test]
    fn test_pressing_layered_key_with_some_transparency_acts_as_highest_defined_active_layer() {
        use key::Context as _;

        // Assemble: layered key (with no layered definitions)
        let mut context = key::composite::Context::default();
        let expected_key = keyboard::Key::new(0x09);
        let layered_key = LayeredKey::new(
            keyboard::Key::new(0x04),
            [Some(expected_key), Some(keyboard::Key::new(0x06)), None],
        );

        // Act: activate all layers, press layered key
        context.handle_event(key::Event::key_event(
            0,
            LayerEvent::LayerActivated(1).into(),
        ));
        context.handle_event(key::Event::key_event(
            0,
            LayerEvent::LayerActivated(3).into(),
        ));
        let keymap_index = 9; // arbitrary
        let key_path = key::key_path(keymap_index);
        let (actual_pressed_key, _actual_event) = layered_key.new_pressed_key(context, key_path);

        // Assert
        let expected_pressed_key = KeyState::Keyboard(expected_key.new_pressed_key());
        assert_eq!(expected_pressed_key, actual_pressed_key.unwrap_resolved(),);
    }

    #[test]
    fn test_deserialize_ron_keyboard() {
        use key::keyboard;

        let actual_key: key::keyboard::Key = ron::from_str("Key(key_code: 0x04)").unwrap();
        let expected_key: key::keyboard::Key = keyboard::Key::new(0x04);
        assert_eq!(expected_key, actual_key);
    }

    #[test]
    fn test_deserialize_ron_option_keyboard() {
        use key::keyboard;

        let actual_key: Option<key::keyboard::Key> =
            ron::from_str("Some(Key(key_code: 0x04))").unwrap();
        let expected_key: Option<key::keyboard::Key> = Some(keyboard::Key::new(0x04));
        assert_eq!(expected_key, actual_key);
    }

    #[test]
    fn test_deserialize_ron_array1_u8() {
        let actual: [u8; 1] = ron::from_str("(5)").unwrap();
        let expected: [u8; 1] = [5];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_ron_array1_option_keyboard() {
        let actual: [Option<key::keyboard::Key>; 1] =
            ron::from_str("(Some(Key(key_code: 0x04)))").unwrap();
        let expected: [Option<key::keyboard::Key>; 1] = [Some(keyboard::Key::new(0x04))];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_json_option_keyboard() {
        let actual: Option<key::keyboard::Key> =
            serde_json::from_str(r#"{"key_code": 4}"#).unwrap();
        let expected: Option<key::keyboard::Key> = Some(keyboard::Key::new(0x04));
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_json_vec1_option_keyboard() {
        let actual: heapless::Vec<Option<key::keyboard::Key>, 1> =
            serde_json::from_str(r#"[{"key_code": 4}]"#).unwrap();
        let mut expected: heapless::Vec<Option<key::keyboard::Key>, 1> = heapless::Vec::new();
        expected.push(Some(keyboard::Key::new(0x04))).unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_json_array1_option_keyboard() {
        let actual: [Option<key::keyboard::Key>; 1] =
            serde_json::from_str(r#"[{"key_code": 4}]"#).unwrap();
        let expected: [Option<key::keyboard::Key>; 1] = [Some(keyboard::Key::new(0x04))];
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_deserialize_ron_layered_key_keyboard_0layer() {
        let actual_key: LayeredKey<key::keyboard::Key> =
            ron::from_str("(base: (key_code: 0x04), layered: [])").unwrap();
        let expected_key: LayeredKey<key::keyboard::Key> =
            LayeredKey::new(key::keyboard::Key::new(0x04), []);
        assert_eq!(expected_key, actual_key);
    }

    #[test]
    fn test_deserialize_json_layered_key_keyboard_0layer() {
        let actual_key: LayeredKey<key::keyboard::Key> =
            serde_json::from_str(r#"{"base": {"key_code": 4}, "layered": []}"#).unwrap();
        let expected_key: LayeredKey<key::keyboard::Key> =
            LayeredKey::new(key::keyboard::Key::new(0x04), []);
        assert_eq!(expected_key, actual_key);
    }

    #[test]
    fn test_deserialize_ron_layered_key_keyboard_1layer_none() {
        let actual_key: LayeredKey<key::keyboard::Key> =
            ron::from_str("LayeredKey(base: Key(key_code: 0x04), layered: [None])").unwrap();
        let expected_key: LayeredKey<key::keyboard::Key> =
            LayeredKey::new(key::keyboard::Key::new(0x04), [None]);
        assert_eq!(expected_key, actual_key);
    }

    #[test]
    fn test_layer_state_array_active_layers() {
        let mut layer_state: [bool; 5] = [false; 5];
        layer_state.activate(1);
        layer_state.activate(2);
        layer_state.activate(4);
        let actual_active_layers: Vec<LayerIndex> = layer_state.active_layers().collect();
        let expected_active_layers: Vec<LayerIndex> = vec![4, 2, 1];

        assert_eq!(expected_active_layers, actual_active_layers);
    }
}
