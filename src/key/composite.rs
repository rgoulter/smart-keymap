//! This module aggregates various [crate::key::System] implementations.

use core::fmt::Debug;
use core::marker::PhantomData;
use core::ops::Index;

use serde::Deserialize;

use crate::{key, keymap};

use crate::init::AUTOMATION_INSTRUCTION_COUNT;
use crate::init::CHORDED_MAX_CHORDS;
use crate::init::CHORDED_MAX_CHORD_SIZE;
use crate::init::CHORDED_MAX_OVERLAPPING_CHORD_SIZE;
use crate::init::LAYERED_LAYER_COUNT;
use crate::init::TAP_DANCE_MAX_DEFINITIONS as TAP_DANCE_MAX_DEF_COUNT;

const CHORDED_MAX_PRESSED_INDICES: usize = CHORDED_MAX_CHORD_SIZE * 2;

/// Type aliases for convenience.
pub type AutomationRef = key::automation::Ref;
/// Type aliases for convenience.
pub type AutomationKey = key::automation::Key;
/// Type aliases for convenience.
pub type AutomationConfig = key::automation::Config<AUTOMATION_INSTRUCTION_COUNT>;
/// Type aliases for convenience.
pub type AutomationContext = key::automation::Context<AUTOMATION_INSTRUCTION_COUNT>;
/// Type aliases for convenience.
pub type AutomationEvent = key::automation::Event;
/// Type aliases for convenience.
pub type AutomationPendingKeyState = key::automation::PendingKeyState;
/// Type aliases for convenience.
pub type AutomationKeyState = key::automation::KeyState;
/// Type aliases for convenience.
pub type AutomationSystem<D> = key::automation::System<Ref, D, AUTOMATION_INSTRUCTION_COUNT>;

/// Type aliases for convenience.
pub type CallbackRef = key::callback::Ref;
/// Type aliases for convenience.
pub type CallbackKey = key::callback::Key;
/// Type aliases for convenience.
pub type CallbackContext = key::callback::Context;
/// Type aliases for convenience.
pub type CallbackEvent = key::callback::Event;
/// Type aliases for convenience.
pub type CallbackPendingKeyState = key::callback::PendingKeyState;
/// Type aliases for convenience.
pub type CallbackKeyState = key::callback::KeyState;
/// Type aliases for convenience.
pub type CallbackSystem<D> = key::callback::System<Ref, D>;

/// Type aliases for convenience.
pub type CapsWordRef = key::caps_word::Ref;
/// Type aliases for convenience.
pub type CapsWordKey = key::caps_word::Key;
/// Type aliases for convenience.
pub type CapsWordContext = key::caps_word::Context;
/// Type aliases for convenience.
pub type CapsWordEvent = key::caps_word::Event;
/// Type aliases for convenience.
pub type CapsWordPendingKeyState = key::caps_word::PendingKeyState;
/// Type aliases for convenience.
pub type CapsWordKeyState = key::caps_word::KeyState;
/// Type aliases for convenience.
pub type CapsWordSystem = key::caps_word::System<Ref>;

/// Type aliases for convenience.
pub type ChordedRef = key::chorded::Ref;
/// Type aliases for convenience.
pub type ChordedKey = key::chorded::Key<
    Ref,
    CHORDED_MAX_CHORDS,
    CHORDED_MAX_CHORD_SIZE,
    CHORDED_MAX_OVERLAPPING_CHORD_SIZE,
    CHORDED_MAX_PRESSED_INDICES,
>;
/// Type aliases for convenience.
pub type ChordedAuxiliaryKey = key::chorded::AuxiliaryKey<
    Ref,
    CHORDED_MAX_CHORDS,
    CHORDED_MAX_CHORD_SIZE,
    CHORDED_MAX_PRESSED_INDICES,
>;
/// Type aliases for convenience.
pub type ChordedConfig = key::chorded::Config<CHORDED_MAX_CHORDS, CHORDED_MAX_CHORD_SIZE>;
/// Type aliases for convenience.
pub type ChordedContext =
    key::chorded::Context<CHORDED_MAX_CHORDS, CHORDED_MAX_CHORD_SIZE, CHORDED_MAX_PRESSED_INDICES>;
/// Type aliases for convenience.
pub type ChordedEvent = key::chorded::Event;
/// Type aliases for convenience.
pub type ChordedPendingKeyState = key::chorded::PendingKeyState<
    CHORDED_MAX_CHORDS,
    CHORDED_MAX_CHORD_SIZE,
    CHORDED_MAX_PRESSED_INDICES,
>;
/// Type aliases for convenience.
pub type ChordedKeyState = key::chorded::KeyState;
/// Type aliases for convenience.
pub type ChordedSystem<D, AuxD> = key::chorded::System<
    Ref,
    D,
    AuxD,
    CHORDED_MAX_CHORDS,
    CHORDED_MAX_CHORD_SIZE,
    CHORDED_MAX_OVERLAPPING_CHORD_SIZE,
    CHORDED_MAX_PRESSED_INDICES,
>;

/// Type aliases for convenience.
pub type ConsumerRef = key::consumer::Ref;
/// Type aliases for convenience.
pub type ConsumerContext = key::consumer::Context;
/// Type aliases for convenience.
pub type ConsumerEvent = key::consumer::Event;
/// Type aliases for convenience.
pub type ConsumerPendingKeyState = key::consumer::PendingKeyState;
/// Type aliases for convenience.
pub type ConsumerKeyState = key::consumer::KeyState;
/// Type aliases for convenience.
pub type ConsumerSystem = key::consumer::System<Ref>;

/// Type aliases for convenience.
pub type CustomRef = key::custom::Ref;
/// Type aliases for convenience.
pub type CustomContext = key::custom::Context;
/// Type aliases for convenience.
pub type CustomEvent = key::custom::Event;
/// Type aliases for convenience.
pub type CustomPendingKeyState = key::custom::PendingKeyState;
/// Type aliases for convenience.
pub type CustomKeyState = key::custom::KeyState;
/// Type aliases for convenience.
pub type CustomSystem = key::custom::System<Ref>;

/// Type aliases for convenience.
pub type KeyboardRef = key::keyboard::Ref;
/// Type aliases for convenience.
pub type KeyboardKey = key::keyboard::Key;
/// Type aliases for convenience.
pub type KeyboardContext = key::keyboard::Context;
/// Type aliases for convenience.
pub type KeyboardEvent = key::keyboard::Event;
/// Type aliases for convenience.
pub type KeyboardPendingKeyState = key::keyboard::PendingKeyState;
/// Type aliases for convenience.
pub type KeyboardKeyState = key::keyboard::KeyState;
/// Type aliases for convenience.
pub type KeyboardSystem<D> = key::keyboard::System<Ref, D>;

/// Type aliases for convenience.
pub type LayeredRef = key::layered::Ref;
/// Type aliases for convenience.
pub type LayeredKey = key::layered::LayeredKey<Ref, LAYERED_LAYER_COUNT>;
/// Type aliases for convenience.
pub type LayeredModifierKey = key::layered::ModifierKey;
/// Type aliases for convenience.
pub type LayeredContext = key::layered::Context<LAYERED_LAYER_COUNT>;
/// Type aliases for convenience.
pub type LayeredEvent = key::layered::LayerEvent;
/// Type aliases for convenience.
pub type LayeredPendingKeyState = key::layered::PendingKeyState;
/// Type aliases for convenience.
pub type LayeredKeyState = key::layered::ModifierKeyState;
/// Type aliases for convenience.
pub type LayeredSystem<LM, L> = key::layered::System<Ref, LM, L, LAYERED_LAYER_COUNT>;

/// Type aliases for convenience.
pub type MouseRef = key::mouse::Ref;
/// Type aliases for convenience.
pub type MouseContext = key::mouse::Context;
/// Type aliases for convenience.
pub type MouseEvent = key::mouse::Event;
/// Type aliases for convenience.
pub type MousePendingKeyState = key::mouse::PendingKeyState;
/// Type aliases for convenience.
pub type MouseKeyState = key::mouse::KeyState;
/// Type aliases for convenience.
pub type MouseSystem = key::mouse::System<Ref>;

/// Type aliases for convenience.
pub type StickyRef = key::sticky::Ref;
/// Type aliases for convenience.
pub type StickyKey = key::sticky::Key;
/// Type aliases for convenience.
pub type StickyConfig = key::sticky::Config;
/// Type aliases for convenience.
pub type StickyContext = key::sticky::Context;
/// Type aliases for convenience.
pub type StickyEvent = key::sticky::Event;
/// Type aliases for convenience.
pub type StickyPendingKeyState = key::sticky::PendingKeyState;
/// Type aliases for convenience.
pub type StickyKeyState = key::sticky::KeyState;
/// Type aliases for convenience.
pub type StickySystem<D> = key::sticky::System<Ref, D>;

/// Type aliases for convenience.
pub type TapDanceRef = key::tap_dance::Ref;
/// Type aliases for convenience.
pub type TapDanceKey = key::tap_dance::Key<Ref, TAP_DANCE_MAX_DEF_COUNT>;
/// Type aliases for convenience.
pub type TapDanceConfig = key::tap_dance::Config;
/// Type aliases for convenience.
pub type TapDanceContext = key::tap_dance::Context;
/// Type aliases for convenience.
pub type TapDanceEvent = key::tap_dance::Event;
/// Type aliases for convenience.
pub type TapDancePendingKeyState = key::tap_dance::PendingKeyState;
/// Type aliases for convenience.
pub type TapDanceKeyState = key::tap_dance::KeyState;
/// Type aliases for convenience.
pub type TapDanceSystem<D> = key::tap_dance::System<Ref, D, TAP_DANCE_MAX_DEF_COUNT>;

/// Type aliases for convenience.
pub type TapHoldRef = key::tap_hold::Ref;
/// Type aliases for convenience.
pub type TapHoldKey = key::tap_hold::Key<Ref>;
/// Type aliases for convenience.
pub type TapHoldConfig = key::tap_hold::Config;
/// Type aliases for convenience.
pub type TapHoldContext = key::tap_hold::Context;
/// Type aliases for convenience.
pub type TapHoldEvent = key::tap_hold::Event;
/// Type aliases for convenience.
pub type TapHoldPendingKeyState = key::tap_hold::PendingKeyState;
/// Type aliases for convenience.
pub type TapHoldKeyState = key::tap_hold::KeyState;
/// Type aliases for convenience.
pub type TapHoldSystem<D> = key::tap_hold::System<Ref, D>;

/// Aggregate enum for key references.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Ref {
    /// [key::automation::Ref] variant.
    Automation(AutomationRef),
    /// [key::callback::Ref] variant.
    Callback(CallbackRef),
    /// [key::caps_word::Ref] variant.
    CapsWord(CapsWordRef),
    /// [key::chorded::Ref] variant.
    Chorded(ChordedRef),
    /// [key::consumer::Ref] variant.
    Consumer(ConsumerRef),
    /// [key::custom::Ref] variant.
    Custom(CustomRef),
    /// [key::keyboard::Ref] variant.
    Keyboard(KeyboardRef),
    /// [key::layered::Ref] variant.
    Layered(LayeredRef),
    /// [key::mouse::Ref] variant.
    Mouse(MouseRef),
    /// [key::sticky::Ref] variant.
    Sticky(StickyRef),
    /// [key::tap_dance::Ref] variant.
    TapDance(TapDanceRef),
    /// [key::tap_hold::Ref] variant.
    TapHold(TapHoldRef),
}

#[cfg(feature = "std")]
impl Default for Ref {
    fn default() -> Self {
        Ref::Keyboard(key::keyboard::Ref::KeyCode(0))
    }
}

/// Aggregate config.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Config {
    /// The automation configuration.
    #[serde(default)]
    pub automation: AutomationConfig,
    /// The chorded configuration.
    #[serde(default)]
    pub chorded: key::chorded::Config<CHORDED_MAX_CHORDS, CHORDED_MAX_CHORD_SIZE>,
    /// The sticky modifier configuration
    #[serde(default)]
    pub sticky: StickyConfig,
    /// The tap dance configuration.
    #[serde(default)]
    pub tap_dance: TapDanceConfig,
    /// The tap hold configuration.
    #[serde(default)]
    pub tap_hold: TapHoldConfig,
}

impl Config {
    /// Constructs a new [Config] with default values.
    pub const fn new() -> Self {
        Config {
            automation: key::automation::Config::new(),
            chorded: key::chorded::Config::new(),
            sticky: key::sticky::Config::new(),
            tap_dance: key::tap_dance::Config::new(),
            tap_hold: key::tap_hold::Config::new(),
        }
    }
}

/// An aggregate context for [key::Context]s.
#[derive(Debug, Clone, Copy)]
pub struct Context {
    keymap_context: keymap::KeymapContext,
    automation: AutomationContext,
    callback: CallbackContext,
    caps_word: CapsWordContext,
    chorded: ChordedContext,
    consumer: ConsumerContext,
    custom: CustomContext,
    keyboard: KeyboardContext,
    layered: LayeredContext,
    mouse: MouseContext,
    sticky: StickyContext,
    tap_dance: TapDanceContext,
    tap_hold: TapHoldContext,
}

impl Context {
    /// Constructs a [Context] from the given [Config].
    pub const fn from_config(config: Config) -> Self {
        Self {
            keymap_context: keymap::KeymapContext::new(),
            automation: key::automation::Context::from_config(config.automation),
            callback: key::callback::Context,
            caps_word: key::caps_word::Context::new(),
            chorded: key::chorded::Context::from_config(config.chorded),
            consumer: key::consumer::Context,
            custom: key::custom::Context,
            keyboard: key::keyboard::Context,
            layered: key::layered::Context::new(),
            mouse: key::mouse::Context,
            sticky: key::sticky::Context::from_config(config.sticky),
            tap_dance: key::tap_dance::Context::from_config(config.tap_dance),
            tap_hold: key::tap_hold::Context::from_config(config.tap_hold),
        }
    }
}

impl Default for Context {
    /// Returns the default context.
    fn default() -> Self {
        Self::from_config(Config::new())
    }
}

impl key::Context for Context {
    type Event = Event;
    fn handle_event(&mut self, event: key::Event<Self::Event>) -> key::KeyEvents<Self::Event> {
        let mut pke = key::KeyEvents::no_events();

        if let Ok(e) = event.try_into_key_event() {
            pke.extend(self.automation.handle_event(e).into_events());
        }

        if let Ok(e) = event.try_into_key_event() {
            pke.extend(self.caps_word.handle_event(e).into_events());
        }

        if let Ok(e) = event.try_into_key_event() {
            pke.extend(self.chorded.handle_event(e).into_events());
        }

        if let Ok(e) = event.try_into_key_event() {
            pke.extend(self.layered.handle_event(e).into_events());
        }

        if let Ok(e) = event.try_into_key_event() {
            pke.extend(self.sticky.handle_event(e).into_events());
        }

        pke
    }
}

impl keymap::SetKeymapContext for Context {
    fn set_keymap_context(&mut self, context: keymap::KeymapContext) {
        self.keymap_context = context;

        self.chorded.update_keymap_context(&context);
        self.tap_hold.update_keymap_context(&context);
    }
}

/// Sum type aggregating the [key::Event] types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// An automation event.
    Automation(AutomationEvent),
    /// A callback event.
    Callback(CallbackEvent),
    /// A caps word event.
    CapsWord(CapsWordEvent),
    /// A chorded event.
    Chorded(ChordedEvent),
    /// A consumer event.
    Consumer(ConsumerEvent),
    /// A custom event.
    Custom(CustomEvent),
    /// A keyboard event.
    Keyboard(KeyboardEvent),
    /// A layer modification event.
    Layered(LayeredEvent),
    /// A mouse event.
    Mouse(MouseEvent),
    /// A sticky modifier event.
    Sticky(StickyEvent),
    /// A tap-dance event.
    TapDance(TapDanceEvent),
    /// A tap-hold event.
    TapHold(TapHoldEvent),
}

impl From<AutomationEvent> for Event {
    fn from(ev: AutomationEvent) -> Self {
        Event::Automation(ev)
    }
}

impl From<CallbackEvent> for Event {
    fn from(ev: CallbackEvent) -> Self {
        Event::Callback(ev)
    }
}

impl From<CapsWordEvent> for Event {
    fn from(ev: CapsWordEvent) -> Self {
        Event::CapsWord(ev)
    }
}

impl From<ChordedEvent> for Event {
    fn from(ev: ChordedEvent) -> Self {
        Event::Chorded(ev)
    }
}

impl From<ConsumerEvent> for Event {
    fn from(ev: ConsumerEvent) -> Self {
        Event::Consumer(ev)
    }
}

impl From<CustomEvent> for Event {
    fn from(ev: CustomEvent) -> Self {
        Event::Custom(ev)
    }
}

impl From<KeyboardEvent> for Event {
    fn from(ev: KeyboardEvent) -> Self {
        Event::Keyboard(ev)
    }
}

impl From<LayeredEvent> for Event {
    fn from(ev: LayeredEvent) -> Self {
        Event::Layered(ev)
    }
}

impl From<MouseEvent> for Event {
    fn from(ev: MouseEvent) -> Self {
        Event::Mouse(ev)
    }
}

impl From<StickyEvent> for Event {
    fn from(ev: StickyEvent) -> Self {
        Event::Sticky(ev)
    }
}

impl From<TapDanceEvent> for Event {
    fn from(ev: TapDanceEvent) -> Self {
        Event::TapDance(ev)
    }
}

impl From<TapHoldEvent> for Event {
    fn from(ev: TapHoldEvent) -> Self {
        Event::TapHold(ev)
    }
}

impl TryFrom<Event> for AutomationEvent {
    type Error = key::EventError;

    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        match ev {
            Event::Automation(ev) => Ok(ev),
            _ => Err(key::EventError::UnmappableEvent),
        }
    }
}

impl TryFrom<Event> for CapsWordEvent {
    type Error = key::EventError;

    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        match ev {
            Event::CapsWord(ev) => Ok(ev),
            _ => Err(key::EventError::UnmappableEvent),
        }
    }
}

impl TryFrom<Event> for ChordedEvent {
    type Error = key::EventError;

    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        match ev {
            Event::Chorded(ev) => Ok(ev),
            _ => Err(key::EventError::UnmappableEvent),
        }
    }
}

impl TryFrom<Event> for ConsumerEvent {
    type Error = key::EventError;

    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        match ev {
            Event::Consumer(ev) => Ok(ev),
            _ => Err(key::EventError::UnmappableEvent),
        }
    }
}

impl TryFrom<Event> for KeyboardEvent {
    type Error = key::EventError;

    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        match ev {
            Event::Keyboard(ev) => Ok(ev),
            _ => Err(key::EventError::UnmappableEvent),
        }
    }
}

impl TryFrom<Event> for LayeredEvent {
    type Error = key::EventError;

    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        match ev {
            Event::Layered(ev) => Ok(ev),
            _ => Err(key::EventError::UnmappableEvent),
        }
    }
}

impl TryFrom<Event> for MouseEvent {
    type Error = key::EventError;

    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        match ev {
            Event::Mouse(ev) => Ok(ev),
            _ => Err(key::EventError::UnmappableEvent),
        }
    }
}

impl TryFrom<Event> for StickyEvent {
    type Error = key::EventError;

    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        match ev {
            Event::Sticky(ev) => Ok(ev),
            _ => Err(key::EventError::UnmappableEvent),
        }
    }
}

impl TryFrom<Event> for TapDanceEvent {
    type Error = key::EventError;

    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        match ev {
            Event::TapDance(ev) => Ok(ev),
            _ => Err(key::EventError::UnmappableEvent),
        }
    }
}

impl TryFrom<Event> for TapHoldEvent {
    type Error = key::EventError;

    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        match ev {
            Event::TapHold(ev) => Ok(ev),
            _ => Err(key::EventError::UnmappableEvent),
        }
    }
}

/// Aggregate enum for pending key state.
#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum PendingKeyState {
    /// Pending key state for [key::automation::PendingKeyState].
    Automation(AutomationPendingKeyState),
    /// Pending key state for [key::callback::PendingKeyState].
    Callback(CallbackPendingKeyState),
    /// Pending key state for [key::caps_word::PendingKeyState].
    CapsWord(CapsWordPendingKeyState),
    /// Pending key state for [key::chorded::PendingKeyState].
    Chorded(ChordedPendingKeyState),
    /// Pending key state for [key::consumer::PendingKeyState].
    Consumer(ConsumerPendingKeyState),
    /// Pending key state for [key::custom::PendingKeyState].
    Custom(CustomPendingKeyState),
    /// Pending key state for [key::keyboard::PendingKeyState].
    Keyboard(KeyboardPendingKeyState),
    /// Pending key state for [key::layered::PendingKeyState].
    Layered(LayeredPendingKeyState),
    /// Pending key state for [key::mouse::PendingKeyState].
    Mouse(MousePendingKeyState),
    /// Pending key state for [key::sticky::PendingKeyState].
    Sticky(StickyPendingKeyState),
    /// Pending key state for [key::tap_dance::PendingKeyState].
    TapDance(TapDancePendingKeyState),
    /// Pending key state for [key::tap_hold::PendingKeyState].
    TapHold(TapHoldPendingKeyState),
}

impl From<AutomationPendingKeyState> for PendingKeyState {
    fn from(pks: AutomationPendingKeyState) -> Self {
        PendingKeyState::Automation(pks)
    }
}

impl From<CallbackPendingKeyState> for PendingKeyState {
    fn from(pks: CallbackPendingKeyState) -> Self {
        PendingKeyState::Callback(pks)
    }
}

impl From<CapsWordPendingKeyState> for PendingKeyState {
    fn from(pks: CapsWordPendingKeyState) -> Self {
        PendingKeyState::CapsWord(pks)
    }
}

impl From<ChordedPendingKeyState> for PendingKeyState {
    fn from(pks: ChordedPendingKeyState) -> Self {
        PendingKeyState::Chorded(pks)
    }
}

impl From<ConsumerPendingKeyState> for PendingKeyState {
    fn from(pks: ConsumerPendingKeyState) -> Self {
        PendingKeyState::Consumer(pks)
    }
}

impl From<CustomPendingKeyState> for PendingKeyState {
    fn from(pks: CustomPendingKeyState) -> Self {
        PendingKeyState::Custom(pks)
    }
}

impl From<KeyboardPendingKeyState> for PendingKeyState {
    fn from(pks: KeyboardPendingKeyState) -> Self {
        PendingKeyState::Keyboard(pks)
    }
}

impl From<LayeredPendingKeyState> for PendingKeyState {
    fn from(pks: LayeredPendingKeyState) -> Self {
        PendingKeyState::Layered(pks)
    }
}

impl From<MousePendingKeyState> for PendingKeyState {
    fn from(pks: MousePendingKeyState) -> Self {
        PendingKeyState::Mouse(pks)
    }
}

impl From<StickyPendingKeyState> for PendingKeyState {
    fn from(pks: StickyPendingKeyState) -> Self {
        PendingKeyState::Sticky(pks)
    }
}

impl From<TapDancePendingKeyState> for PendingKeyState {
    fn from(pks: TapDancePendingKeyState) -> Self {
        PendingKeyState::TapDance(pks)
    }
}

impl From<TapHoldPendingKeyState> for PendingKeyState {
    fn from(pks: TapHoldPendingKeyState) -> Self {
        PendingKeyState::TapHold(pks)
    }
}

impl<'pks> TryFrom<&'pks mut PendingKeyState> for &'pks mut ChordedPendingKeyState {
    type Error = ();

    fn try_from(pks: &'pks mut PendingKeyState) -> Result<Self, Self::Error> {
        match pks {
            PendingKeyState::Chorded(pks) => Ok(pks),
            _ => Err(()),
        }
    }
}

impl<'pks> TryFrom<&'pks mut PendingKeyState> for &'pks mut TapDancePendingKeyState {
    type Error = ();

    fn try_from(pks: &'pks mut PendingKeyState) -> Result<Self, Self::Error> {
        match pks {
            PendingKeyState::TapDance(pks) => Ok(pks),
            _ => Err(()),
        }
    }
}

impl<'pks> TryFrom<&'pks mut PendingKeyState> for &'pks mut TapHoldPendingKeyState {
    type Error = ();

    fn try_from(pks: &'pks mut PendingKeyState) -> Result<Self, Self::Error> {
        match pks {
            PendingKeyState::TapHold(pks) => Ok(pks),
            _ => Err(()),
        }
    }
}

/// Aggregate enum for key state. (i.e. pressed key data).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyState {
    /// No-op key state.
    NoOp, // e.g. chorded::AuxiliaryKey's state is a no-op
    /// Key state for [key::automation::KeyState].
    Automation(AutomationKeyState),
    /// Key state for [key::callback::KeyState].
    Callback(CallbackKeyState),
    /// Key state for [key::caps_word::KeyState].
    CapsWord(CapsWordKeyState),
    /// Key state for [key::chorded::KeyState].
    Chorded(ChordedKeyState),
    /// Key state for [key::consumer::KeyState].
    Consumer(ConsumerKeyState),
    /// Key state for [key::custom::KeyState].
    Custom(CustomKeyState),
    /// Key state for [key::keyboard::KeyState].
    Keyboard(KeyboardKeyState),
    /// Key state for [key::layered::ModifierKeyState].
    LayerModifier(LayeredKeyState),
    /// Key state for [key::mouse::KeyState].
    Mouse(MouseKeyState),
    /// Key state for [key::sticky::KeyState].
    Sticky(StickyKeyState),
    /// Key state for [key::tap_dance::KeyState].
    TapDance(TapDanceKeyState),
    /// Key state for [key::tap_hold::KeyState].
    TapHold(TapHoldKeyState),
}

impl From<key::NoOpKeyState> for KeyState {
    fn from(_: key::NoOpKeyState) -> Self {
        KeyState::NoOp
    }
}

impl From<AutomationKeyState> for KeyState {
    fn from(ks: AutomationKeyState) -> Self {
        KeyState::Automation(ks)
    }
}

impl From<CallbackKeyState> for KeyState {
    fn from(ks: CallbackKeyState) -> Self {
        KeyState::Callback(ks)
    }
}

impl From<CapsWordKeyState> for KeyState {
    fn from(ks: CapsWordKeyState) -> Self {
        KeyState::CapsWord(ks)
    }
}

impl From<ChordedKeyState> for KeyState {
    fn from(ks: ChordedKeyState) -> Self {
        KeyState::Chorded(ks)
    }
}

impl From<ConsumerKeyState> for KeyState {
    fn from(ks: ConsumerKeyState) -> Self {
        KeyState::Consumer(ks)
    }
}

impl From<CustomKeyState> for KeyState {
    fn from(ks: CustomKeyState) -> Self {
        KeyState::Custom(ks)
    }
}

impl From<KeyboardKeyState> for KeyState {
    fn from(ks: KeyboardKeyState) -> Self {
        KeyState::Keyboard(ks)
    }
}

impl From<LayeredKeyState> for KeyState {
    fn from(ks: LayeredKeyState) -> Self {
        KeyState::LayerModifier(ks)
    }
}

impl From<MouseKeyState> for KeyState {
    fn from(ks: MouseKeyState) -> Self {
        KeyState::Mouse(ks)
    }
}

impl From<TapDanceKeyState> for KeyState {
    fn from(ks: TapDanceKeyState) -> Self {
        KeyState::TapDance(ks)
    }
}

impl From<TapHoldKeyState> for KeyState {
    fn from(ks: TapHoldKeyState) -> Self {
        KeyState::TapHold(ks)
    }
}

impl From<StickyKeyState> for KeyState {
    fn from(ks: StickyKeyState) -> Self {
        KeyState::Sticky(ks)
    }
}

/// Convenience trait for the data storage types.
pub trait Keys {
    /// Type used by [key::automation::System].
    type Automation: Debug + Index<usize, Output = AutomationKey>;
    /// Type used by [key::callback::System].
    type Callback: Debug + Index<usize, Output = CallbackKey>;
    /// Type used by [key::chorded::System].
    type Chorded: Debug + Index<usize, Output = ChordedKey>;
    /// Type used by [key::chorded::System].
    type ChordedAuxiliary: Debug + Index<usize, Output = ChordedAuxiliaryKey>;
    /// Type used by [key::keyboard::System].
    type Keyboard: Debug + Index<usize, Output = KeyboardKey>;
    /// Type used by [key::layered::System].
    type LayerModifiers: Debug + Index<usize, Output = LayeredModifierKey>;
    /// Type used by [key::layered::System].
    type Layered: Debug + Index<usize, Output = LayeredKey>;
    /// Type used by [key::sticky::System].
    type Sticky: Debug + Index<usize, Output = StickyKey>;
    /// Type used by [key::tap_dance::System].
    type TapDance: Debug + Index<usize, Output = TapDanceKey>;
    /// Type used by [key::tap_hold::System].
    type TapHold: Debug + Index<usize, Output = TapHoldKey>;
}

/// Array-based data implementations.
#[derive(Debug)]
pub struct KeyArrays<
    const AUTOMATION: usize,
    const CALLBACK: usize,
    const CHORDED: usize,
    const CHORDED_AUXILIARY: usize,
    const KEYBOARD: usize,
    const LAYERED: usize,
    const LAYER_MODIFIERS: usize,
    const STICKY: usize,
    const TAP_DANCE: usize,
    const TAP_HOLD: usize,
>;

impl<
        const AUTOMATION: usize,
        const CALLBACK: usize,
        const CHORDED: usize,
        const CHORDED_AUXILIARY: usize,
        const KEYBOARD: usize,
        const LAYERED: usize,
        const LAYER_MODIFIERS: usize,
        const STICKY: usize,
        const TAP_DANCE: usize,
        const TAP_HOLD: usize,
    > Keys
    for KeyArrays<
        AUTOMATION,
        CALLBACK,
        CHORDED,
        CHORDED_AUXILIARY,
        KEYBOARD,
        LAYERED,
        LAYER_MODIFIERS,
        STICKY,
        TAP_DANCE,
        TAP_HOLD,
    >
{
    type Automation = [AutomationKey; AUTOMATION];
    type Callback = [CallbackKey; CALLBACK];
    type Chorded = [ChordedKey; CHORDED];
    type ChordedAuxiliary = [ChordedAuxiliaryKey; CHORDED_AUXILIARY];
    type Keyboard = [KeyboardKey; KEYBOARD];
    type LayerModifiers = [LayeredModifierKey; LAYER_MODIFIERS];
    type Layered = [LayeredKey; LAYERED];
    type Sticky = [StickyKey; STICKY];
    type TapDance = [TapDanceKey; TAP_DANCE];
    type TapHold = [TapHoldKey; TAP_HOLD];
}

/// Vec-based data implementations.
#[derive(Debug)]
#[cfg(feature = "std")]
pub struct KeyVecs;

#[cfg(feature = "std")]
impl Keys for KeyVecs {
    type Automation = Vec<AutomationKey>;
    type Callback = Vec<CallbackKey>;
    type Chorded = Vec<ChordedKey>;
    type ChordedAuxiliary = Vec<ChordedAuxiliaryKey>;
    type Keyboard = Vec<KeyboardKey>;
    type LayerModifiers = Vec<LayeredModifierKey>;
    type Layered = Vec<LayeredKey>;
    type Sticky = Vec<StickyKey>;
    type TapDance = Vec<TapDanceKey>;
    type TapHold = Vec<TapHoldKey>;
}

/// Aggregate [key::System] implementation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct System<D: Keys> {
    automation: AutomationSystem<D::Automation>,
    callback: CallbackSystem<D::Callback>,
    caps_word: CapsWordSystem,
    consumer: ConsumerSystem,
    chorded: ChordedSystem<D::Chorded, D::ChordedAuxiliary>,
    custom: CustomSystem,
    keyboard: KeyboardSystem<D::Keyboard>,
    layered: LayeredSystem<D::LayerModifiers, D::Layered>,
    mouse: MouseSystem,
    sticky: StickySystem<D::Sticky>,
    tap_dance: TapDanceSystem<D::TapDance>,
    tap_hold: TapHoldSystem<D::TapHold>,
    marker: PhantomData<D>,
}

impl<
        const AUTOMATION: usize,
        const CALLBACK: usize,
        const CHORDED: usize,
        const CHORDED_AUXILIARY: usize,
        const KEYBOARD: usize,
        const LAYERED: usize,
        const LAYER_MODIFIERS: usize,
        const STICKY: usize,
        const TAP_DANCE: usize,
        const TAP_HOLD: usize,
    >
    System<
        KeyArrays<
            AUTOMATION,
            CALLBACK,
            CHORDED,
            CHORDED_AUXILIARY,
            KEYBOARD,
            LAYERED,
            LAYER_MODIFIERS,
            STICKY,
            TAP_DANCE,
            TAP_HOLD,
        >,
    >
{
    /// Constructs a new [System].
    pub const fn array_based(
        automation: AutomationSystem<[AutomationKey; AUTOMATION]>,
        callback: CallbackSystem<[CallbackKey; CALLBACK]>,
        chorded: ChordedSystem<[ChordedKey; CHORDED], [ChordedAuxiliaryKey; CHORDED_AUXILIARY]>,
        keyboard: KeyboardSystem<[KeyboardKey; KEYBOARD]>,
        layered: LayeredSystem<[LayeredModifierKey; LAYER_MODIFIERS], [LayeredKey; LAYERED]>,
        sticky: StickySystem<[StickyKey; STICKY]>,
        tap_dance: TapDanceSystem<[TapDanceKey; TAP_DANCE]>,
        tap_hold: TapHoldSystem<[TapHoldKey; TAP_HOLD]>,
    ) -> Self {
        System {
            automation,
            callback,
            caps_word: key::caps_word::System::new(),
            consumer: key::consumer::System::new(),
            chorded,
            custom: key::custom::System::new(),
            keyboard,
            layered,
            mouse: key::mouse::System::new(),
            sticky,
            tap_dance,
            tap_hold,
            marker: PhantomData,
        }
    }
}

#[cfg(feature = "std")]
impl System<KeyVecs> {
    /// Constructs a new [System].
    pub const fn vec_based(
        automation: AutomationSystem<Vec<AutomationKey>>,
        callback: CallbackSystem<Vec<CallbackKey>>,
        chorded: ChordedSystem<Vec<ChordedKey>, Vec<ChordedAuxiliaryKey>>,
        keyboard: KeyboardSystem<Vec<KeyboardKey>>,
        layered: LayeredSystem<Vec<LayeredModifierKey>, Vec<LayeredKey>>,
        sticky: StickySystem<Vec<StickyKey>>,
        tap_dance: TapDanceSystem<Vec<TapDanceKey>>,
        tap_hold: TapHoldSystem<Vec<TapHoldKey>>,
    ) -> Self {
        System {
            automation,
            callback,
            caps_word: key::caps_word::System::new(),
            consumer: key::consumer::System::new(),
            chorded,
            custom: key::custom::System::new(),
            keyboard,
            layered,
            mouse: key::mouse::System::new(),
            sticky,
            tap_dance,
            tap_hold,
            marker: PhantomData,
        }
    }
}

impl<K: Debug + Keys> key::System<Ref> for System<K> {
    type Ref = Ref;
    type Context = Context;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        keymap_index: u16,
        context: &Self::Context,
        key_ref: Ref,
    ) -> (
        key::PressedKeyResult<Ref, Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        match key_ref {
            Ref::Automation(key_ref) => {
                let (pkr, pke) =
                    self.automation
                        .new_pressed_key(keymap_index, &context.automation, key_ref);
                (pkr.into_result(), pke.into_events())
            }
            Ref::Callback(key_ref) => {
                let (pkr, pke) =
                    self.callback
                        .new_pressed_key(keymap_index, &context.callback, key_ref);
                (pkr.into_result(), pke.into_events())
            }
            Ref::CapsWord(key_ref) => {
                let (pkr, pke) =
                    self.caps_word
                        .new_pressed_key(keymap_index, &context.caps_word, key_ref);
                (pkr.into_result(), pke.into_events())
            }
            Ref::Chorded(key_ref) => {
                let (pkr, pke) =
                    self.chorded
                        .new_pressed_key(keymap_index, &context.chorded, key_ref);
                (pkr.into_result(), pke.into_events())
            }
            Ref::Consumer(key_ref) => {
                let (pkr, pke) =
                    self.consumer
                        .new_pressed_key(keymap_index, &context.consumer, key_ref);
                (pkr.into_result(), pke.into_events())
            }
            Ref::Custom(key_ref) => {
                let (pkr, pke) =
                    self.custom
                        .new_pressed_key(keymap_index, &context.custom, key_ref);
                (pkr.into_result(), pke.into_events())
            }
            Ref::Keyboard(key_ref) => {
                let (pkr, pke) =
                    self.keyboard
                        .new_pressed_key(keymap_index, &context.keyboard, key_ref);
                (pkr.into_result(), pke.into_events())
            }
            Ref::Layered(key_ref) => {
                let (pkr, pke) =
                    self.layered
                        .new_pressed_key(keymap_index, &context.layered, key_ref);
                (pkr.into_result(), pke.into_events())
            }
            Ref::Mouse(key_ref) => {
                let (pkr, pke) = self
                    .mouse
                    .new_pressed_key(keymap_index, &context.mouse, key_ref);
                (pkr.into_result(), pke.into_events())
            }
            Ref::Sticky(key_ref) => {
                let (pkr, pke) =
                    self.sticky
                        .new_pressed_key(keymap_index, &context.sticky, key_ref);
                (pkr.into_result(), pke.into_events())
            }
            Ref::TapDance(key_ref) => {
                let (pkr, pke) =
                    self.tap_dance
                        .new_pressed_key(keymap_index, &context.tap_dance, key_ref);
                (pkr.into_result(), pke.into_events())
            }
            Ref::TapHold(key_ref) => {
                let (pkr, pke) =
                    self.tap_hold
                        .new_pressed_key(keymap_index, &context.tap_hold, key_ref);
                (pkr.into_result(), pke.into_events())
            }
        }
    }

    fn update_pending_state(
        &self,
        pending_state: &mut Self::PendingKeyState,
        keymap_index: u16,
        context: &Self::Context,
        key_ref: Ref,
        event: key::Event<Self::Event>,
    ) -> (Option<key::NewPressedKey<Ref>>, key::KeyEvents<Self::Event>) {
        match (key_ref, pending_state) {
            (Ref::Chorded(key_ref), PendingKeyState::Chorded(pending_state)) => {
                if let Ok(event) = event.try_into_key_event() {
                    let (maybe_npk, pke) = self.chorded.update_pending_state(
                        pending_state,
                        keymap_index,
                        &context.chorded,
                        key_ref,
                        event,
                    );
                    (maybe_npk, pke.into_events())
                } else {
                    (None, key::KeyEvents::no_events())
                }
            }
            (Ref::TapDance(key_ref), PendingKeyState::TapDance(pending_state)) => {
                if let Ok(event) = event.try_into_key_event() {
                    let (maybe_npk, pke) = self.tap_dance.update_pending_state(
                        pending_state,
                        keymap_index,
                        &context.tap_dance,
                        key_ref,
                        event,
                    );
                    (maybe_npk, pke.into_events())
                } else {
                    (None, key::KeyEvents::no_events())
                }
            }
            (Ref::TapHold(key_ref), PendingKeyState::TapHold(pending_state)) => {
                if let Ok(event) = event.try_into_key_event() {
                    let (maybe_npk, pke) = self.tap_hold.update_pending_state(
                        pending_state,
                        keymap_index,
                        &context.tap_hold,
                        key_ref,
                        event,
                    );
                    (maybe_npk, pke.into_events())
                } else {
                    (None, key::KeyEvents::no_events())
                }
            }
            (_, _) => panic!("Mismatched key_ref and key_state variants"),
        }
    }

    fn update_state(
        &self,
        key_state: &mut Self::KeyState,
        key_ref: &Self::Ref,
        context: &Self::Context,
        keymap_index: u16,
        event: key::Event<Self::Event>,
    ) -> key::KeyEvents<Self::Event> {
        match (key_ref, key_state) {
            (Ref::Automation(key_ref), KeyState::Automation(key_state)) => {
                if let Ok(event) = event.try_into_key_event() {
                    let pke = self.automation.update_state(
                        key_state,
                        key_ref,
                        &context.automation,
                        keymap_index,
                        event,
                    );
                    pke.into_events()
                } else {
                    key::KeyEvents::no_events()
                }
            }
            (Ref::Consumer(key_ref), KeyState::Consumer(key_state)) => {
                if let Ok(event) = event.try_into_key_event() {
                    let pke = self.consumer.update_state(
                        key_state,
                        key_ref,
                        &context.consumer,
                        keymap_index,
                        event,
                    );
                    pke.into_events()
                } else {
                    key::KeyEvents::no_events()
                }
            }
            (Ref::Keyboard(key_ref), KeyState::Keyboard(key_state)) => {
                if let Ok(event) = event.try_into_key_event() {
                    let pke = self.keyboard.update_state(
                        key_state,
                        key_ref,
                        &context.keyboard,
                        keymap_index,
                        event,
                    );
                    pke.into_events()
                } else {
                    key::KeyEvents::no_events()
                }
            }
            (Ref::Layered(key_ref), KeyState::LayerModifier(key_state)) => {
                if let Ok(event) = event.try_into_key_event() {
                    let pke = self.layered.update_state(
                        key_state,
                        key_ref,
                        &context.layered,
                        keymap_index,
                        event,
                    );
                    pke.into_events()
                } else {
                    key::KeyEvents::no_events()
                }
            }
            (Ref::Sticky(key_ref), KeyState::Sticky(key_state)) => {
                if let Ok(event) = event.try_into_key_event() {
                    let pke = self.sticky.update_state(
                        key_state,
                        key_ref,
                        &context.sticky,
                        keymap_index,
                        event,
                    );
                    pke.into_events()
                } else {
                    key::KeyEvents::no_events()
                }
            }
            (_, _) => key::KeyEvents::no_events(),
        }
    }

    fn key_output(
        &self,
        key_ref: &Self::Ref,
        key_state: &Self::KeyState,
    ) -> Option<key::KeyOutput> {
        match (key_ref, key_state) {
            (Ref::Consumer(r), KeyState::Consumer(ks)) => self.consumer.key_output(r, ks),
            (Ref::Custom(r), KeyState::Custom(ks)) => self.custom.key_output(r, ks),
            (Ref::Keyboard(r), KeyState::Keyboard(ks)) => self.keyboard.key_output(r, ks),
            (Ref::Mouse(r), KeyState::Mouse(ks)) => self.mouse.key_output(r, ks),
            (Ref::Sticky(r), KeyState::Sticky(ks)) => self.sticky.key_output(r, ks),
            (_, _) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sizeof_ref() {
        assert_eq!(3, core::mem::size_of::<Ref>());
    }

    #[test]
    fn test_sizeof_event() {
        assert_eq!(12, core::mem::size_of::<Event>());
    }
}
