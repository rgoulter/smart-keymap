use core::fmt::Debug;
use core::marker::Copy;
use core::ops::Index;

use serde::Deserialize;

use crate::input;
use crate::key;

/// The type used for layer index.
pub type LayerIndex = usize;

/// The type used for set of active layers in ModifierKey.
/// (Limited to [MAX_BITSET_LAYER] layers.)
pub type LayerBitset = u32;

/// The maximum number of layers that can be represented in a [LayerBitset].
pub const MAX_BITSET_LAYER: usize = 8 * core::mem::size_of::<LayerBitset>() - 1;

/// Reference for a keyboard key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Ref {
    /// Ref to a layer modifier key.
    Modifier(u8),
    /// Ref to a layered key.
    Layered(u8),
}

/// Modifier layer key affects what layers are active.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum ModifierKey {
    /// Activates the given layer when the held.
    Hold(LayerIndex),
    /// Toggles whether the given layer is active when pressed.
    Toggle(LayerIndex),
    /// Sets the set of active layers to the given layers when the key is pressed.
    SetActiveLayers(LayerBitset),
    /// Sets the default layer.
    Default(LayerIndex),
}

impl ModifierKey {
    /// Create a new [ModifierKey] that activates the given layer when held.
    pub const fn hold(layer: LayerIndex) -> Self {
        ModifierKey::Hold(layer)
    }

    /// Create a new [ModifierKey] that toggles the given layer.
    pub const fn toggle(layer: LayerIndex) -> Self {
        ModifierKey::Toggle(layer)
    }

    /// Create a new [ModifierKey] that sets the active layers to the given slice of layers when pressed.
    ///
    /// Each LayerIndex in the slice must be less than [MAX_BITSET_LAYER].
    pub const fn set_active_layers(layers: &[LayerIndex]) -> Self {
        let mut bitset = 0;

        let mut idx = 0;
        while idx < layers.len() {
            let layer = layers[idx];
            if layer < MAX_BITSET_LAYER {
                bitset |= 1 << layer;
            } else {
                panic!("LayerIndex must be less than MAX_BITSET_LAYER");
            }
            idx += 1;
        }

        ModifierKey::SetActiveLayers(bitset)
    }

    /// Create a new [ModifierKey] that sets the active layers bitset.
    pub const fn set_active_layers_from_bitset(bitset: LayerBitset) -> Self {
        ModifierKey::SetActiveLayers(bitset)
    }

    /// Create a new [ModifierKey] that sets the default layer.
    pub const fn default(layer: LayerIndex) -> Self {
        ModifierKey::Default(layer)
    }

    /// Create a new [input::PressedKey] and [key::ScheduledEvent] for the given keymap index.
    ///
    /// Pressing a [ModifierKey::Hold] emits a [LayerEvent::LayerActivated] event.
    pub fn new_pressed_key(&self) -> (ModifierKeyState, LayerEvent) {
        match self {
            ModifierKey::Hold(layer) => (ModifierKeyState, LayerEvent::LayerActivated(*layer)),
            ModifierKey::Toggle(layer) => (ModifierKeyState, LayerEvent::LayerToggled(*layer)),
            ModifierKey::SetActiveLayers(layer_set) => {
                (ModifierKeyState, LayerEvent::LayersSet(*layer_set))
            }
            ModifierKey::Default(layer) => (ModifierKeyState, LayerEvent::DefaultLayerSet(*layer)),
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
            layer_index <= L,
            "layer must be less than array length of {}",
            L
        );
        self[layer_index - 1] = true;
    }

    fn deactivate(&mut self, layer_index: LayerIndex) {
        debug_assert!(
            layer_index <= L,
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
pub struct Context<const LAYER_COUNT: usize> {
    default_layer: Option<LayerIndex>,
    active_layers: [bool; LAYER_COUNT],
}

impl<const LAYER_COUNT: usize> Context<LAYER_COUNT> {
    /// Create a new [Context].
    pub const fn new() -> Self {
        Context {
            default_layer: None,
            active_layers: [false; LAYER_COUNT],
        }
    }
}

impl<const LAYER_COUNT: usize> Default for Context<LAYER_COUNT> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const LAYER_COUNT: usize> Context<LAYER_COUNT> {
    /// Activate the given layer.
    pub fn activate_layer(&mut self, layer: LayerIndex) {
        self.active_layers.activate(layer);
    }

    /// Get the active layers.
    pub fn layer_state(&self) -> &[bool; LAYER_COUNT] {
        &self.active_layers
    }

    /// Updates the context with the [LayerEvent].
    fn handle_event(&mut self, event: LayerEvent) {
        match event {
            LayerEvent::LayerActivated(layer) => {
                self.active_layers.activate(layer);
            }
            LayerEvent::LayerDeactivated(layer) => {
                self.active_layers.deactivate(layer);
            }
            LayerEvent::LayerToggled(layer) => {
                if self.active_layers[layer - 1] {
                    self.active_layers.deactivate(layer);
                } else {
                    self.active_layers.activate(layer);
                }
            }
            LayerEvent::LayersSet(layer_set) => {
                let max_layer = 1 + LAYER_COUNT.min(MAX_BITSET_LAYER);

                // layer 0 is always active.
                for li in 1..max_layer {
                    if (layer_set & (1 << li)) != 0 {
                        self.active_layers.activate(li);
                    } else {
                        self.active_layers.deactivate(li);
                    }
                }
            }
            LayerEvent::DefaultLayerSet(0) => self.default_layer = None,
            LayerEvent::DefaultLayerSet(layer) => self.default_layer = Some(layer),
        }
    }
}

impl<const LAYER_COUNT: usize> key::Context for Context<LAYER_COUNT> {
    type Event = LayerEvent;

    fn handle_event(&mut self, event: key::Event<Self::Event>) -> key::KeyEvents<Self::Event> {
        match event {
            key::Event::Key { key_event, .. } => {
                self.handle_event(key_event);
                key::KeyEvents::no_events()
            }
            _ => key::KeyEvents::no_events(),
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
pub trait Layers<R>: Copy + Debug {
    /// Get the highest active key, if any, for the given [LayerState].
    fn highest_active_key<LS: LayerState>(
        &self,
        layer_state: &LS,
        default_layer: Option<LayerIndex>,
    ) -> Option<(LayerIndex, R)>;
    /// Constructs layers; return Err if the iterable has more keys than Layers can store.
    fn from_iterable<I: IntoIterator<Item = Option<R>>>(keys: I) -> Result<Self, LayersError>;
}

impl<R: Copy + Debug, const L: usize> Layers<R> for [Option<R>; L] {
    fn highest_active_key<LS: LayerState>(
        &self,
        layer_state: &LS,
        default_layer: Option<LayerIndex>,
    ) -> Option<(LayerIndex, R)> {
        for layer_index in layer_state.active_layers() {
            if self[layer_index - 1].is_some() {
                return self[layer_index - 1].map(|k| (layer_index, k));
            }
        }

        match default_layer {
            Some(layer_index) if self[layer_index - 1].is_some() => {
                self[layer_index - 1].map(|k| (layer_index, k))
            }
            _ => None,
        }
    }

    fn from_iterable<I: IntoIterator<Item = Option<R>>>(keys: I) -> Result<Self, LayersError> {
        let mut layered: [Option<R>; L] = [None; L];
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
pub const fn layered_keys<K: Copy, const L: usize, const LAYER_COUNT: usize>(
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
pub struct LayeredKey<R: Copy + Debug + PartialEq, const LAYER_COUNT: usize> {
    /// The base key, used when no layers are active.
    pub base: R,
    /// The layered keys, used when the corresponding layer is active.
    #[serde(deserialize_with = "deserialize_layered")]
    #[serde(bound(deserialize = "R: Deserialize<'de>"))]
    pub layered: [Option<R>; LAYER_COUNT],
}

/// Deserialize a [Layers].
fn deserialize_layered<'de, R, L: Layers<R>, D>(deserializer: D) -> Result<L, D::Error>
where
    R: Deserialize<'de>,
    D: serde::Deserializer<'de>,
{
    let keys_vec: heapless::Vec<Option<R>, 64> = Deserialize::deserialize(deserializer)?;

    L::from_iterable(keys_vec).map_err(serde::de::Error::custom)
}

impl<R: Copy + Debug + PartialEq, const LAYER_COUNT: usize> LayeredKey<R, LAYER_COUNT> {
    /// Constructs a new [LayeredKey].
    pub const fn new<const L: usize>(base: R, layered: [Option<R>; L]) -> Self {
        let layered = layered_keys(layered);
        Self { base, layered }
    }
}

impl<R: Copy + Debug + PartialEq, const LAYER_COUNT: usize> LayeredKey<R, LAYER_COUNT> {
    /// Presses the key, using the highest active key, if any.
    fn new_pressed_key(&self, context: &Context<LAYER_COUNT>) -> key::NewPressedKey<R> {
        let (_layer, passthrough_ref) = self
            .layered
            .highest_active_key(context.layer_state(), context.default_layer)
            .unwrap_or((0, self.base));

        key::NewPressedKey::key(passthrough_ref)
    }
}

/// Events from [ModifierKey] which affect [Context].
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LayerEvent {
    /// Activates the given layer.
    LayerActivated(LayerIndex),
    /// Deactivates the given layer.
    LayerDeactivated(LayerIndex),
    /// Toggles the given layer.
    LayerToggled(LayerIndex),
    /// Sets the active layers to the given set of layers.
    LayersSet(LayerBitset),
    /// Changes the default layer.
    DefaultLayerSet(LayerIndex),
}

/// Struct for layer system pending key state. (No pending state).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PendingKeyState;

/// [crate::key::KeyState] of [ModifierKey].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModifierKeyState;

impl ModifierKeyState {
    /// Handle the given event for the given key.
    pub fn handle_event(
        &mut self,
        keymap_index: u16,
        event: key::Event<LayerEvent>,
        key: &ModifierKey,
    ) -> Option<LayerEvent> {
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
            ModifierKey::Toggle(_) => None,
            ModifierKey::SetActiveLayers(_layer_set) => None,
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

/// The [key::System] implementation for layer system keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct System<
    R: Copy + Debug + PartialEq,
    ModifierKeys: Index<usize, Output = ModifierKey>,
    LayeredKeys: Index<usize, Output = LayeredKey<R, LAYER_COUNT>>,
    const LAYER_COUNT: usize,
> {
    modifier_keys: ModifierKeys,
    layered_keys: LayeredKeys,
}

impl<
        R: Copy + Debug + PartialEq,
        ModifierKeys: Index<usize, Output = ModifierKey>,
        LayeredKeys: Index<usize, Output = LayeredKey<R, LAYER_COUNT>>,
        const LAYER_COUNT: usize,
    > System<R, ModifierKeys, LayeredKeys, LAYER_COUNT>
{
    /// Constructs a new [System] with the given key data.
    pub const fn new(modifier_keys: ModifierKeys, layered_keys: LayeredKeys) -> Self {
        Self {
            modifier_keys,
            layered_keys,
        }
    }
}

impl<
        R: Copy + Debug + PartialEq,
        ModifierKeys: Debug + Index<usize, Output = ModifierKey>,
        LayeredKeys: Debug + Index<usize, Output = LayeredKey<R, LAYER_COUNT>>,
        const LAYER_COUNT: usize,
    > key::System<R> for System<R, ModifierKeys, LayeredKeys, LAYER_COUNT>
{
    type Ref = Ref;
    type Context = Context<LAYER_COUNT>;
    type Event = LayerEvent;
    type PendingKeyState = PendingKeyState;
    type KeyState = ModifierKeyState;

    fn new_pressed_key(
        &self,
        keymap_index: u16,
        context: &Self::Context,
        key_ref: Ref,
    ) -> (
        key::PressedKeyResult<R, Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        match key_ref {
            Ref::Modifier(i) => {
                let key = self.modifier_keys[i as usize];
                let (m_ks, lmod_ev) = key.new_pressed_key();
                let pks = key::PressedKeyResult::Resolved(m_ks);
                let pke = key::KeyEvents::event(key::Event::key_event(keymap_index, lmod_ev));
                (pks, pke)
            }
            Ref::Layered(i) => {
                let key = &self.layered_keys[i as usize];
                let npk = key.new_pressed_key(context);
                (
                    key::PressedKeyResult::NewPressedKey(npk),
                    key::KeyEvents::no_events(),
                )
            }
        }
    }

    fn update_pending_state(
        &self,
        _pending_state: &mut Self::PendingKeyState,
        _keymap_index: u16,
        _context: &Self::Context,
        _key_ref: Ref,
        _event: key::Event<Self::Event>,
    ) -> (Option<key::NewPressedKey<R>>, key::KeyEvents<Self::Event>) {
        panic!()
    }

    fn update_state(
        &self,
        key_state: &mut Self::KeyState,
        key_ref: &Self::Ref,
        _context: &Self::Context,
        keymap_index: u16,
        event: key::Event<Self::Event>,
    ) -> key::KeyEvents<Self::Event> {
        match key_ref {
            Ref::Modifier(mod_key_index) => {
                let mod_key = &self.modifier_keys[*mod_key_index as usize];
                let maybe_ev = key_state.handle_event(keymap_index, event, mod_key);
                maybe_ev.map_or(key::KeyEvents::no_events(), |ev| {
                    key::KeyEvents::event(key::Event::key_event(keymap_index, ev))
                })
            }
            _ => key::KeyEvents::no_events(),
        }
    }

    fn key_output(
        &self,
        _key_ref: &Self::Ref,
        _key_state: &Self::KeyState,
    ) -> Option<key::KeyOutput> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::key::keyboard;

    use crate::key::System as _;

    const LAYER_COUNT: usize = 8;

    type Context = super::Context<LAYER_COUNT>;

    #[test]
    fn test_sizeof_ref() {
        assert_eq!(2, core::mem::size_of::<Ref>());
    }

    #[test]
    fn test_sizeof_event() {
        assert_eq!(16, core::mem::size_of::<LayerEvent>());
    }

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
                &key,
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
            .handle_event(keymap_index, different_key_released_ev, &key)
            .into_iter()
            .next();

        // Assert: the pressed key should not emit an event
        if actual_events.is_some() {
            panic!("Expected no event emitted");
        }
    }

    #[test]
    fn test_context_handling_event_adjusts_active_layers() {
        let mut context = Context::default();

        context.handle_event(LayerEvent::LayerActivated(2));

        let actual_active_layers = &context.active_layers[0..3];
        assert_eq!(&[false, true, false], actual_active_layers);
    }

    #[test]
    fn test_pressing_layered_key_acts_as_base_key_when_no_layers_active() {
        // Assemble
        let context = Context::default();
        let expected_ref = keyboard::Ref::KeyCode(0x04);
        let layered_key = LayeredKey::new(
            expected_ref,
            [
                Some(keyboard::Ref::KeyCode(0x05)),
                Some(keyboard::Ref::KeyCode(0x06)),
                Some(keyboard::Ref::KeyCode(0x07)),
            ],
        );
        let system = System::new([], [layered_key]);

        // Act: without activating a layer, press the layered key
        let keymap_index = 9; // arbitrary
        let key_ref = Ref::Layered(0);
        let (pkr, _pke) = system.new_pressed_key(keymap_index, &context, key_ref);

        // Assert
        let expected_pkr =
            key::PressedKeyResult::NewPressedKey(key::NewPressedKey::Key(expected_ref));
        assert_eq!(expected_pkr, pkr,);
    }

    // Terminology:
    //   "defined layer" = LayeredKey.layered[] is Some for that layer;
    //   "active layer" = Context.active_layers[] = true for that layer.

    #[test]
    fn test_pressing_layered_key_falls_through_undefined_active_layers() {
        // Assemble: layered key (with no layered definitions)
        let mut context = Context::default();
        let expected_ref = keyboard::Ref::KeyCode(0x04);
        let layered_key = LayeredKey::new(expected_ref, [None, None, None]);
        let system = System::new([], [layered_key]);

        // Act: activate all layers, press layered key
        context.handle_event(LayerEvent::LayerActivated(1));
        context.handle_event(LayerEvent::LayerActivated(2));
        context.handle_event(LayerEvent::LayerActivated(3));
        let keymap_index = 9; // arbitrary
        let key_ref = Ref::Layered(0);
        let (pkr, _pke) = system.new_pressed_key(keymap_index, &context, key_ref);

        // Assert
        let expected_pkr =
            key::PressedKeyResult::NewPressedKey(key::NewPressedKey::Key(expected_ref));
        assert_eq!(expected_pkr, pkr,);
    }

    #[test]
    fn test_pressing_layered_key_acts_as_highest_defined_active_layer() {
        // Assemble: layered key (with no layered definitions)
        let mut context = Context::default();
        let expected_ref = keyboard::Ref::KeyCode(0x09);
        let layered_key = LayeredKey::new(
            keyboard::Ref::KeyCode(0x04),
            [
                Some(keyboard::Ref::KeyCode(0x05)),
                Some(keyboard::Ref::KeyCode(0x06)),
                Some(expected_ref),
            ],
        );
        let system = System::new([], [layered_key]);

        // Act: activate all layers, press layered key
        context.handle_event(LayerEvent::LayerActivated(1));
        context.handle_event(LayerEvent::LayerActivated(2));
        context.handle_event(LayerEvent::LayerActivated(3));
        let keymap_index = 9; // arbitrary
        let key_ref = Ref::Layered(0);
        let (pkr, _pke) = system.new_pressed_key(keymap_index, &context, key_ref);

        // Assert
        let expected_pkr =
            key::PressedKeyResult::NewPressedKey(key::NewPressedKey::Key(expected_ref));
        assert_eq!(expected_pkr, pkr,);
    }

    #[test]
    fn test_pressing_layered_key_with_some_transparency_acts_as_highest_defined_active_layer() {
        // Assemble: layered key (with no layered definitions)
        let mut context = Context::default();
        let expected_ref = keyboard::Ref::KeyCode(0x09);
        let layered_key = LayeredKey::new(
            keyboard::Ref::KeyCode(0x04),
            [Some(expected_ref), Some(keyboard::Ref::KeyCode(0x06)), None],
        );
        let system = System::new([], [layered_key]);

        // Act: activate all layers, press layered key
        context.handle_event(LayerEvent::LayerActivated(1));
        context.handle_event(LayerEvent::LayerActivated(3));
        let keymap_index = 9; // arbitrary
        let key_ref = Ref::Layered(0);
        let (pkr, _pke) = system.new_pressed_key(keymap_index, &context, key_ref);

        // Assert
        let expected_pkr =
            key::PressedKeyResult::NewPressedKey(key::NewPressedKey::Key(expected_ref));
        assert_eq!(expected_pkr, pkr,);
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

    #[test]
    fn test_pressing_toggle_modifier_key_emits_event_layer_toggled() {
        // Assemble
        let layer = 1;
        let key = ModifierKey::Toggle(layer);

        // Act
        let (_pressed_key, layer_event) = key.new_pressed_key();

        // Assert
        assert_eq!(LayerEvent::LayerToggled(layer), layer_event);
    }
}
