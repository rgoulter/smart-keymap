use crate::input;
use crate::key_definitions;

#[derive(Debug, Clone, Copy)]
pub struct KeyDefinition {
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

impl From<Event> for key_definitions::Event<Event> {
    fn from(event: Event) -> Self {
        key_definitions::Event::Key(event)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PressedKey {
    keymap_index: u16,
    state: TapHoldState,
}

impl PressedKey {
    pub fn new(keymap_index: u16) -> (Self, key_definitions::ScheduledEvent<Event>) {
        (
            Self {
                keymap_index,
                state: TapHoldState::Pending,
            },
            key_definitions::ScheduledEvent::after(
                200,
                Event::TapHoldTimeout { keymap_index }.into(),
            ),
        )
    }

    pub fn keymap_index(&self) -> u16 {
        self.keymap_index
    }

    pub fn key_code(&self, key_def: &KeyDefinition) -> Option<u8> {
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
        key_def: &KeyDefinition,
        event: key_definitions::Event<Event>,
    ) -> heapless::Vec<key_definitions::Event<Event>, 2> {
        match event {
            key_definitions::Event::Input(input::Event::Press { .. }) => {
                // TapHold: any interruption resolves pending TapHold as Hold.
                self.resolve(TapHoldState::Hold);
                heapless::Vec::new()
            }
            key_definitions::Event::Input(input::Event::Release { keymap_index }) => {
                if keymap_index == self.keymap_index {
                    // TapHold: resolved as tap.
                    self.resolve(TapHoldState::Tap);
                }

                match &self.state {
                    TapHoldState::Tap => {
                        let key_code = key_def.tap;
                        let mut emitted_events: heapless::Vec<key_definitions::Event<Event>, 2> =
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
            key_definitions::Event::Key(Event::TapHoldTimeout { .. }) => {
                // Key held long enough to resolve as hold.
                self.resolve(TapHoldState::Hold);
                heapless::Vec::new()
            }
            _ => heapless::Vec::new(),
        }
    }
}
