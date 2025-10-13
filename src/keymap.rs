#[cfg(feature = "std")]
mod distinct_reports;
mod event_scheduler;
/// The HID keyboard reporter.
pub mod hid_keyboard_reporter;
#[cfg(feature = "std")]
mod observed_eb_keymap;
#[cfg(feature = "std")]
mod observed_keymap;

use core::cmp::PartialEq;
use core::fmt::Debug;
use core::marker::Copy;
use core::ops::Index;

use serde::Deserialize;

use crate::input;
use crate::key;

use key::Event;

#[cfg(feature = "std")]
pub use distinct_reports::DistinctReports;
use event_scheduler::EventScheduler;
use hid_keyboard_reporter::HIDKeyboardReporter;
#[cfg(feature = "std")]
pub use observed_eb_keymap::ObservedKeymap as ObservedEventBasedKeymap;
#[cfg(feature = "std")]
pub use observed_keymap::ObservedKeymap;

/// Maximum number of pressed keys supported.
pub const MAX_PRESSED_KEYS: usize = 16;

const MAX_QUEUED_INPUT_EVENTS: usize = 32;

/// Number of ticks before the next input event is processed in tick().
pub const INPUT_QUEUE_TICK_DELAY: u8 = 1;

/// Constructs an HID report or a sequence of key codes from the given sequence of [key::KeyOutput].
#[derive(Debug, PartialEq)]
pub struct KeymapOutput {
    pressed_key_codes: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }>,
}

impl Default for KeymapOutput {
    fn default() -> Self {
        Self {
            pressed_key_codes: heapless::Vec::new(),
        }
    }
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

        result.extend(
            self.pressed_key_codes
                .iter()
                .flat_map(|ko| match ko.key_code() {
                    key::KeyUsage::Keyboard(kc) => Some(kc),
                    _ => None,
                }),
        );

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
            .flat_map(|ko| match ko.key_code() {
                key::KeyUsage::Keyboard(kc) => Some(kc),
                _ => None,
            })
            .filter(|&kc| kc != 0);

        for (i, key_code) in key_codes.take(6).enumerate() {
            report[i + 2] = key_code;
        }

        report
    }

    /// Returns the pressed consumer codes.
    pub fn pressed_consumer_codes(&self) -> heapless::Vec<u8, 24> {
        self.pressed_key_codes
            .iter()
            .flat_map(|ko| match ko.key_code() {
                key::KeyUsage::Consumer(uc) => Some(uc),
                _ => None,
            })
            .collect()
    }

    /// Returns the pressed custom codes.
    pub fn pressed_custom_codes(&self) -> heapless::Vec<u8, 24> {
        self.pressed_key_codes
            .iter()
            .flat_map(|ko| match ko.key_code() {
                key::KeyUsage::Custom(kc) => Some(kc),
                _ => None,
            })
            .collect()
    }

    /// Returns the combined pressed mouse output.
    pub fn pressed_mouse_output(&self) -> key::MouseOutput {
        self.pressed_key_codes
            .iter()
            .filter_map(|ko| match ko.key_code() {
                key::KeyUsage::Mouse(mo) => Some(mo),
                _ => None,
            })
            .fold(key::MouseOutput::NO_OUTPUT, |acc, mo| acc.combine(&mo))
    }
}

#[derive(Debug)]
struct PendingState<R, Ev, PKS> {
    keymap_index: u16,
    key_ref: R,
    pending_key_state: PKS,
    queued_events: heapless::Vec<key::Event<Ev>, { MAX_PRESSED_KEYS }>,
}

/// Commands for managing Bluetooth profiles. (BLE pairing and bonding).
#[derive(Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum BluetoothProfileCommand {
    /// Disconnect the current profile.
    Disconnect,
    /// Clear the current profile. (Start pairing mode).
    Clear,
    /// Clear all profiles. (Start pairing mode).
    ClearAll,
    /// Switch to the previous profile.
    Previous,
    /// Switch to the next profile.
    Next,
    /// Switch to the given profile index.
    Select(u8),
}

/// Callbacks for effect keys in the keymap.
#[derive(Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum KeymapCallback {
    /// Reset the keyboard
    Reset,
    /// Reset the keyboard to bootloader
    ResetToBootloader,
    /// Reset the keyboard to bootloader
    Bluetooth(BluetoothProfileCommand),
    /// A custom callback. Its behaviour is specific to the firmware implementation.
    Custom(u8, u8),
}

/// Context provided from the keymap to the smart keys.
#[derive(Debug, Clone, Copy)]
pub struct KeymapContext {
    /// Number of milliseconds since keymap has been initialized.
    pub time_ms: u32,

    /// Number of milliseconds since keymap received an input event.
    pub idle_time_ms: u32,
}

impl KeymapContext {
    /// Constructs a new default keymap context.
    pub const fn new() -> Self {
        KeymapContext {
            time_ms: 0,
            idle_time_ms: 0,
        }
    }
}

/// Trait for setting the keymap context.
pub trait SetKeymapContext {
    /// Sets the keymap context.
    fn set_keymap_context(&mut self, context: KeymapContext);
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
pub struct Keymap<I: Index<usize, Output = R>, R, Ctx, Ev: Debug, PKS, KS, S> {
    key_refs: I,
    key_system: S,
    context: Ctx,
    pressed_inputs: heapless::Vec<input::PressedInput<R, KS>, { MAX_PRESSED_KEYS }>,
    event_scheduler: EventScheduler<Ev>,
    ms_per_tick: u8,
    idle_time: u32,
    hid_reporter: HIDKeyboardReporter,
    pending_key_state: Option<PendingState<R, Ev, PKS>>,
    input_queue: heapless::spsc::Queue<input::Event, { MAX_QUEUED_INPUT_EVENTS }>,
    input_queue_delay_counter: u8,
    callbacks: heapless::LinearMap<KeymapCallback, CallbackFunction, 2>,
}

impl<
        I: Debug + Index<usize, Output = R>,
        R: Debug,
        Ctx: Debug,
        Ev: Debug,
        PKS: Debug,
        KS: Debug,
        S: Debug,
    > core::fmt::Debug for Keymap<I, R, Ctx, Ev, PKS, KS, S>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Keymap")
            .field("context", &self.context)
            .field("event_scheduler", &self.event_scheduler)
            .field("ms_per_tick", &self.ms_per_tick)
            .field("idle_time", &self.idle_time)
            .field("hid_reporter", &self.hid_reporter)
            .field("input_queue", &self.input_queue)
            .field("input_queue_delay_counter", &self.input_queue_delay_counter)
            .field("pending_key_state", &self.pending_key_state)
            .field("pressed_inputs", &self.pressed_inputs)
            .finish_non_exhaustive()
    }
}

impl<
        I: Debug + Index<usize, Output = R>,
        R: Copy + Debug,
        Ctx: Debug + key::Context<Event = Ev> + SetKeymapContext,
        Ev: Copy + Debug,
        PKS: Debug,
        KS: Copy + Debug + From<key::NoOpKeyState>,
        S: key::System<R, Ref = R, Context = Ctx, Event = Ev, PendingKeyState = PKS, KeyState = KS>,
    > Keymap<I, R, Ctx, Ev, PKS, KS, S>
{
    /// Constructs a new keymap with the given key definitions and context.
    pub const fn new(key_refs: I, context: Ctx, key_system: S) -> Self {
        Self {
            key_refs,
            key_system,
            context,
            pressed_inputs: heapless::Vec::new(),
            event_scheduler: EventScheduler::new(),
            ms_per_tick: 1,
            idle_time: 0,
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
        self.ms_per_tick = 1;
        self.idle_time = 0;
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

    /// Sets the number of ms per tick().
    pub fn set_ms_per_tick(&mut self, ms_per_tick: u8) {
        self.ms_per_tick = ms_per_tick;
    }

    // If the pending key state is resolved,
    //  then clear the pending key state.
    fn resolve_pending_key_state(&mut self, key_state: KS) {
        if let Some(PendingState {
            keymap_index,
            key_ref,
            queued_events,
            ..
        }) = self.pending_key_state.take()
        {
            // Cancel events which were scheduled for the (pending) key.
            self.event_scheduler
                .cancel_events_for_keymap_index(keymap_index);

            // Add the pending state's pressed key to pressed inputs
            self.pressed_inputs
                .push(input::PressedInput::pressed_key(
                    keymap_index,
                    key_ref,
                    key_state,
                ))
                .unwrap();

            // Schedule each of the queued events,
            //  delaying each consecutive event by a tick
            //  (in order to allow press/release events to affect the HID report)
            let mut i = 1;
            let mut old_input_queue: heapless::spsc::Queue<input::Event, MAX_QUEUED_INPUT_EVENTS> =
                core::mem::take(&mut self.input_queue);

            // Partition the events from the pending keymap index
            //  separately from the other queued events.
            // (Only queue the *last* event from the pending keymap index).
            let (pending_input_ev, queued_events): (
                heapless::Vec<key::Event<Ev>, { MAX_PRESSED_KEYS }>,
                heapless::Vec<key::Event<Ev>, { MAX_PRESSED_KEYS }>,
            ) = queued_events.iter().partition(|ev| match ev {
                key::Event::Input(input::Event::Press {
                    keymap_index: queued_kmi,
                }) => *queued_kmi == keymap_index,
                key::Event::Input(input::Event::Release {
                    keymap_index: queued_kmi,
                }) => *queued_kmi == keymap_index,
                key::Event::Key {
                    keymap_index: queued_kmi,
                    ..
                } => *queued_kmi == keymap_index,
                _ => false,
            });

            for ev in queued_events.iter().chain(pending_input_ev.last()) {
                match ev {
                    key::Event::Input(ie) => {
                        self.input_queue.enqueue(*ie).unwrap();
                    }
                    _ => {
                        self.event_scheduler.schedule_after(i, *ev);
                        i += 1;
                    }
                }
            }

            while let Some(ie) = old_input_queue.dequeue() {
                self.input_queue.enqueue(ie).unwrap();
            }

            self.handle_pending_events();

            // The resolved key state has output. Emit this as an event.
            if let Some(key_output) = self.key_system.key_output(&key_ref, &key_state) {
                let km_ev = KeymapEvent::ResolvedKeyOutput {
                    keymap_index,
                    key_output,
                };
                self.handle_event(key::Event::Keymap(km_ev));
            }
        }
    }

    /// Handles input events.
    ///
    /// Discards the input event if the input queue is full.
    pub fn handle_input(&mut self, ev: input::Event) {
        self.idle_time = 0;

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

    fn has_pressed_input_with_keymap_index(&self, keymap_index: u16) -> bool {
        self.pressed_inputs.iter().any(|pi| match pi {
            &input::PressedInput::Key(input::PressedKey {
                keymap_index: ki, ..
            }) => keymap_index == ki,
            _ => false,
        })
    }

    fn update_pending_state(&mut self, ev: key::Event<Ev>) {
        if let Some(PendingState {
            keymap_index,
            key_ref,
            pending_key_state,
            queued_events,
            ..
        }) = &mut self.pending_key_state
        {
            let (mut maybe_npk, pke) = self.key_system.update_pending_state(
                pending_key_state,
                *keymap_index,
                &self.context,
                *key_ref,
                ev,
            );

            pke.into_iter()
                .for_each(|sch_ev| self.event_scheduler.schedule_event(sch_ev));

            while let Some(npk) = maybe_npk.take() {
                let pkr = match npk {
                    key::NewPressedKey::Key(new_key_ref) => {
                        *key_ref = new_key_ref;
                        let (pkr, pke) = self.key_system.new_pressed_key(
                            *keymap_index,
                            &self.context,
                            new_key_ref,
                        );
                        pke.into_iter()
                            .for_each(|sch_ev| self.event_scheduler.schedule_event(sch_ev));
                        pkr
                    }
                    key::NewPressedKey::NoOp => {
                        let no_op_ks: KS = key::NoOpKeyState.into();
                        key::PressedKeyResult::Resolved(no_op_ks)
                    }
                };

                match pkr {
                    key::PressedKeyResult::Resolved(ks) => {
                        self.resolve_pending_key_state(ks);
                        break;
                    }
                    key::PressedKeyResult::NewPressedKey(key::NewPressedKey::Key(new_key_ref)) => {
                        maybe_npk = Some(key::NewPressedKey::Key(new_key_ref));
                    }
                    key::PressedKeyResult::NewPressedKey(key::NewPressedKey::NoOp) => {
                        self.resolve_pending_key_state(key::NoOpKeyState.into());
                        break;
                    }
                    key::PressedKeyResult::Pending(pks) => {
                        *pending_key_state = pks;

                        // Since the pending key state resolved into another pending key state,
                        //  we re-queue all the input events that had been received.
                        let orig_input_queue = core::mem::take(&mut self.input_queue);
                        while let Some(ev) = queued_events.pop() {
                            if let key::Event::Input(input_ev) = ev {
                                self.input_queue.enqueue(input_ev).unwrap();
                            }
                        }
                        orig_input_queue.iter().for_each(|&ev| {
                            self.input_queue.enqueue(ev).unwrap();
                        });
                    }
                }
            }
        }
    }

    fn process_input(&mut self, ev: input::Event) {
        if let Some(pending_state) = &mut self.pending_key_state {
            pending_state.queued_events.push(ev.into()).unwrap();
            self.update_pending_state(ev.into());
        } else {
            // Update each of the pressed keys with the event.
            self.pressed_inputs.iter_mut().for_each(|pi| {
                if let input::PressedInput::Key(input::PressedKey {
                    key_ref,
                    key_state,
                    keymap_index,
                }) = pi
                {
                    self.key_system
                        .update_state(key_state, key_ref, &self.context, *keymap_index, ev.into())
                        .into_iter()
                        .for_each(|sch_ev| self.event_scheduler.schedule_event(sch_ev));
                }
            });

            self.context
                .handle_event(ev.into())
                .into_iter()
                .for_each(|sch_ev| self.event_scheduler.schedule_event(sch_ev));

            match ev {
                input::Event::Press { keymap_index }
                    if !self.has_pressed_input_with_keymap_index(keymap_index) =>
                {
                    let mut maybe_key_ref = Some(self.key_refs[keymap_index as usize]);

                    while let Some(key_ref) = maybe_key_ref.take() {
                        let (pkr, pke) =
                            self.key_system
                                .new_pressed_key(keymap_index, &self.context, key_ref);

                        pke.into_iter()
                            .for_each(|sch_ev| self.event_scheduler.schedule_event(sch_ev));

                        match pkr {
                            key::PressedKeyResult::Resolved(key_state) => {
                                self.pressed_inputs
                                    .push(input::PressedInput::pressed_key(
                                        keymap_index,
                                        key_ref,
                                        key_state,
                                    ))
                                    .unwrap();

                                // The resolved key state has output. Emit this as an event.
                                if let Some(key_output) =
                                    self.key_system.key_output(&key_ref, &key_state)
                                {
                                    let km_ev = KeymapEvent::ResolvedKeyOutput {
                                        keymap_index,
                                        key_output,
                                    };
                                    self.handle_event(key::Event::Keymap(km_ev));
                                }
                            }
                            key::PressedKeyResult::NewPressedKey(key::NewPressedKey::Key(
                                new_key_ref,
                            )) => {
                                maybe_key_ref = Some(new_key_ref);
                            }
                            key::PressedKeyResult::NewPressedKey(key::NewPressedKey::NoOp) => {
                                let key_state: KS = key::NoOpKeyState.into();

                                self.pressed_inputs
                                    .push(input::PressedInput::pressed_key(
                                        keymap_index,
                                        key_ref,
                                        key_state,
                                    ))
                                    .unwrap();
                            }
                            key::PressedKeyResult::Pending(pending_key_state) => {
                                self.pending_key_state = Some(PendingState {
                                    keymap_index,
                                    key_ref,
                                    pending_key_state,
                                    queued_events: heapless::Vec::new(),
                                });
                            }
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

                _ => {}
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
        self.update_pending_state(ev);

        // Update each of the pressed keys with the event.
        self.pressed_inputs.iter_mut().for_each(|pi| {
            if let input::PressedInput::Key(input::PressedKey {
                key_state,
                key_ref,
                keymap_index,
            }) = pi
            {
                self.key_system
                    .update_state(key_state, key_ref, &self.context, *keymap_index, ev)
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
        let km_context = KeymapContext {
            time_ms: self.event_scheduler.schedule_counter,
            idle_time_ms: self.idle_time,
        };
        self.context.set_keymap_context(km_context);

        if !self.input_queue.is_empty() && self.input_queue_delay_counter == 0 {
            let ie = self.input_queue.dequeue().unwrap();
            self.process_input(ie);
            self.input_queue_delay_counter = INPUT_QUEUE_TICK_DELAY;
        }

        if self.input_queue_delay_counter > 0 {
            self.input_queue_delay_counter -= 1;
        }

        self.event_scheduler.tick(self.ms_per_tick);

        self.handle_pending_events();

        self.idle_time += self.ms_per_tick as u32;
    }

    /// Returns the the pressed key outputs.
    pub fn pressed_keys(&self) -> heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> {
        let pressed_key_codes = self.pressed_inputs.iter().filter_map(|pi| match pi {
            input::PressedInput::Key(input::PressedKey {
                key_ref, key_state, ..
            }) => self.key_system.key_output(key_ref, key_state),
            &input::PressedInput::Virtual(key_output) => Some(key_output),
        });

        pressed_key_codes.collect()
    }

    fn tick_by(&mut self, delta_ms: u32) {
        if delta_ms == 0 {
            self.tick();
        } else {
            for _ in 0..(delta_ms / self.ms_per_tick as u32) {
                self.tick();
            }
        }
    }

    /// Handles input events.
    ///
    /// Discards the input event if the input queue is full.
    ///
    /// Returns the time in ms until the next scheduled event, if any.
    ///  (Time until next tick, if any, will always be >0, so 0 can be used as "NO EVENTS")
    pub fn handle_input_after_time(&mut self, delta_ms: u32, ev: input::Event) -> Option<u32> {
        self.tick_by(delta_ms);
        self.handle_input(ev);
        let next_event_time = self.event_scheduler.next_event_time();
        debug_assert!(next_event_time != Some(0));
        next_event_time
    }

    /// If the event scheduler has a next scheduled event,
    ///  it ticks the keymap forward to that event,
    ///  returning the time in ms until the following event.
    ///
    /// Otherwise, does nothing and returns None.
    pub fn tick_to_next_scheduled_event(&mut self) -> Option<u32> {
        if let Some(delta_ms) = self.event_scheduler.next_event_time() {
            self.tick_by(delta_ms);
            self.event_scheduler.next_event_time()
        } else {
            None
        }
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
    fn test_keymap_many_input_events_without_tick_or_report() {
        // Assemble
        let mut keymap = {
            use crate as smart_keymap;
            use smart_keymap::key::composite as key_system;

            use key_system::Context;
            use key_system::Ref;
            const KEY_COUNT: usize = 1;
            const KEY_REFS: [Ref; KEY_COUNT] = [smart_keymap::key::composite::Ref::Keyboard(
                smart_keymap::key::keyboard::Ref::KeyCode(0x04),
            )];
            const CONTEXT: Context = Context::from_config(key_system::Config::new());

            smart_keymap::keymap::Keymap::new(
                KEY_REFS,
                CONTEXT,
                smart_keymap::key::composite::System::array_based(
                    smart_keymap::key::automation::System::new([]),
                    smart_keymap::key::callback::System::new([]),
                    smart_keymap::key::chorded::System::new([], []),
                    smart_keymap::key::keyboard::System::new([]),
                    smart_keymap::key::layered::System::new([], []),
                    smart_keymap::key::sticky::System::new([]),
                    smart_keymap::key::tap_dance::System::new([]),
                    smart_keymap::key::tap_hold::System::new([]),
                ),
            )
        };

        // Act
        for _ in 0..100 {
            keymap.handle_input(input::Event::Press { keymap_index: 0 });
            keymap.handle_input(input::Event::Release { keymap_index: 0 });
        }

        // Assert
        // (expect no panics)
    }
}
