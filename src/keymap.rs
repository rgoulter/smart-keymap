use core::fmt::Debug;
use core::ops::IndexMut;

use crate::input;
use crate::key;

use key::{composite, Context, Event, PressedKey};

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

/// Constructs an HID report or a sequence of key codes from the given sequence of [key::KeyOutput].
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

/// Transforms output from the keymap so it's suitable for HID keyboard reports.
///
/// e.g. limits output to one new pressed key per sent report,
///  so that the USB host doesn't confuse the sequence of pressed keys.
#[derive(Debug)]
pub struct HIDKeyboardReporter {
    pressed_key_outputs: heapless::Vec<key::KeyOutput, 16>,
    num_reportable_keys: u8,
}

impl HIDKeyboardReporter {
    /// Constructs a new HIDKeyboardReporter.
    pub const fn new() -> Self {
        Self {
            pressed_key_outputs: heapless::Vec::new(),
            num_reportable_keys: 1,
        }
    }

    /// Transforms the keymap output to a HID keyboard report.
    pub fn init(&mut self) {
        self.pressed_key_outputs.clear();
        self.num_reportable_keys = 1;
    }

    /// Updates the state of the HIDKeyboardReporter with the given pressed key outputs.
    pub fn update(&mut self, pressed_key_outputs: heapless::Vec<key::KeyOutput, 16>) {
        // e.g.
        //  WAS: A B C
        //  NOW: A   C D
        //   -> released B, pressed D
        let mut prev_iter = self.pressed_key_outputs.iter();
        let mut new_iter = pressed_key_outputs.iter();

        while let Some(new_key_output) = new_iter.next() {
            while let Some(prev_key_output) = prev_iter.next() {
                if prev_key_output == new_key_output {
                    // Same key output in both
                    break;
                } else {
                    // The key in the previous report doesn't match key in new report;
                    //  hence, it has been released.
                    if self.num_reportable_keys > 1 {
                        self.num_reportable_keys -= 1;
                    }
                }
            }
        }

        while let Some(_) = prev_iter.next() {
            // The key in the previous report, but not in new report.
            //  hence, it has been released.
            if self.num_reportable_keys > 1 {
                self.num_reportable_keys -= 1;
            }
        }

        self.pressed_key_outputs = pressed_key_outputs;
    }

    /// Indicate an HID report was sent. Allows reporting one more key in the next report.
    pub fn report_sent(&mut self) {
        if self.pressed_key_outputs.len() > self.num_reportable_keys.into() {
            self.num_reportable_keys += 1;
        }
    }

    /// Gets the filtered pressed key outputs, suitable for sending for HID reports.
    pub fn reportable_key_outputs(&self) -> heapless::Vec<key::KeyOutput, 16> {
        self.pressed_key_outputs
            .clone()
            .into_iter()
            .take(self.num_reportable_keys as usize)
            .collect()
    }
}

/// For tracking distinct HID reports from the keymap.
#[cfg(feature = "std")]
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DistinctReports(Vec<[u8; 8]>);

#[cfg(feature = "std")]
impl DistinctReports {
    /// Constructs a new DistinctReports.
    pub fn new() -> Self {
        Self(vec![[0; 8]])
    }

    /// Adds the report to the distinct reports.
    pub fn update(&mut self, report: [u8; 8]) {
        match self.0.last() {
            Some(last_report) if last_report == &report => {}
            _ => self.0.push(report),
        }
    }
}

#[derive(Debug)]
struct PendingState {
    keymap_index: u16,
    queued_events: heapless::Vec<key::Event<composite::Event>, 16>,
}

/// State for a keymap that handles input, and outputs HID keyboard reports.
#[derive(Debug)]
pub struct Keymap<I> {
    key_definitions: I,
    context: composite::Context,
    pressed_inputs: heapless::Vec<input::PressedInput<composite::PressedKey>, 16>,
    event_scheduler: EventScheduler<composite::Event>,
    hid_reporter: HIDKeyboardReporter,
    pending_key_state: Option<PendingState>,
}

impl<
        K: key::Key<
                Context = composite::Context,
                Event = composite::Event,
                PressedKey = composite::PressedKey,
            > + ?Sized,
        I: IndexMut<usize, Output = K>,
    > Keymap<I>
{
    /// Constructs a new keymap with the given key definitions and context.
    pub const fn new(key_definitions: I, context: composite::Context) -> Self {
        Self {
            key_definitions,
            context,
            pressed_inputs: heapless::Vec::new(),
            event_scheduler: EventScheduler::new(),
            hid_reporter: HIDKeyboardReporter::new(),
            pending_key_state: None,
        }
    }

    /// Initializes or resets the keyboard to an initial state.
    pub fn init(&mut self) {
        self.pressed_inputs.clear();
        self.event_scheduler.init();
        self.hid_reporter.init();
        self.pending_key_state = None;
    }

    /// Handles input events.
    pub fn handle_input(&mut self, ev: input::Event) {
        if let Some(PendingState { queued_events, .. }) = &mut self.pending_key_state {
            queued_events.push(ev.into()).unwrap();
        } else {
            // Update each of the pressed keys with the event.
            self.pressed_inputs.iter_mut().for_each(|pi| {
                if let input::PressedInput::Key { pressed_key, .. } = pi {
                    pressed_key
                        .handle_event(self.context, ev.into())
                        .into_iter()
                        .for_each(|sch_ev| self.event_scheduler.schedule_event(sch_ev));
                }
            });

            self.handle_all_pending_events();

            match ev {
                input::Event::Press { keymap_index } => {
                    let key = &mut self.key_definitions[keymap_index as usize];

                    let (pk, pke) = key.new_pressed_key(self.context, keymap_index);

                    pke.into_iter()
                        .for_each(|sch_ev| self.event_scheduler.schedule_event(sch_ev));

                    self.pressed_inputs
                        .push(input::PressedInput::new_pressed_key(pk))
                        .unwrap();
                }
                input::Event::Release { keymap_index } => {
                    self.pressed_inputs
                        .iter()
                        .position(|pi| match pi {
                            input::PressedInput::Key {
                                pressed_key:
                                    input::PressedKey {
                                        keymap_index: ki, ..
                                    },
                                ..
                            } => keymap_index == *ki,
                            _ => false,
                        })
                        .map(|i| self.pressed_inputs.remove(i));

                    self.event_scheduler
                        .cancel_events_for_keymap_index(keymap_index);
                }

                input::Event::VirtualKeyPress {
                    key_code,
                    pressed_keymap_index,
                } => {
                    // Insert into pressed_keys before the pressed key with the
                    //  given keymap index.
                    let pressed_key = input::PressedInput::Virtual { key_code };
                    let pos = self
                        .pressed_inputs
                        .iter()
                        .position(|k| match k {
                            input::PressedInput::Key { pressed_key, .. } => {
                                pressed_key.keymap_index == pressed_keymap_index
                            }
                            _ => false,
                        })
                        .unwrap_or(self.pressed_inputs.len());
                    self.pressed_inputs.insert(pos, pressed_key).unwrap();
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
    }

    fn handle_event(&mut self, ev: key::Event<composite::Event>) {
        // Update each of the pressed keys with the event.
        self.pressed_inputs.iter_mut().for_each(|pi| {
            if let input::PressedInput::Key { pressed_key, .. } = pi {
                pressed_key
                    .handle_event(self.context, ev)
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

    fn handle_all_pending_events(&mut self) {
        // take from pending
        while let Some(ev) = self.event_scheduler.dequeue() {
            self.handle_event(ev);
        }
    }

    /// Advances the state of the keymap by one tick.
    pub fn tick(&mut self) {
        self.event_scheduler.tick();

        self.handle_all_pending_events();
    }

    /// Returns the the pressed key outputs.
    pub fn pressed_keys(&self) -> heapless::Vec<key::KeyOutput, 16> {
        let pressed_key_codes = self
            .pressed_inputs
            .iter()
            .take_while(|pi| match pi {
                input::PressedInput::Key { pressed_key, .. } => {
                    pressed_key.key_output().is_resolved()
                }
                _ => true,
            })
            .filter_map(|pi| match pi {
                input::PressedInput::Key { pressed_key, .. } => {
                    pressed_key.key_output().to_option()
                }
                input::PressedInput::Virtual { key_code } => {
                    Some(key::KeyOutput::from_key_code(*key_code))
                }
            });

        pressed_key_codes.collect()
    }

    /// Updates the keymap indicating a report is sent; returns the reportable keymap output.
    pub fn report_output(&mut self) -> KeymapOutput {
        self.hid_reporter.update(self.pressed_keys());
        let output = KeymapOutput::new(self.hid_reporter.reportable_key_outputs());
        self.hid_reporter.report_sent();
        output
    }

    /// Returns the current HID keyboard report.
    #[doc(hidden)]
    pub fn boot_keyboard_report(&self) -> [u8; 8] {
        KeymapOutput::new(self.pressed_keys()).as_hid_boot_keyboard_report()
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
        assert_eq!(expected_report, actual_report);
    }

    #[test]
    fn test_hid_keyboard_reporter_reports_single_keypress() {
        // Assemble
        let mut input: heapless::Vec<key::KeyOutput, 16> = heapless::Vec::new();
        input.push(key::KeyOutput::from_key_code(0x04)).unwrap();

        let mut reporter = HIDKeyboardReporter::new();

        // Act
        reporter.update(input);
        let actual_outputs = reporter.reportable_key_outputs();

        // Assert
        let expected_outputs: heapless::Vec<key::KeyOutput, 16> = [0x04]
            .iter()
            .map(|kc| key::KeyOutput::from_key_code(*kc))
            .collect();
        assert_eq!(expected_outputs, actual_outputs);
    }

    #[test]
    fn test_hid_keyboard_reporter_reports_single_new_keypress_per_report_sent() {
        // Assemble
        let input: heapless::Vec<key::KeyOutput, 16> = [0x04, 0x05]
            .iter()
            .map(|kc| key::KeyOutput::from_key_code(*kc))
            .collect();

        let mut reporter = HIDKeyboardReporter::new();

        // Act
        reporter.update(input);
        let actual_outputs = reporter.reportable_key_outputs();

        // Assert
        let expected_outputs: heapless::Vec<key::KeyOutput, 16> = [0x04]
            .iter()
            .map(|kc| key::KeyOutput::from_key_code(*kc))
            .collect();
        assert_eq!(expected_outputs, actual_outputs);
    }

    #[test]
    fn test_hid_keyboard_reporter_reports_more_keypresses_after_report_sent() {
        // Assemble
        let input: heapless::Vec<key::KeyOutput, 16> = [0x04, 0x05]
            .iter()
            .map(|kc| key::KeyOutput::from_key_code(*kc))
            .collect();

        let mut reporter = HIDKeyboardReporter::new();

        // Act
        reporter.update(input);
        reporter.report_sent();
        let actual_outputs = reporter.reportable_key_outputs();

        // Assert
        let expected_outputs: heapless::Vec<key::KeyOutput, 16> = [0x04, 0x05]
            .iter()
            .map(|kc| key::KeyOutput::from_key_code(*kc))
            .collect();
        assert_eq!(expected_outputs, actual_outputs);
    }

    #[test]
    fn test_hid_keyboard_reporter_reports_updates_for_key_releases() {
        // Assemble
        let input: heapless::Vec<key::KeyOutput, 16> = [0x04, 0x05]
            .iter()
            .map(|kc| key::KeyOutput::from_key_code(*kc))
            .collect();
        let input_after_key_released: heapless::Vec<key::KeyOutput, 16> = [0x05]
            .iter()
            .map(|kc| key::KeyOutput::from_key_code(*kc))
            .collect();
        let input_after_more_keys_pressed: heapless::Vec<key::KeyOutput, 16> = [0x05, 0x06, 0x07]
            .iter()
            .map(|kc| key::KeyOutput::from_key_code(*kc))
            .collect();

        let mut reporter = HIDKeyboardReporter::new();

        // Act
        reporter.update(input);
        reporter.report_sent(); // now may report 2 keys
        assert_eq!(2, reporter.num_reportable_keys);
        reporter.update(input_after_key_released); // 1 key released; so, only may report 1 key
        assert_eq!(1, reporter.num_reportable_keys);
        reporter.report_sent();
        assert_eq!(1, reporter.num_reportable_keys);
        reporter.update(input_after_more_keys_pressed); // 1+2 new pressed in KM; only 2 should reported
        reporter.report_sent();
        assert_eq!(2, reporter.num_reportable_keys);
        let actual_outputs = reporter.reportable_key_outputs();

        // Assert
        let expected_outputs: heapless::Vec<key::KeyOutput, 16> = [0x05, 0x06]
            .iter()
            .map(|kc| key::KeyOutput::from_key_code(*kc))
            .collect();
        assert_eq!(
            KeymapOutput::new(expected_outputs).as_hid_boot_keyboard_report(),
            KeymapOutput::new(actual_outputs).as_hid_boot_keyboard_report(),
        );
    }

    #[test]
    fn test_keymap_with_keyboard_key_with_composite_context() {
        use key::composite::{Context, Event, PressedKey};
        use key::keyboard;
        use tuples::Keys1;

        // Assemble
        type Ctx = Context;
        type K = composite::Layered<composite::TapHold<keyboard::Key>>;
        let keys: Keys1<K, Context, Event, PressedKey> = Keys1::new((composite::Layered(
            composite::TapHold(keyboard::Key::new(0x04)),
        ),));
        let context: Ctx = composite::DEFAULT_CONTEXT;
        let mut keymap = Keymap::new(keys, context);

        // Act
        keymap.handle_input(input::Event::Press { keymap_index: 0 });
        let actual_report = keymap.boot_keyboard_report();

        // Assert
        let expected_report: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];
        assert_eq!(expected_report, actual_report);
    }

    #[test]
    fn test_keymap_with_composite_keyboard_key() {
        use key::{composite, keyboard};
        use tuples::Keys1;

        use composite::{Context, Event, PressedKey};

        // Assemble
        let keys: Keys1<composite::Key, Context, Event, PressedKey> =
            Keys1::new((composite::Key::keyboard(keyboard::Key::new(0x04)),));
        let context: Context = composite::DEFAULT_CONTEXT;
        let mut keymap = Keymap::new(keys, context);

        // Act
        keymap.handle_input(input::Event::Press { keymap_index: 0 });
        let actual_report = keymap.boot_keyboard_report();

        // Assert
        let expected_report: [u8; 8] = [0, 0, 0x04, 0, 0, 0, 0, 0];
        assert_eq!(expected_report, actual_report);
    }
}
