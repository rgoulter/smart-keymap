use core::fmt::Debug;
use core::ops::IndexMut;

use crate::input;
use crate::key;

use key::{composite, Context, Event};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct ScheduledEvent<E: Debug> {
    time: u32,
    event: Event<E>,
}

#[derive(Debug)]
struct EventScheduler<E: Debug> {
    pending_events: heapless::spsc::Queue<Event<E>, 256>,
    scheduled_events: heapless::Vec<ScheduledEvent<E>, 16>,
    schedule_counter: u32,
}

impl<E: Debug> EventScheduler<E> {
    pub const fn new() -> Self {
        Self {
            pending_events: heapless::spsc::Queue::new(),
            scheduled_events: heapless::Vec::new(),
            schedule_counter: 0,
        }
    }

    pub fn init(&mut self) {
        while self.pending_events.dequeue().is_some() {}
        self.scheduled_events.clear();
        self.schedule_counter = 0;
    }

    pub fn schedule_event(&mut self, scheduled_event: key::ScheduledEvent<E>) {
        match scheduled_event.schedule {
            key::Schedule::Immediate => {
                self.pending_events.enqueue(scheduled_event.event).unwrap();
            }
            key::Schedule::After(delay) => {
                self.schedule_after(delay as u32, scheduled_event.event);
            }
        }
    }

    pub fn schedule_after(&mut self, delay: u32, event: Event<E>) {
        let time = self.schedule_counter + delay;
        // binary sort insertion;
        //  smallest at *end* (quick to pop off),
        //  highest at *start*.
        let pos = self
            .scheduled_events
            .binary_search_by(|sch_item| sch_item.time.cmp(&delay).reverse())
            .unwrap_or_else(|e| e);
        self.scheduled_events
            .insert(pos, ScheduledEvent { time, event })
            .unwrap();
    }

    pub fn cancel_events_for_keymap_index(&mut self, keymap_index: u16) {
        self.scheduled_events
            .retain(|ScheduledEvent { event, .. }| match event {
                Event::Key {
                    keymap_index: ki, ..
                } => *ki != keymap_index,
                _ => true,
            });
    }

    pub fn tick(&mut self) {
        self.schedule_counter += 1;
        let scheduled_ready =
            if let Some(ScheduledEvent { time, .. }) = self.scheduled_events.last() {
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

    pub fn dequeue(&mut self) -> Option<Event<E>> {
        self.pending_events.dequeue()
    }
}

/// Output from the keymap, used to build HID reports.
#[derive(Debug)]
pub struct KeymapOutput {
    pressed_key_codes: heapless::Vec<key::KeyOutput, 16>,
}

impl KeymapOutput {
    /// Constructs a new keymap output.
    pub fn new(pressed_key_codes: heapless::Vec<key::KeyOutput, 16>) -> Self {
        Self { pressed_key_codes }
    }

    /// Returns the pressed key codes.
    pub fn pressed_key_codes(&self) -> heapless::Vec<u8, 24> {
        let mut result = heapless::Vec::new();

        let modifiers = self
            .pressed_key_codes
            .iter()
            .fold(key::KeyboardModifiers::new(), |acc, &ko| {
                acc.union(&ko.key_modifiers())
            });

        result.extend(modifiers.as_key_codes());

        result.extend(self.pressed_key_codes.iter().map(|ko| ko.key_code()));

        result
    }

    /// Returns the current HID keyboard report.
    pub fn as_hid_boot_keyboard_report(&self) -> [u8; 8] {
        let mut report = [0u8; 8];

        let modifiers = self
            .pressed_key_codes
            .iter()
            .fold(key::KeyboardModifiers::new(), |acc, &ko| {
                acc.union(&ko.key_modifiers())
            });

        report[0] = modifiers.as_byte();

        let key_codes = self
            .pressed_key_codes
            .iter()
            .map(|ko| ko.key_code())
            .filter(|&kc| kc != 0);

        for (i, key_code) in key_codes.take(6).enumerate() {
            report[i + 2] = key_code;
        }

        report
    }
}

/// State for a keymap that handles input, and outputs HID keyboard reports.
#[derive(Debug)]
pub struct Keymap<
    I: IndexMut<
            usize,
            Output = dyn key::dynamic::Key<
                key::composite::Event,
                Context = key::composite::Context<L>,
            >,
        > + crate::tuples::KeysReset,
    L: key::layered::LayerImpl = key::layered::ArrayImpl<0>,
> {
    key_definitions: I,
    context: composite::Context<L>,
    pressed_inputs: heapless::Vec<input::PressedInput, 16>,
    event_scheduler: EventScheduler<composite::Event>,
}

impl<
        I: IndexMut<
                usize,
                Output = dyn key::dynamic::Key<
                    key::composite::Event,
                    Context = key::composite::Context<L>,
                >,
            > + crate::tuples::KeysReset,
        L: key::layered::LayerImpl,
    > Keymap<I, L>
{
    /// Constructs a new keymap with the given key definitions and context.
    pub const fn new(key_definitions: I, context: composite::Context<L>) -> Self {
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
                key.handle_event(self.context, ev.into())
                    .into_iter()
                    .for_each(|sch_ev| self.event_scheduler.schedule_event(sch_ev));
            }
        });

        self.handle_all_pending_events();

        match ev {
            input::Event::Press { keymap_index } => {
                let key = &mut self.key_definitions[keymap_index as usize];

                key.handle_event(self.context, ev.into())
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

                self.event_scheduler
                    .cancel_events_for_keymap_index(keymap_index);
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

        self.handle_all_pending_events();
    }

    fn handle_all_pending_events(&mut self) {
        // take from pending
        while let Some(ev) = self.event_scheduler.dequeue() {
            // Update each of the pressed keys with the event.
            self.pressed_inputs.iter_mut().for_each(|pi| {
                if let input::PressedInput::Key { keymap_index, .. } = pi {
                    let key = &mut self.key_definitions[*keymap_index as usize];
                    key.handle_event(self.context, ev)
                        .into_iter()
                        .for_each(|sch_ev| self.event_scheduler.schedule_event(sch_ev));
                }
            });

            // Update context with the event
            if let key::Event::Key { key_event, .. } = ev {
                self.context.handle_event(key_event);
            }

            if let Event::Input(input_ev) = ev {
                self.handle_input(input_ev);
            }
        }
    }

    /// Advances the state of the keymap by one tick.
    pub fn tick(&mut self) {
        self.event_scheduler.tick();

        self.handle_all_pending_events();
    }

    /// Returns the the pressed key codes.
    pub fn pressed_keys(&self) -> KeymapOutput {
        let pressed_key_codes = self.pressed_inputs.iter().filter_map(|pi| match pi {
            input::PressedInput::Key { keymap_index, .. } => {
                let key = &self.key_definitions[*keymap_index as usize];
                key.key_output()
            }
            input::PressedInput::Virtual { key_code } => {
                Some(key::KeyOutput::from_key_code(*key_code))
            }
        });

        KeymapOutput::new(pressed_key_codes.collect())
    }

    /// Returns the current HID keyboard report.
    pub fn boot_keyboard_report(&self) -> [u8; 8] {
        self.pressed_keys().as_hid_boot_keyboard_report()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tuples;

    #[test]
    fn test_keymap_output_pressed_key_codes_includes_modifier_key_code() {
        // Assemble - include modifier key left ctrl
        let mut input: heapless::Vec<key::KeyOutput, 16> = heapless::Vec::new();
        input.push(key::KeyOutput::from_key_code(0x04)).unwrap();
        input.push(key::KeyOutput::from_key_code(0xE0)).unwrap();

        // Act - construct the output
        let keymap_output = KeymapOutput::new(input);
        let pressed_key_codes = keymap_output.pressed_key_codes();

        // Assert - check the 0xE0 gets included as a key code.
        assert!(pressed_key_codes.contains(&0xE0))
    }

    #[test]
    fn test_keymap_output_as_hid_boot_keyboard_report_gathers_modifiers() {
        // Assemble - include modifier key left ctrl
        let mut input: heapless::Vec<key::KeyOutput, 16> = heapless::Vec::new();
        input.push(key::KeyOutput::from_key_code(0x04)).unwrap();
        input.push(key::KeyOutput::from_key_code(0xE0)).unwrap();

        // Act - construct the output
        let keymap_output = KeymapOutput::new(input);
        let actual_report: [u8; 8] = keymap_output.as_hid_boot_keyboard_report();

        // Assert - check the 0xE0 gets considered as a "modifier".
        let expected_report: [u8; 8] = [0x01, 0, 0x04, 0, 0, 0, 0, 0];
        assert_eq!(actual_report, expected_report);
    }

    #[test]
    fn test_keymap_with_keyboard_key_with_composite_context() {
        use key::composite::{Context, Event};
        use key::keyboard;
        use tuples::Keys1;

        // Assemble
        const NUM_LAYERS: usize = 0;
        type L = crate::key::layered::ArrayImpl<NUM_LAYERS>;
        type Ctx = Context<L>;
        let keys: Keys1<keyboard::Key, Context, Event> = Keys1::new((keyboard::Key::new(0x04),));
        let context: Ctx = Ctx {
            layer_context: crate::key::layered::Context {
                active_layers: [false; NUM_LAYERS],
            },
        };
        let mut keymap = Keymap::new(keys, context);

        // Act
        keymap.handle_input(input::Event::Press { keymap_index: 0 });
        let actual_report = keymap.boot_keyboard_report();

        // Assert
        let expected_report: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];
        assert_eq!(actual_report, expected_report);
    }

    #[test]
    fn test_keymap_with_tap_hold_key_with_composite_context_key_tapped() {
        use key::composite::{Context, Event};
        use key::{keyboard, tap_hold};
        use tuples::Keys1;

        // Assemble
        let keys: Keys1<tap_hold::Key<keyboard::Key>, Context, Event> =
            Keys1::new((tap_hold::Key {
                tap: keyboard::Key::new(0x04),
                hold: keyboard::Key::new(0xE0),
            },));
        let context: Context = Context {
            layer_context: crate::key::layered::Context {
                active_layers: [false; 0],
            },
        };
        let mut keymap = Keymap::new(keys, context);

        // Act
        keymap.handle_input(input::Event::Press { keymap_index: 0 });
        keymap.handle_input(input::Event::Release { keymap_index: 0 });
        let actual_report = keymap.boot_keyboard_report();

        // Assert
        let expected_report: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];
        assert_eq!(actual_report, expected_report);
    }

    #[test]
    fn test_keymap_with_tap_hold_key_with_composite_context_key_unaffected_by_prev_key_release() {
        use key::composite::{Context, Event};
        use key::{keyboard, tap_hold};
        use tuples::Keys1;

        // When a tap-hold key is pressed,
        //  it schedules a Timeout event after 200 ticks.
        // In case of releasing, then pressing the key a second time within 200 ticks,
        //  we do not want the first Timeout to affect the second key press.

        // Assemble
        let keys: Keys1<tap_hold::Key<keyboard::Key>, Context, Event> =
            Keys1::new((tap_hold::Key {
                tap: keyboard::Key::new(0x04),
                hold: keyboard::Key::new(0xE0),
            },));
        let context: Context = Context {
            layer_context: crate::key::layered::Context {
                active_layers: [false; 0],
            },
        };
        let mut keymap = Keymap::new(keys, context);

        // Act
        // Press key (starting a 200 tick timeout),
        keymap.handle_input(input::Event::Press { keymap_index: 0 });
        // Release, then press key a second time before 200 ticks.
        keymap.handle_input(input::Event::Release { keymap_index: 0 });
        for _ in 0..150 {
            keymap.tick();
        }
        keymap.handle_input(input::Event::Press { keymap_index: 0 });
        // Tick a few more times, until the first timeout would be scheduled,
        // (but before the second timeout is scheduled)
        for _ in 0..100 {
            // 250
            keymap.tick();
        }
        let actual_report = keymap.boot_keyboard_report();

        // Assert
        // Second timeout not invoked, key is still "Pending" state.
        let expected_report: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(actual_report, expected_report);
    }

    #[test]
    fn test_keymap_with_composite_keyboard_key() {
        use key::{composite, keyboard};
        use tuples::Keys1;

        use composite::{Context, Event};

        // Assemble
        let keys: Keys1<composite::Key, Context, Event> =
            Keys1::new((composite::Key::keyboard(keyboard::Key::new(0x04)),));
        let context: Context = Context {
            layer_context: crate::key::layered::Context {
                active_layers: [false; 0],
            },
        };
        let mut keymap = Keymap::new(keys, context);

        // Act
        keymap.handle_input(input::Event::Press { keymap_index: 0 });
        let actual_report = keymap.boot_keyboard_report();

        // Assert
        let expected_report: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];
        assert_eq!(actual_report, expected_report);
    }

    #[test]
    fn test_keymap_with_layered_key_press_active_layer_when_layer_mod_held() {
        use key::{composite, keyboard, layered};
        use tuples::Keys2;

        // Assemble
        const NUM_LAYERS: usize = 1;
        type NK = composite::DefaultNestableKey;
        type L = layered::ArrayImpl<NUM_LAYERS>;
        type Ctx = composite::Context<L>;
        type Ev = composite::Event;
        type MK = layered::ModifierKey;
        type LK = layered::LayeredKey<NK, L>;
        let keys: Keys2<MK, LK, Ctx, Ev> = tuples::Keys2::new((
            layered::ModifierKey::Hold(0),
            layered::LayeredKey::new(keyboard::Key::new(0x04), [Some(keyboard::Key::new(0x05))]),
        ));
        let context: Ctx = Ctx {
            layer_context: layered::Context {
                active_layers: [false; NUM_LAYERS],
            },
        };

        let mut keymap = Keymap::new(keys, context);

        // Act
        keymap.handle_input(input::Event::Press { keymap_index: 0 });
        keymap.handle_input(input::Event::Press { keymap_index: 1 });
        let actual_report = keymap.boot_keyboard_report();

        // Assert
        let expected_report: [u8; 8] = [0, 0, 0x05, 0, 0, 0, 0, 0];
        assert_eq!(actual_report, expected_report);
    }

    #[test]
    fn test_keymap_with_composite_layered_key_press_base_key() {
        use key::{composite, keyboard, layered};
        use tuples::Keys2;

        // Assemble
        const NUM_LAYERS: usize = 1;
        type NK = composite::DefaultNestableKey;
        type L = layered::ArrayImpl<NUM_LAYERS>;
        type T = composite::CompositeImpl<L, NK>;
        type Ctx = composite::Context<L>;
        type Ev = composite::Event;
        type K = composite::Key<T>;
        let keys: Keys2<K, K, Ctx, Ev> = tuples::Keys2::new((
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                keyboard::Key::new(0x04),
                [Some(keyboard::Key::new(0x05))],
            )),
        ));
        let context: Ctx = Ctx {
            layer_context: layered::Context {
                active_layers: [false; NUM_LAYERS],
            },
        };
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
        use key::{composite, keyboard, layered};
        use tuples::Keys2;

        // Assemble
        const NUM_LAYERS: usize = 1;
        type NK = composite::DefaultNestableKey;
        type L = layered::ArrayImpl<NUM_LAYERS>;
        type T = composite::CompositeImpl<L, NK>;
        type Ctx = composite::Context<L>;
        type Ev = composite::Event;
        type K = composite::Key<T>;
        let keys: Keys2<K, K, Ctx, Ev> = tuples::Keys2::new((
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                keyboard::Key::new(0x04),
                [Some(keyboard::Key::new(0x05))],
            )),
        ));
        let context: Ctx = Ctx {
            layer_context: layered::Context {
                active_layers: [false; NUM_LAYERS],
            },
        };

        let mut keymap = Keymap::new(keys, context);

        // Act
        keymap.handle_input(input::Event::Press { keymap_index: 0 });
        keymap.handle_input(input::Event::Press { keymap_index: 1 });
        let actual_report = keymap.boot_keyboard_report();

        // Assert
        let expected_report: [u8; 8] = [0, 0, 0x05, 0, 0, 0, 0, 0];
        assert_eq!(actual_report, expected_report);
    }

    #[test]
    fn test_keymap_with_composite_layered_key_press_retained_when_layer_mod_released() {
        use key::{composite, keyboard, layered};
        use tuples::Keys2;

        // Assemble
        const NUM_LAYERS: usize = 1;
        type NK = composite::DefaultNestableKey;
        type L = layered::ArrayImpl<NUM_LAYERS>;
        type T = composite::CompositeImpl<L, NK>;
        type Ctx = composite::Context<L>;
        type Ev = composite::Event;
        type K = composite::Key<T>;
        let keys: Keys2<K, K, Ctx, Ev> = tuples::Keys2::new((
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                keyboard::Key::new(0x04),
                [Some(keyboard::Key::new(0x05))],
            )),
        ));
        let context: Ctx = Ctx {
            layer_context: layered::Context {
                active_layers: [false; NUM_LAYERS],
            },
        };
        let mut keymap = Keymap::new(keys, context);

        // Act
        keymap.handle_input(input::Event::Press { keymap_index: 0 });
        keymap.handle_input(input::Event::Press { keymap_index: 1 });
        keymap.handle_input(input::Event::Release { keymap_index: 0 });
        let actual_report = keymap.boot_keyboard_report();

        // Assert
        let expected_report: [u8; 8] = [0, 0, 0x05, 0, 0, 0, 0, 0];
        assert_eq!(actual_report, expected_report);
    }

    #[test]
    fn test_keymap_with_composite_layered_key_uses_base_when_pressed_after_layer_mod_released() {
        use key::{composite, keyboard, layered};
        use tuples::Keys2;

        // Assemble
        const NUM_LAYERS: usize = 1;
        type NK = composite::DefaultNestableKey;
        type L = layered::ArrayImpl<NUM_LAYERS>;
        type T = composite::CompositeImpl<L, NK>;
        type Ctx = composite::Context<L>;
        type Ev = composite::Event;
        type K = composite::Key<T>;
        let keys: Keys2<K, K, Ctx, Ev> = tuples::Keys2::new((
            K::layer_modifier(layered::ModifierKey::Hold(0)),
            K::layered(layered::LayeredKey::new(
                keyboard::Key::new(0x04),
                [Some(keyboard::Key::new(0x05))],
            )),
        ));
        let context: Ctx = Ctx {
            layer_context: layered::Context {
                active_layers: [false; NUM_LAYERS],
            },
        };
        let mut keymap = Keymap::new(keys, context);

        // Act
        keymap.handle_input(input::Event::Press { keymap_index: 0 });
        keymap.handle_input(input::Event::Release { keymap_index: 0 });
        keymap.handle_input(input::Event::Press { keymap_index: 1 });
        let actual_report = keymap.boot_keyboard_report();

        // Assert
        let expected_report: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];
        assert_eq!(actual_report, expected_report);
    }
}
