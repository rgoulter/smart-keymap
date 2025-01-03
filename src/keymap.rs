use core::fmt::Debug;
use core::ops::IndexMut;

use crate::input;
use crate::key;

use key::{composite, Context, Event};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
struct ScheduledEvent<E> {
    time: u32,
    event: Event<E>,
}

#[derive(Debug)]
struct EventScheduler {
    pending_events: heapless::spsc::Queue<Event<composite::Event>, 256>,
    scheduled_events:
        heapless::BinaryHeap<ScheduledEvent<composite::Event>, heapless::binary_heap::Min, 256>,
    schedule_counter: u32,
}

impl EventScheduler {
    pub const fn new() -> Self {
        Self {
            pending_events: heapless::spsc::Queue::new(),
            scheduled_events: heapless::BinaryHeap::new(),
            schedule_counter: 0,
        }
    }

    pub fn init(&mut self) {
        while self.pending_events.dequeue().is_some() {}
        self.scheduled_events.clear();
        self.schedule_counter = 0;
    }

    pub fn schedule_event(&mut self, scheduled_event: key::ScheduledEvent<composite::Event>) {
        match scheduled_event.schedule {
            key::Schedule::Immediate => {
                self.pending_events.enqueue(scheduled_event.event).unwrap();
            }
            key::Schedule::After(delay) => {
                self.schedule_after(delay as u32, scheduled_event.event);
            }
        }
    }

    pub fn schedule_after(&mut self, delay: u32, event: Event<composite::Event>) {
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
    }

    pub fn dequeue(&mut self) -> Option<Event<composite::Event>> {
        self.pending_events.dequeue()
    }
}

/// State for a keymap that handles input, and outputs HID keyboard reports.
#[derive(Debug)]
pub struct Keymap<
    I: IndexMut<
            usize,
            Output = dyn key::dynamic::Key<
                key::composite::Event,
                Context = key::composite::Context<L, key::composite::DefaultNestableKey>,
            >,
        > + crate::tuples::KeysReset,
    const L: key::layered::LayerIndex = 0,
> {
    key_definitions: I,
    context: composite::Context<L, composite::DefaultNestableKey>,
    pressed_inputs: heapless::Vec<input::PressedInput, 16>,
    event_scheduler: EventScheduler,
}

impl<
        I: IndexMut<
                usize,
                Output = dyn key::dynamic::Key<
                    key::composite::Event,
                    Context = key::composite::Context<L, key::composite::DefaultNestableKey>,
                >,
            > + crate::tuples::KeysReset,
        const L: key::layered::LayerIndex,
    > Keymap<I, L>
{
    /// Constructs a new keymap with the given key definitions and context.
    pub const fn new(
        key_definitions: I,
        context: composite::Context<L, composite::DefaultNestableKey>,
    ) -> Self {
        Self {
            key_definitions,
            context,
            pressed_inputs: heapless::Vec::new(),
            event_scheduler: EventScheduler::new(),
        }
    }

    /// Initializes or resets the keyboard to an initial state.
    pub fn init(&mut self) {
        self.pressed_inputs.clear();
        self.event_scheduler.init();
        self.key_definitions.reset();
    }

    /// Handles input events.
    pub fn handle_input(&mut self, ev: input::Event) {
        // Update each of the pressed keys with the event.
        self.pressed_inputs.iter_mut().for_each(|pi| {
            if let input::PressedInput::Key { keymap_index, .. } = pi {
                let key = &mut self.key_definitions[*keymap_index as usize];
                key.handle_event(&self.context, ev.into())
                    .into_iter()
                    .for_each(|sch_ev| self.event_scheduler.schedule_event(sch_ev));
            }
        });

        match ev {
            input::Event::Press { keymap_index } => {
                let key = &mut self.key_definitions[keymap_index as usize];

                key.handle_event(&self.context, ev.into())
                    .into_iter()
                    .for_each(|sch_ev| self.event_scheduler.schedule_event(sch_ev));

                self.pressed_inputs
                    .push(input::PressedInput::new_pressed_key(keymap_index))
                    .unwrap();
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

    /// Advances the state of the keymap by one tick.
    pub fn tick(&mut self) {
        self.event_scheduler.tick();

        // take from pending
        if let Some(ev) = self.event_scheduler.dequeue() {
            // Update each of the pressed keys with the event.
            self.pressed_inputs.iter_mut().for_each(|pi| {
                if let input::PressedInput::Key { keymap_index, .. } = pi {
                    let key = &mut self.key_definitions[*keymap_index as usize];
                    key.handle_event(&self.context, ev)
                        .into_iter()
                        .for_each(|sch_ev| self.event_scheduler.schedule_event(sch_ev));
                }
            });

            // Update context with the event
            if let key::Event::Key(ev) = ev {
                self.context.handle_event(ev);
            }

            if let Event::Input(input_ev) = ev {
                self.handle_input(input_ev);
            }
        }
    }

    /// Returns the current HID keyboard report.
    pub fn boot_keyboard_report(&self) -> [u8; 8] {
        let mut report = [0u8; 8];

        let pressed_keys = self.pressed_inputs.iter().filter_map(|pi| match pi {
            input::PressedInput::Key { keymap_index, .. } => {
                let key = &self.key_definitions[*keymap_index as usize];
                key.key_code()
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tuples;

    #[test]
    fn test_keymap_with_simple_key_with_composite_context() {
        use key::composite::{Context, DefaultNestableKey, Event};
        use key::simple;
        use tuples::Keys1;

        // Assemble
        let keys: Keys1<simple::Key, Context<0, DefaultNestableKey>, Event> =
            Keys1::new((simple::Key(0x04),));
        let context = composite::Context::new();
        let mut keymap = Keymap::new(keys, context);

        // Act
        keymap.handle_input(input::Event::Press { keymap_index: 0 });
        keymap.tick();
        let actual_report = keymap.boot_keyboard_report();

        // Assert
        let expected_report: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];
        assert_eq!(actual_report, expected_report);
    }

    #[test]
    fn test_keymap_with_composite_simple_key() {
        use key::{composite, simple};
        use tuples::Keys1;

        use composite::{Context, DefaultNestableKey, Event};

        // Assemble
        let keys: Keys1<composite::Key, Context<0, DefaultNestableKey>, Event> =
            Keys1::new((composite::Key::<0>::Simple(simple::Key(0x04)),));
        let context = composite::Context::new();
        let mut keymap = Keymap::new(keys, context);

        // Act
        keymap.handle_input(input::Event::Press { keymap_index: 0 });
        keymap.tick();
        let actual_report = keymap.boot_keyboard_report();

        // Assert
        let expected_report: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];
        assert_eq!(actual_report, expected_report);
    }

    #[test]
    fn test_keymap_with_composite_layered_key_press_base_key() {
        use key::{composite, layered, simple};
        use tuples::Keys2;

        use composite::{Context, DefaultNestableKey, Event};

        // Assemble
        const L: layered::LayerIndex = 1;
        let keys: Keys2<
            composite::Key<L>,
            composite::Key<L>,
            Context<L, DefaultNestableKey>,
            Event,
        > = tuples::Keys2::new((
            composite::Key::<L>::LayerModifier(layered::ModifierKey::Hold(0)),
            composite::Key::<L>::Layered(layered::LayeredKey::new(
                simple::Key(0x04),
                [Some(simple::Key(0x05))],
            )),
        ));
        let context = composite::Context::<L, composite::DefaultNestableKey>::new();
        let mut keymap = Keymap::new(keys, context);

        // Act
        keymap.handle_input(input::Event::Press { keymap_index: 1 });
        let actual_report = keymap.boot_keyboard_report();

        // Assert
        let expected_report: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];
        assert_eq!(actual_report, expected_report);
    }

    #[test]
    fn test_keymap_with_composite_layered_key_press_active_layer_when_layer_mod_held() {
        use key::{composite, layered, simple};
        use tuples::Keys2;

        use composite::{Context, DefaultNestableKey, Event};

        // Assemble
        const L: layered::LayerIndex = 1;
        let keys: Keys2<
            composite::Key<L>,
            composite::Key<L>,
            Context<L, DefaultNestableKey>,
            Event,
        > = tuples::Keys2::new((
            composite::Key::<L>::LayerModifier(layered::ModifierKey::Hold(0)),
            composite::Key::<L>::Layered(layered::LayeredKey::new(
                simple::Key(0x04),
                [Some(simple::Key(0x05))],
            )),
        ));
        let context = composite::Context::<L, composite::DefaultNestableKey>::new();
        let mut keymap = Keymap::new(keys, context);

        // Act
        keymap.handle_input(input::Event::Press { keymap_index: 0 });
        keymap.tick();
        keymap.handle_input(input::Event::Press { keymap_index: 1 });
        let actual_report = keymap.boot_keyboard_report();

        // Assert
        let expected_report: [u8; 8] = [0, 0, 0x05, 0, 0, 0, 0, 0];
        assert_eq!(actual_report, expected_report);
    }

    #[test]
    fn test_keymap_with_composite_layered_key_press_retained_when_layer_mod_released() {
        use key::{composite, layered, simple};
        use tuples::Keys2;

        use composite::{Context, DefaultNestableKey, Event};

        // Assemble
        const L: layered::LayerIndex = 1;
        let keys: Keys2<
            composite::Key<L>,
            composite::Key<L>,
            Context<L, DefaultNestableKey>,
            Event,
        > = tuples::Keys2::new((
            composite::Key::<L>::LayerModifier(layered::ModifierKey::Hold(0)),
            composite::Key::<L>::Layered(layered::LayeredKey::new(
                simple::Key(0x04),
                [Some(simple::Key(0x05))],
            )),
        ));
        let context = composite::Context::<L, composite::DefaultNestableKey>::new();
        let mut keymap = Keymap::new(keys, context);

        // Act
        keymap.handle_input(input::Event::Press { keymap_index: 0 });
        keymap.tick();
        keymap.handle_input(input::Event::Press { keymap_index: 1 });
        keymap.tick();
        keymap.handle_input(input::Event::Release { keymap_index: 0 });
        keymap.tick();
        let actual_report = keymap.boot_keyboard_report();

        // Assert
        let expected_report: [u8; 8] = [0, 0, 0x05, 0, 0, 0, 0, 0];
        assert_eq!(actual_report, expected_report);
    }

    #[test]
    fn test_keymap_with_composite_layered_key_uses_base_when_pressed_after_layer_mod_released() {
        use key::{composite, layered, simple};
        use tuples::Keys2;

        use composite::{Context, DefaultNestableKey, Event};

        // Assemble
        const L: layered::LayerIndex = 1;
        let keys: Keys2<
            composite::Key<L>,
            composite::Key<L>,
            Context<L, DefaultNestableKey>,
            Event,
        > = tuples::Keys2::new((
            composite::Key::<L>::LayerModifier(layered::ModifierKey::Hold(0)),
            composite::Key::<L>::Layered(layered::LayeredKey::new(
                simple::Key(0x04),
                [Some(simple::Key(0x05))],
            )),
        ));
        let context = composite::Context::<L, composite::DefaultNestableKey>::new();
        let mut keymap = Keymap::new(keys, context);

        // Act
        keymap.handle_input(input::Event::Press { keymap_index: 0 });
        keymap.tick();
        keymap.handle_input(input::Event::Release { keymap_index: 0 });
        keymap.tick();
        keymap.handle_input(input::Event::Press { keymap_index: 1 });
        keymap.tick();
        let actual_report = keymap.boot_keyboard_report();

        // Assert
        let expected_report: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];
        assert_eq!(actual_report, expected_report);
    }
}
