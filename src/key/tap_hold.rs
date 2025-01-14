#![doc = include_str!("doc_de_tap_hold.md")]

use core::fmt::Debug;

use serde::Deserialize;

use crate::input;
use crate::key;

use super::PressedKey as _;

/// A key with tap-hold functionality.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key<K: key::Key> {
    /// The 'tap' key.
    pub tap: K,
    /// The 'hold' key.
    pub hold: K,
}

impl<K: key::Key> key::Key for Key<K> {
    type Context = key::ModifierKeyContext<(), K::Context>;
    type ContextEvent = ();
    type Event = Event<K::Event>;
    type PressedKeyState = PressedKeyState<K>;

    fn new_pressed_key(
        &self,
        _context: Self::Context,
        keymap_index: u16,
    ) -> (
        input::PressedKey<Self, Self::PressedKeyState>,
        key::PressedKeyEvents<Self::Event>,
    ) {
        (
            input::PressedKey {
                keymap_index,
                key: *self,
                pressed_key_state: PressedKeyState::new(),
            },
            key::PressedKeyEvents::scheduled_event(
                200,
                key::Event::key_event(keymap_index, Event::TapHoldTimeout { keymap_index }),
            ),
        )
    }
}

/// The state of a tap-hold key.
#[derive(Debug, Clone, Copy)]
pub enum TapHoldState<T> {
    /// Not yet resolved as tap or hold.
    Pending,
    /// Resolved as tap.
    Tap(T),
    /// Resolved as hold.
    Hold(T),
}

/// Events emitted by a tap-hold key.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event<KE: Copy + Debug + PartialEq> {
    /// Event indicating the key has been held long enough to resolve as hold.
    TapHoldTimeout {
        /// The keymap index of the key the timeout is for.
        keymap_index: u16,
    },
    /// Event for (or from) the tap or hold keys of the tap-hold key.
    Passthrough {
        /// The event for the nested key.
        event: KE,
    },
}

/// The state of a pressed tap-hold key.
#[allow(type_alias_bounds)]
pub type PressedKeyState<K: key::Key> = TapHoldState<input::PressedKey<K, K::PressedKeyState>>;

/// Convenience type for a pressed tap-hold key.
pub type PressedKey<K> = input::PressedKey<Key<K>, PressedKeyState<K>>;

impl<K: key::Key> PressedKeyState<K> {
    /// Constructs the initial pressed key state
    fn new() -> PressedKeyState<K> {
        TapHoldState::Pending
    }
    /// Resolves the state of the key, unless it has already been resolved.
    fn resolve(&mut self, state: PressedKeyState<K>) {
        if let TapHoldState::Pending = self {
            *self = state;
        }
    }
}

impl<K: key::Key> key::PressedKeyState<Key<K>> for PressedKeyState<K> {
    type Event = Event<K::Event>;

    /// Returns at most 2 events
    fn handle_event_for(
        &mut self,
        key::ModifierKeyContext {
            context: _,
            inner_context,
        }: <Key<K> as key::Key>::Context,
        keymap_index: u16,
        key: &Key<K>,
        event: key::Event<Event<K::Event>>,
    ) -> key::PressedKeyEvents<Self::Event> {
        match event {
            key::Event::Input(input::Event::Press { .. }) => {
                // TapHold: any interruption resolves pending TapHold as Hold.
                let (hold_pk, hold_pke) = key.hold.new_pressed_key(inner_context, keymap_index);
                self.resolve(TapHoldState::Hold(hold_pk));
                hold_pke.map_events(|sch_ev| {
                    sch_ev.map_key_event(|ev| Event::Passthrough { event: ev })
                })
            }
            key::Event::Input(input::Event::Release { keymap_index: ki }) => {
                let mut pke = key::PressedKeyEvents::no_events();
                if keymap_index == ki {
                    // TapHold: resolved as tap.
                    let (tap_pk, tap_pke) = key.tap.new_pressed_key(inner_context, keymap_index);
                    self.resolve(TapHoldState::Tap(tap_pk));
                    pke.extend(tap_pke.map_events(|sch_ev| {
                        sch_ev.map_key_event(|ev| Event::Passthrough { event: ev })
                    }));
                }

                match &self {
                    TapHoldState::Tap(pk) => {
                        if let Some(key_output) = pk.key_output() {
                            let key_code = key_output.key_code();
                            let press_ev = input::Event::VirtualKeyPress { key_code };
                            let release_ev = input::Event::VirtualKeyRelease { key_code };
                            let mut events: key::PressedKeyEvents<Self::Event> =
                                key::PressedKeyEvents::event(press_ev.into());
                            events.schedule_event(10, release_ev.into());
                            pke.extend(events);
                        }

                        pke
                    }
                    _ => pke,
                }
            }
            key::Event::Key {
                key_event: Event::TapHoldTimeout { .. },
                ..
            } => {
                // Key held long enough to resolve as hold.
                let (hold_pk, hold_pke) = key.hold.new_pressed_key(inner_context, keymap_index);
                self.resolve(TapHoldState::Hold(hold_pk));
                hold_pke.map_events(|sch_ev| {
                    sch_ev.map_key_event(|ev| Event::Passthrough { event: ev })
                })
            }
            _ => key::PressedKeyEvents::no_events(),
        }
    }

    fn key_output(&self, _key: &Key<K>) -> Option<key::KeyOutput> {
        match self {
            TapHoldState::Tap(pk) => pk.key_output(),
            TapHoldState::Hold(pk) => pk.key_output(),
            _ => None,
        }
    }
}
