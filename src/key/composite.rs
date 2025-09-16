//! This module aggregates various [key::System] implementations.
// #![doc = include_str!("doc_de_composite.md")]

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
    /// [key::sticky::Ref] variant.
    Sticky(key::sticky::Ref),
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
    // /// The tap dance configuration.
    // #[serde(default)]
    // pub tap_dance: key::tap_dance::Config,
    /// The tap hold configuration.
    #[serde(default)]
    pub tap_hold: key::tap_hold::Config,
}

/// The default config.
pub const DEFAULT_CONFIG: Config = Config {
    chorded: key::chorded::DEFAULT_CONFIG,
    sticky: key::sticky::DEFAULT_CONFIG,
    // tap_dance: key::tap_dance::DEFAULT_CONFIG,
    tap_hold: key::tap_hold::DEFAULT_CONFIG,
};

/// An aggregate context for [key::Context]s.
#[derive(Debug, Clone, Copy)]
pub struct Context {
    keymap_context: keymap::KeymapContext,
    // caps_word_context: key::caps_word::Context,
    chorded: key::chorded::Context,
    layered: key::layered::Context,
    // tap_dance_context: key::tap_dance::Context,
    tap_hold: key::tap_hold::Context,
    sticky: key::sticky::Context,
}

/// The default context.
pub const DEFAULT_CONTEXT: Context = Context {
    keymap_context: keymap::DEFAULT_KEYMAP_CONTEXT,
    // caps_word_context: key::caps_word::DEFAULT_CONTEXT,
    chorded: key::chorded::DEFAULT_CONTEXT,
    layered: key::layered::DEFAULT_CONTEXT,
    sticky: key::sticky::DEFAULT_CONTEXT,
    // tap_dance_context: key::tap_dance::DEFAULT_CONTEXT,
    tap_hold: key::tap_hold::DEFAULT_CONTEXT,
};

impl Context {
    /// Constructs a [Context] from the given [Config].
    pub const fn from_config(config: Config) -> Self {
        Self {
            chorded: key::chorded::Context::from_config(config.chorded),
            sticky: key::sticky::Context::from_config(config.sticky),
            // tap_dance_context: key::tap_dance::Context::from_config(config.tap_dance),
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

        // let caps_word_ev = self.caps_word_context.handle_event(event);
        // pke.extend(caps_word_ev);

        if let Ok(e) = event.try_into_key_event(|e| e.try_into()) {
            let sticky_ev = self.sticky.handle_event(e);
            pke.extend(sticky_ev.into_events());
        }

        if let Ok(e) = event.try_into_key_event(|e| e.try_into()) {
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

// impl<'c> From<&'c Context> for &'c key::caps_word::Context {
//     fn from(ctx: &'c Context) -> Self {
//         &ctx.caps_word_context
//     }
// }

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

// impl<'c> From<&'c Context> for &'c key::tap_dance::Context {
//     fn from(ctx: &'c Context) -> Self {
//         &ctx.tap_dance_context
//     }
// }

// impl<'c> From<&'c Context> for &'c key::tap_hold::Context {
//     fn from(ctx: &'c Context) -> Self {
//         &ctx.tap_hold_context
//     }
// }

/// Sum type aggregating the [key::Event] types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    // /// A caps word event.
    // CapsWord(key::caps_word::Event),
    /// A chorded event.
    Chorded(key::chorded::Event),
    /// A sticky modifier event.
    Sticky(key::sticky::Event),
    // /// A tap-dance event.
    // TapDance(key::tap_dance::Event),
    /// A tap-hold event.
    TapHold(key::tap_hold::Event),
    /// A layer modification event.
    LayerModification(key::layered::LayerEvent),
}

// impl From<key::caps_word::Event> for Event {
//     fn from(ev: key::caps_word::Event) -> Self {
//         Event::CapsWord(ev)
//     }
// }

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

// impl From<key::tap_dance::Event> for Event {
//     fn from(ev: key::tap_dance::Event) -> Self {
//         Event::TapDance(ev)
//     }
// }

impl From<key::tap_hold::Event> for Event {
    fn from(ev: key::tap_hold::Event) -> Self {
        Event::TapHold(ev)
    }
}

impl TryFrom<Event> for key::keyboard::Event {
    type Error = key::EventError;

    fn try_from(_ev: Event) -> Result<Self, Self::Error> {
        Err(key::EventError::UnmappableEvent)
    }
}

// impl TryFrom<Event> for key::caps_word::Event {
//     type Error = key::EventError;

//     fn try_from(ev: Event) -> Result<Self, Self::Error> {
//         match ev {
//             Event::CapsWord(ev) => Ok(ev),
//             _ => Err(key::EventError::UnmappableEvent),
//         }
//     }
// }

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

// impl TryFrom<Event> for key::tap_dance::Event {
//     type Error = key::EventError;

//     fn try_from(ev: Event) -> Result<Self, Self::Error> {
//         match ev {
//             Event::TapDance(ev) => Ok(ev),
//             _ => Err(key::EventError::UnmappableEvent),
//         }
//     }
// }

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
pub enum PendingKeyState {
    // /// Pending key state for [key::tap_dance::PendingKeyState].
    // TapDance(key::tap_dance::PendingKeyState),
    /// Pending key state for [key::tap_hold::PendingKeyState].
    TapHold(key::tap_hold::PendingKeyState),
    /// Pending key state for [key::chorded::PendingKeyState].
    Chorded(key::chorded::PendingKeyState),
}

// impl From<key::tap_dance::PendingKeyState> for PendingKeyState {
//     fn from(pks: key::tap_dance::PendingKeyState) -> Self {
//         PendingKeyState::TapDance(pks)
//     }
// }

impl From<key::tap_hold::PendingKeyState> for PendingKeyState {
    fn from(pks: key::tap_hold::PendingKeyState) -> Self {
        PendingKeyState::TapHold(pks)
    }
}

impl From<key::chorded::PendingKeyState> for PendingKeyState {
    fn from(pks: key::chorded::PendingKeyState) -> Self {
        PendingKeyState::Chorded(pks)
    }
}

// impl<'pks> TryFrom<&'pks mut PendingKeyState> for &'pks mut key::tap_dance::PendingKeyState {
//     type Error = ();

//     fn try_from(pks: &'pks mut PendingKeyState) -> Result<Self, Self::Error> {
//         match pks {
//             PendingKeyState::TapDance(pks) => Ok(pks),
//             _ => Err(()),
//         }
//     }
// }

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
    /// Key state for [key::keyboard::KeyState].
    Keyboard(key::keyboard::KeyState),
    /// Key state for [key::layered::ModifierKeyState].
    LayerModifier(key::layered::ModifierKeyState),
    /// Key state for [key::sticky::KeyState].
    Sticky(key::sticky::KeyState),
    // /// Key state for [key::custom::KeyState].
    // Custom(key::custom::KeyState),
}

impl From<key::NoOpKeyState> for KeyState {
    fn from(_: key::NoOpKeyState) -> Self {
        KeyState::NoOp
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

impl From<key::sticky::KeyState> for KeyState {
    fn from(ks: key::sticky::KeyState) -> Self {
        KeyState::Sticky(ks)
    }
}

// impl From<key::custom::KeyState> for KeyState {
//     fn from(ks: key::custom::KeyState) -> Self {
//         KeyState::Custom(ks)
//     }
// }

// impl key::KeyState for KeyState {
//     type Context = Context;
//     type Event = Event;

//     fn handle_event(
//         &mut self,
//         context: &Self::Context,
//         keymap_index: u16,
//         event: key::Event<Self::Event>,
//     ) -> key::KeyEvents<Self::Event> {
//         match self {
//             KeyState::Keyboard(_) => key::KeyEvents::no_events(),
//             // KeyState::LayerModifier(ks) => {
//             //     if let Ok(ev) = event.try_into_key_event(|e| e.try_into()) {
//             //         let l_ev = ks.handle_event(keymap_index, ev);
//             //         if let Some(l_ev) = l_ev {
//             //             let c_ev = Event::LayerModification(l_ev);
//             //             key::KeyEvents::event(key::Event::key_event(keymap_index, c_ev))
//             //         } else {
//             //             key::KeyEvents::no_events()
//             //         }
//             //     } else {
//             //         key::KeyEvents::no_events()
//             //     }
//             // }
//             // KeyState::Sticky(ks) => {
//             //     if let Ok(ev) = event.try_into_key_event(|e| e.try_into()) {
//             //         let ctx = context.into();
//             //         let ke = ks.handle_event(ctx, keymap_index, ev);
//             //         ke.into_events()
//             //     } else {
//             //         key::KeyEvents::no_events()
//             //     }
//             // }
//             KeyState::NoOp => key::KeyEvents::no_events(),
//             // KeyState::Custom(_) => key::KeyEvents::no_events(),
//         }
//     }

//     fn key_output(&self) -> Option<key::KeyOutput> {
//         match self {
//             KeyState::Keyboard(ks) => todo!(),
//             // KeyState::LayerModifier(_) => None,
//             // KeyState::Sticky(ks) => ks.key_output(),
//             KeyState::NoOp => None,
//             // KeyState::Custom(ks) => Some(ks.key_output()),
//         }
//     }
// }

/// Convenience trait for the data storage types.
pub trait Keys {
    /// Type used by [key::keyboard::System].
    type Keyboard: Debug + Index<usize, Output = key::keyboard::Key>;
    /// Type used by [key::sticky::System].
    type Sticky: Debug + Index<usize, Output = key::sticky::Key>;
    /// Type used by [key::tap_hold::System].
    type TapHold: Debug + Index<usize, Output = key::tap_hold::Key<Ref>>;
    /// Type used by [key::layered::System].
    type LayerModifiers: Debug + Index<usize, Output = key::layered::ModifierKey>;
    /// Type used by [key::layered::System].
    type Layered: Debug + Index<usize, Output = key::layered::LayeredKey<Ref>>;
    /// Type used by [key::chorded::System].
    type Chorded: Debug + Index<usize, Output = key::chorded::Key<Ref>>;
    /// Type used by [key::chorded::System].
    type ChordedAuxiliary: Debug + Index<usize, Output = key::chorded::AuxiliaryKey<Ref>>;
}

/// Array-based data implementations.
#[derive(Debug)]
pub struct KeyArrays<
    const KEYBOARD: usize,
    const STICKY: usize,
    const TAP_HOLD: usize,
    const LAYER_MODIFIERS: usize,
    const LAYERED: usize,
    const CHORDED: usize,
    const CHORDED_AUXILIARY: usize,
>;

impl<
        const KEYBOARD: usize,
        const STICKY: usize,
        const TAP_HOLD: usize,
        const LAYER_MODIFIERS: usize,
        const LAYERED: usize,
        const CHORDED: usize,
        const CHORDED_AUXILIARY: usize,
    > Keys
    for KeyArrays<KEYBOARD, STICKY, TAP_HOLD, LAYER_MODIFIERS, LAYERED, CHORDED, CHORDED_AUXILIARY>
{
    type Keyboard = [key::keyboard::Key; KEYBOARD];
    type Sticky = [key::sticky::Key; STICKY];
    type TapHold = [key::tap_hold::Key<Ref>; TAP_HOLD];
    type LayerModifiers = [key::layered::ModifierKey; LAYER_MODIFIERS];
    type Layered = [key::layered::LayeredKey<Ref>; LAYERED];
    type Chorded = [key::chorded::Key<Ref>; CHORDED];
    type ChordedAuxiliary = [key::chorded::AuxiliaryKey<Ref>; CHORDED_AUXILIARY];
}

/// Vec-based data implementations.
#[derive(Debug)]
#[cfg(feature = "std")]
pub struct KeyVecs;

#[cfg(feature = "std")]
impl Keys for KeyVecs {
    type Keyboard = Vec<key::keyboard::Key>;
    type Sticky = Vec<key::sticky::Key>;
    type TapHold = Vec<key::tap_hold::Key<Ref>>;
    type LayerModifiers = Vec<key::layered::ModifierKey>;
    type Layered = Vec<key::layered::LayeredKey<Ref>>;
    type Chorded = Vec<key::chorded::Key<Ref>>;
    type ChordedAuxiliary = Vec<key::chorded::AuxiliaryKey<Ref>>;
}

/// Aggregate [key::System] implementation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct System<D: Keys> {
    keyboard: key::keyboard::System<D::Keyboard>,
    sticky: key::sticky::System<D::Sticky>,
    tap_hold: key::tap_hold::System<Ref, D::TapHold>,
    layered: key::layered::System<Ref, D::LayerModifiers, D::Layered>,
    chorded: key::chorded::System<Ref, D::Chorded, D::ChordedAuxiliary>,
    marker: PhantomData<D>,
}

impl<
        const KEYBOARD: usize,
        const STICKY: usize,
        const TAP_HOLD: usize,
        const LAYER_MODIFIERS: usize,
        const LAYERED: usize,
        const CHORDED: usize,
        const CHORDED_AUXILIARY: usize,
    >
    System<
        KeyArrays<KEYBOARD, STICKY, TAP_HOLD, LAYER_MODIFIERS, LAYERED, CHORDED, CHORDED_AUXILIARY>,
    >
{
    /// Constructs a new [System].
    pub const fn array_based(
        keyboard: key::keyboard::System<[key::keyboard::Key; KEYBOARD]>,
        sticky: key::sticky::System<[key::sticky::Key; STICKY]>,
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
            keyboard,
            sticky,
            tap_hold,
            layered,
            chorded,
            marker: PhantomData,
        }
    }
}

#[cfg(feature = "std")]
impl System<KeyVecs> {
    /// Constructs a new [System].
    pub const fn vec_based(
        keyboard: key::keyboard::System<<KeyVecs as Keys>::Keyboard>,
        sticky: key::sticky::System<<KeyVecs as Keys>::Sticky>,
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
            keyboard,
            sticky,
            tap_hold,
            layered,
            chorded,
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
            Ref::Keyboard(key_ref) => {
                let (pkr, pke) =
                    self.keyboard
                        .new_pressed_key(keymap_index, &key::keyboard::Context, key_ref);
                (
                    pkr.map(|_| panic!(), KeyState::Keyboard),
                    pke.map_events(|_| panic!()),
                )
            }
            Ref::Sticky(key_ref) => {
                let (pkr, pke) =
                    self.sticky
                        .new_pressed_key(keymap_index, &context.sticky, key_ref);
                (
                    pkr.map(|_| panic!(), KeyState::Sticky),
                    pke.map_events(Into::into),
                )
            }
            Ref::TapHold(key_ref) => {
                let (pkr, pke) =
                    self.tap_hold
                        .new_pressed_key(keymap_index, &context.tap_hold, key_ref);
                (
                    pkr.map(Into::into, |_| panic!()),
                    pke.map_events(Into::into),
                )
            }
            Ref::Layered(key_ref) => {
                let (pkr, pke) =
                    self.layered
                        .new_pressed_key(keymap_index, &context.layered, key_ref);
                (
                    pkr.map(|_| panic!(), KeyState::LayerModifier),
                    pke.map_events(Into::into),
                )
            }
            Ref::Chorded(key_ref) => {
                let (pkr, pke) =
                    self.chorded
                        .new_pressed_key(keymap_index, &context.chorded, key_ref);
                (
                    pkr.map(Into::into, |_| panic!()),
                    pke.map_events(Into::into),
                )
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
            (Ref::TapHold(key_ref), PendingKeyState::TapHold(pending_state)) => {
                if let Ok(event) = event.try_into_key_event(TryInto::try_into) {
                    let (maybe_npk, pke) = self.tap_hold.update_pending_state(
                        pending_state,
                        keymap_index,
                        &context.tap_hold,
                        key_ref,
                        event,
                    );
                    (maybe_npk, pke.map_events(Into::into))
                } else {
                    (None, key::KeyEvents::no_events())
                }
            }
            (Ref::Chorded(key_ref), PendingKeyState::Chorded(pending_state)) => {
                if let Ok(event) = event.try_into_key_event(TryInto::try_into) {
                    let (maybe_npk, pke) = self.chorded.update_pending_state(
                        pending_state,
                        keymap_index,
                        &context.chorded,
                        key_ref,
                        event,
                    );
                    (maybe_npk, pke.map_events(Into::into))
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
                if let Ok(event) = event.try_into_key_event(TryInto::try_into) {
                    let pke =
                        <key::keyboard::System<K::Keyboard> as key::System<Ref>>::update_state(
                            &self.keyboard,
                            key_state,
                            key_ref,
                            context.into(),
                            keymap_index,
                            event,
                        );
                    pke.map_events(|_| panic!())
                } else {
                    key::KeyEvents::no_events()
                }
            }
            (Ref::Sticky(key_ref), KeyState::Sticky(key_state)) => {
                if let Ok(event) = event.try_into_key_event(TryInto::try_into) {
                    let pke = <key::sticky::System<K::Sticky> as key::System<Ref>>::update_state(
                        &self.sticky,
                        key_state,
                        key_ref,
                        context.into(),
                        keymap_index,
                        event,
                    );
                    pke.map_events(Into::into)
                } else {
                    key::KeyEvents::no_events()
                }
            }
            (Ref::Layered(key_ref), KeyState::LayerModifier(key_state)) => {
                if let Ok(event) = event.try_into_key_event(TryInto::try_into) {
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
                    pke.map_events(Into::into)
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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_composite_pressedkey_layerpressedmodifier_handles_release_event() {
//         use crate::input;
//         use key::{composite, Key, KeyState};

//         // Assemble
//         type Ctx = composite::Context;
//         type K = composite::Key;
//         let keymap_index: u16 = 0;
//         let key_path = key::key_path(keymap_index);
//         let key = K::layer_modifier(key::layered::ModifierKey::Hold(1));
//         let context: Ctx = DEFAULT_CONTEXT;
//         let (pressed_lmod_key, _) = key.new_pressed_key(&context, key_path);

//         // Act
//         let events = pressed_lmod_key.unwrap_resolved().handle_event(
//             &context,
//             keymap_index,
//             key::Event::Input(input::Event::Release { keymap_index }),
//         );

//         // Assert
//         let _key_ev = match events.into_iter().next().map(|sch_ev| sch_ev.event) {
//             Some(key::Event::Key {
//                 key_event:
//                     Event::LayerModification(key::layered::LayerEvent::LayerDeactivated(layer_index)),
//                 ..
//             }) => {
//                 assert_eq!(1, layer_index);
//             }
//             _ => panic!("Expected an Event::Key(LayerModification(LayerDeactivated(layer)))"),
//         };
//     }

//     #[test]
//     fn test_composite_context_updates_with_composite_layermodifier_press_event() {
//         use key::{composite, Context, Key};

//         // Assemble
//         type Ctx = composite::Context;
//         type K = composite::Key;
//         let keys: [K; 2] = [
//             K::layer_modifier(key::layered::ModifierKey::Hold(1)),
//             K::layered(key::layered::LayeredKey::new(
//                 key::keyboard::Key::new(0x04).into(),
//                 [Some(key::keyboard::Key::new(0x05).into())],
//             )),
//         ];
//         let mut context: Ctx = DEFAULT_CONTEXT;
//         let keymap_index: u16 = 0;
//         let key_path = key::key_path(keymap_index);
//         let (_pressed_key, pressed_key_events) =
//             keys[keymap_index as usize].new_pressed_key(&context, key_path);
//         let maybe_ev = pressed_key_events.into_iter().next();

//         // Act
//         let event = match maybe_ev {
//             Some(key::ScheduledEvent { event, .. }) => event,
//             _ => panic!("Expected Some(ScheduledEvent(Event::Key(_)))"),
//         };
//         context.handle_event(event);
//         let actual_active_layers = context.layer_context.layer_state();

//         // Assert
//         let expected_active_layers = &[true];
//         assert_eq!(expected_active_layers[0..1], actual_active_layers[0..1]);
//     }

//     #[test]
//     fn test_composite_context_updates_with_composite_layerpressedmodifier_release_event() {
//         use crate::input;
//         use key::{composite, Context, Key, KeyState};

//         // Assemble
//         type Ctx = composite::Context;
//         type K = composite::Key;
//         let keys: [K; 2] = [
//             K::layer_modifier(key::layered::ModifierKey::Hold(1)),
//             K::layered(key::layered::LayeredKey::new(
//                 key::keyboard::Key::new(0x04).into(),
//                 [Some(key::keyboard::Key::new(0x05).into())],
//             )),
//         ];
//         let mut context: Ctx = DEFAULT_CONTEXT;
//         let keymap_index: u16 = 0;
//         let key_path = key::key_path(keymap_index);
//         let (pressed_lmod_key, _) = keys[keymap_index as usize].new_pressed_key(&context, key_path);
//         context.layer_context.activate_layer(1);
//         let events = pressed_lmod_key.unwrap_resolved().handle_event(
//             &context,
//             0,
//             key::Event::Input(input::Event::Release { keymap_index: 0 }),
//         );
//         let key_ev = match events.into_iter().next().map(|sch_ev| sch_ev.event) {
//             Some(key_event) => key_event,
//             _ => panic!("Expected an Event::Key(_)"),
//         };

//         // Act
//         context.handle_event(key_ev);
//         let actual_active_layers = context.layer_context.layer_state();

//         // Assert
//         let expected_active_layers = &[false];
//         assert_eq!(expected_active_layers[0..1], actual_active_layers[0..1]);
//     }

//     #[test]
//     fn test_composite_keyboard_pressed_key_has_key_code_for_composite_keyboard_key_def() {
//         use key::{composite, Key, KeyState};

//         // Assemble
//         type Ctx = composite::Context;
//         type K = composite::Key;
//         let keys: [K; 3] = [
//             K::layer_modifier(key::layered::ModifierKey::Hold(1)),
//             K::layered(key::layered::LayeredKey::new(
//                 key::keyboard::Key::new(0x04).into(),
//                 [Some(key::keyboard::Key::new(0x05).into())],
//             )),
//             K::keyboard(key::keyboard::Key::new(0x06)),
//         ];
//         let context: Ctx = DEFAULT_CONTEXT;

//         // Act
//         let keymap_index: u16 = 2;
//         let key_path = key::key_path(keymap_index);
//         let (pressed_key, _) = keys[keymap_index as usize].new_pressed_key(&context, key_path);
//         let actual_keycode = pressed_key.unwrap_resolved().key_output();

//         // Assert
//         let expected_keycode = Some(key::KeyOutput::from_key_code(0x06));
//         assert_eq!(expected_keycode, actual_keycode);
//     }

//     #[test]
//     fn test_composite_keyboard_pressed_key_has_key_code_for_composite_layered_key_def() {
//         use key::{composite, Key, KeyState};

//         // Assemble
//         type Ctx = composite::Context;
//         type K = composite::Key;
//         let keys: [K; 3] = [
//             K::layer_modifier(key::layered::ModifierKey::Hold(1)),
//             K::layered(key::layered::LayeredKey::new(
//                 key::keyboard::Key::new(0x04).into(),
//                 [Some(key::keyboard::Key::new(0x05).into())],
//             )),
//             K::keyboard(key::keyboard::Key::new(0x06)),
//         ];
//         let context: Ctx = DEFAULT_CONTEXT;

//         // Act
//         let keymap_index: u16 = 1;
//         let key_path = key::key_path(keymap_index);
//         let (pressed_key, _) = keys[keymap_index as usize].new_pressed_key(&context, key_path);
//         let actual_keycode = pressed_key.unwrap_resolved().key_output();

//         // Assert
//         let expected_keycode = Some(key::KeyOutput::from_key_code(0x04));
//         assert_eq!(expected_keycode, actual_keycode);
//     }
// }
