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
        _context: &Self::Context,
        keymap_index: u16,
    ) -> (
        input::PressedKey<Self, Self::PressedKeyState>,
        Option<key::ScheduledEvent<Self::Event>>,
    ) {
        (
            input::PressedKey {
                keymap_index,
                key: *self,
                pressed_key_state: PressedKeyState {
                    state: TapHoldState::Pending,
                },
            },
            Some(key::ScheduledEvent::after(
                200,
                Event::TapHoldTimeout { keymap_index }.into(),
            )),
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

impl From<Event> for key::Event<Event> {
    fn from(event: Event) -> Self {
        key::Event::Key(event)
    }
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
    ) -> impl IntoIterator<Item = key::Event<Self::Event>> {
        match event {
            key::Event::Input(input::Event::Press { .. }) => {
                // TapHold: any interruption resolves pending TapHold as Hold.
                self.resolve(TapHoldState::Hold);
                heapless::Vec::new()
            }
            key::Event::Input(input::Event::Release { keymap_index: ki }) => {
                if keymap_index == ki {
                    // TapHold: resolved as tap.
                    self.resolve(TapHoldState::Tap);
                }

                match &self.state {
                    TapHoldState::Tap => {
                        let key_code = key.tap;
                        let mut emitted_events: heapless::Vec<key::Event<Event>, 2> =
                            heapless::Vec::new();
                        emitted_events
                            .push(input::Event::VirtualKeyPress { key_code }.into())
                            .unwrap();
                        emitted_events
                            .push(input::Event::VirtualKeyRelease { key_code }.into())
                            .unwrap();
                        emitted_events
                    }
                    _ => heapless::Vec::new(),
                }
            }
            key::Event::Key(Event::TapHoldTimeout { .. }) => {
                // Key held long enough to resolve as hold.
                self.resolve(TapHoldState::Hold);
                heapless::Vec::new()
            }
            _ => heapless::Vec::new(),
        }
    }

    fn key_code(&self, key: &Key) -> Option<u8> {
        match self.state {
            TapHoldState::Tap => Some(key.tap),
            TapHoldState::Hold => Some(key.hold),
            _ => None,
        }
    }
}
