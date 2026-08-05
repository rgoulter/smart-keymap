#[cfg(feature = "std")]
mod distinct_reports;
mod event_scheduler;
/// The HID keyboard reporter.
pub mod hid_keyboard_reporter;
mod input_event_queue;
#[cfg(feature = "std")]
mod observed_eb_keymap;
#[cfg(feature = "std")]
mod observed_keymap;
mod pending;

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
use input_event_queue::InputEventQueue;
#[cfg(feature = "std")]
pub use observed_eb_keymap::ObservedKeymap as ObservedEventBasedKeymap;
#[cfg(feature = "std")]
pub use observed_keymap::ObservedKeymap;

/// Maximum number of pressed keys supported.
pub const MAX_PRESSED_KEYS: usize = 16;

pub(crate) const MAX_QUEUED_INPUT_EVENTS: usize = 32;

/// Constructs an HID report or a sequence of key codes from the given sequence of [key::KeyOutput].
#[derive(Debug, Default, PartialEq)]
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

/// Max recent physical presses tracked in [KeymapContext] (for quick-tap, etc.).
pub const MAX_RECENT_PRESSES: usize = 8;

/// Context provided from the keymap to the smart keys.
#[derive(Debug, Clone, Copy, Default)]
pub struct KeymapContext {
    /// Number of milliseconds since keymap has been initialized.
    pub time_ms: u32,

    /// Number of milliseconds since keymap received an input event.
    pub idle_time_ms: u32,

    /// Aggregate keyboard modifiers from already-pressed inputs (physical + virtual).
    ///
    /// Updated by the keymap before press handling and on tick
    ///  so families can branch without maintaining their own mod-tracking.
    pub pressed_modifiers: key::KeyboardModifiers,

    /// Recent physical presses as `(keymap_index, time_ms)`, oldest first.
    ///
    /// Used by features such as tap-hold `quick_tap_ms` (ZMK-style re-tap).
    pub recent_presses: [(u16, u32); MAX_RECENT_PRESSES],

    /// Number of valid entries in [Self::recent_presses].
    pub recent_press_count: u8,
}

impl KeymapContext {
    /// Constructs a new default keymap context.
    pub const fn new() -> Self {
        KeymapContext {
            time_ms: 0,
            idle_time_ms: 0,
            pressed_modifiers: key::KeyboardModifiers::NONE,
            recent_presses: [(0, 0); MAX_RECENT_PRESSES],
            recent_press_count: 0,
        }
    }

    /// Most recent press time for `keymap_index`, if still in the ring.
    pub fn last_press_time_ms(&self, keymap_index: u16) -> Option<u32> {
        self.recent_presses[..self.recent_press_count as usize]
            .iter()
            .rev()
            .find(|(ki, _)| *ki == keymap_index)
            .map(|(_, t)| *t)
    }
}

/// Trait for setting the keymap context.
pub trait SetKeymapContext {
    /// Sets the keymap context.
    fn set_keymap_context(&mut self, context: KeymapContext);
}

/// Report-level policy hints from aggregate context (feature-agnostic).
///
/// Families that need to hide held modifiers from the host
///  (e.g. mod-conditioned morphs) expose a suppress mask;
///  the keymap applies it when folding [`Keymap::pressed_keys`].
pub trait ReportHints {
    /// Modifier bits to strip from the HID report.
    fn suppressed_modifiers(&self) -> key::KeyboardModifiers {
        key::KeyboardModifiers::NONE
    }
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
    /// Ring of recent physical presses for [KeymapContext::recent_presses].
    recent_presses: [(u16, u32); MAX_RECENT_PRESSES],
    recent_press_count: u8,
    hid_reporter: HIDKeyboardReporter,
    pending_state: Option<pending::PendingState<R, Ev, PKS>>,
    input_queue: InputEventQueue<{ MAX_QUEUED_INPUT_EVENTS }>,
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
            .field("pending_state", &self.pending_state)
            .field("pressed_inputs", &self.pressed_inputs)
            .finish_non_exhaustive()
    }
}

impl<
        I: Debug + Index<usize, Output = R>,
        R: Copy + Debug,
        Ctx: Debug + key::Context<Event = Ev> + SetKeymapContext + ReportHints,
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
            recent_presses: [(0, 0); MAX_RECENT_PRESSES],
            recent_press_count: 0,
            hid_reporter: HIDKeyboardReporter::new(),
            pending_state: None,
            input_queue: InputEventQueue::new(),
            callbacks: heapless::LinearMap::new(),
        }
    }

    /// Initializes or resets the keyboard to an initial state.
    ///
    /// Resets [key::Context] from each family's config (clearing active layers and
    /// other runtime state while keeping that keymap's config), and clears pressed
    /// keys, pending work, and HID report state.
    pub fn init(&mut self) {
        self.context.reset();
        self.pressed_inputs.clear();
        self.event_scheduler.init();
        self.hid_reporter.init();
        self.pending_state = None;
        self.input_queue.clear();
        self.ms_per_tick = 1;
        self.idle_time = 0;
        self.recent_presses = [(0, 0); MAX_RECENT_PRESSES];
        self.recent_press_count = 0;
    }

    /// Record a physical press in the recent-press ring (for quick-tap, etc.).
    fn record_recent_press(&mut self, keymap_index: u16) {
        let time_ms = self.event_scheduler.schedule_counter;
        // Drop any prior entry for this index so a re-press updates the time.
        let mut write = 0usize;
        let count = self.recent_press_count as usize;
        for read in 0..count {
            if self.recent_presses[read].0 != keymap_index {
                self.recent_presses[write] = self.recent_presses[read];
                write += 1;
            }
        }
        if write == MAX_RECENT_PRESSES {
            // Evict oldest.
            for i in 0..(MAX_RECENT_PRESSES - 1) {
                self.recent_presses[i] = self.recent_presses[i + 1];
            }
            write = MAX_RECENT_PRESSES - 1;
        }
        self.recent_presses[write] = (keymap_index, time_ms);
        self.recent_press_count = (write + 1) as u8;
    }

    /// Clears all registered callbacks.
    pub fn clear_callbacks(&mut self) {
        self.callbacks.clear();
    }

    /// Registers the given callback to the keymap.
    ///
    /// Only one callback is set for each callback id.
    pub fn set_callback(&mut self, callback_id: KeymapCallback, callback_fn: fn() -> ()) {
        let _ = self
            .callbacks
            .insert(callback_id, CallbackFunction::Rust(callback_fn));
    }

    /// Registers the given callback to the keymap.
    ///
    /// Only one callback is set for each callback id.
    pub fn set_callback_extern(
        &mut self,
        callback_id: KeymapCallback,
        callback_fn: extern "C" fn() -> (),
    ) {
        let _ = self
            .callbacks
            .insert(callback_id, CallbackFunction::ExternC(callback_fn));
    }

    /// Sets the number of ms per tick().
    pub fn set_ms_per_tick(&mut self, ms_per_tick: u8) {
        self.ms_per_tick = ms_per_tick;
    }

    // If the pending key state is resolved,
    //  then clear the pending key state.
    //
    // Replay uses only `queued_events` (the session log).
    // Inputs still waiting in the pending `ingest_queue` (the delay line)
    //  are intentionally omitted from replay -
    //  they were not yet paced/applied during pending -
    //  and are transferred to the global `input_queue` tail
    //  to run post-resolve in normal order.
    fn resolve_pending_key_state(&mut self, key_state: KS) {
        if let Some(pending::PendingState {
            keymap_index,
            key_ref,
            mut queued_events,
            mut ingest_queue,
            ..
        }) = self.pending_state.take()
        {
            // Cancel events which were scheduled for the (pending) key.
            self.event_scheduler
                .cancel_events_for_keymap_index(keymap_index);

            // Add the pending state's pressed key to pressed inputs
            let _ = self.pressed_inputs.push(input::PressedInput::pressed_key(
                keymap_index,
                key_ref,
                key_state,
            ));

            // Session-log replay is prepended onto the global queue so it
            //  runs before any never-logged delay-line inputs transferred next.
            pending::dispatch_replayed_events(
                pending::KeyResolution::Resolved { keymap_index },
                &mut queued_events,
                &mut self.input_queue,
                &mut self.event_scheduler,
            );

            // Transfer remaining pending delay-line traffic to the global tail.
            let mut remaining = ingest_queue.take_all();
            self.input_queue.append_all(&mut remaining);

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
    /// Physical inputs enter a delay line first so at most one is processed
    ///  per tick (one-tick pacing gate), including while a key is pending.
    /// While pending, that delay line is the pending session's `ingest_queue`;
    ///  otherwise it is the global `input_queue`.
    /// Tap-hold and chorded interrupt logic depend on that spacing
    ///  (`tests/rust/tap_hold/hold_on_interrupt_tap.rs`).
    ///
    /// Silently discards the input event if the active input queue is full.
    pub fn handle_input(&mut self, ev: input::Event) {
        let ready = if let Some(pending_state) = self.pending_state.as_mut() {
            pending_state.ingest_queue.push_back_or_ignore(ev);
            pending_state.ingest_queue.pop_front_if_ready()
        } else {
            self.input_queue.push_back_or_ignore(ev);
            self.input_queue.pop_front_if_ready()
        };

        if let Some(ie) = ready {
            // Process before clearing idle_time
            //  so families that snapshot KeymapContext
            //  (required_idle_time, pressed_modifiers)
            //  still see idle accumulated since the previous input.
            self.process_input(ie);
            self.set_active_input_delay();
        }

        self.idle_time = 0;
    }

    /// After processing one input, re-arm the active delay line.
    ///
    /// If processing resolved pending state, the global queue is active;
    ///  if a pending session remains (or was just created), its ingest queue is.
    fn set_active_input_delay(&mut self) {
        if let Some(pending_state) = self.pending_state.as_mut() {
            pending_state.ingest_queue.set_delay();
        } else {
            self.input_queue.set_delay();
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
        if let Some(pending::PendingState {
            keymap_index,
            key_ref,
            pending_key_state,
            queued_events,
            ingest_queue,
            ..
        }) = self.pending_state.as_mut()
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

                        // Nested pending: re-feed session-log inputs chronologically
                        //  into the current pending delay line (not the global queue).
                        pending::dispatch_replayed_events(
                            pending::KeyResolution::Pending,
                            queued_events,
                            ingest_queue,
                            &mut self.event_scheduler,
                        );
                    }
                }
            }
        }
    }

    fn process_input(&mut self, ev: input::Event) {
        if let Some(pending_state) = self.pending_state.as_mut() {
            // Paced input from the delay line: record in the session log, then apply.
            pending_state.record_input(ev);
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
                    // Snapshot held mods / recent presses before branching.
                    self.push_keymap_context();

                    let mut maybe_key_ref = Some(self.key_refs[keymap_index as usize]);

                    while let Some(key_ref) = maybe_key_ref.take() {
                        let (pkr, pke) =
                            self.key_system
                                .new_pressed_key(keymap_index, &self.context, key_ref);

                        pke.into_iter()
                            .for_each(|sch_ev| self.event_scheduler.schedule_event(sch_ev));

                        match pkr {
                            key::PressedKeyResult::Resolved(key_state) => {
                                let _ = self.pressed_inputs.push(input::PressedInput::pressed_key(
                                    keymap_index,
                                    key_ref,
                                    key_state,
                                ));

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

                                let _ = self.pressed_inputs.push(input::PressedInput::pressed_key(
                                    keymap_index,
                                    key_ref,
                                    key_state,
                                ));
                            }
                            key::PressedKeyResult::Pending(pending_key_state) => {
                                // Fresh pending session owns its own delay line,
                                //  armed so the next physical input is deferred.
                                // Move any inputs already sitting in the global
                                //  queue (e.g. a rapid release pushed before this
                                //  press was processed) into the new local delay
                                //  line so they are paced while pending.
                                let mut pending_state = pending::PendingState::new(
                                    keymap_index,
                                    key_ref,
                                    pending_key_state,
                                );
                                let mut remaining = self.input_queue.take_all();
                                pending_state.ingest_queue.append_all(&mut remaining);
                                self.pending_state = Some(pending_state);
                            }
                        }
                    }

                    // Record after press handling so quick-tap sees the *prior* press.
                    self.record_recent_press(keymap_index);
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
                    let _ = self.pressed_inputs.push(pressed_key);
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

        let was_pending = self.pending_state.is_some();

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
            if was_pending {
                // `update_pending_state` already ran above.
                // Only record for replay if still pending;
                //  do not re-apply or fall through to the non-pending press path.
                if let Some(pending_state) = self.pending_state.as_mut() {
                    pending_state.record_input(input_ev);
                }
                self.handle_pending_events();
            } else {
                self.process_input(input_ev);
            }
        }
    }

    fn handle_pending_events(&mut self) {
        // take from pending
        while let Some(ev) = self.event_scheduler.dequeue() {
            self.handle_event(ev);
        }
    }

    /// Aggregate keyboard modifiers from already-pressed inputs (no suppress).
    fn aggregate_pressed_modifiers(&self) -> key::KeyboardModifiers {
        self.pressed_inputs
            .iter()
            .filter_map(|pi| match pi {
                input::PressedInput::Key(input::PressedKey {
                    key_ref, key_state, ..
                }) => self.key_system.key_output(key_ref, key_state),
                &input::PressedInput::Virtual(key_output) => Some(key_output),
            })
            .fold(key::KeyboardModifiers::NONE, |acc, ko| {
                acc.union(&ko.key_modifiers())
            })
    }

    fn push_keymap_context(&mut self) {
        let km_context = KeymapContext {
            time_ms: self.event_scheduler.schedule_counter,
            idle_time_ms: self.idle_time,
            pressed_modifiers: self.aggregate_pressed_modifiers(),
            recent_presses: self.recent_presses,
            recent_press_count: self.recent_press_count,
        };
        self.context.set_keymap_context(km_context);
    }

    /// Advances the state of the keymap by one tick.
    pub fn tick(&mut self) {
        self.push_keymap_context();

        let ready = if let Some(pending_state) = self.pending_state.as_mut() {
            pending_state.ingest_queue.pop_front_if_ready()
        } else {
            self.input_queue.pop_front_if_ready()
        };

        if let Some(ie) = ready {
            self.process_input(ie);
            self.set_active_input_delay();
        }

        // Always tick the global delay gate so it does not go stale
        //  across a pending session (e.g. on resolve transfer).
        self.input_queue.tick_delay();
        if let Some(pending_state) = self.pending_state.as_mut() {
            pending_state.ingest_queue.tick_delay();
        }

        self.event_scheduler.tick(self.ms_per_tick);

        self.handle_pending_events();

        self.idle_time += self.ms_per_tick as u32;
    }

    /// Returns the the pressed key outputs.
    pub fn pressed_keys(&self) -> heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> {
        let suppress = self.context.suppressed_modifiers();
        let pressed_key_codes = self.pressed_inputs.iter().filter_map(|pi| {
            let ko = match pi {
                input::PressedInput::Key(input::PressedKey {
                    key_ref, key_state, ..
                }) => self.key_system.key_output(key_ref, key_state)?,
                &input::PressedInput::Virtual(key_output) => key_output,
            };
            let ko = ko.without_modifiers(suppress);
            // Drop pure-mod outputs that are fully suppressed.
            if ko == key::KeyOutput::NO_OUTPUT {
                None
            } else {
                Some(ko)
            }
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

    /// Whether the keymap has pending state that requires polling.
    pub fn requires_polling(&self) -> bool {
        !self.event_scheduler.pending_events.is_empty()
            || !self.input_queue.is_empty()
            || self
                .pending_state
                .as_ref()
                .is_some_and(|ps| !ps.ingest_queue.is_empty())
    }

    #[doc(hidden)]
    pub fn has_scheduled_events(&self) -> bool {
        !self.event_scheduler.pending_events.is_empty()
            || !self.event_scheduler.scheduled_events.is_empty()
            || !self.input_queue.is_empty()
            || self
                .pending_state
                .as_ref()
                .is_some_and(|ps| !ps.ingest_queue.is_empty())
    }
}

/// Test-only inspection hooks for pending-state / input-queue pacing.
///
/// Used by `smart-keymap-full-system-std` integration tests (cannot live as
/// `#[cfg(test)]` unit helpers because that crate is a separate package).
#[cfg(feature = "std")]
#[doc(hidden)]
impl<
        I: Debug + Index<usize, Output = R>,
        R: Copy + Debug,
        Ctx: Debug + key::Context<Event = Ev> + SetKeymapContext + ReportHints,
        Ev: Copy + Debug,
        PKS: Debug,
        KS: Copy + Debug + From<key::NoOpKeyState>,
        S: key::System<R, Ref = R, Context = Ctx, Event = Ev, PendingKeyState = PKS, KeyState = KS>,
    > Keymap<I, R, Ctx, Ev, PKS, KS, S>
{
    /// Whether a pending key state is active.
    pub fn test_is_pending(&self) -> bool {
        self.pending_state.is_some()
    }

    /// Length of the pending session log, if pending.
    pub fn test_pending_queued_events_len(&self) -> Option<usize> {
        self.pending_state
            .as_ref()
            .map(|pending_state| pending_state.queued_events.len())
    }

    /// Session-log input events (only `Event::Input` variants),
    ///  in log order.
    pub fn test_pending_session_log_inputs(&self) -> Option<heapless::Vec<input::Event, 16>> {
        self.pending_state.as_ref().map(|pending_state| {
            let mut inputs = heapless::Vec::new();
            for ev in pending_state.queued_events.iter() {
                if let key::Event::Input(ie) = ev {
                    let _ = inputs.push(*ie);
                }
            }
            inputs
        })
    }

    /// Length of the active delay line
    ///  (pending `ingest_queue` while pending, else global `input_queue`).
    pub fn test_input_queue_len(&self) -> usize {
        if let Some(pending_state) = self.pending_state.as_ref() {
            pending_state.ingest_queue.len()
        } else {
            self.input_queue.len()
        }
    }

    /// Delay gate of the active delay line
    ///  (pending `ingest_queue` while pending, else global `input_queue`).
    pub fn test_input_queue_delay(&self) -> bool {
        if let Some(pending_state) = self.pending_state.as_ref() {
            pending_state.ingest_queue.delay()
        } else {
            self.input_queue.delay()
        }
    }

    /// Schedule an immediate key event and process pending events.
    pub fn test_handle_scheduled_key_event(&mut self, ev: key::Event<Ev>) {
        self.event_scheduler
            .schedule_event(key::ScheduledEvent::immediate(ev));
        self.handle_pending_events();
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
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
    fn test_keymap_output_pressed_consumer_codes() {
        let mut input: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = heapless::Vec::new();
        input
            .push(key::KeyOutput::from_consumer_code(0xE9))
            .unwrap();

        let keymap_output = KeymapOutput::new(input);
        assert_eq!(
            heapless::Vec::<u8, 24>::from_slice(&[0xE9]).unwrap(),
            keymap_output.pressed_consumer_codes()
        );
    }

    #[test]
    fn test_keymap_output_pressed_mouse_output_combines_buttons() {
        let mut input: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = heapless::Vec::new();
        input
            .push(key::KeyOutput::from_mouse_output(key::MouseOutput {
                pressed_buttons: 0b001,
                ..key::MouseOutput::NO_OUTPUT
            }))
            .unwrap();
        input
            .push(key::KeyOutput::from_mouse_output(key::MouseOutput {
                pressed_buttons: 0b010,
                ..key::MouseOutput::NO_OUTPUT
            }))
            .unwrap();

        let keymap_output = KeymapOutput::new(input);
        assert_eq!(
            key::MouseOutput {
                pressed_buttons: 0b011,
                ..key::MouseOutput::NO_OUTPUT
            },
            keymap_output.pressed_mouse_output()
        );
    }

    #[test]
    fn test_keymap_context_default_is_zeroed() {
        let context = KeymapContext::new();
        assert_eq!(0, context.time_ms);
        assert_eq!(0, context.idle_time_ms);
    }
}
