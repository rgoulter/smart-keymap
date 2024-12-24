use crate::input;
use crate::key;
use key::{simple, tap_hold};

trait PressedKey {
    type Event;
    fn handle_event(
        &mut self,
        key_definition: KeyDefinition,
        event: key::Event<Self::Event>,
    ) -> impl IntoIterator<Item = key::Event<Self::Event>>;
    fn key_code(&self, key_definition: KeyDefinition) -> Option<u8>;
}

#[derive(Debug, Clone, Copy)]
pub enum KeyDefinition {
    Simple(simple::KeyDefinition),
    TapHold(tap_hold::KeyDefinition),
}

impl KeyDefinition {
    fn new_pressed_key(
        keymap_index: u16,
        key_definition: KeyDefinition,
    ) -> (
        CompositePressedKey,
        Option<key::ScheduledEvent<CompositeEvent>>,
    ) {
        match key_definition {
            KeyDefinition::Simple(_) => {
                let pressed_key = simple::PressedKey::new();
                (pressed_key.into(), None)
            }
            KeyDefinition::TapHold(_) => {
                let (pressed_key, new_event) = tap_hold::PressedKey::new(keymap_index);
                (pressed_key.into(), Some(new_event.into()))
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CompositePressedKey {
    Simple(simple::PressedKey),
    TapHold(tap_hold::PressedKey),
}

impl From<simple::PressedKey> for CompositePressedKey {
    fn from(pk: simple::PressedKey) -> Self {
        CompositePressedKey::Simple(pk)
    }
}

impl From<tap_hold::PressedKey> for CompositePressedKey {
    fn from(pk: tap_hold::PressedKey) -> Self {
        CompositePressedKey::TapHold(pk)
    }
}

impl CompositePressedKey {
    pub fn as_pressed_input(self, keymap_index: u16) -> input::PressedInput<Self> {
        input::PressedInput::Key {
            keymap_index,
            key: self,
        }
    }
}

impl PressedKey for CompositePressedKey {
    type Event = CompositeEvent;

    fn key_code(&self, key_definition: KeyDefinition) -> Option<u8> {
        match self {
            CompositePressedKey::Simple(pk) => match key_definition {
                KeyDefinition::Simple(key_def) => Some(pk.key_code(&key_def)),
                _ => None,
            },

            CompositePressedKey::TapHold(pk) => match key_definition {
                KeyDefinition::TapHold(key_def) => pk.key_code(&key_def),
                _ => None,
            },
        }
    }

    fn handle_event(
        &mut self,
        key_definition: KeyDefinition,
        event: key::Event<Self::Event>,
    ) -> impl IntoIterator<Item = key::Event<Self::Event>> {
        if let CompositePressedKey::TapHold(tap_hold) = self {
            if let KeyDefinition::TapHold(key_def) = key_definition {
                if let Ok(ev) = key::Event::try_from(event) {
                    let events: heapless::Vec<key::Event<tap_hold::Event>, 2> =
                        tap_hold.handle_event(&key_def, ev);
                    events.into_iter().map(|ev| ev.into()).collect()
                } else {
                    heapless::Vec::<key::Event<Self::Event>, 2>::new()
                }
            } else {
                heapless::Vec::new()
            }
        } else {
            heapless::Vec::new()
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum CompositeEvent {
    TapHold(tap_hold::Event),
}

impl From<key::Event<tap_hold::Event>> for key::Event<CompositeEvent> {
    fn from(ev: key::Event<tap_hold::Event>) -> Self {
        match ev {
            key::Event::Input(ev) => key::Event::Input(ev),
            key::Event::Key(ev) => key::Event::Key(CompositeEvent::TapHold(ev)),
        }
    }
}

impl From<key::ScheduledEvent<tap_hold::Event>> for key::ScheduledEvent<CompositeEvent> {
    fn from(ev: key::ScheduledEvent<tap_hold::Event>) -> Self {
        Self {
            schedule: ev.schedule,
            event: ev.event.into(),
        }
    }
}

#[allow(unused)]
pub enum EventError {
    UnmappableEvent,
}

impl TryFrom<key::Event<CompositeEvent>> for key::Event<tap_hold::Event> {
    type Error = EventError;

    fn try_from(ev: key::Event<CompositeEvent>) -> Result<Self, Self::Error> {
        match ev {
            key::Event::Input(ev) => Ok(key::Event::Input(ev)),
            key::Event::Key(CompositeEvent::TapHold(ev)) => Ok(key::Event::Key(ev)),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct ScheduledEvent {
    time: u32,
    event: key::Event<CompositeEvent>,
}

/// The engine (set of key definition systems),
///  and key definitions.
pub struct Keymap<const N: usize> {
    key_definitions: [KeyDefinition; N],
    pressed_inputs: heapless::Vec<input::PressedInput<CompositePressedKey>, N>,
    pending_events: heapless::spsc::Queue<key::Event<CompositeEvent>, 256>,
    scheduled_events: heapless::BinaryHeap<ScheduledEvent, heapless::binary_heap::Min, 256>,
    schedule_counter: u32,
}

impl<const N: usize> Keymap<N> {
    pub const fn new(key_definitions: [KeyDefinition; N]) -> Self {
        Self {
            key_definitions,
            pressed_inputs: heapless::Vec::new(),
            pending_events: heapless::spsc::Queue::new(),
            scheduled_events: heapless::BinaryHeap::new(),
            schedule_counter: 0,
        }
    }

    pub fn init(&mut self) {
        self.pressed_inputs.clear();
        while self.pending_events.dequeue().is_some() {}
        self.scheduled_events.clear();
        self.schedule_counter = 0;
    }

    pub fn handle_input(&mut self, ev: input::Event) {
        // Update each of the PressedKeys with the event.
        self.pressed_inputs.iter_mut().for_each(|pi| {
            if let input::PressedInput::Key { keymap_index, key } = pi {
                let key_definition = self.key_definitions[*keymap_index as usize];
                let events = key.handle_event(key_definition, ev.into());
                events
                    .into_iter()
                    .for_each(|ev: key::Event<CompositeEvent>| {
                        self.pending_events.enqueue(ev).unwrap()
                    });
            }
        });

        match ev {
            input::Event::Press { keymap_index } => {
                let key_definition = self.key_definitions[keymap_index as usize];
                let (pressed_key, new_event) =
                    KeyDefinition::new_pressed_key(keymap_index, key_definition);
                self.pressed_inputs
                    .push(pressed_key.as_pressed_input(keymap_index))
                    .unwrap();

                if let Some(new_event) = new_event {
                    self.schedule_event(new_event);
                }
            }
            input::Event::Release { keymap_index } => {
                self.pressed_inputs
                    .iter()
                    .position(|&pi| match pi {
                        input::PressedInput::Key {
                            keymap_index: ki, ..
                        } => ki == keymap_index,
                        _ => false,
                    })
                    .map(|i| self.pressed_inputs.remove(i));
            }
            input::Event::VirtualKeyPress { key_code } => {
                // Add to pressed keys.
                let pressed_key = input::PressedInput::Virtual { key_code };
                self.pressed_inputs.push(pressed_key).unwrap();
            }
            input::Event::VirtualKeyRelease { key_code } => {
                // Remove from pressed keys.
                self.pressed_inputs
                    .iter()
                    .position(|&k| match k {
                        input::PressedInput::Virtual { key_code: kc } => key_code == kc,
                        _ => false,
                    })
                    .map(|i| self.pressed_inputs.remove(i));
            }
        }
    }

    fn schedule_event<T>(&mut self, scheduled_event: key::ScheduledEvent<T>)
    where
        key::Event<CompositeEvent>: From<key::Event<T>>,
    {
        match scheduled_event.schedule {
            key::Schedule::Immediate => {
                self.pending_events
                    .enqueue(scheduled_event.event.into())
                    .unwrap();
            }
            key::Schedule::After(delay) => {
                self.schedule_after(delay as u32, scheduled_event.event.into());
            }
        }
    }

    pub fn schedule_after(&mut self, delay: u32, event: key::Event<CompositeEvent>) {
        let time = self.schedule_counter + delay;
        self.scheduled_events
            .push(ScheduledEvent { time, event })
            .unwrap();
    }

    pub fn tick(&mut self) {
        self.schedule_counter += 1;
        let scheduled_ready =
            if let Some(ScheduledEvent { time, .. }) = self.scheduled_events.peek() {
                *time <= self.schedule_counter
            } else {
                false
            };
        if scheduled_ready {
            if let Some(ScheduledEvent { event, .. }) = self.scheduled_events.pop() {
                self.pending_events.enqueue(event).unwrap();
            }
        }

        // take from pending
        if let Some(ev) = self.pending_events.dequeue() {
            // Update each of the PressedKeys with the event.
            self.pressed_inputs.iter_mut().for_each(|pi| {
                if let input::PressedInput::Key { keymap_index, key } = pi {
                    let key_definition = self.key_definitions[*keymap_index as usize];
                    let events = key.handle_event(key_definition, ev);
                    events
                        .into_iter()
                        .for_each(|ev: key::Event<CompositeEvent>| {
                            self.pending_events.enqueue(ev).unwrap()
                        });
                }
            });

            if let key::Event::Input(input_ev) = ev {
                self.handle_input(input_ev);
            }
        }
    }

    pub fn boot_keyboard_report(&self) -> [u8; 8] {
        let mut report = [0u8; 8];

        let pressed_keys = self.pressed_inputs.iter().filter_map(|&pi| match pi {
            input::PressedInput::Key { keymap_index, key } => {
                let key_definition = self.key_definitions[keymap_index as usize];
                key.key_code(key_definition)
            }
            input::PressedInput::Virtual { key_code } => Some(key_code),
        });

        let (modifier_keys, key_codes): (heapless::Vec<u8, N>, heapless::Vec<u8, N>) =
            pressed_keys.partition(|&kc| (0xE0..=0xE7).contains(&kc));

        let modifier = modifier_keys
            .iter()
            .fold(0u8, |acc, &kc| acc | (1 << (kc - 0xE0)));
        report[0] = modifier;

        for (i, key_code) in key_codes.iter().take(6).enumerate() {
            report[i + 2] = *key_code;
        }
        report
    }
}
