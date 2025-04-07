use core::fmt::Debug;

#[cfg(feature = "std")]
use serde::Deserialize;

use crate::key;

use super::BaseKey;
use super::{Context, Event, KeyState, PendingKeyState, PressedKeyResult};

/// Trait for types which can be nested in [TapHoldKey] variants.
pub trait TapHoldNestable:
    key::Key<
        Context = Context,
        Event = Event,
        KeyState = KeyState,
        PendingKeyState = PendingKeyState,
    > + Copy
    + PartialEq
    + Into<BaseKey>
{
}

impl TapHoldNestable for key::layered::ModifierKey {}
impl TapHoldNestable for key::callback::Key {}
impl TapHoldNestable for key::caps_word::Key {}
impl TapHoldNestable for key::sticky::Key {}
impl TapHoldNestable for key::keyboard::Key {}
impl TapHoldNestable for BaseKey {}

/// An aggregate of [key::Key] types.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "std", derive(Deserialize))]
#[cfg_attr(feature = "std", serde(untagged))]
pub enum TapHoldKey<K: TapHoldNestable> {
    /// A tap-hold key.
    TapHold(key::tap_hold::Key<K>),
    /// A non-tap-hold key.
    Pass(K),
}

/// Newtype for [TapHoldNestable] keys so they can implement [key::Key].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TapHold<K: TapHoldNestable>(pub K);

impl<K: TapHoldNestable> key::Key for key::tap_hold::Key<K> {
    type Context = Context;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        key_path: key::KeyPath,
    ) -> (PressedKeyResult, key::KeyEvents<Self::Event>) {
        let (th_pks, sch_ev) = self.new_pressed_key(context.into(), key_path.clone());
        let pk = key::PressedKeyResult::Pending(key_path, PendingKeyState::TapHold(th_pks));
        let pke = key::KeyEvents::scheduled_event(sch_ev.into_scheduled_event());
        (pk, pke)
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
            PendingKeyState::TapHold(th_pks) => {
                if let Ok(th_ev) = event.try_into_key_event(|e| e.try_into()) {
                    let th_state = th_pks.handle_event(context.into(), keymap_index, th_ev);
                    if let Some(th_state) = th_state {
                        let (i, nk) = match th_state {
                            key::tap_hold::TapHoldState::Tap => (0, self.tap),
                            key::tap_hold::TapHoldState::Hold => (1, self.hold),
                        };
                        let (pkr, pke) = nk.new_pressed_key(context, key_path);
                        // PRESSED KEY PATH: add Tap Hold item (0 = tap, 1 = hold)
                        let pkr = pkr.add_path_item(i);

                        let ks = match pkr {
                            // "Pending key resolves into pending key" to be implemented later.
                            key::PressedKeyResult::Pending(_, _) => todo!(),
                            key::PressedKeyResult::Resolved(ks) => ks,
                        };

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
            // 0 = tap, 1 = hold
            [0, path @ ..] => self.tap.lookup(path),
            [1, path @ ..] => self.hold.lookup(path),
            _ => panic!(),
        }
    }
}

impl<K: TapHoldNestable> key::Key for TapHoldKey<K> {
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
            TapHoldKey::TapHold(key) => {
                <key::tap_hold::Key<K> as key::Key>::new_pressed_key(key, context, key_path)
            }
            TapHoldKey::Pass(key) => key.new_pressed_key(context, key_path),
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
            TapHoldKey::TapHold(key) => key.handle_event(pending_state, context, key_path, event),
            TapHoldKey::Pass(key) => key.handle_event(pending_state, context, key_path, event),
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
            TapHoldKey::TapHold(key) => key.lookup(path),
            TapHoldKey::Pass(key) => key.lookup(path),
        }
    }
}

impl<K: TapHoldNestable> key::Key for TapHold<K> {
    type Context = Context;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        key_path: key::KeyPath,
    ) -> (PressedKeyResult, key::KeyEvents<Self::Event>) {
        let TapHold(key) = self;
        key.new_pressed_key(context, key_path)
    }

    fn handle_event(
        &self,
        pending_state: &mut Self::PendingKeyState,
        context: Self::Context,
        key_path: key::KeyPath,
        event: key::Event<Self::Event>,
    ) -> (Option<Self::KeyState>, key::KeyEvents<Self::Event>) {
        let TapHold(key) = self;
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
        let TapHold(key) = self;
        key.lookup(path)
    }
}

impl<K: TapHoldNestable> From<key::tap_hold::Key<K>> for TapHoldKey<K> {
    fn from(key: key::tap_hold::Key<K>) -> Self {
        TapHoldKey::TapHold(key)
    }
}

impl<K: Into<BaseKey>> From<K> for TapHoldKey<BaseKey> {
    fn from(key: K) -> Self {
        TapHoldKey::Pass(key.into())
    }
}

impl TapHoldKey<BaseKey> {
    /// Constructs a [TapHoldKey] from the given [key::keyboard::Key].
    pub const fn keyboard(key: key::keyboard::Key) -> Self {
        Self::Pass(BaseKey::Keyboard(key))
    }

    /// Constructs a [TapHoldKey] from the given [key::tap_hold::Key].
    pub const fn tap_hold(key: key::tap_hold::Key<BaseKey>) -> Self {
        Self::TapHold(key)
    }

    /// Constructs a [TapHoldKey] from the given [key::layered::ModifierKey].
    pub const fn layer_modifier(key: key::layered::ModifierKey) -> Self {
        Self::Pass(BaseKey::LayerModifier(key))
    }
}

impl TapHold<key::keyboard::Key> {
    /// Constructs a [TapHold] newtype from the given key.
    pub const fn keyboard(key: key::keyboard::Key) -> Self {
        Self(key)
    }
}

impl TapHold<key::layered::ModifierKey> {
    /// Constructs a [TapHold] newtypefrom the given key.
    pub const fn layer_modifier(key: key::layered::ModifierKey) -> Self {
        Self(key)
    }
}

impl TapHold<BaseKey> {
    /// Constructs a [TapHold] newtype from the given key.
    pub const fn base_key(key: BaseKey) -> Self {
        Self(key)
    }
}
