//! This module aggregates various [crate::key::System] implementations.

use core::fmt::Debug;
use core::marker::PhantomData;
use core::ops::Index;

use serde::Deserialize;

use crate::{key, keymap};

/// Aggregate enum for key references.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Ref {
    /// [key::keyboard::Ref] variant.
    Keyboard(key::keyboard::Ref),
    /// [key::caps_word::Ref] variant.
    CapsWord(key::caps_word::Ref),
    /// [key::callback::Ref] variant.
    Callback(key::callback::Ref),
    /// [key::sticky::Ref] variant.
    Sticky(key::sticky::Ref),
    /// [key::custom::Ref] variant.
    Custom(key::custom::Ref),
    /// [key::tap_dance::Ref] variant.
    TapDance(key::tap_dance::Ref),
    /// [key::tap_hold::Ref] variant.
    TapHold(key::tap_hold::Ref),
    /// [key::layered::Ref] variant.
    Layered(key::layered::Ref),
    /// [key::chorded::Ref] variant.
    Chorded(key::chorded::Ref),
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
    /// The chorded configuration.
    #[serde(default)]
    pub chorded: key::chorded::Config,
    /// The sticky modifier configuration
    #[serde(default)]
    pub sticky: key::sticky::Config,
    /// The tap dance configuration.
    #[serde(default)]
    pub tap_dance: key::tap_dance::Config,
    /// The tap hold configuration.
    #[serde(default)]
    pub tap_hold: key::tap_hold::Config,
}

/// The default config.
pub const DEFAULT_CONFIG: Config = Config {
    chorded: key::chorded::DEFAULT_CONFIG,
    sticky: key::sticky::DEFAULT_CONFIG,
    tap_dance: key::tap_dance::DEFAULT_CONFIG,
    tap_hold: key::tap_hold::DEFAULT_CONFIG,
};

/// An aggregate context for [key::Context]s.
#[derive(Debug, Clone, Copy)]
pub struct Context {
    keymap_context: keymap::KeymapContext,
    caps_word_context: key::caps_word::Context,
    chorded: key::chorded::Context,
    layered: key::layered::Context,
    tap_dance: key::tap_dance::Context,
    tap_hold: key::tap_hold::Context,
    sticky: key::sticky::Context,
}

/// The default context.
pub const DEFAULT_CONTEXT: Context = Context {
    keymap_context: keymap::DEFAULT_KEYMAP_CONTEXT,
    caps_word_context: key::caps_word::DEFAULT_CONTEXT,
    chorded: key::chorded::DEFAULT_CONTEXT,
    layered: key::layered::DEFAULT_CONTEXT,
    sticky: key::sticky::DEFAULT_CONTEXT,
    tap_dance: key::tap_dance::DEFAULT_CONTEXT,
    tap_hold: key::tap_hold::DEFAULT_CONTEXT,
};

impl Context {
    /// Constructs a [Context] from the given [Config].
    pub const fn from_config(config: Config) -> Self {
        Self {
            chorded: key::chorded::Context::from_config(config.chorded),
            sticky: key::sticky::Context::from_config(config.sticky),
            tap_dance: key::tap_dance::Context::from_config(config.tap_dance),
            tap_hold: key::tap_hold::Context::from_config(config.tap_hold),
            ..DEFAULT_CONTEXT
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
    fn handle_event(&mut self, event: key::Event<Self::Event>) -> key::KeyEvents<Self::Event> {
        let mut pke = key::KeyEvents::no_events();

        let caps_word_ev = self.caps_word_context.handle_event(event);
        pke.extend(caps_word_ev);

        if let Ok(e) = event.try_into_key_event() {
            let sticky_ev = self.sticky.handle_event(e);
            pke.extend(sticky_ev.into_events());
        }

        if let Ok(e) = event.try_into_key_event() {
            self.chorded.handle_event(e);
        }

        if let key::Event::Key {
            key_event: Event::LayerModification(ev),
            ..
        } = event
        {
            self.layered.handle_event(ev);
        }

        pke
    }
}

impl keymap::SetKeymapContext for Context {
    fn set_keymap_context(&mut self, context: keymap::KeymapContext) {
        self.keymap_context = context;

        self.tap_hold.update_keymap_context(&context);
    }
}

impl<'c> From<&'c Context> for &'c keymap::KeymapContext {
    fn from(ctx: &'c Context) -> Self {
        &ctx.keymap_context
    }
}

impl<'c> From<&'c Context> for &'c key::keyboard::Context {
    fn from(_ctx: &'c Context) -> Self {
        &key::keyboard::Context
    }
}

impl<'c> From<&'c Context> for &'c key::caps_word::Context {
    fn from(ctx: &'c Context) -> Self {
        &ctx.caps_word_context
    }
}

impl<'c> From<&'c Context> for &'c key::callback::Context {
    fn from(_ctx: &'c Context) -> Self {
        &key::callback::Context
    }
}

impl<'c> From<&'c Context> for &'c key::chorded::Context {
    fn from(ctx: &'c Context) -> Self {
        &ctx.chorded
    }
}

impl<'c> From<&'c Context> for &'c key::layered::Context {
    fn from(ctx: &'c Context) -> Self {
        &ctx.layered
    }
}

impl<'c> From<&'c Context> for &'c key::sticky::Context {
    fn from(ctx: &'c Context) -> Self {
        &ctx.sticky
    }
}

impl<'c> From<&'c Context> for &'c key::custom::Context {
    fn from(_ctx: &'c Context) -> Self {
        &key::custom::Context
    }
}

impl<'c> From<&'c Context> for &'c key::tap_dance::Context {
    fn from(ctx: &'c Context) -> Self {
        &ctx.tap_dance
    }
}

impl<'c> From<&'c Context> for &'c key::tap_hold::Context {
    fn from(ctx: &'c Context) -> Self {
        &ctx.tap_hold
    }
}

/// Sum type aggregating the [key::Event] types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// A keyboard event.
    Keyboard(key::keyboard::Event),
    /// A caps word event.
    CapsWord(key::caps_word::Event),
    /// A callback event.
    Callback(key::callback::Event),
    /// A custom event.
    Custom(key::custom::Event),
    /// A chorded event.
    Chorded(key::chorded::Event),
    /// A sticky modifier event.
    Sticky(key::sticky::Event),
    /// A tap-dance event.
    TapDance(key::tap_dance::Event),
    /// A tap-hold event.
    TapHold(key::tap_hold::Event),
    /// A layer modification event.
    LayerModification(key::layered::LayerEvent),
}

impl From<key::keyboard::Event> for Event {
    fn from(ev: key::keyboard::Event) -> Self {
        Event::Keyboard(ev)
    }
}

impl From<key::caps_word::Event> for Event {
    fn from(ev: key::caps_word::Event) -> Self {
        Event::CapsWord(ev)
    }
}

impl From<key::callback::Event> for Event {
    fn from(ev: key::callback::Event) -> Self {
        Event::Callback(ev)
    }
}

impl From<key::custom::Event> for Event {
    fn from(ev: key::custom::Event) -> Self {
        Event::Custom(ev)
    }
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

impl From<key::sticky::Event> for Event {
    fn from(ev: key::sticky::Event) -> Self {
        Event::Sticky(ev)
    }
}

impl From<key::tap_dance::Event> for Event {
    fn from(ev: key::tap_dance::Event) -> Self {
        Event::TapDance(ev)
    }
}

impl From<key::tap_hold::Event> for Event {
    fn from(ev: key::tap_hold::Event) -> Self {
        Event::TapHold(ev)
    }
}

impl TryFrom<Event> for key::keyboard::Event {
    type Error = key::EventError;

    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        match ev {
            Event::Keyboard(ev) => Ok(ev),
            _ => Err(key::EventError::UnmappableEvent),
        }
    }
}

impl TryFrom<Event> for key::caps_word::Event {
    type Error = key::EventError;

    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        match ev {
            Event::CapsWord(ev) => Ok(ev),
            _ => Err(key::EventError::UnmappableEvent),
        }
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

impl TryFrom<Event> for key::sticky::Event {
    type Error = key::EventError;

    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        match ev {
            Event::Sticky(ev) => Ok(ev),
            _ => Err(key::EventError::UnmappableEvent),
        }
    }
}

impl TryFrom<Event> for key::tap_dance::Event {
    type Error = key::EventError;

    fn try_from(ev: Event) -> Result<Self, Self::Error> {
        match ev {
            Event::TapDance(ev) => Ok(ev),
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

/// Aggregate enum for pending key state.
#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum PendingKeyState {
    /// Pending key state for [key::keyboard::PendingKeyState].
    Keyboard(key::keyboard::PendingKeyState),
    /// Pending key state for [key::caps_word::PendingKeyState].
    CapsWord(key::caps_word::PendingKeyState),
    /// Pending key state for [key::callback::PendingKeyState].
    Callback(key::callback::PendingKeyState),
    /// Pending key state for [key::sticky::PendingKeyState].
    Sticky(key::sticky::PendingKeyState),
    /// Pending key state for [key::custom::PendingKeyState].
    Custom(key::custom::PendingKeyState),
    /// Pending key state for [key::layered::PendingKeyState].
    Layered(key::layered::PendingKeyState),
    /// Pending key state for [key::tap_dance::PendingKeyState].
    TapDance(key::tap_dance::PendingKeyState),
    /// Pending key state for [key::tap_hold::PendingKeyState].
    TapHold(key::tap_hold::PendingKeyState),
    /// Pending key state for [key::chorded::PendingKeyState].
    Chorded(key::chorded::PendingKeyState),
}

impl From<key::keyboard::PendingKeyState> for PendingKeyState {
    fn from(pks: key::keyboard::PendingKeyState) -> Self {
        PendingKeyState::Keyboard(pks)
    }
}

impl From<key::caps_word::PendingKeyState> for PendingKeyState {
    fn from(pks: key::caps_word::PendingKeyState) -> Self {
        PendingKeyState::CapsWord(pks)
    }
}

impl From<key::callback::PendingKeyState> for PendingKeyState {
    fn from(pks: key::callback::PendingKeyState) -> Self {
        PendingKeyState::Callback(pks)
    }
}

impl From<key::sticky::PendingKeyState> for PendingKeyState {
    fn from(pks: key::sticky::PendingKeyState) -> Self {
        PendingKeyState::Sticky(pks)
    }
}

impl From<key::custom::PendingKeyState> for PendingKeyState {
    fn from(pks: key::custom::PendingKeyState) -> Self {
        PendingKeyState::Custom(pks)
    }
}

impl From<key::tap_dance::PendingKeyState> for PendingKeyState {
    fn from(pks: key::tap_dance::PendingKeyState) -> Self {
        PendingKeyState::TapDance(pks)
    }
}

impl From<key::tap_hold::PendingKeyState> for PendingKeyState {
    fn from(pks: key::tap_hold::PendingKeyState) -> Self {
        PendingKeyState::TapHold(pks)
    }
}

impl From<key::layered::PendingKeyState> for PendingKeyState {
    fn from(pks: key::layered::PendingKeyState) -> Self {
        PendingKeyState::Layered(pks)
    }
}

impl From<key::chorded::PendingKeyState> for PendingKeyState {
    fn from(pks: key::chorded::PendingKeyState) -> Self {
        PendingKeyState::Chorded(pks)
    }
}

impl<'pks> TryFrom<&'pks mut PendingKeyState> for &'pks mut key::tap_dance::PendingKeyState {
    type Error = ();

    fn try_from(pks: &'pks mut PendingKeyState) -> Result<Self, Self::Error> {
        match pks {
            PendingKeyState::TapDance(pks) => Ok(pks),
            _ => Err(()),
        }
    }
}

impl<'pks> TryFrom<&'pks mut PendingKeyState> for &'pks mut key::tap_hold::PendingKeyState {
    type Error = ();

    fn try_from(pks: &'pks mut PendingKeyState) -> Result<Self, Self::Error> {
        match pks {
            PendingKeyState::TapHold(pks) => Ok(pks),
            _ => Err(()),
        }
    }
}

impl<'pks> TryFrom<&'pks mut PendingKeyState> for &'pks mut key::chorded::PendingKeyState {
    type Error = ();

    fn try_from(pks: &'pks mut PendingKeyState) -> Result<Self, Self::Error> {
        match pks {
            PendingKeyState::Chorded(pks) => Ok(pks),
            _ => Err(()),
        }
    }
}

/// Aggregate enum for key state. (i.e. pressed key data).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyState {
    /// No-op key state.
    NoOp, // e.g. chorded::AuxiliaryKey's state is a no-op
    /// Key state for [key::callback::KeyState].
    Callback(key::callback::KeyState),
    /// Key state for [key::caps_word::KeyState].
    CapsWord(key::caps_word::KeyState),
    /// Key state for [key::chorded::KeyState].
    Chorded(key::chorded::KeyState),
    /// Key state for [key::custom::KeyState].
    Custom(key::custom::KeyState),
    /// Key state for [key::keyboard::KeyState].
    Keyboard(key::keyboard::KeyState),
    /// Key state for [key::layered::ModifierKeyState].
    LayerModifier(key::layered::ModifierKeyState),
    /// Key state for [key::sticky::KeyState].
    Sticky(key::sticky::KeyState),
    /// Key state for [key::tap_dance::KeyState].
    TapDance(key::tap_dance::KeyState),
    /// Key state for [key::tap_hold::KeyState].
    TapHold(key::tap_hold::KeyState),
}

impl From<key::NoOpKeyState> for KeyState {
    fn from(_: key::NoOpKeyState) -> Self {
        KeyState::NoOp
    }
}

impl From<key::callback::KeyState> for KeyState {
    fn from(ks: key::callback::KeyState) -> Self {
        KeyState::Callback(ks)
    }
}

impl From<key::caps_word::KeyState> for KeyState {
    fn from(ks: key::caps_word::KeyState) -> Self {
        KeyState::CapsWord(ks)
    }
}

impl From<key::chorded::KeyState> for KeyState {
    fn from(ks: key::chorded::KeyState) -> Self {
        KeyState::Chorded(ks)
    }
}

impl From<key::custom::KeyState> for KeyState {
    fn from(ks: key::custom::KeyState) -> Self {
        KeyState::Custom(ks)
    }
}

impl From<key::keyboard::KeyState> for KeyState {
    fn from(ks: key::keyboard::KeyState) -> Self {
        KeyState::Keyboard(ks)
    }
}

impl From<key::layered::ModifierKeyState> for KeyState {
    fn from(ks: key::layered::ModifierKeyState) -> Self {
        KeyState::LayerModifier(ks)
    }
}

impl From<key::tap_dance::KeyState> for KeyState {
    fn from(ks: key::tap_dance::KeyState) -> Self {
        KeyState::TapDance(ks)
    }
}

impl From<key::tap_hold::KeyState> for KeyState {
    fn from(ks: key::tap_hold::KeyState) -> Self {
        KeyState::TapHold(ks)
    }
}

impl From<key::sticky::KeyState> for KeyState {
    fn from(ks: key::sticky::KeyState) -> Self {
        KeyState::Sticky(ks)
    }
}

/// Convenience trait for the data storage types.
pub trait Keys {
    /// Type used by [key::callback::System].
    type Callback: Debug + Index<usize, Output = key::callback::Key>;
    /// Type used by [key::chorded::System].
    type Chorded: Debug + Index<usize, Output = key::chorded::Key<Ref>>;
    /// Type used by [key::chorded::System].
    type ChordedAuxiliary: Debug + Index<usize, Output = key::chorded::AuxiliaryKey<Ref>>;
    /// Type used by [key::keyboard::System].
    type Keyboard: Debug + Index<usize, Output = key::keyboard::Key>;
    /// Type used by [key::layered::System].
    type LayerModifiers: Debug + Index<usize, Output = key::layered::ModifierKey>;
    /// Type used by [key::layered::System].
    type Layered: Debug + Index<usize, Output = key::layered::LayeredKey<Ref>>;
    /// Type used by [key::sticky::System].
    type Sticky: Debug + Index<usize, Output = key::sticky::Key>;
    /// Type used by [key::tap_dance::System].
    type TapDance: Debug + Index<usize, Output = key::tap_dance::Key<Ref>>;
    /// Type used by [key::tap_hold::System].
    type TapHold: Debug + Index<usize, Output = key::tap_hold::Key<Ref>>;
}

/// Array-based data implementations.
#[derive(Debug)]
pub struct KeyArrays<
    const KEYBOARD: usize,
    const CALLBACK: usize,
    const STICKY: usize,
    const TAP_DANCE: usize,
    const TAP_HOLD: usize,
    const LAYER_MODIFIERS: usize,
    const LAYERED: usize,
    const CHORDED: usize,
    const CHORDED_AUXILIARY: usize,
>;

impl<
        const KEYBOARD: usize,
        const CALLBACK: usize,
        const STICKY: usize,
        const TAP_DANCE: usize,
        const TAP_HOLD: usize,
        const LAYER_MODIFIERS: usize,
        const LAYERED: usize,
        const CHORDED: usize,
        const CHORDED_AUXILIARY: usize,
    > Keys
    for KeyArrays<
        KEYBOARD,
        CALLBACK,
        STICKY,
        TAP_DANCE,
        TAP_HOLD,
        LAYER_MODIFIERS,
        LAYERED,
        CHORDED,
        CHORDED_AUXILIARY,
    >
{
    type Callback = [key::callback::Key; CALLBACK];
    type Chorded = [key::chorded::Key<Ref>; CHORDED];
    type ChordedAuxiliary = [key::chorded::AuxiliaryKey<Ref>; CHORDED_AUXILIARY];
    type Keyboard = [key::keyboard::Key; KEYBOARD];
    type LayerModifiers = [key::layered::ModifierKey; LAYER_MODIFIERS];
    type Layered = [key::layered::LayeredKey<Ref>; LAYERED];
    type Sticky = [key::sticky::Key; STICKY];
    type TapDance = [key::tap_dance::Key<Ref>; TAP_DANCE];
    type TapHold = [key::tap_hold::Key<Ref>; TAP_HOLD];
}

/// Vec-based data implementations.
#[derive(Debug)]
#[cfg(feature = "std")]
pub struct KeyVecs;

#[cfg(feature = "std")]
impl Keys for KeyVecs {
    type Callback = Vec<key::callback::Key>;
    type Chorded = Vec<key::chorded::Key<Ref>>;
    type ChordedAuxiliary = Vec<key::chorded::AuxiliaryKey<Ref>>;
    type Keyboard = Vec<key::keyboard::Key>;
    type LayerModifiers = Vec<key::layered::ModifierKey>;
    type Layered = Vec<key::layered::LayeredKey<Ref>>;
    type Sticky = Vec<key::sticky::Key>;
    type TapDance = Vec<key::tap_dance::Key<Ref>>;
    type TapHold = Vec<key::tap_hold::Key<Ref>>;
}

/// Aggregate [key::System] implementation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct System<D: Keys> {
    callback: key::callback::System<D::Callback>,
    caps_word: key::caps_word::System,
    chorded: key::chorded::System<Ref, D::Chorded, D::ChordedAuxiliary>,
    custom: key::custom::System,
    keyboard: key::keyboard::System<D::Keyboard>,
    layered: key::layered::System<Ref, D::LayerModifiers, D::Layered>,
    sticky: key::sticky::System<D::Sticky>,
    tap_dance: key::tap_dance::System<Ref, D::TapDance>,
    tap_hold: key::tap_hold::System<Ref, D::TapHold>,
    marker: PhantomData<D>,
}

impl<
        const KEYBOARD: usize,
        const CALLBACK: usize,
        const STICKY: usize,
        const TAP_DANCE: usize,
        const TAP_HOLD: usize,
        const LAYER_MODIFIERS: usize,
        const LAYERED: usize,
        const CHORDED: usize,
        const CHORDED_AUXILIARY: usize,
    >
    System<
        KeyArrays<
            KEYBOARD,
            CALLBACK,
            STICKY,
            TAP_DANCE,
            TAP_HOLD,
            LAYER_MODIFIERS,
            LAYERED,
            CHORDED,
            CHORDED_AUXILIARY,
        >,
    >
{
    /// Constructs a new [System].
    pub const fn array_based(
        keyboard: key::keyboard::System<[key::keyboard::Key; KEYBOARD]>,
        callback: key::callback::System<[key::callback::Key; CALLBACK]>,
        sticky: key::sticky::System<[key::sticky::Key; STICKY]>,
        tap_dance: key::tap_dance::System<Ref, [key::tap_dance::Key<Ref>; TAP_DANCE]>,
        tap_hold: key::tap_hold::System<Ref, [key::tap_hold::Key<Ref>; TAP_HOLD]>,
        layered: key::layered::System<
            Ref,
            [key::layered::ModifierKey; LAYER_MODIFIERS],
            [key::layered::LayeredKey<Ref>; LAYERED],
        >,
        chorded: key::chorded::System<
            Ref,
            [key::chorded::Key<Ref>; CHORDED],
            [key::chorded::AuxiliaryKey<Ref>; CHORDED_AUXILIARY],
        >,
    ) -> Self {
        System {
            callback,
            caps_word: key::caps_word::System::new(),
            chorded,
            custom: key::custom::System::new(),
            keyboard,
            layered,
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
        keyboard: key::keyboard::System<<KeyVecs as Keys>::Keyboard>,
        callback: key::callback::System<<KeyVecs as Keys>::Callback>,
        sticky: key::sticky::System<<KeyVecs as Keys>::Sticky>,
        tap_dance: key::tap_dance::System<Ref, <KeyVecs as Keys>::TapDance>,
        tap_hold: key::tap_hold::System<Ref, <KeyVecs as Keys>::TapHold>,
        layered: key::layered::System<
            Ref,
            <KeyVecs as Keys>::LayerModifiers,
            <KeyVecs as Keys>::Layered,
        >,
        chorded: key::chorded::System<
            Ref,
            <KeyVecs as Keys>::Chorded,
            <KeyVecs as Keys>::ChordedAuxiliary,
        >,
    ) -> Self {
        System {
            callback,
            caps_word: key::caps_word::System::new(),
            chorded,
            custom: key::custom::System::new(),
            keyboard,
            layered,
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
            Ref::Callback(key_ref) => {
                let (pkr, pke) =
                    self.callback
                        .new_pressed_key(keymap_index, context.into(), key_ref);
                (pkr.into_result(), pke.into_events())
            }
            Ref::CapsWord(key_ref) => {
                let (pkr, pke) =
                    self.caps_word
                        .new_pressed_key(keymap_index, context.into(), key_ref);
                (pkr.into_result(), pke.into_events())
            }
            Ref::Chorded(key_ref) => {
                let (pkr, pke) =
                    self.chorded
                        .new_pressed_key(keymap_index, context.into(), key_ref);
                (pkr.into_result(), pke.into_events())
            }
            Ref::Custom(key_ref) => {
                let (pkr, pke) = self
                    .custom
                    .new_pressed_key(keymap_index, context.into(), key_ref);
                (pkr.into_result(), pke.into_events())
            }
            Ref::Keyboard(key_ref) => {
                let (pkr, pke) =
                    self.keyboard
                        .new_pressed_key(keymap_index, context.into(), key_ref);
                (pkr.into_result(), pke.into_events())
            }
            Ref::Layered(key_ref) => {
                let (pkr, pke) =
                    self.layered
                        .new_pressed_key(keymap_index, context.into(), key_ref);
                (pkr.into_result(), pke.into_events())
            }
            Ref::Sticky(key_ref) => {
                let (pkr, pke) = self
                    .sticky
                    .new_pressed_key(keymap_index, context.into(), key_ref);
                (pkr.into_result(), pke.into_events())
            }
            Ref::TapDance(key_ref) => {
                let (pkr, pke) =
                    self.tap_dance
                        .new_pressed_key(keymap_index, context.into(), key_ref);
                (pkr.into_result(), pke.into_events())
            }
            Ref::TapHold(key_ref) => {
                let (pkr, pke) =
                    self.tap_hold
                        .new_pressed_key(keymap_index, context.into(), key_ref);
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
                        context.into(),
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
                        context.into(),
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
                        context.into(),
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
            (Ref::Keyboard(key_ref), KeyState::Keyboard(key_state)) => {
                if let Ok(event) = event.try_into_key_event() {
                    let pke =
                        <key::keyboard::System<K::Keyboard> as key::System<Ref>>::update_state(
                            &self.keyboard,
                            key_state,
                            key_ref,
                            context.into(),
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
                    let pke =
                        <key::layered::System<Ref, K::LayerModifiers, K::Layered> as key::System<
                            Ref,
                        >>::update_state(
                            &self.layered,
                            key_state,
                            key_ref,
                            context.into(),
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
                    let pke = <key::sticky::System<K::Sticky> as key::System<Ref>>::update_state(
                        &self.sticky,
                        key_state,
                        key_ref,
                        context.into(),
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
            (Ref::Custom(r), KeyState::Custom(ks)) => {
                <key::custom::System as key::System<Ref>>::key_output(&self.custom, r, ks)
            }
            (Ref::Keyboard(r), KeyState::Keyboard(ks)) => {
                <key::keyboard::System<K::Keyboard> as key::System<Ref>>::key_output(
                    &self.keyboard,
                    r,
                    ks,
                )
            }
            (Ref::Sticky(r), KeyState::Sticky(ks)) => {
                <key::sticky::System<K::Sticky> as key::System<Ref>>::key_output(
                    &self.sticky,
                    r,
                    ks,
                )
            }
            (_, _) => None,
        }
    }
}
