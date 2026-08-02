use core::fmt::Debug;
use core::marker::Copy;
use core::ops::{BitAnd, BitOr, Index, Not};

use serde::Deserialize;

use crate::input;
use crate::key;
use crate::key::KeyboardModifiers;
use crate::slice::Slice;

/// The type used for layer index.
///
/// Layer modifiers and layer events use **1-based** indices (`1` = first non-base
/// layer). Index `0` is reserved (base / always active). Call sites must only
/// pass indices in range for the keymap's layer count; out-of-range values are
/// a construction bug and are checked with [`debug_assert`] in debug builds.
pub type LayerIndex = u32;

/// Fixed-capacity bitset of layers for modifier / conditional-layer state.
///
/// Bit `i` corresponds to layer index `i`. Capacity is [Self::BITS] layers
/// (indices `0..=[MAX_BITSET_LAYER]`).
///
/// This is a thin `no_std` newtype over [`u32`] so keymap `const` data can build
/// bitsets without a heap or an external bitset crate. Widening capacity later
/// is a storage-type change (`u64`, `u128`, or multi-limb) behind the same API.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, Eq, PartialEq, Deserialize)]
#[serde(transparent)]
pub struct LayerBitset(u32);

impl LayerBitset {
    /// Number of representable layer bits.
    pub const BITS: usize = 32;

    /// Empty bitset (no layers selected).
    pub const EMPTY: Self = Self(0);

    /// Bitset with every representable layer bit set.
    pub const ALL: Self = Self(u32::MAX);

    /// Construct from raw bits (bit `i` = layer `i`).
    pub const fn from_bits(bits: u32) -> Self {
        Self(bits)
    }

    /// Raw bit pattern.
    pub const fn bits(self) -> u32 {
        self.0
    }

    /// Returns true if bit `index` is set.
    pub const fn contains(self, index: usize) -> bool {
        index < Self::BITS && (self.0 & (1u32 << index)) != 0
    }

    /// Returns a copy with bit `index` set, if `index` is in range.
    pub const fn insert(self, index: usize) -> Self {
        if index < Self::BITS {
            Self(self.0 | (1u32 << index))
        } else {
            self
        }
    }

    /// Returns a copy with bit `index` cleared, if `index` is in range.
    pub const fn remove(self, index: usize) -> Self {
        if index < Self::BITS {
            Self(self.0 & !(1u32 << index))
        } else {
            self
        }
    }

    /// Returns true if every bit set in `other` is also set in `self`.
    pub const fn is_superset_of(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl Default for LayerBitset {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl From<u32> for LayerBitset {
    fn from(bits: u32) -> Self {
        Self::from_bits(bits)
    }
}

impl From<LayerBitset> for u32 {
    fn from(bitset: LayerBitset) -> Self {
        bitset.bits()
    }
}

impl BitAnd for LayerBitset {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitOr for LayerBitset {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl Not for LayerBitset {
    type Output = Self;

    fn not(self) -> Self::Output {
        Self(!self.0)
    }
}

/// The maximum layer bit index representable in a [LayerBitset].
///
/// For a 32-bit [LayerBitset], this is 31 (bits 0..=31, i.e. 32 layers).
pub const MAX_BITSET_LAYER: usize = LayerBitset::BITS - 1;

/// The bitset with all representable modifier layers in the mask.
///
/// Includes bit [MAX_BITSET_LAYER].
pub const BITSET_MASK_ALL: LayerBitset = LayerBitset::ALL;

/// Struct for modifying layers with a bitset.
#[repr(C)]
#[derive(Debug, Deserialize, Clone, Copy, Eq, PartialEq)]
pub struct ModifierBitset {
    /// The set of layers modified.
    pub layers: LayerBitset,
    /// The mask for which layers are affected by the modification.
    #[serde(default = "default_modifier_bitset_mask")]
    pub mask: LayerBitset,
}

fn default_modifier_bitset_mask() -> LayerBitset {
    BITSET_MASK_ALL
}

/// Reference for a keyboard key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Ref {
    /// Ref to a layer modifier key.
    Modifier(u8),
    /// Ref to a layered key.
    Layered(u8),
}

/// Target of a layer-lock action.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum LayerLockTarget {
    /// Highest currently active layer.
    HighestActive,
    /// A specific layer.
    Layer(LayerIndex),
}

/// Modifier layer key affects what layers are active.
#[derive(Debug, Deserialize, Clone, Copy, PartialEq)]
pub enum ModifierKey {
    /// Activates the given layer when held.
    Hold(LayerIndex, KeyboardModifiers),
    /// Toggles whether the given layer is active when pressed.
    Toggle(LayerIndex),
    /// Sticky layer modifier, similar to sticky modifier key.
    ///
    /// Acts the same as `Hold` variant if interrupted.
    /// If tapped, then the layer is activated for the next key tap.
    Sticky(LayerIndex),
    /// Sets the set of active layers to the given layers when the key is pressed.
    SetActiveLayers(ModifierBitset),
    /// Sets the default layer.
    Default(LayerIndex),
    /// Layer lock key.
    ///
    /// Inverts lock on the [LayerLockTarget]: if unlocked, lock and activate; if locked,
    /// unlock and deactivate.
    ///
    /// While a layer is locked, releasing a [`ModifierKey::Hold`] that activated it leaves the
    /// layer active. Pressing that [`ModifierKey::Hold`] again unlocks and turns the layer off.
    Lock(LayerLockTarget),
}

impl ModifierKey {
    /// Create a new [ModifierKey] that activates the given layer when held.
    pub const fn hold(layer: LayerIndex) -> Self {
        ModifierKey::Hold(layer, key::KeyboardModifiers::NONE)
    }

    /// Adds keyboard modifiers to a Hold modifier key.
    pub const fn with_keyboard_modifiers(self, mods: key::KeyboardModifiers) -> Self {
        match self {
            ModifierKey::Hold(layer, _) => ModifierKey::Hold(layer, mods),
            other => other,
        }
    }

    /// Create a new [ModifierKey] that activates the layer when held, or makes it sticky when tapped.
    pub const fn sticky(layer: LayerIndex) -> Self {
        ModifierKey::Sticky(layer)
    }

    /// Create a new [ModifierKey] that toggles the given layer.
    pub const fn toggle(layer: LayerIndex) -> Self {
        ModifierKey::Toggle(layer)
    }

    /// Create a new [ModifierKey] that sets the active layers to the given slice of layers when pressed.
    ///
    /// Each LayerIndex in the slice must be at most [MAX_BITSET_LAYER].
    pub const fn set_active_layers(layers: &[LayerIndex]) -> Self {
        let mut bitset = LayerBitset::EMPTY;

        let mut idx = 0;
        while idx < layers.len() {
            let layer = layers[idx] as usize;
            if layer <= MAX_BITSET_LAYER {
                bitset = bitset.insert(layer);
            } else {
                panic!("LayerIndex must be at most MAX_BITSET_LAYER");
            }
            idx += 1;
        }

        let mask = BITSET_MASK_ALL;
        ModifierKey::SetActiveLayers(ModifierBitset {
            layers: bitset,
            mask,
        })
    }

    /// Create a new [ModifierKey] that sets the active layers bitset.
    pub const fn set_active_layers_from_bitset(bitset: LayerBitset) -> Self {
        let mask = BITSET_MASK_ALL;
        ModifierKey::SetActiveLayers(ModifierBitset {
            layers: bitset,
            mask,
        })
    }

    /// Create a new [ModifierKey] that sets the active layers bitset.
    pub const fn set_active_layers_from_bitset_with_mask(
        layers: LayerBitset,
        mask: LayerBitset,
    ) -> Self {
        ModifierKey::SetActiveLayers(ModifierBitset { layers, mask })
    }

    /// Create a new [ModifierKey] that sets the default layer.
    pub const fn default(layer: LayerIndex) -> Self {
        ModifierKey::Default(layer)
    }

    /// Create a [ModifierKey::Lock] that targets the highest currently active layer.
    pub const fn lock() -> Self {
        ModifierKey::Lock(LayerLockTarget::HighestActive)
    }

    /// Create a [ModifierKey::Lock] that targets a specific layer.
    pub const fn lock_layer(layer: LayerIndex) -> Self {
        ModifierKey::Lock(LayerLockTarget::Layer(layer))
    }

    /// Create a new [input::PressedKey] and [key::ScheduledEvent] for the given keymap index.
    ///
    /// Pressing a [ModifierKey::Hold] emits a [LayerEvent::Activated] event.
    pub fn new_pressed_key(&self) -> (ModifierKeyState, Option<LayerEvent>) {
        match self {
            ModifierKey::Hold(layer, _) => {
                (ModifierKeyState::new(), Some(LayerEvent::Activated(*layer)))
            }
            ModifierKey::Toggle(layer) => {
                (ModifierKeyState::new(), Some(LayerEvent::Toggled(*layer)))
            }
            ModifierKey::Sticky(layer) => (
                ModifierKeyState::sticky(),
                Some(LayerEvent::StickyActivated(*layer)),
            ),
            ModifierKey::SetActiveLayers(modifier_bitset) => (
                ModifierKeyState::new(),
                Some(LayerEvent::Set(*modifier_bitset)),
            ),
            ModifierKey::Default(layer) => (
                ModifierKeyState::new(),
                Some(LayerEvent::SetDefault(*layer)),
            ),
            ModifierKey::Lock(target) => (
                ModifierKeyState::new(),
                Some(LayerEvent::LockInvert(*target)),
            ),
        }
    }
}

impl From<LayerEvent> for () {
    fn from(_: LayerEvent) -> Self {}
}

/// Style of activating a layer.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActivationStyle {
    /// Regular layer activation.
    Regular,
    /// Sticky layer activation.
    ///
    /// Sticky layer activation is similar to sticky key modifiers.
    /// The sticky layer activation is implemented using the sticky layer modifier key.
    Sticky,
}

/// State of an individual layer: active or inactive.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Activity {
    /// The layer is active.
    Active(ActivationStyle),
    /// The layer is inactive.
    Inactive,
}

impl Activity {
    /// Returns true if the layer is active.
    pub fn is_active(&self) -> bool {
        matches!(self, Activity::Active(_))
    }
}

/// Tracks state of active layers.
pub trait LayerState: Copy + Debug {
    /// Activate the given layer.
    fn activate(&mut self, layer: LayerIndex, style: ActivationStyle);
    /// Deactivate the given layer.
    fn deactivate(&mut self, layer: LayerIndex);
    /// Get the active layers, from highest active layer to lowest.
    fn active_layers(&self) -> impl Iterator<Item = LayerIndex>;
}

impl<const L: usize> LayerState for [Activity; L] {
    fn activate(&mut self, layer_index: LayerIndex, style: ActivationStyle) {
        let layer_index: usize = layer_index as usize;
        // 1-based layer indices; keymap construction / NCL validation must ensure range.
        debug_assert!(
            (1..=L).contains(&layer_index),
            "layer must be in 1..={} (got {})",
            L,
            layer_index
        );
        self[layer_index - 1] = Activity::Active(style);
    }

    fn deactivate(&mut self, layer_index: LayerIndex) {
        let layer_index: usize = layer_index as usize;
        debug_assert!(
            (1..=L).contains(&layer_index),
            "layer must be in 1..={} (got {})",
            L,
            layer_index
        );
        self[layer_index - 1] = Activity::Inactive;
    }

    fn active_layers(&self) -> impl Iterator<Item = LayerIndex> {
        self.iter().enumerate().rev().filter_map(|(i, activity)| {
            if activity.is_active() {
                Some(i as LayerIndex + 1)
            } else {
                None
            }
        })
    }
}

struct ActiveLayersDebugHelper<'a, const LAYER_COUNT: usize> {
    active_layers: &'a [Activity; LAYER_COUNT],
}

impl<const LAYER_COUNT: usize> core::fmt::Debug for ActiveLayersDebugHelper<'_, LAYER_COUNT> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Reverse-find the last Active layer to avoid printing large arrays.
        let last_active_pos = self
            .active_layers
            .iter()
            .rposition(|&pc| pc.is_active())
            .map_or(0, |pos| pos + 1);
        if last_active_pos < LAYER_COUNT {
            f.debug_list()
                .entries(&self.active_layers[..last_active_pos])
                .finish_non_exhaustive()
        } else {
            f.debug_list().entries(&self.active_layers[..]).finish()
        }
    }
}

/// Conditional layer rule: activate [Self::then_layer] while every layer in
/// [Self::if_layers] is active.
///
/// Layer indices are 1-based (same as [ModifierKey]). Bit `i` of
/// [Self::if_layers] corresponds to layer `i` (same layout as [ModifierBitset]).
#[repr(C)]
#[derive(Debug, Deserialize, Clone, Copy, Eq, PartialEq)]
pub struct ConditionalLayer {
    /// Layer activated while the condition holds.
    pub then_layer: LayerIndex,
    /// Bitset of layers that must all be active.
    pub if_layers: LayerBitset,
}

impl ConditionalLayer {
    /// Constructs a rule from a then-layer and an if-layers bitset.
    pub const fn new(then_layer: LayerIndex, if_layers: LayerBitset) -> Self {
        Self {
            then_layer,
            if_layers,
        }
    }

    /// Constructs a rule from a then-layer and if-layer indices.
    pub const fn from_if_layers(then_layer: LayerIndex, if_layers: &[LayerIndex]) -> Self {
        let mut bitset = LayerBitset::EMPTY;
        let mut i = 0;
        while i < if_layers.len() {
            let layer = if_layers[i] as usize;
            if layer <= MAX_BITSET_LAYER {
                bitset = bitset.insert(layer);
            }
            i += 1;
        }
        Self {
            then_layer,
            if_layers: bitset,
        }
    }
}

/// Configuration for layered keys / sticky layers / conditional layers.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Config<const CONDITIONAL_LAYER_COUNT: usize = 0> {
    /// Timeout (ms) after which an unused sticky layer deactivates.
    ///
    /// When [None], sticky layers stay active until another key is used.
    ///
    /// The timeout starts when a sticky layer modifier is *released*
    /// without interruption (sticky committed). If another key is pressed
    /// while sticky is active, the timeout is cancelled / ignored.
    #[serde(default)]
    pub sticky_timeout: Option<u16>,

    /// Rules that activate a then-layer when all of their if-layers are active.
    #[serde(default)]
    pub conditional_layers: Slice<ConditionalLayer, CONDITIONAL_LAYER_COUNT>,
}

/// Default layered config (no sticky timeout, no conditional layers).
pub const DEFAULT_CONFIG: Config = Config {
    sticky_timeout: None,
    conditional_layers: Slice::from_slice(&[]),
};

impl<const CONDITIONAL_LAYER_COUNT: usize> Config<CONDITIONAL_LAYER_COUNT> {
    /// Constructs a new default [Config].
    pub const fn new() -> Self {
        Self {
            sticky_timeout: None,
            conditional_layers: Slice::from_slice(&[]),
        }
    }
}

impl<const CONDITIONAL_LAYER_COUNT: usize> Default for Config<CONDITIONAL_LAYER_COUNT> {
    fn default() -> Self {
        Self::new()
    }
}

/// [crate::key::Context] for [LayeredKey] that tracks active layers.
#[derive(Clone, Copy)]
pub struct Context<const LAYER_COUNT: usize, const CONDITIONAL_LAYER_COUNT: usize = 0> {
    config: Config<CONDITIONAL_LAYER_COUNT>,
    default_layer: Option<LayerIndex>,
    active_layers: [Activity; LAYER_COUNT],
    /// Bitset of locked layers (bit `i` = layer `i` is locked).
    ///
    /// Locked layers stay active when a [ModifierKey::Hold] for that layer is released.
    locked_layers: LayerBitset,
    // Keymap index which was pressed while a layer was sticky.
    pressed_keymap_index: Option<u16>,
    // Invalidates pending sticky-timeout events when advanced.
    sticky_timeout_id: u8,
}

impl<const LAYER_COUNT: usize, const CONDITIONAL_LAYER_COUNT: usize> Debug
    for Context<LAYER_COUNT, CONDITIONAL_LAYER_COUNT>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Context")
            .field("config", &self.config)
            .field("default_layer", &self.default_layer)
            .field(
                "active_layers",
                &ActiveLayersDebugHelper {
                    active_layers: &self.active_layers,
                },
            )
            .field("locked_layers", &self.locked_layers)
            .field("pressed_keymap_index", &self.pressed_keymap_index)
            .field("sticky_timeout_id", &self.sticky_timeout_id)
            .finish()
    }
}

impl<const LAYER_COUNT: usize, const CONDITIONAL_LAYER_COUNT: usize>
    Context<LAYER_COUNT, CONDITIONAL_LAYER_COUNT>
{
    /// Create a new [Context].
    pub const fn new() -> Self {
        Self::from_config(Config::new())
    }

    /// Constructs a context from the given config.
    pub const fn from_config(config: Config<CONDITIONAL_LAYER_COUNT>) -> Self {
        Context {
            config,
            default_layer: None,
            active_layers: [Activity::Inactive; LAYER_COUNT],
            locked_layers: LayerBitset::EMPTY,
            pressed_keymap_index: None,
            sticky_timeout_id: 0,
        }
    }

    /// Re-construct from context's [Config], clearing active layers and sticky state.
    pub fn reset(&mut self) {
        *self = Self::from_config(self.config);
    }

    fn invalidate_sticky_timeouts(&mut self) {
        self.sticky_timeout_id = self.sticky_timeout_id.wrapping_add(1);
    }

    fn deactivate_sticky_layer(&mut self, layer: LayerIndex) {
        self.active_layers.deactivate(layer);
        self.clear_layer_lock(layer);
        self.pressed_keymap_index = None;
        self.invalidate_sticky_timeouts();
        self.apply_conditional_layers();
    }

    /// Returns true if `layer` is currently locked.
    pub fn is_layer_locked(&self, layer: LayerIndex) -> bool {
        self.locked_layers.contains(layer as usize)
    }

    fn set_layer_lock(&mut self, layer: LayerIndex) {
        self.locked_layers = self.locked_layers.insert(layer as usize);
    }

    fn clear_layer_lock(&mut self, layer: LayerIndex) {
        self.locked_layers = self.locked_layers.remove(layer as usize);
    }

    /// Highest active layer index, if any (1-based layers only).
    fn highest_active_layer(&self) -> Option<LayerIndex> {
        self.active_layers.active_layers().next()
    }

    /// Invert lock on `layer`.
    ///
    /// If unlocked: lock and turn the layer on.
    /// If locked: unlock and turn the layer off.
    ///
    /// `layer` must be a valid 1-based index (`1..=LAYER_COUNT`); debug builds assert this
    /// via [LayerState::activate] / [LayerState::deactivate].
    fn lock_invert(&mut self, layer: LayerIndex) {
        if self.is_layer_locked(layer) {
            self.clear_layer_lock(layer);
            self.active_layers.deactivate(layer);
            if self.sticky_layer() == Some(layer) {
                self.pressed_keymap_index = None;
                self.invalidate_sticky_timeouts();
            }
        } else {
            self.set_layer_lock(layer);
            self.active_layers.activate(layer, ActivationStyle::Regular);
            // Lock supersedes sticky wait on this layer.
            if self.sticky_layer() == Some(layer) {
                self.pressed_keymap_index = None;
                self.invalidate_sticky_timeouts();
            } else {
                self.invalidate_sticky_timeouts();
            }
        }
        self.apply_conditional_layers();
    }

    /// Bitset of currently active layers (bit `i` = layer `i`).
    fn active_layers_bitset(&self) -> LayerBitset {
        let max_layer = 1 + LAYER_COUNT.min(MAX_BITSET_LAYER);
        (1..max_layer).fold(LayerBitset::EMPTY, |bits, li| {
            if self.active_layers[li - 1].is_active() {
                bits.insert(li)
            } else {
                bits
            }
        })
    }

    /// One pass over conditional rules; returns whether any then-layer changed.
    fn apply_conditional_layers_once(&mut self) -> bool {
        // Copy rules so we can mutate active_layers while iterating.
        let rules = self.config.conditional_layers;
        let active = self.active_layers_bitset();
        rules.as_slice().iter().fold(false, |changed, rule| {
            let should = active.is_superset_of(rule.if_layers);
            let is_active = active.contains(rule.then_layer as usize);
            if should == is_active {
                changed
            } else if should {
                self.active_layers
                    .activate(rule.then_layer, ActivationStyle::Regular);
                true
            } else if self.is_layer_locked(rule.then_layer) {
                // Locked layers stay active even when conditional if-layers drop.
                changed
            } else {
                self.active_layers.deactivate(rule.then_layer);
                true
            }
        })
    }

    /// Evaluate conditional layer rules until stable, or up to one pass per rule.
    ///
    /// Nested rules (a then-layer used as an if-layer of another rule) settle
    /// without depending on rule definition order. The pass limit bounds the
    /// work for a pure dependency chain and prevents non-termination if rules
    /// disagree about the same then-layer.
    fn apply_conditional_layers(&mut self) {
        for _ in 0..CONDITIONAL_LAYER_COUNT {
            if !self.apply_conditional_layers_once() {
                break;
            }
        }
    }
}

impl<const LAYER_COUNT: usize, const CONDITIONAL_LAYER_COUNT: usize> Default
    for Context<LAYER_COUNT, CONDITIONAL_LAYER_COUNT>
{
    fn default() -> Self {
        Self::new()
    }
}

impl<const LAYER_COUNT: usize, const CONDITIONAL_LAYER_COUNT: usize>
    Context<LAYER_COUNT, CONDITIONAL_LAYER_COUNT>
{
    /// Get the active layers.
    pub fn layer_state(&self) -> &[Activity; LAYER_COUNT] {
        &self.active_layers
    }

    fn sticky_layer(&self) -> Option<LayerIndex> {
        self.active_layers
            .iter()
            .position(|&a| a == Activity::Active(ActivationStyle::Sticky))
            .map(|i| i as LayerIndex + 1)
    }

    /// Updates the context with the [LayerEvent].
    ///
    /// Returns scheduled events (e.g. sticky timeout).
    fn handle_layer_event(&mut self, event: LayerEvent) -> key::KeyEvents<LayerEvent> {
        match event {
            LayerEvent::Activated(layer) => {
                if self.is_layer_locked(layer) {
                    // Hold pressed while layer is locked: unlock and turn off.
                    self.clear_layer_lock(layer);
                    self.active_layers.deactivate(layer);
                } else {
                    self.active_layers.activate(layer, ActivationStyle::Regular);
                }
                // Sticky cancelled / converted to hold — drop any sticky timeout.
                self.invalidate_sticky_timeouts();
                self.apply_conditional_layers();
                key::KeyEvents::no_events()
            }
            LayerEvent::Deactivated(layer) => {
                if self.is_layer_locked(layer) {
                    // Locked layers stay on when the Hold that activated them is released.
                    key::KeyEvents::no_events()
                } else {
                    self.active_layers.deactivate(layer);
                    self.invalidate_sticky_timeouts();
                    self.apply_conditional_layers();
                    key::KeyEvents::no_events()
                }
            }
            LayerEvent::StickyActivated(layer) => {
                self.active_layers.activate(layer, ActivationStyle::Sticky);
                self.pressed_keymap_index = None;
                // Previous sticky wait is superseded while this sticky key is held.
                self.invalidate_sticky_timeouts();
                self.apply_conditional_layers();
                key::KeyEvents::no_events()
            }
            LayerEvent::StickyReleased => {
                // Sticky key released without interruption: layer stays sticky.
                // Arm optional timeout for "no next key pressed".
                if self.sticky_layer().is_none() {
                    key::KeyEvents::no_events()
                } else {
                    self.invalidate_sticky_timeouts();
                    let timeout_id = self.sticky_timeout_id;
                    match self.config.sticky_timeout {
                        Some(timeout) => {
                            key::KeyEvents::scheduled_event(key::ScheduledEvent::after(
                                timeout,
                                key::Event::key_event(0, LayerEvent::StickyTimeout(timeout_id)),
                            ))
                        }
                        None => key::KeyEvents::no_events(),
                    }
                }
            }
            LayerEvent::StickyTimeout(timeout_id) => {
                if timeout_id == self.sticky_timeout_id && self.pressed_keymap_index.is_none() {
                    if let Some(layer) = self.sticky_layer() {
                        // deactivate_sticky_layer applies conditionals.
                        self.deactivate_sticky_layer(layer);
                    }
                }
                key::KeyEvents::no_events()
            }
            LayerEvent::Toggled(layer) => {
                if self.active_layers[layer as usize - 1].is_active() {
                    self.active_layers.deactivate(layer);
                    self.clear_layer_lock(layer);
                } else {
                    self.active_layers.activate(layer, ActivationStyle::Regular);
                }
                self.apply_conditional_layers();
                key::KeyEvents::no_events()
            }
            LayerEvent::Set(ModifierBitset { layers, mask }) => {
                let max_layer = 1 + LAYER_COUNT.min(MAX_BITSET_LAYER);

                // layer 0 is always active.
                for li in 1..max_layer {
                    if mask.contains(li) {
                        if layers.contains(li) {
                            self.active_layers
                                .activate(li as LayerIndex, ActivationStyle::Regular);
                        } else {
                            self.active_layers.deactivate(li as LayerIndex);
                            self.clear_layer_lock(li as LayerIndex);
                        }
                    }
                }
                self.apply_conditional_layers();
                key::KeyEvents::no_events()
            }
            LayerEvent::SetDefault(0) => {
                self.default_layer = None;
                key::KeyEvents::no_events()
            }
            LayerEvent::SetDefault(layer) => {
                self.default_layer = Some(layer);
                key::KeyEvents::no_events()
            }
            LayerEvent::LockInvert(target) => {
                let layer = match target {
                    LayerLockTarget::HighestActive => self.highest_active_layer(),
                    LayerLockTarget::Layer(layer) => Some(layer),
                };
                if let Some(layer) = layer {
                    self.lock_invert(layer);
                }
                key::KeyEvents::no_events()
            }
        }
    }

    /// Updates the context with the [key::Event].
    fn handle_event(&mut self, event: key::Event<LayerEvent>) -> key::KeyEvents<LayerEvent> {
        match event {
            key::Event::Input(input::Event::Press { keymap_index, .. }) => {
                if let Some(sticky_layer_index) = self.sticky_layer() {
                    if self.pressed_keymap_index.is_some() {
                        // The sticky layer modifier has already been used;
                        // the sticky layer should be deactivated for subsequent presses.
                        self.deactivate_sticky_layer(sticky_layer_index);
                    } else {
                        // Next key is using the sticky layer; cancel timeout.
                        self.invalidate_sticky_timeouts();
                        self.pressed_keymap_index = Some(keymap_index);
                    }
                }
                key::KeyEvents::no_events()
            }
            key::Event::Input(input::Event::Release { keymap_index, .. }) => {
                if let Some(sticky_layer_index) = self.sticky_layer() {
                    if self.pressed_keymap_index == Some(keymap_index) {
                        self.deactivate_sticky_layer(sticky_layer_index);
                    }
                }
                key::KeyEvents::no_events()
            }
            key::Event::Key { key_event, .. } => self.handle_layer_event(key_event),
            _ => key::KeyEvents::no_events(),
        }
    }
}

impl<const LAYER_COUNT: usize, const CONDITIONAL_LAYER_COUNT: usize> key::Context
    for Context<LAYER_COUNT, CONDITIONAL_LAYER_COUNT>
{
    type Event = LayerEvent;

    fn handle_event(&mut self, event: key::Event<Self::Event>) -> key::KeyEvents<Self::Event> {
        self.handle_event(event)
    }

    fn reset(&mut self) {
        Context::reset(self);
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
            if self[layer_index as usize - 1].is_some() {
                return self[layer_index as usize - 1].map(|k| (layer_index, k));
            }
        }

        match default_layer {
            Some(layer_index) if self[layer_index as usize - 1].is_some() => {
                self[layer_index as usize - 1].map(|k| (layer_index, k))
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
    fn new_pressed_key<const CONDITIONAL_LAYER_COUNT: usize>(
        &self,
        context: &Context<LAYER_COUNT, CONDITIONAL_LAYER_COUNT>,
    ) -> key::NewPressedKey<R> {
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
    /// Activates the given layer (1-based; see [LayerIndex]).
    Activated(LayerIndex),
    /// Deactivates the given layer (1-based; see [LayerIndex]).
    Deactivated(LayerIndex),
    /// Toggles the given layer (1-based; see [LayerIndex]).
    Toggled(LayerIndex),
    /// Activates the given layer as sticky (on sticky-mod press).
    StickyActivated(LayerIndex),
    /// Sticky layer modifier released without interruption (sticky committed).
    StickyReleased,
    /// Sticky layer timed out while unused.
    ///
    /// The payload is a generation id used to ignore stale timeouts.
    StickyTimeout(u8),
    /// Sets the active layers to the given set of layers.
    Set(ModifierBitset),
    /// Changes the default layer.
    SetDefault(LayerIndex),
    /// Invert lock on the given [LayerLockTarget].
    ///
    /// See [ModifierKey::Lock].
    LockInvert(LayerLockTarget),
}

/// Struct for layer system pending key state. (No pending state).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PendingKeyState;

/// Whether the pressed Sticky modifier key is "sticky" or "regular".
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Behavior {
    /// Key state is "sticky". (Will activate sticky modifier when released).
    Sticky,
    /// Key state is "regular". (No sticky modifiers activated when released).
    Regular,
}

/// [crate::key::KeyState] of [ModifierKey].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModifierKeyState {
    behavior: Behavior,
}

impl Default for ModifierKeyState {
    fn default() -> Self {
        Self::new()
    }
}

impl ModifierKeyState {
    /// Constructs a regular ModifierKeyState
    pub fn new() -> Self {
        Self {
            behavior: Behavior::Regular,
        }
    }

    /// Constructs a sticky ModifierKeyState
    pub fn sticky() -> Self {
        Self {
            behavior: Behavior::Sticky,
        }
    }

    /// Handle the given event for the given key.
    pub fn handle_event(
        &mut self,
        keymap_index: u16,
        event: key::Event<LayerEvent>,
        key: &ModifierKey,
    ) -> Option<LayerEvent> {
        match key {
            ModifierKey::Hold(layer, _) => match event {
                key::Event::Input(input::Event::Release { keymap_index: ki }) => {
                    if keymap_index == ki {
                        Some(LayerEvent::Deactivated(*layer))
                    } else {
                        None
                    }
                }
                _ => None,
            },
            ModifierKey::Toggle(_) => None,
            ModifierKey::Sticky(layer) => match event {
                key::Event::Input(input::Event::Press { keymap_index: _ }) => {
                    if self.behavior == Behavior::Sticky {
                        // Another key pressed while sticky modifier is held; make self regular
                        self.behavior = Behavior::Regular;
                        // Change the layer state to *regular*
                        Some(LayerEvent::Activated(*layer))
                    } else {
                        None
                    }
                }
                key::Event::Input(input::Event::Release { keymap_index: ki })
                    if keymap_index == ki =>
                {
                    match self.behavior {
                        Behavior::Regular => Some(LayerEvent::Deactivated(*layer)),
                        // Sticky key tapped (released without interruption):
                        // layer stays sticky; arm optional timeout via context.
                        Behavior::Sticky => Some(LayerEvent::StickyReleased),
                    }
                }
                _ => None,
            },
            ModifierKey::SetActiveLayers(_modifier_bitset) => None,
            ModifierKey::Default(layer) => match event {
                key::Event::Input(input::Event::Release { keymap_index: ki }) => {
                    if keymap_index == ki {
                        Some(LayerEvent::SetDefault(*layer))
                    } else {
                        None
                    }
                }
                _ => None,
            },
            ModifierKey::Lock(_) => None,
        }
    }
}

/// The [key::System] implementation for layer system keys.
///
/// `CONDITIONAL_LAYER_COUNT` is carried so the system's context
/// matches the layered [Config] / [Context] used by the keymap
/// (rules live on context, not system).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct System<
    R: Copy + Debug + PartialEq,
    ModifierKeys: Index<usize, Output = ModifierKey>,
    LayeredKeys: Index<usize, Output = LayeredKey<R, LAYER_COUNT>>,
    const LAYER_COUNT: usize,
    const CONDITIONAL_LAYER_COUNT: usize = 0,
> {
    modifier_keys: ModifierKeys,
    layered_keys: LayeredKeys,
}

impl<
        R: Copy + Debug + PartialEq,
        ModifierKeys: Index<usize, Output = ModifierKey>,
        LayeredKeys: Index<usize, Output = LayeredKey<R, LAYER_COUNT>>,
        const LAYER_COUNT: usize,
        const CONDITIONAL_LAYER_COUNT: usize,
    > System<R, ModifierKeys, LayeredKeys, LAYER_COUNT, CONDITIONAL_LAYER_COUNT>
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
        const CONDITIONAL_LAYER_COUNT: usize,
    > key::System<R>
    for System<R, ModifierKeys, LayeredKeys, LAYER_COUNT, CONDITIONAL_LAYER_COUNT>
{
    type Ref = Ref;
    type Context = Context<LAYER_COUNT, CONDITIONAL_LAYER_COUNT>;
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
            Ref::Modifier(mod_key_index) => {
                let key = self.modifier_keys[mod_key_index as usize];
                let (m_ks, maybe_lmod_ev) = key.new_pressed_key();
                let pks = key::PressedKeyResult::Resolved(m_ks);
                let pke = match maybe_lmod_ev {
                    Some(lmod_ev) => {
                        key::KeyEvents::event(key::Event::key_event(keymap_index, lmod_ev))
                    }
                    None => key::KeyEvents::no_events(),
                };
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
        key_ref: &Self::Ref,
        _key_state: &Self::KeyState,
    ) -> Option<key::KeyOutput> {
        if let Ref::Modifier(mod_key_index) = key_ref {
            let key = self.modifier_keys[*mod_key_index as usize];
            match key {
                ModifierKey::Hold(_, mods) if mods != key::KeyboardModifiers::NONE => {
                    Some(key::KeyOutput::from_key_modifiers(mods))
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
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
    fn test_sizeof_modifier_bitset() {
        // Two LayerBitset (u32) fields.
        assert_eq!(8, core::mem::size_of::<ModifierBitset>());
    }

    #[test]
    fn test_sizeof_event() {
        // Firmware RAM budget on 32-bit targets: LayerEvent::Set is a ModifierBitset.
        assert_eq!(12, core::mem::size_of::<LayerEvent>());
    }

    #[test]
    fn test_layer_bitset_capacity() {
        assert_eq!(32, LayerBitset::BITS);
        assert_eq!(31, MAX_BITSET_LAYER);
        assert!(LayerBitset::EMPTY.insert(31).contains(31));
        assert!(!LayerBitset::EMPTY.insert(32).contains(32));
        assert!(!LayerBitset::ALL.remove(31).contains(31));
        assert!(LayerBitset::ALL.is_superset_of(LayerBitset::from_bits(0b1010)));
    }

    #[test]
    fn deserialize_set_active_layers_record_json() {
        let key: ModifierKey =
            serde_json::from_str(r#"{"SetActiveLayers": {"layers": 5, "mask": 3}}"#).unwrap();
        assert_eq!(
            ModifierKey::SetActiveLayers(ModifierBitset {
                layers: LayerBitset::from_bits(5),
                mask: LayerBitset::from_bits(3),
            }),
            key,
        );

        let key: ModifierKey =
            serde_json::from_str(r#"{"SetActiveLayers": {"layers": 5}}"#).unwrap();
        assert_eq!(
            ModifierKey::SetActiveLayers(ModifierBitset {
                layers: LayerBitset::from_bits(5),
                mask: BITSET_MASK_ALL,
            }),
            key,
        );
    }

    #[test]
    fn test_pressing_hold_modifier_key_emits_event_activate_layer() {
        let layer = 1;
        let key = ModifierKey::hold(layer);

        let (_pressed_key, layer_event) = key.new_pressed_key();

        assert_eq!(Some(LayerEvent::Activated(layer)), layer_event);
    }

    #[test]
    fn test_releasing_hold_modifier_key_emits_event_deactivate_layer() {
        // Assemble: press a Hold layer modifier key
        let layer = 1;
        let key = ModifierKey::hold(layer);
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
            let expected_layer_event = LayerEvent::Deactivated(layer);
            assert_eq!(expected_layer_event, actual_layer_event);
        } else {
            panic!("Expected Some LayerDeactivated event");
        }
    }

    #[test]
    fn test_releasing_different_hold_modifier_key_does_not_emit_event() {
        // Assemble: press a Hold layer modifier key
        let layer = 1;
        let key = ModifierKey::hold(layer);
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

        context.handle_layer_event(LayerEvent::Activated(2));

        let actual_active_layers = &context.active_layers[0..3];
        assert_eq!(
            &[
                Activity::Inactive,
                Activity::Active(ActivationStyle::Regular),
                Activity::Inactive
            ],
            actual_active_layers
        );
    }

    /// Classic tri-layer: layer 3 active iff layers 1 and 2 are both active.
    fn tri_layer_context() -> super::Context<LAYER_COUNT, 1> {
        super::Context::from_config(Config {
            sticky_timeout: None,
            conditional_layers: Slice::from_slice(&[ConditionalLayer::from_if_layers(3, &[1, 2])]),
        })
    }

    #[test]
    fn test_conditional_layer_partial_if_layers_does_not_activate_then() {
        // Assemble
        let mut context = tri_layer_context();

        // Act
        context.handle_layer_event(LayerEvent::Activated(1));

        // Assert
        assert_eq!(Activity::Inactive, context.active_layers[2]);
    }

    #[test]
    fn test_conditional_layer_activates_when_all_if_layers_active() {
        // Assemble
        let mut context = tri_layer_context();
        context.handle_layer_event(LayerEvent::Activated(1));

        // Act
        context.handle_layer_event(LayerEvent::Activated(2));

        // Assert
        assert_eq!(
            Activity::Active(ActivationStyle::Regular),
            context.active_layers[2]
        );
    }

    #[test]
    fn test_conditional_layer_deactivates_when_if_layer_releases() {
        // Assemble
        let mut context = tri_layer_context();
        context.handle_layer_event(LayerEvent::Activated(1));
        context.handle_layer_event(LayerEvent::Activated(2));

        // Act
        context.handle_layer_event(LayerEvent::Deactivated(1));

        // Assert
        assert_eq!(Activity::Inactive, context.active_layers[2]);
    }

    #[test]
    fn test_locked_conditional_layer_stays_active_when_if_layers_release() {
        // Assemble: lower+raise → adjust; lock adjust while both if-layers held
        let mut context = tri_layer_context();
        context.handle_layer_event(LayerEvent::Activated(1));
        context.handle_layer_event(LayerEvent::Activated(2));
        assert!(context.active_layers[2].is_active());
        context.handle_layer_event(LayerEvent::LockInvert(LayerLockTarget::Layer(3)));
        assert!(context.is_layer_locked(3));

        // Act: release if-layers (would normally turn adjust off)
        context.handle_layer_event(LayerEvent::Deactivated(1));
        context.handle_layer_event(LayerEvent::Deactivated(2));

        // Assert: lock keeps adjust on
        assert!(context.is_layer_locked(3));
        assert!(context.active_layers[2].is_active());
    }

    #[test]
    fn test_conditional_layer_nested_fixed_point() {
        // Assemble: C ← A ∧ B; E ← C ∧ D  (layers 1,2 → 3; 3,4 → 5)
        let mut context = super::Context::<LAYER_COUNT, 2>::from_config(Config {
            sticky_timeout: None,
            conditional_layers: Slice::from_slice(&[
                ConditionalLayer::from_if_layers(3, &[1, 2]),
                ConditionalLayer::from_if_layers(5, &[3, 4]),
            ]),
        });
        context.handle_layer_event(LayerEvent::Activated(1));
        context.handle_layer_event(LayerEvent::Activated(2));

        // Act: activate D; fixed-point should turn on C (already) and E
        context.handle_layer_event(LayerEvent::Activated(4));

        // Assert
        assert_eq!(
            [
                Activity::Active(ActivationStyle::Regular), // 3
                Activity::Active(ActivationStyle::Regular), // 4
                Activity::Active(ActivationStyle::Regular), // 5
            ],
            context.active_layers[2..5]
        );
    }

    #[test]
    fn test_conditional_layer_sticky_if_layer_counts_as_active() {
        // Assemble
        let mut context = tri_layer_context();
        context.handle_layer_event(LayerEvent::StickyActivated(1));

        // Act
        context.handle_layer_event(LayerEvent::Activated(2));

        // Assert
        assert_eq!(
            Activity::Active(ActivationStyle::Regular),
            context.active_layers[2]
        );
    }

    #[test]
    fn test_conditional_layer_set_active_layers_reevaluates() {
        // Assemble
        let mut context = tri_layer_context();
        context.handle_layer_event(LayerEvent::Activated(1));
        context.handle_layer_event(LayerEvent::Activated(2));

        // Act: clear layer 1 via Set (mask only layer 1)
        context.handle_layer_event(LayerEvent::Set(ModifierBitset {
            layers: LayerBitset::EMPTY,
            mask: LayerBitset::EMPTY.insert(1),
        }));

        // Assert
        assert_eq!(Activity::Inactive, context.active_layers[2]);
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
        context.handle_layer_event(LayerEvent::Activated(1));
        context.handle_layer_event(LayerEvent::Activated(2));
        context.handle_layer_event(LayerEvent::Activated(3));
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
        context.handle_layer_event(LayerEvent::Activated(1));
        context.handle_layer_event(LayerEvent::Activated(2));
        context.handle_layer_event(LayerEvent::Activated(3));
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
        context.handle_layer_event(LayerEvent::Activated(1));
        context.handle_layer_event(LayerEvent::Activated(3));
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
        let mut layer_state: [Activity; 5] = [Activity::Inactive; 5];
        layer_state.activate(1, ActivationStyle::Regular);
        layer_state.activate(2, ActivationStyle::Regular);
        layer_state.activate(4, ActivationStyle::Regular);
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
        assert_eq!(Some(LayerEvent::Toggled(layer)), layer_event);
    }

    #[test]
    fn test_pressing_lock_key_emits_lock_invert_highest() {
        // Assemble
        let key = ModifierKey::lock();

        // Act
        let (_pressed_key, layer_event) = key.new_pressed_key();

        // Assert
        assert_eq!(
            Some(LayerEvent::LockInvert(LayerLockTarget::HighestActive)),
            layer_event
        );
    }

    #[test]
    fn test_pressing_lock_layer_key_emits_lock_invert_layer() {
        // Assemble
        let key = ModifierKey::lock_layer(2);

        // Act
        let (_pressed_key, layer_event) = key.new_pressed_key();

        // Assert
        assert_eq!(
            Some(LayerEvent::LockInvert(LayerLockTarget::Layer(2))),
            layer_event
        );
    }

    #[test]
    fn test_lock_highest_keeps_layer_after_hold_release() {
        // Assemble: hold activates layer 1, then lock highest
        let mut context = Context::default();
        context.handle_layer_event(LayerEvent::Activated(1));
        context.handle_layer_event(LayerEvent::LockInvert(LayerLockTarget::HighestActive));
        assert!(context.is_layer_locked(1));
        assert!(context.active_layers[0].is_active());

        // Act: hold release (would normally deactivate)
        context.handle_layer_event(LayerEvent::Deactivated(1));

        // Assert: still active and locked
        assert!(context.is_layer_locked(1));
        assert!(context.active_layers[0].is_active());
    }

    #[test]
    fn test_lock_again_unlocks_and_deactivates() {
        // Assemble: lock highest after hold, then release hold
        let mut context = Context::default();
        context.handle_layer_event(LayerEvent::Activated(1));
        context.handle_layer_event(LayerEvent::LockInvert(LayerLockTarget::HighestActive));
        context.handle_layer_event(LayerEvent::Deactivated(1));
        assert!(context.is_layer_locked(1));
        assert!(context.active_layers[0].is_active());

        // Act: lock again (unlock)
        context.handle_layer_event(LayerEvent::LockInvert(LayerLockTarget::HighestActive));

        // Assert
        assert!(!context.is_layer_locked(1));
        assert!(!context.active_layers[0].is_active());
    }

    #[test]
    fn test_hold_press_while_locked_unlocks() {
        // Assemble: lock layer, then release hold so only lock keeps it active
        let mut context = Context::default();
        context.handle_layer_event(LayerEvent::Activated(1));
        context.handle_layer_event(LayerEvent::LockInvert(LayerLockTarget::HighestActive));
        context.handle_layer_event(LayerEvent::Deactivated(1));
        assert!(context.is_layer_locked(1));
        assert!(context.active_layers[0].is_active());

        // Act: press hold again while locked
        context.handle_layer_event(LayerEvent::Activated(1));

        // Assert
        assert!(!context.is_layer_locked(1));
        assert!(!context.active_layers[0].is_active());
    }

    #[test]
    fn test_lock_specific_layer_activates_when_inactive() {
        // Assemble
        let mut context = Context::default();

        // Act
        context.handle_layer_event(LayerEvent::LockInvert(LayerLockTarget::Layer(2)));

        // Assert
        assert!(context.is_layer_locked(2));
        assert!(context.active_layers[1].is_active());
    }

    #[test]
    fn test_lock_no_active_layer_is_noop() {
        // Assemble
        let mut context = Context::default();

        // Act
        context.handle_layer_event(LayerEvent::LockInvert(LayerLockTarget::HighestActive));

        // Assert
        assert_eq!(LayerBitset::EMPTY, context.locked_layers);
        assert!(!context.active_layers.iter().any(|a| a.is_active()));
    }

    #[test]
    fn deserialize_lock_json() {
        // Assemble / Act
        let highest: ModifierKey = serde_json::from_str(r#"{"Lock":"HighestActive"}"#).unwrap();
        let layer: ModifierKey = serde_json::from_str(r#"{"Lock":{"Layer":3}}"#).unwrap();

        // Assert
        assert_eq!(ModifierKey::Lock(LayerLockTarget::HighestActive), highest);
        assert_eq!(ModifierKey::Lock(LayerLockTarget::Layer(3)), layer);
    }
}
