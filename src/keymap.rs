use core::fmt::Debug;

use crate::input;
use crate::key;

use key::{composite, Event, Key, PressedKey};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct ScheduledEvent<E> {
    time: u32,
    event: Event<E>,
}

/// The engine (set of key definition systems),
///  and key definitions.
pub struct Keymap<const N: usize, K: Key = composite::Key> {
    key_definitions: [K; N],
    context: K::Context,
    pressed_inputs: heapless::Vec<input::PressedInput<K::PressedKey>, 16>,
    pending_events: heapless::spsc::Queue<Event<K::Event>, 256>,
    scheduled_events:
        heapless::BinaryHeap<ScheduledEvent<K::Event>, heapless::binary_heap::Min, 256>,
    schedule_counter: u32,
}

impl<const N: usize, K: Key> Keymap<N, K> {
    pub const fn new(key_definitions: [K; N], context: K::Context) -> Self {
        Self {
            key_definitions,
            context,
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
                let key_definition = &self.key_definitions[*keymap_index as usize];
                let events = key.handle_event(key_definition, ev.into());
                events
                    .into_iter()
                    .for_each(|ev: Event<K::Event>| self.pending_events.enqueue(ev).unwrap());
            }
        });

        match ev {
            input::Event::Press { keymap_index } => {
                let key_definition = &self.key_definitions[keymap_index as usize];
                let (pressed_key, new_event) =
                    key_definition.new_pressed_key(&self.context, keymap_index);
                self.pressed_inputs
                    .push(input::PressedInput::new_pressed_key(
                        keymap_index,
                        pressed_key,
                    ))
                    .unwrap();

                if let Some(new_event) = new_event {
                    self.schedule_event(new_event);
                }
            }
            input::Event::Release { keymap_index } => {
                self.pressed_inputs
                    .iter()
                    .position(|pi| match pi {
                        input::PressedInput::Key {
                            keymap_index: ki, ..
                        } => keymap_index == *ki,
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
                    .position(|k| match k {
                        input::PressedInput::Virtual { key_code: kc } => key_code == *kc,
                        _ => false,
                    })
                    .map(|i| self.pressed_inputs.remove(i));
            }
        }
    }

    fn schedule_event(&mut self, scheduled_event: key::ScheduledEvent<K::Event>) {
        match scheduled_event.schedule {
            key::Schedule::Immediate => {
                self.pending_events.enqueue(scheduled_event.event).unwrap();
            }
            key::Schedule::After(delay) => {
                self.schedule_after(delay as u32, scheduled_event.event);
            }
        }
    }

    pub fn schedule_after(&mut self, delay: u32, event: Event<K::Event>) {
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
                    let key_definition = &self.key_definitions[*keymap_index as usize];
                    let events = key.handle_event(key_definition, ev);
                    events
                        .into_iter()
                        .for_each(|ev: Event<K::Event>| self.pending_events.enqueue(ev).unwrap());
                }
            });

            if let Event::Input(input_ev) = ev {
                self.handle_input(input_ev);
            }
        }
    }

    pub fn boot_keyboard_report(&self) -> [u8; 8] {
        let mut report = [0u8; 8];

        let pressed_keys = self.pressed_inputs.iter().filter_map(|pi| match pi {
            input::PressedInput::Key { keymap_index, key } => {
                let key_definition = &self.key_definitions[*keymap_index as usize];
                key.key_code(key_definition)
            }
            input::PressedInput::Virtual { key_code } => Some(*key_code),
        });

        let (modifier_keys, key_codes): (heapless::Vec<u8, 16>, heapless::Vec<u8, 16>) =
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
