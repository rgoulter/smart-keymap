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

impl<K: key::Key> Key<K> {
    /// Constructs a new tap-hold key.
    pub const fn new(tap: K, hold: K) -> Key<K> {
        Key { tap, hold }
    }
}

impl<K: key::Key> key::Key for Key<K> {
    type Context = key::ModifierKeyContext<(), K::Context>;
    type ContextEvent = ();
    type Event = key::ModifierKeyEvent<Event, K::Event>;
    type PressedKeyState = PressedKeyState<K>;

    fn new_pressed_key(
        &self,
        _context: Self::Context,
        keymap_index: u16,
    ) -> (
        input::PressedKey<Self, Self::PressedKeyState>,
        key::PressedKeyEvents<Self::Event>,
    ) {
        let timeout_ev = Event::TapHoldTimeout;
        (
            input::PressedKey {
                keymap_index,
                key: *self,
                pressed_key_state: PressedKeyState::new(),
            },
            key::PressedKeyEvents::scheduled_event(
                200,
                key::Event::key_event(keymap_index, key::ModifierKeyEvent::Modifier(timeout_ev)),
            ),
        )
    }
}

/// The state of a tap-hold key.
#[derive(Debug, Clone, Copy)]
pub enum TapHoldState {
    /// Not yet resolved as tap or hold.
    Pending,
    /// Resolved as tap.
    Tap,
    /// Resolved as hold.
    Hold,
}

/// Events emitted by a tap-hold key.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// Event indicating the key has been held long enough to resolve as hold.
    TapHoldTimeout,
}

/// The state of a pressed tap-hold key.
#[derive(Debug, Clone, Copy)]
pub struct PressedKeyState<K: key::Key> {
    state: TapHoldState,
    pressed_key: Option<input::PressedKey<K, K::PressedKeyState>>,
}

/// Convenience type for a pressed tap-hold key.
pub type PressedKey<K> = input::PressedKey<Key<K>, PressedKeyState<K>>;

impl<K: key::Key> PressedKeyState<K> {
    /// Constructs the initial pressed key state
    fn new() -> PressedKeyState<K> {
        PressedKeyState {
            state: TapHoldState::Pending,
            pressed_key: None,
        }
    }
    /// Resolves the state of the key, unless it has already been resolved.
    fn resolve(&mut self, state: TapHoldState) {
        if let TapHoldState::Pending = self.state {
            self.state = state;
        }
    }
}

impl<K: key::Key> key::PressedKeyState<Key<K>> for PressedKeyState<K> {
    type Event = key::ModifierKeyEvent<Event, K::Event>;

    /// Returns at most 2 events
    fn handle_event_for(
        &mut self,
        key::ModifierKeyContext {
            context: _,
            inner_context,
        }: <Key<K> as key::Key>::Context,
        keymap_index: u16,
        key: &Key<K>,
        event: key::Event<Self::Event>,
    ) -> key::PressedKeyEvents<Self::Event> {
        let mut pke = key::PressedKeyEvents::no_events();

        // Add events from inner pk handle_event
        if let Ok(inner_ev) = event.try_into_key_event(|mke| mke.try_into_inner()) {
            if let Some(pk) = &mut self.pressed_key {
                let pk_ev = pk
                    .pressed_key_state
                    .handle_event_for(inner_context, keymap_index, &pk.key, inner_ev)
                    .map_events(|ev| key::ModifierKeyEvent::Inner(ev));
                pke.extend(pk_ev);
            }
        }

        match event {
            key::Event::Input(input::Event::Press { .. }) => {
                // TapHold: any interruption resolves pending TapHold as Hold.
                let (hold_pk, hold_pke) = key.hold.new_pressed_key(inner_context, keymap_index);
                self.resolve(TapHoldState::Hold);
                self.pressed_key = Some(hold_pk);
                pke.extend(hold_pke.map_events(|ev| key::ModifierKeyEvent::Inner(ev)));
                pke
            }
            key::Event::Input(input::Event::Release { keymap_index: ki }) => {
                if keymap_index == ki {
                    // TapHold: resolved as tap.
                    let (tap_pk, tap_pke) = key.tap.new_pressed_key(inner_context, keymap_index);
                    self.resolve(TapHoldState::Tap);
                    self.pressed_key = Some(tap_pk);
                    pke.extend(tap_pke.map_events(|ev| key::ModifierKeyEvent::Inner(ev)));
                }

                match (self.state, &self.pressed_key) {
                    (TapHoldState::Tap, Some(pk)) => {
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
                key_event: key::ModifierKeyEvent::Modifier(Event::TapHoldTimeout),
                ..
            } => {
                // Key held long enough to resolve as hold.
                let (hold_pk, hold_pke) = key.hold.new_pressed_key(inner_context, keymap_index);
                self.resolve(TapHoldState::Hold);
                self.pressed_key = Some(hold_pk);
                pke.extend(hold_pke.map_events(|ev| key::ModifierKeyEvent::Inner(ev)));
                pke
            }
            _ => pke,
        }
    }

    fn key_output(&self, _key: &Key<K>) -> Option<key::KeyOutput> {
        self.pressed_key.as_ref().and_then(|pk| pk.key_output())
    }
}
