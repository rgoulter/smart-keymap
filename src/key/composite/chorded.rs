use core::fmt::Debug;

#[cfg(feature = "std")]
use serde::Deserialize;

use crate::key;

use super::BaseKey;
use super::TapHoldKey;
use super::{Context, Event, KeyState, PendingKeyState, PressedKeyResult};
use super::{Layered, LayeredKey, LayeredNestable};

/// Trait for types which can be nested in [ChordedKey] variants.
pub trait ChordedNestable:
    key::Key<
        Context = Context,
        Event = Event,
        KeyState = KeyState,
        PendingKeyState = PendingKeyState,
    > + Copy
    + PartialEq
{
}

impl<K: LayeredNestable> ChordedNestable for Layered<K> {}
impl<K: LayeredNestable> ChordedNestable for LayeredKey<K> {}

/// An aggregate of [key::Key] types.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "std", derive(Deserialize))]
#[cfg_attr(feature = "std", serde(untagged))]
pub enum ChordedKey<K: ChordedNestable> {
    /// A chorded key.
    Chorded(key::chorded::Key<K>),
    /// A chorded key.
    Auxiliary(key::chorded::AuxiliaryKey<K>),
    /// Non-chorded,
    Pass(K),
}

/// Newtype for [ChordedNestable] keys so they can implement [key::Key].
#[derive(Debug, Clone, Copy)]
pub struct Chorded<K: ChordedNestable>(pub K);

impl<
        K: key::Key<
            Context = crate::init::Context,
            Event = crate::init::Event,
            PendingKeyState = crate::init::PendingKeyState,
            KeyState = crate::init::KeyState,
        >,
    > key::Key for key::chorded::Key<K>
{
    type Context = Context;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        key_path: key::KeyPath,
    ) -> (PressedKeyResult, key::KeyEvents<Self::Event>) {
        self.new_pressed_key(context, key_path)
    }

    fn handle_event(
        &self,
        pending_state: &mut Self::PendingKeyState,
        context: Self::Context,
        key_path: key::KeyPath,
        event: key::Event<Self::Event>,
    ) -> (Option<Self::KeyState>, key::KeyEvents<Self::Event>) {
        let keymap_index: u16 = key_path[0];
        match pending_state {
            PendingKeyState::Chorded(ch_pks) => {
                if let Ok(ch_ev) = event.try_into_key_event(|e| e.try_into()) {
                    let ch_state = ch_pks.handle_event(context.into(), keymap_index, ch_ev);
                    if let Some(ch_state) = ch_state {
                        let (i, nk) = match ch_state {
                            key::chorded::ChordResolution::Chord => (1, &self.chord),
                            key::chorded::ChordResolution::Passthrough => (0, &self.passthrough),
                        };
                        let (pkr, mut pke) = nk.new_pressed_key(context, key_path);
                        // PRESSED KEY PATH: add Chord (0 = passthrough, 1 = chord)
                        let pkr = pkr.add_path_item(i);

                        let ks = match pkr {
                            // "Pending key resolves into pending key" to be implemented later.
                            key::PressedKeyResult::Pending(_, _) => todo!(),
                            key::PressedKeyResult::Resolved(ks) => ks,
                        };

                        let ch_r_ev = key::chorded::Event::ChordResolved(ch_state);
                        let sch_ev = key::ScheduledEvent::immediate(key::Event::key_event(
                            keymap_index,
                            ch_r_ev.into(),
                        ));
                        pke.add_event(sch_ev);

                        (Some(ks), pke)
                    } else {
                        (None, key::KeyEvents::no_events())
                    }
                } else {
                    (None, key::KeyEvents::no_events())
                }
            }
            _ => (None, key::KeyEvents::no_events()),
        }
    }

    fn lookup(
        &self,
        path: &[u16],
    ) -> &dyn key::Key<
        Context = Self::Context,
        Event = Self::Event,
        PendingKeyState = Self::PendingKeyState,
        KeyState = Self::KeyState,
    > {
        match path {
            [] => self,
            // 0 = passthrough, 1 = chord
            [0, path @ ..] => self.passthrough.lookup(path),
            [1, path @ ..] => self.chord.lookup(path),
            _ => panic!(),
        }
    }
}

impl<K: ChordedNestable> key::Key for key::chorded::AuxiliaryKey<K> {
    type Context = Context;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        key_path: key::KeyPath,
    ) -> (PressedKeyResult, key::KeyEvents<Self::Event>) {
        self.new_pressed_key(context, key_path)
    }

    fn handle_event(
        &self,
        pending_state: &mut Self::PendingKeyState,
        context: Self::Context,
        key_path: key::KeyPath,
        event: key::Event<Self::Event>,
    ) -> (Option<Self::KeyState>, key::KeyEvents<Self::Event>) {
        let keymap_index = key_path[0];
        match pending_state {
            PendingKeyState::Chorded(ch_pks) => {
                if let Ok(ch_ev) = event.try_into_key_event(|e| e.try_into()) {
                    let ch_state = ch_pks.handle_event(context.into(), keymap_index, ch_ev);
                    if let Some(key::chorded::ChordResolution::Passthrough) = ch_state {
                        let nk = &self.passthrough;
                        let (pkr, mut pke) = nk.new_pressed_key(context, key_path);

                        // n.b. no need to add to key path; chorded aux_key only nests the passthrough key.

                        let ks = match pkr {
                            // "Pending key resolves into pending key" to be implemented later.
                            key::PressedKeyResult::Pending(_, _) => todo!(),
                            key::PressedKeyResult::Resolved(ks) => ks,
                        };

                        let ch_r_ev = key::chorded::Event::ChordResolved(
                            key::chorded::ChordResolution::Passthrough,
                        );
                        let sch_ev = key::ScheduledEvent::immediate(key::Event::key_event(
                            keymap_index,
                            ch_r_ev.into(),
                        ));
                        pke.add_event(sch_ev);

                        (Some(ks), pke)
                    } else if let Some(key::chorded::ChordResolution::Chord) = ch_state {
                        let ch_r_ev = key::chorded::Event::ChordResolved(
                            key::chorded::ChordResolution::Chord,
                        );
                        let pke = key::KeyEvents::event(key::Event::key_event(
                            keymap_index,
                            ch_r_ev.into(),
                        ));

                        (Some(KeyState::NoOp), pke)
                    } else {
                        (None, key::KeyEvents::no_events())
                    }
                } else {
                    (None, key::KeyEvents::no_events())
                }
            }
            _ => (None, key::KeyEvents::no_events()),
        }
    }

    fn lookup(
        &self,
        path: &[u16],
    ) -> &dyn key::Key<
        Context = Self::Context,
        Event = Self::Event,
        PendingKeyState = Self::PendingKeyState,
        KeyState = Self::KeyState,
    > {
        match path {
            [] => self,
            _ => self.passthrough.lookup(path),
        }
    }
}

impl<K: ChordedNestable> key::Key for ChordedKey<K> {
    type Context = Context;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        key_path: key::KeyPath,
    ) -> (PressedKeyResult, key::KeyEvents<Self::Event>) {
        match self {
            ChordedKey::Chorded(key) => key.new_pressed_key(context, key_path),
            ChordedKey::Auxiliary(key) => key.new_pressed_key(context, key_path),
            ChordedKey::Pass(key) => key.new_pressed_key(context, key_path),
        }
    }

    fn handle_event(
        &self,
        pending_state: &mut Self::PendingKeyState,
        context: Self::Context,
        key_path: key::KeyPath,
        event: key::Event<Self::Event>,
    ) -> (Option<Self::KeyState>, key::KeyEvents<Self::Event>) {
        match self {
            ChordedKey::Chorded(key) => key.handle_event(pending_state, context, key_path, event),
            ChordedKey::Auxiliary(key) => key.handle_event(pending_state, context, key_path, event),
            ChordedKey::Pass(key) => key.handle_event(pending_state, context, key_path, event),
        }
    }

    fn lookup(
        &self,
        path: &[u16],
    ) -> &dyn key::Key<
        Context = Self::Context,
        Event = Self::Event,
        PendingKeyState = Self::PendingKeyState,
        KeyState = Self::KeyState,
    > {
        match self {
            ChordedKey::Chorded(key) => key.lookup(path),
            ChordedKey::Auxiliary(key) => key.lookup(path),
            ChordedKey::Pass(key) => key.lookup(path),
        }
    }
}

impl<K: ChordedNestable> key::Key for Chorded<K> {
    type Context = Context;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        key_path: key::KeyPath,
    ) -> (PressedKeyResult, key::KeyEvents<Self::Event>) {
        let Chorded(key) = self;
        key.new_pressed_key(context, key_path)
    }

    fn handle_event(
        &self,
        pending_state: &mut Self::PendingKeyState,
        context: Self::Context,
        key_path: key::KeyPath,
        event: key::Event<Self::Event>,
    ) -> (Option<Self::KeyState>, key::KeyEvents<Self::Event>) {
        let Chorded(key) = self;
        key.handle_event(pending_state, context, key_path, event)
    }

    fn lookup(
        &self,
        path: &[u16],
    ) -> &dyn key::Key<
        Context = Self::Context,
        Event = Self::Event,
        PendingKeyState = Self::PendingKeyState,
        KeyState = Self::KeyState,
    > {
        let Chorded(key) = self;
        key.lookup(path)
    }
}

impl ChordedKey<LayeredKey<TapHoldKey<BaseKey>>> {
    /// Constructs a [ChordedKey] from the given [key::keyboard::Key].
    pub const fn keyboard(key: key::keyboard::Key) -> Self {
        ChordedKey::Pass(LayeredKey::Pass(TapHoldKey::keyboard(key)))
    }

    /// Constructs a [ChordedKey] from the given [key::tap_hold::Key].
    pub const fn tap_hold(key: key::tap_hold::Key<BaseKey>) -> Self {
        ChordedKey::Pass(LayeredKey::Pass(TapHoldKey::tap_hold(key)))
    }

    /// Constructs a [ChordedKey] from the given [key::layered::ModifierKey].
    pub const fn layer_modifier(key: key::layered::ModifierKey) -> Self {
        ChordedKey::Pass(LayeredKey::Pass(TapHoldKey::layer_modifier(key)))
    }
}

impl<K: LayeredNestable> ChordedKey<LayeredKey<K>> {
    /// Constructs a [ChordedKey] from the given [key::layered::LayeredKey].
    pub const fn layered(key: key::layered::LayeredKey<K>) -> Self {
        ChordedKey::Pass(LayeredKey::Layered(key))
    }
}
