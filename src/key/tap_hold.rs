use crate::input;
use crate::key;

#[derive(Debug, Clone, Copy)]
pub struct Key {
    pub tap: u8,
    pub hold: u8,
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
pub struct PressedKey {
    keymap_index: u16,
    state: TapHoldState,
}

impl PressedKey {
    pub fn new(keymap_index: u16) -> (Self, key::ScheduledEvent<Event>) {
        (
            Self {
                keymap_index,
                state: TapHoldState::Pending,
            },
            key::ScheduledEvent::after(200, Event::TapHoldTimeout { keymap_index }.into()),
        )
    }

    pub fn key_code(&self, key_def: &Key) -> Option<u8> {
        match self.state {
            TapHoldState::Tap => Some(key_def.tap),
            TapHoldState::Hold => Some(key_def.hold),
            _ => None,
        }
    }

    fn resolve(&mut self, state: TapHoldState) {
        if let TapHoldState::Pending = self.state {
            self.state = state;
        }
    }

    pub fn handle_event(
        &mut self,
        key_def: &Key,
        event: key::Event<Event>,
    ) -> heapless::Vec<key::Event<Event>, 2> {
        match event {
            key::Event::Input(input::Event::Press { .. }) => {
                // TapHold: any interruption resolves pending TapHold as Hold.
                self.resolve(TapHoldState::Hold);
                heapless::Vec::new()
            }
            key::Event::Input(input::Event::Release { keymap_index }) => {
                if keymap_index == self.keymap_index {
                    // TapHold: resolved as tap.
                    self.resolve(TapHoldState::Tap);
                }

                match &self.state {
                    TapHoldState::Tap => {
                        let key_code = key_def.tap;
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
}
