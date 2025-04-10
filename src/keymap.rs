use core::cmp::PartialEq;
use core::fmt::Debug;
use core::marker::Copy;
use core::ops::Index;

use serde::Deserialize;

use crate::input;
use crate::key;

use key::Event;

const MAX_PENDING_EVENTS: usize = 32;
const MAX_SCHEDULED_EVENTS: usize = 32;

/// Maximum number of pressed keys supported.
pub const MAX_PRESSED_KEYS: usize = 16;

const MAX_QUEUED_INPUT_EVENTS: usize = 32;

/// Number of ticks before the next input event is processed in tick().
pub const INPUT_QUEUE_TICK_DELAY: u8 = 1;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct ScheduledEvent<E: Debug> {
    time: u32,
    event: Event<E>,
}

#[derive(Debug)]
struct EventScheduler<E: Debug> {
    pending_events: heapless::spsc::Queue<Event<E>, { MAX_PENDING_EVENTS }>,
    scheduled_events: heapless::Vec<ScheduledEvent<E>, { MAX_SCHEDULED_EVENTS }>,
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
                self.enqueue_event(scheduled_event.event);
            }
            key::Schedule::After(delay) => {
                self.schedule_after(delay as u32, scheduled_event.event);
            }
        }
    }

    pub fn enqueue_event(&mut self, event: Event<E>) {
        self.pending_events.enqueue(event).unwrap();
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
                &Event::Key {
                    keymap_index: ki, ..
                } => ki != keymap_index,
                _ => true,
            });
    }

    pub fn tick(&mut self) {
        self.schedule_counter += 1;
        let scheduled_ready =
            if let Some(&ScheduledEvent { time, .. }) = self.scheduled_events.last() {
                time <= self.schedule_counter
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
    pressed_key_codes: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }>,
}

impl KeymapOutput {
    /// Constructs a new keymap output.
    pub fn new(pressed_key_codes: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }>) -> Self {
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
    pressed_key_outputs: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }>,
    num_reportable_keys: u8,
}

impl Default for HIDKeyboardReporter {
    fn default() -> Self {
        Self::new()
    }
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
    pub fn update(
        &mut self,
        pressed_key_outputs: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }>,
    ) {
        // e.g.
        //  WAS: A B C
        //  NOW: A   C D
        //   -> released B, pressed D
        let mut prev_iter = self.pressed_key_outputs.iter();
        let new_iter = pressed_key_outputs.iter();

        for new_key_output in new_iter {
            for prev_key_output in prev_iter.by_ref() {
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

        for _ in prev_iter {
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
    pub fn reportable_key_outputs(&self) -> heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> {
        self.pressed_key_outputs
            .clone()
            .into_iter()
            .take(self.num_reportable_keys as usize)
            .collect()
    }
}

/// For tracking distinct HID reports from the keymap.
#[cfg(feature = "std")]
#[derive(Debug, Clone, Eq)]
pub struct DistinctReports(Vec<[u8; 8]>);

#[cfg(feature = "std")]
impl Default for DistinctReports {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "std")]
impl core::cmp::PartialEq for DistinctReports {
    fn eq(&self, other: &Self) -> bool {
        // First element in DistinctReports should be [0; 8].
        if self.0[0] != other.0[0] {
            return false;
        }

        let mut i: usize = 1;
        let mut j: usize = 1;

        let self_len = self.0.len();
        let other_len = other.0.len();

        // Compare the rest of the elements.
        while i < self_len && j < other_len {
            // Ignore [0; 8] elements.
            // (The reports are distinct; so, no two elements should be equal)
            while (i < self_len - 1) && self.0[i] == [0; 8] {
                i += 1;
            }
            while (j < other_len - 1) && other.0[j] == [0; 8] {
                j += 1;
            }

            if self.0[i] != other.0[j] {
                return false;
            }

            i += 1;
            j += 1;
        }

        i == self_len && j == other_len
    }
}

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

    /// Access reports as slice of reports.
    pub fn reports(&self) -> &[[u8; 8]] {
        self.0.as_slice()
    }
}

#[derive(Debug)]
struct PendingState<Ev, PKS> {
    key_path: key::KeyPath,
    pending_key_state: PKS,
    queued_events: heapless::Vec<key::Event<Ev>, { MAX_PRESSED_KEYS }>,
}

/// Callbacks for effect keys in the keymap.
#[derive(Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum KeymapCallback {
    /// Reset the keyboard
    Reset,
    /// Reset the keyboard to bootloader
    ResetToBootloader,
}

/// Events related to the keymap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeymapEvent {
    /// Callback event (emitted by callback key).
    Callback(KeymapCallback),
    /// A pressed key resolved to a state with this key output.
    ResolvedKeyOutput {
        /// The keymap index of the key which resolved to the output.
        keymap_index: u16,
        /// The resolved key output.
        key_output: key::KeyOutput,
    },
}

#[derive(Debug)]
enum CallbackFunction {
    /// C callback
    ExternC(extern "C" fn() -> ()),
    /// Rust callback
    Rust(fn() -> ()),
}

/// State for a keymap that handles input, and outputs HID keyboard reports.
pub struct Keymap<Ctx, Ev: Debug, PKS, KS, I> {
    key_definitions: I,
    context: Ctx,
    pressed_inputs: heapless::Vec<input::PressedInput<KS>, { MAX_PRESSED_KEYS }>,
    event_scheduler: EventScheduler<Ev>,
    hid_reporter: HIDKeyboardReporter,
    pending_key_state: Option<PendingState<Ev, PKS>>,
    input_queue: heapless::spsc::Queue<input::Event, { MAX_QUEUED_INPUT_EVENTS }>,
    input_queue_delay_counter: u8,
    callbacks: heapless::LinearMap<KeymapCallback, CallbackFunction, 2>,
}

impl<
        Ctx: Debug,
        Ev: Debug,
        PKS: Debug,
        KS: Debug,
        K: key::Key<Context = Ctx, Event = Ev, PendingKeyState = PKS, KeyState = KS> + ?Sized,
        I: Index<usize, Output = K>,
    > core::fmt::Debug for Keymap<Ctx, Ev, PKS, KS, I>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Keymap")
            .field("context", &self.context)
            .field("event_scheduler", &self.event_scheduler)
            .field("hid_reporter", &self.hid_reporter)
            .field("input_queue", &self.input_queue)
            .field("input_queue_delay_counter", &self.input_queue_delay_counter)
            .field("pending_key_state", &self.pending_key_state)
            .field("pressed_inputs", &self.pressed_inputs)
            .finish_non_exhaustive()
    }
}

impl<
        Ctx: key::Context<Event = Ev>,
        Ev: Copy + Debug + PartialEq,
        PKS,
        KS: key::KeyState<Context = Ctx, Event = Ev> + Copy,
        K: key::Key<Context = Ctx, Event = Ev, PendingKeyState = PKS, KeyState = KS> + ?Sized,
        I: Index<usize, Output = K>,
    > Keymap<Ctx, Ev, PKS, KS, I>
{
    /// Constructs a new keymap with the given key definitions and context.
    pub const fn new(key_definitions: I, context: Ctx) -> Self {
        Self {
            key_definitions,
            context,
            pressed_inputs: heapless::Vec::new(),
            event_scheduler: EventScheduler::new(),
            hid_reporter: HIDKeyboardReporter::new(),
            pending_key_state: None,
            input_queue: heapless::spsc::Queue::new(),
            input_queue_delay_counter: 0,
            callbacks: heapless::LinearMap::new(),
        }
    }

    /// Initializes or resets the keyboard to an initial state.
    pub fn init(&mut self) {
        self.pressed_inputs.clear();
        self.event_scheduler.init();
        self.hid_reporter.init();
        self.pending_key_state = None;
        while !self.input_queue.is_empty() {
            self.input_queue.dequeue().unwrap();
        }
        self.input_queue_delay_counter = 0;
    }

    /// Registers the given callback to the keymap.
    ///
    /// Only one callback is set for each callback id.
    pub fn set_callback(&mut self, callback_id: KeymapCallback, callback_fn: fn() -> ()) {
        self.callbacks
            .insert(callback_id, CallbackFunction::Rust(callback_fn))
            .unwrap();
    }

    /// Registers the given callback to the keymap.
    ///
    /// Only one callback is set for each callback id.
    pub fn set_callback_extern(
        &mut self,
        callback_id: KeymapCallback,
        callback_fn: extern "C" fn() -> (),
    ) {
        self.callbacks
            .insert(callback_id, CallbackFunction::ExternC(callback_fn))
            .unwrap();
    }

    // If the pending key state is resolved,
    //  then clear the pending key state.
    fn resolve_pending_key_state(&mut self, key_state: KS) {
        if let Some(PendingState {
            key_path,
            queued_events,
            ..
        }) = self.pending_key_state.take()
        {
            // Cancel events which were scheduled for the (pending) key.
            let keymap_index = key_path[0];
            self.event_scheduler
                .cancel_events_for_keymap_index(keymap_index);

            // Add the pending state's pressed key to pressed inputs
            self.pressed_inputs
                .push(input::PressedInput::pressed_key(key_state, keymap_index))
                .unwrap();

            // Schedule each of the queued events,
            //  delaying each consecutive event by a tick
            //  (in order to allow press/release events to affect the HID report)
            let mut i = 1;
            let mut old_input_queue: heapless::spsc::Queue<input::Event, MAX_QUEUED_INPUT_EVENTS> =
                core::mem::take(&mut self.input_queue);
            for ev in queued_events {
                match ev {
                    key::Event::Input(ie) => {
                        self.input_queue.enqueue(ie).unwrap();
                    }
                    _ => {
                        self.event_scheduler.schedule_after(i, ev);
                        i += 1;
                    }
                }
            }

            while let Some(ie) = old_input_queue.dequeue() {
                self.input_queue.enqueue(ie).unwrap();
            }

            self.handle_pending_events();

            // The resolved key state has output. Emit this as an event.
            if let Some(key_state) = key_state.key_output() {
                let km_ev = KeymapEvent::ResolvedKeyOutput {
                    keymap_index,
                    key_output: key_state,
                };
                self.handle_event(key::Event::Keymap(km_ev));
            }
        }
    }

    /// Handles input events.
    pub fn handle_input(&mut self, ev: input::Event) {
        if self.input_queue.is_full() {
            return;
        }

        self.input_queue.enqueue(ev).unwrap();

        if self.input_queue_delay_counter == 0 {
            let ie = self.input_queue.dequeue().unwrap();
            self.process_input(ie);
            self.input_queue_delay_counter = INPUT_QUEUE_TICK_DELAY;
        }
    }

    fn process_input(&mut self, ev: input::Event) {
        if let Some(PendingState {
            key_path,
            pending_key_state,
            queued_events,
            ..
        }) = &mut self.pending_key_state
        {
            queued_events.push(ev.into()).unwrap();

            let pending_key = &self.key_definitions[key_path[0] as usize];
            let pending_key = pending_key.lookup(&key_path[1..]);
            let (ks, pke) = pending_key.handle_event(
                pending_key_state,
                self.context,
                key_path.clone(),
                ev.into(),
            );

            pke.into_iter()
                .for_each(|sch_ev| self.event_scheduler.schedule_event(sch_ev));

            match ks {
                Some(key::PressedKeyResult::Resolved(ks)) => {
                    self.resolve_pending_key_state(ks);
                }
                Some(key::PressedKeyResult::Pending(kp, pks)) => {
                    *key_path = kp;
                    *pending_key_state = pks;
                }
                None => {}
            }
        } else {
            // Update each of the pressed keys with the event.
            self.pressed_inputs.iter_mut().for_each(|pi| {
                if let input::PressedInput::Key(pressed_key) = pi {
                    pressed_key
                        .handle_event(self.context, ev.into())
                        .into_iter()
                        .for_each(|sch_ev| self.event_scheduler.schedule_event(sch_ev));
                }
            });

            self.context
                .handle_event(ev.into())
                .into_iter()
                .for_each(|sch_ev| self.event_scheduler.schedule_event(sch_ev));

            match ev {
                input::Event::Press { keymap_index } => {
                    let key = &self.key_definitions[keymap_index as usize];

                    let mut key_path = key::KeyPath::new();
                    key_path.push(keymap_index).unwrap();
                    let (pk, pke) = key.new_pressed_key(self.context, key_path);

                    pke.into_iter()
                        .for_each(|sch_ev| self.event_scheduler.schedule_event(sch_ev));

                    match pk {
                        key::PressedKeyResult::Resolved(key_state) => {
                            self.pressed_inputs
                                .push(input::PressedInput::pressed_key(key_state, keymap_index))
                                .unwrap();

                            // The resolved key state has output. Emit this as an event.
                            if let Some(key_output) = key_state.key_output() {
                                let km_ev = KeymapEvent::ResolvedKeyOutput {
                                    keymap_index,
                                    key_output,
                                };
                                self.handle_event(key::Event::Keymap(km_ev));
                            }
                        }
                        key::PressedKeyResult::Pending(key_path, pending_key_state) => {
                            self.pending_key_state = Some(PendingState {
                                key_path,
                                pending_key_state,
                                queued_events: heapless::Vec::new(),
                            });
                        }
                    }
                }
                input::Event::Release { keymap_index } => {
                    self.pressed_inputs
                        .iter()
                        .position(|pi| match pi {
                            &input::PressedInput::Key(input::PressedKey {
                                keymap_index: ki,
                                ..
                            }) => keymap_index == ki,
                            _ => false,
                        })
                        .map(|i| self.pressed_inputs.remove(i));

                    self.event_scheduler
                        .cancel_events_for_keymap_index(keymap_index);
                }

                input::Event::VirtualKeyPress { key_output } => {
                    let pressed_key = input::PressedInput::Virtual(key_output);
                    self.pressed_inputs.push(pressed_key).unwrap();
                }
                input::Event::VirtualKeyRelease { key_output } => {
                    // Remove from pressed keys.
                    self.pressed_inputs
                        .iter()
                        .position(|k| match k {
                            input::PressedInput::Virtual(ko) => key_output == *ko,
                            _ => false,
                        })
                        .map(|i| self.pressed_inputs.remove(i));
                }
            }
        }

        self.handle_pending_events();
    }

    // Called from handle_all_pending_events,
    //  and for handling the (resolving) queue of events from pending key state.
    fn handle_event(&mut self, ev: key::Event<Ev>) {
        if let key::Event::Keymap(KeymapEvent::Callback(callback_id)) = ev {
            match self.callbacks.get(&callback_id) {
                Some(CallbackFunction::Rust(callback_fn)) => {
                    callback_fn();
                }
                Some(CallbackFunction::ExternC(callback_fn)) => {
                    callback_fn();
                }
                None => {}
            }
        }

        // pending state needs to handle events
        if let Some(PendingState {
            key_path,
            pending_key_state,
            ..
        }) = &mut self.pending_key_state
        {
            let pending_key = &self.key_definitions[key_path[0] as usize];
            let pending_key = pending_key.lookup(&key_path[1..]);
            let (ks, pke) =
                pending_key.handle_event(pending_key_state, self.context, key_path.clone(), ev);

            pke.into_iter()
                .for_each(|sch_ev| self.event_scheduler.schedule_event(sch_ev));

            match ks {
                Some(key::PressedKeyResult::Resolved(ks)) => {
                    self.resolve_pending_key_state(ks);
                }
                Some(key::PressedKeyResult::Pending(kp, pks)) => {
                    *key_path = kp;
                    *pending_key_state = pks;
                }
                None => {}
            }
        }

        // Update each of the pressed keys with the event.
        self.pressed_inputs.iter_mut().for_each(|pi| {
            if let input::PressedInput::Key(input::PressedKey {
                key_state,
                keymap_index,
            }) = pi
            {
                key_state
                    .handle_event(self.context, *keymap_index, ev)
                    .into_iter()
                    .for_each(|sch_ev| self.event_scheduler.schedule_event(sch_ev));
            }
        });

        // Update context with the event
        self.context
            .handle_event(ev)
            .into_iter()
            .for_each(|sch_ev| self.event_scheduler.schedule_event(sch_ev));

        if let Event::Input(input_ev) = ev {
            self.process_input(input_ev);
        }
    }

    fn handle_pending_events(&mut self) {
        // take from pending
        while let Some(ev) = self.event_scheduler.dequeue() {
            self.handle_event(ev);
        }
    }

    /// Advances the state of the keymap by one tick.
    pub fn tick(&mut self) {
        if !self.input_queue.is_empty() && self.input_queue_delay_counter == 0 {
            let ie = self.input_queue.dequeue().unwrap();
            self.process_input(ie);
            self.input_queue_delay_counter = INPUT_QUEUE_TICK_DELAY;
        }

        if self.input_queue_delay_counter > 0 {
            self.input_queue_delay_counter -= 1;
        }

        self.event_scheduler.tick();

        self.handle_pending_events();
    }

    /// Returns the the pressed key outputs.
    pub fn pressed_keys(&self) -> heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> {
        let pressed_key_codes = self.pressed_inputs.iter().filter_map(|pi| match pi {
            input::PressedInput::Key(pressed_key) => pressed_key.key_output(),
            &input::PressedInput::Virtual(key_output) => Some(key_output),
        });

        pressed_key_codes.collect()
    }

    /// Updates the keymap indicating a report is sent; returns the reportable keymap output.
    pub fn report_output(&mut self) -> KeymapOutput {
        self.hid_reporter.update(self.pressed_keys());
        self.hid_reporter.report_sent();

        KeymapOutput::new(self.hid_reporter.reportable_key_outputs())
    }

    /// Returns the current HID keyboard report.
    #[doc(hidden)]
    pub fn boot_keyboard_report(&self) -> [u8; 8] {
        KeymapOutput::new(self.pressed_keys()).as_hid_boot_keyboard_report()
    }

    #[doc(hidden)]
    pub fn has_scheduled_events(&self) -> bool {
        !self.event_scheduler.pending_events.is_empty()
            || !self.event_scheduler.scheduled_events.is_empty()
            || !self.input_queue.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::tuples;

    #[test]
    fn test_keymap_output_pressed_key_codes_includes_modifier_key_code() {
        // Assemble - include modifier key left ctrl
        let mut input: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = heapless::Vec::new();
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
        let mut input: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = heapless::Vec::new();
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
        let mut input: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = heapless::Vec::new();
        input.push(key::KeyOutput::from_key_code(0x04)).unwrap();

        let mut reporter = HIDKeyboardReporter::new();

        // Act
        reporter.update(input);
        let actual_outputs = reporter.reportable_key_outputs();

        // Assert
        let expected_outputs: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = [0x04]
            .iter()
            .map(|&kc| key::KeyOutput::from_key_code(kc))
            .collect();
        assert_eq!(expected_outputs, actual_outputs);
    }

    #[test]
    fn test_hid_keyboard_reporter_reports_single_new_keypress_per_report_sent() {
        // Assemble
        let input: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = [0x04, 0x05]
            .iter()
            .map(|&kc| key::KeyOutput::from_key_code(kc))
            .collect();

        let mut reporter = HIDKeyboardReporter::new();

        // Act
        reporter.update(input);
        let actual_outputs = reporter.reportable_key_outputs();

        // Assert
        let expected_outputs: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = [0x04]
            .iter()
            .map(|&kc| key::KeyOutput::from_key_code(kc))
            .collect();
        assert_eq!(expected_outputs, actual_outputs);
    }

    #[test]
    fn test_hid_keyboard_reporter_reports_more_keypresses_after_report_sent() {
        // Assemble
        let input: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = [0x04, 0x05]
            .iter()
            .map(|&kc| key::KeyOutput::from_key_code(kc))
            .collect();

        let mut reporter = HIDKeyboardReporter::new();

        // Act
        reporter.update(input);
        reporter.report_sent();
        let actual_outputs = reporter.reportable_key_outputs();

        // Assert
        let expected_outputs: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = [0x04, 0x05]
            .iter()
            .map(|&kc| key::KeyOutput::from_key_code(kc))
            .collect();
        assert_eq!(expected_outputs, actual_outputs);
    }

    #[test]
    fn test_hid_keyboard_reporter_reports_updates_for_key_releases() {
        // Assemble
        let input: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = [0x04, 0x05]
            .iter()
            .map(|&kc| key::KeyOutput::from_key_code(kc))
            .collect();
        let input_after_key_released: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = [0x05]
            .iter()
            .map(|&kc| key::KeyOutput::from_key_code(kc))
            .collect();
        let input_after_more_keys_pressed: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> =
            [0x05, 0x06, 0x07]
                .iter()
                .map(|&kc| key::KeyOutput::from_key_code(kc))
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
        let expected_outputs: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = [0x05, 0x06]
            .iter()
            .map(|&kc| key::KeyOutput::from_key_code(kc))
            .collect();
        assert_eq!(
            KeymapOutput::new(expected_outputs).as_hid_boot_keyboard_report(),
            KeymapOutput::new(actual_outputs).as_hid_boot_keyboard_report(),
        );
    }

    #[test]
    fn test_keymap_with_keyboard_key_with_composite_context() {
        use key::composite;
        use key::keyboard;
        use tuples::Keys1;

        use composite::{Context, Event, KeyState, PendingKeyState};

        // Assemble
        type Ctx = Context;
        type K = composite::Chorded<composite::Layered<composite::TapHold<keyboard::Key>>>;
        let keys: Keys1<K, Context, Event, PendingKeyState, KeyState> =
            Keys1::new((composite::Chorded(composite::Layered(composite::TapHold(
                keyboard::Key::new(0x04),
            ))),));
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

        use composite::{Context, Event, KeyState, PendingKeyState};

        // Assemble
        let keys: Keys1<composite::Key, Context, Event, PendingKeyState, KeyState> =
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

    #[test]
    fn test_keymap_many_input_events_without_tick_or_report() {
        use key::{composite, keyboard};
        use tuples::Keys1;

        use composite::{Context, Event, KeyState, PendingKeyState};

        // Assemble
        let keys: Keys1<composite::Key, Context, Event, PendingKeyState, KeyState> =
            Keys1::new((composite::Key::keyboard(keyboard::Key::new(0x04)),));
        let context: Context = composite::DEFAULT_CONTEXT;
        let mut keymap = Keymap::new(keys, context);

        // Act
        for _ in 0..100 {
            keymap.handle_input(input::Event::Press { keymap_index: 0 });
            keymap.handle_input(input::Event::Release { keymap_index: 0 });
        }

        // Assert
        // (expect no panics)
    }
}
