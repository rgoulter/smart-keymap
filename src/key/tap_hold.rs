#![doc = include_str!("doc_de_tap_hold.md")]

use serde::Deserialize;

use crate::input;
use crate::key;

/// A key with tap-hold functionality.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key {
    /// The 'tap' key.
    pub tap: u8,
    /// The 'hold' key.
    pub hold: u8,
}

impl From<Event> for () {
    fn from(_: Event) -> Self {}
}

impl key::Key for Key {
    type Context = ();
    type ContextEvent = ();
    type Event = Event;
    type PressedKeyState = PressedKeyState;

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
                pressed_key_state: PressedKeyState {
                    state: TapHoldState::Pending,
                },
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
pub enum TapHoldState {
    /// Not yet resolved as tap or hold.
    Pending,
    /// Resolved as tap.
    Tap,
    /// Resolved as hold.
    Hold,
}

/// Events emitted by a tap-hold key.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Event {
    /// Event indicating the key has been held long enough to resolve as hold.
    TapHoldTimeout {
        /// The keymap index of the key the timeout is for.
        keymap_index: u16,
    },
}

/// The state of a pressed tap-hold key.
#[derive(Debug, Clone, Copy)]
pub struct PressedKeyState {
    state: TapHoldState,
}

/// Convenience type for a pressed tap-hold key.
pub type PressedKey = input::PressedKey<Key, PressedKeyState>;

impl PressedKeyState {
    /// Resolves the state of the key, unless it has already been resolved.
    fn resolve(&mut self, state: TapHoldState) {
        if let TapHoldState::Pending = self.state {
            self.state = state;
        }
    }
}

impl key::PressedKeyState<Key> for PressedKeyState {
    type Event = Event;

    /// Returns at most 2 events
    fn handle_event_for(
        &mut self,
        keymap_index: u16,
        key: &Key,
        event: key::Event<Event>,
    ) -> key::PressedKeyEvents<Self::Event> {
        match event {
            key::Event::Input(input::Event::Press { .. }) => {
                // TapHold: any interruption resolves pending TapHold as Hold.
                self.resolve(TapHoldState::Hold);
                key::PressedKeyEvents::no_events()
            }
            key::Event::Input(input::Event::Release { keymap_index: ki }) => {
                if keymap_index == ki {
                    // TapHold: resolved as tap.
                    self.resolve(TapHoldState::Tap);
                }

                match &self.state {
                    TapHoldState::Tap => {
                        let key_code = key.tap;
                        let press_ev = input::Event::VirtualKeyPress { key_code };
                        let release_ev = input::Event::VirtualKeyRelease { key_code };
                        let mut events: key::PressedKeyEvents<Self::Event> =
                            key::PressedKeyEvents::event(press_ev.into());
                        events.schedule_event(10, release_ev.into());
                        events
                    }
                    _ => key::PressedKeyEvents::no_events(),
                }
            }
            key::Event::Key {
                key_event: Event::TapHoldTimeout { .. },
                ..
            } => {
                // Key held long enough to resolve as hold.
                self.resolve(TapHoldState::Hold);
                key::PressedKeyEvents::no_events()
            }
            _ => key::PressedKeyEvents::no_events(),
        }
    }

    fn key_output(&self, key: &Key) -> Option<key::KeyOutput> {
        match self.state {
            TapHoldState::Tap => Some(key::KeyOutput::from_key_code(key.tap)),
            TapHoldState::Hold => Some(key::KeyOutput::from_key_code(key.hold)),
            _ => None,
        }
    }
}
