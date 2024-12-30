use serde::Deserialize;

use crate::input;
use crate::key;

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key {
    pub tap: u8,
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

#[derive(Debug, Clone, Copy)]
pub enum TapHoldState {
    Pending,
    Tap,
    Hold,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Event {
    TapHoldTimeout { keymap_index: u16 },
}

impl From<Event> for key::Event<Event> {
    fn from(event: Event) -> Self {
        key::Event::Key(event)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PressedKeyState {
    state: TapHoldState,
}

pub type PressedKey = input::PressedKey<Key, PressedKeyState>;

impl PressedKeyState {
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
