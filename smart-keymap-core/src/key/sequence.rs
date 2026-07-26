//! Sequence keys (ordered key sequences).
//!
//! After a [Ref::SequenceStart](crate::key::sequence::Ref::SequenceStart) key arms sequence
//! mode on [Context](crate::key::sequence::Context), subsequent presses of
//! sequence member keys append to a buffer in
//! [Context](crate::key::sequence::Context). An exact match resolves to a bound
//! key (looked up from primary sequence keys).
//!
//! Behaviour (v1):
//! - SequenceStart: no HID output; arms mode (or restarts if already armed).
//! - Steps: press edges only; buffer lives on
//!   [Context](crate::key::sequence::Context) (no pending session).
//! - Timeout: per-step, refreshed on each valid step; exact match on timeout
//!   resolves (first config entry wins); strict prefix only aborts.
//! - Unknown / dead path: abort without sequence output.
//! - When mode is inactive, member keys act as passthrough.

use core::fmt::Debug;
use core::ops::Index;

use serde::Deserialize;

use crate::{input, key, keymap, slice::Slice};

/// Reference for a sequence key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Ref {
    /// Primary sequence key (lowest index in a sequence definition).
    ///
    /// Owns the resolved nested key for each sequence where this index is primary.
    /// Other sequence members are [`Ref::Auxiliary`].
    Sequence(u8),
    /// Non-primary sequence member.
    Auxiliary(u8),
    /// Arms (or restarts) sequence mode. JSON/Nickel token is `SequenceStart`.
    SequenceStart,
}

/// Identifier of a sequence in [`Config::sequences`].
pub type SequenceId = u8;

/// Ordered keymap indices for one sequence.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(from = "heapless::Vec<u16, MAX_SEQUENCE_LEN>")]
pub struct SequenceIndices<const MAX_SEQUENCE_LEN: usize> {
    indices: Slice<u16, MAX_SEQUENCE_LEN>,
}

impl<const MAX_SEQUENCE_LEN: usize> SequenceIndices<MAX_SEQUENCE_LEN> {
    /// Constructs from a slice (must fit `MAX_SEQUENCE_LEN`).
    pub const fn from_slice(indices: &[u16]) -> Self {
        Self {
            indices: Slice::from_slice(indices),
        }
    }

    /// Indices as a slice.
    pub const fn as_slice(&self) -> &[u16] {
        self.indices.as_slice()
    }
}

impl<const MAX_SEQUENCE_LEN: usize> From<heapless::Vec<u16, MAX_SEQUENCE_LEN>>
    for SequenceIndices<MAX_SEQUENCE_LEN>
{
    fn from(v: heapless::Vec<u16, MAX_SEQUENCE_LEN>) -> Self {
        Self::from_slice(&v)
    }
}

/// Sequence definitions and timing.
#[derive(Deserialize, Clone, Copy, PartialEq)]
pub struct Config<const MAX_SEQUENCES: usize, const MAX_SEQUENCE_LEN: usize> {
    /// Per-step timeout in milliseconds (refreshed on each valid step).
    #[serde(default = "default_timeout")]
    pub timeout: u16,

    /// Sequences as ordered keymap index lists.
    pub sequences: Slice<SequenceIndices<MAX_SEQUENCE_LEN>, MAX_SEQUENCES>,

    /// Minimum idle time (ms) before SequenceStart can arm mode.
    pub required_idle_time: Option<u16>,
}

impl<const MAX_SEQUENCES: usize, const MAX_SEQUENCE_LEN: usize> core::fmt::Debug
    for Config<MAX_SEQUENCES, MAX_SEQUENCE_LEN>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Config")
            .field("timeout", &self.timeout)
            .field("sequences", &self.sequences.as_slice())
            .field("required_idle_time", &self.required_idle_time)
            .finish()
    }
}

/// Default per-step timeout (ms).
pub const DEFAULT_TIMEOUT: u16 = 1000;

const fn default_timeout() -> u16 {
    DEFAULT_TIMEOUT
}

impl<const MAX_SEQUENCES: usize, const MAX_SEQUENCE_LEN: usize>
    Config<MAX_SEQUENCES, MAX_SEQUENCE_LEN>
{
    /// Empty config with default timeout.
    pub const fn new() -> Self {
        Self {
            timeout: DEFAULT_TIMEOUT,
            sequences: Slice::from_slice(&[]),
            required_idle_time: None,
        }
    }
}

impl<const MAX_SEQUENCES: usize, const MAX_SEQUENCE_LEN: usize> Default
    for Config<MAX_SEQUENCES, MAX_SEQUENCE_LEN>
{
    fn default() -> Self {
        Self::new()
    }
}

/// Outcome of the most recent press while sequence mode was considered.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PressOutcome {
    /// Mode was not armed for this press.
    Inactive,
    /// Step accepted; waiting for more keys or timeout.
    Continue,
    /// Sequence completed with this id — emit bound key.
    Resolved(SequenceId),
    /// Aborted; no sequence output.
    Aborted,
}

/// Global sequence mode state.
#[derive(Clone, Copy, PartialEq)]
pub struct Context<const MAX_SEQUENCES: usize, const MAX_SEQUENCE_LEN: usize> {
    config: Config<MAX_SEQUENCES, MAX_SEQUENCE_LEN>,
    mode_active: bool,
    idle_time_ms: u32,
    timeout_generation: u16,
    buffer: [u16; MAX_SEQUENCE_LEN],
    buffer_len: usize,
    /// Set on each Input press; read by sequence keys in `new_pressed_key`.
    last_press_outcome: PressOutcome,
}

impl<const MAX_SEQUENCES: usize, const MAX_SEQUENCE_LEN: usize> Debug
    for Context<MAX_SEQUENCES, MAX_SEQUENCE_LEN>
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Context")
            .field("config", &self.config)
            .field("mode_active", &self.mode_active)
            .field("idle_time_ms", &self.idle_time_ms)
            .field("timeout_generation", &self.timeout_generation)
            .field("buffer", &&self.buffer[..self.buffer_len])
            .field("last_press_outcome", &self.last_press_outcome)
            .finish()
    }
}

impl<const MAX_SEQUENCES: usize, const MAX_SEQUENCE_LEN: usize>
    Context<MAX_SEQUENCES, MAX_SEQUENCE_LEN>
{
    /// Constructs from config.
    pub const fn from_config(config: Config<MAX_SEQUENCES, MAX_SEQUENCE_LEN>) -> Self {
        Self {
            config,
            mode_active: false,
            idle_time_ms: 0,
            timeout_generation: 0,
            buffer: [0; MAX_SEQUENCE_LEN],
            buffer_len: 0,
            last_press_outcome: PressOutcome::Inactive,
        }
    }

    /// Clears runtime mode state; keeps config.
    pub fn reset(&mut self) {
        *self = Self::from_config(self.config);
    }

    /// Whether sequence mode is armed.
    pub fn is_armed(&self) -> bool {
        self.mode_active
    }

    /// Config reference.
    pub fn config(&self) -> &Config<MAX_SEQUENCES, MAX_SEQUENCE_LEN> {
        &self.config
    }

    /// Outcome of the latest input press (for sequence keys).
    pub fn last_press_outcome(&self) -> PressOutcome {
        self.last_press_outcome
    }

    fn sufficient_idle_time(&self) -> bool {
        self.idle_time_ms >= self.config.required_idle_time.unwrap_or(0) as u32
    }

    fn bump_timeout(&mut self) -> u16 {
        self.timeout_generation = self.timeout_generation.wrapping_add(1);
        self.timeout_generation
    }

    fn arm(&mut self) {
        self.mode_active = true;
        self.buffer_len = 0;
        self.bump_timeout();
        self.last_press_outcome = PressOutcome::Inactive;
    }

    fn disarm(&mut self) {
        self.mode_active = false;
        self.buffer_len = 0;
        self.bump_timeout();
    }

    fn buffer_slice(&self) -> &[u16] {
        &self.buffer[..self.buffer_len]
    }

    fn schedule_timeout(&self, gen_id: u16) -> key::KeyEvents<Event> {
        key::KeyEvents::scheduled_event(key::ScheduledEvent::after(
            self.config.timeout,
            key::Event::key_event(0, Event::Timeout(gen_id)),
        ))
    }

    fn candidates_for_buffer(&self) -> heapless::Vec<SequenceId, MAX_SEQUENCES> {
        let buffer = self.buffer_slice();
        self.config
            .sequences
            .iter()
            .enumerate()
            .filter(|(_, seq)| {
                let s = seq.as_slice();
                s.len() >= buffer.len() && s[..buffer.len()] == *buffer
            })
            .map(|(id, _)| id as SequenceId)
            .collect()
    }

    fn exact_match_id(&self, candidates: &[SequenceId]) -> Option<SequenceId> {
        let buffer = self.buffer_slice();
        candidates
            .iter()
            .copied()
            .find(|&id| self.config.sequences[id as usize].as_slice() == buffer)
    }

    fn has_longer(&self, candidates: &[SequenceId]) -> bool {
        candidates
            .iter()
            .any(|&id| self.config.sequences[id as usize].as_slice().len() > self.buffer_len)
    }

    /// Append a press and update [Self::last_press_outcome].
    fn step_press(&mut self, keymap_index: u16) {
        if self.buffer_len >= MAX_SEQUENCE_LEN {
            self.disarm();
            self.last_press_outcome = PressOutcome::Aborted;
        } else {
            self.buffer[self.buffer_len] = keymap_index;
            self.buffer_len += 1;
            let candidates = self.candidates_for_buffer();
            match (
                self.exact_match_id(&candidates),
                self.has_longer(&candidates),
            ) {
                (Some(id), false) => {
                    self.disarm();
                    self.last_press_outcome = PressOutcome::Resolved(id);
                }
                (None, false) => {
                    // No candidates (dead path).
                    self.disarm();
                    self.last_press_outcome = PressOutcome::Aborted;
                }
                _ => {
                    // Still waiting (maybe exact+longer, or only longer).
                    self.last_press_outcome = PressOutcome::Continue;
                    self.bump_timeout();
                }
            }
        }
    }

    /// Updates idle time from the keymap engine.
    pub fn update_keymap_context(
        &mut self,
        keymap::KeymapContext { idle_time_ms, .. }: &keymap::KeymapContext,
    ) {
        self.idle_time_ms = *idle_time_ms;
    }

    fn handle_event(&mut self, event: key::Event<Event>) -> key::KeyEvents<Event> {
        match event {
            key::Event::Input(input::Event::Press { keymap_index }) => {
                if self.mode_active {
                    self.step_press(keymap_index);
                    match self.last_press_outcome {
                        PressOutcome::Continue => self.schedule_timeout(self.timeout_generation),
                        PressOutcome::Resolved(id) => key::KeyEvents::event(key::Event::key_event(
                            keymap_index,
                            Event::SequenceResolved(id),
                        )),
                        PressOutcome::Aborted => key::KeyEvents::event(key::Event::key_event(
                            keymap_index,
                            Event::Aborted,
                        )),
                        PressOutcome::Inactive => key::KeyEvents::no_events(),
                    }
                } else {
                    self.last_press_outcome = PressOutcome::Inactive;
                    key::KeyEvents::no_events()
                }
            }
            key::Event::Key {
                key_event: Event::Arm,
                ..
            } => {
                if self.sufficient_idle_time() {
                    self.arm();
                    self.schedule_timeout(self.timeout_generation)
                } else {
                    key::KeyEvents::no_events()
                }
            }
            key::Event::Key {
                key_event: Event::Restart,
                ..
            } => {
                self.arm();
                self.schedule_timeout(self.timeout_generation)
            }
            key::Event::Key {
                key_event: Event::Timeout(gen),
                ..
            } => {
                if self.mode_active && gen == self.timeout_generation {
                    // v1: timeout always aborts without sequence output
                    //  (including exact match).
                    // Emitting a binding needs a press path; pure timeout has none.
                    self.disarm();
                    self.last_press_outcome = PressOutcome::Aborted;
                }
                key::KeyEvents::no_events()
            }
            _ => key::KeyEvents::no_events(),
        }
    }
}

impl<const MAX_SEQUENCES: usize, const MAX_SEQUENCE_LEN: usize> key::Context
    for Context<MAX_SEQUENCES, MAX_SEQUENCE_LEN>
{
    type Event = Event;

    fn handle_event(&mut self, event: key::Event<Self::Event>) -> key::KeyEvents<Self::Event> {
        self.handle_event(event)
    }

    fn reset(&mut self) {
        Context::reset(self);
    }
}

/// Sequence family events.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Event {
    /// Arm sequence mode (from SequenceStart when inactive).
    Arm,
    /// Restart sequence mode (from SequenceStart when already armed).
    Restart,
    /// Timeout; generation must match context generation.
    Timeout(u16),
    /// A sequence resolved.
    SequenceResolved(SequenceId),
    /// Sequence aborted.
    Aborted,
}

/// No pending state (buffer lives on [`Context`]).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PendingKeyState;

/// No key state for sequence keys (resolution nests to another ref).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState;

/// Primary sequence key: bindings + passthrough.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key<R: Copy, const MAX_OVERLAPPING: usize> {
    /// Sequences for which this key is primary (lowest index), with bound keys.
    pub sequences: Slice<(SequenceId, R), MAX_OVERLAPPING>,
    /// Key when sequence mode is inactive.
    pub passthrough: R,
}

impl<R: Copy, const MAX_OVERLAPPING: usize> Key<R, MAX_OVERLAPPING> {
    /// Constructs a primary sequence key.
    pub const fn new(sequences: &[(SequenceId, R)], passthrough: R) -> Self {
        Self {
            sequences: Slice::from_slice(sequences),
            passthrough,
        }
    }

    /// Bound key for sequence id, if this primary owns it.
    pub fn binding_for(&self, id: SequenceId) -> Option<R> {
        self.sequences
            .iter()
            .find(|(sid, _)| *sid == id)
            .map(|(_, r)| *r)
    }
}

/// Non-primary sequence member.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct AuxiliaryKey<R: Copy> {
    /// Key when sequence mode is inactive.
    pub passthrough: R,
}

impl<R: Copy> AuxiliaryKey<R> {
    /// Constructs an auxiliary sequence key.
    pub const fn new(passthrough: R) -> Self {
        Self { passthrough }
    }
}

/// Sequence [key::System].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct System<
    R: Copy + Debug + PartialEq,
    Keys: Index<usize, Output = Key<R, MAX_OVERLAPPING>> + AsRef<[Key<R, MAX_OVERLAPPING>]>,
    AuxiliaryKeys: Index<usize, Output = AuxiliaryKey<R>>,
    const MAX_SEQUENCES: usize,
    const MAX_SEQUENCE_LEN: usize,
    const MAX_OVERLAPPING: usize,
> {
    keys: Keys,
    auxiliary_keys: AuxiliaryKeys,
}

impl<
        R: Copy + Debug + PartialEq,
        Keys: Index<usize, Output = Key<R, MAX_OVERLAPPING>> + AsRef<[Key<R, MAX_OVERLAPPING>]>,
        AuxiliaryKeys: Index<usize, Output = AuxiliaryKey<R>>,
        const MAX_SEQUENCES: usize,
        const MAX_SEQUENCE_LEN: usize,
        const MAX_OVERLAPPING: usize,
    > System<R, Keys, AuxiliaryKeys, MAX_SEQUENCES, MAX_SEQUENCE_LEN, MAX_OVERLAPPING>
{
    /// Constructs the system from key data arrays.
    pub const fn new(keys: Keys, auxiliary_keys: AuxiliaryKeys) -> Self {
        Self {
            keys,
            auxiliary_keys,
        }
    }

    fn binding_for(&self, id: SequenceId) -> Option<R> {
        self.keys.as_ref().iter().find_map(|k| k.binding_for(id))
    }
}

impl<
        R: Copy + Debug + PartialEq,
        Keys: Debug + Index<usize, Output = Key<R, MAX_OVERLAPPING>> + AsRef<[Key<R, MAX_OVERLAPPING>]>,
        AuxiliaryKeys: Debug + Index<usize, Output = AuxiliaryKey<R>>,
        const MAX_SEQUENCES: usize,
        const MAX_SEQUENCE_LEN: usize,
        const MAX_OVERLAPPING: usize,
    > key::System<R>
    for System<R, Keys, AuxiliaryKeys, MAX_SEQUENCES, MAX_SEQUENCE_LEN, MAX_OVERLAPPING>
{
    type Ref = Ref;
    type Context = Context<MAX_SEQUENCES, MAX_SEQUENCE_LEN>;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        keymap_index: u16,
        context: &Self::Context,
        key_ref: Ref,
    ) -> (
        key::PressedKeyResult<R, Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        match key_ref {
            Ref::SequenceStart => {
                let ev = if context.is_armed() {
                    // Was armed before this press;
                    //  Context may have aborted on the start index step.
                    // Restart either way.
                    Event::Restart
                } else {
                    Event::Arm
                };
                let pke = key::KeyEvents::event(key::Event::key_event(keymap_index, ev));
                (
                    key::PressedKeyResult::NewPressedKey(key::NewPressedKey::NoOp),
                    pke,
                )
            }
            Ref::Sequence(i) | Ref::Auxiliary(i) => {
                let passthrough = match key_ref {
                    Ref::Sequence(idx) => self.keys[idx as usize].passthrough,
                    Ref::Auxiliary(idx) => self.auxiliary_keys[idx as usize].passthrough,
                    Ref::SequenceStart => unreachable!(),
                };
                let _ = i;

                // Context already processed this Input press
                //  (handle_event before new_pressed_key).
                // Use last_press_outcome.
                match context.last_press_outcome() {
                    PressOutcome::Inactive => (
                        key::PressedKeyResult::NewPressedKey(key::NewPressedKey::key(passthrough)),
                        key::KeyEvents::no_events(),
                    ),
                    PressOutcome::Continue | PressOutcome::Aborted => (
                        key::PressedKeyResult::NewPressedKey(key::NewPressedKey::NoOp),
                        key::KeyEvents::no_events(),
                    ),
                    PressOutcome::Resolved(id) => {
                        if let Some(r) = self.binding_for(id) {
                            (
                                key::PressedKeyResult::NewPressedKey(key::NewPressedKey::key(r)),
                                key::KeyEvents::no_events(),
                            )
                        } else {
                            (
                                key::PressedKeyResult::NewPressedKey(key::NewPressedKey::NoOp),
                                key::KeyEvents::no_events(),
                            )
                        }
                    }
                }
            }
        }
    }

    fn update_pending_state(
        &self,
        _pending_state: &mut Self::PendingKeyState,
        _keymap_index: u16,
        _context: &Self::Context,
        _key_ref: Ref,
        _event: key::Event<Self::Event>,
    ) -> (Option<key::NewPressedKey<R>>, key::KeyEvents<Self::Event>) {
        (None, key::KeyEvents::no_events())
    }

    fn update_state(
        &self,
        _key_state: &mut Self::KeyState,
        _key_ref: &Self::Ref,
        _context: &Self::Context,
        _keymap_index: u16,
        _event: key::Event<Self::Event>,
    ) -> key::KeyEvents<Self::Event> {
        key::KeyEvents::no_events()
    }

    fn key_output(
        &self,
        _key_ref: &Self::Ref,
        _key_state: &Self::KeyState,
    ) -> Option<key::KeyOutput> {
        None
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    const MAX_SEQUENCES: usize = 4;
    const MAX_SEQUENCE_LEN: usize = 4;

    type Ctx = Context<MAX_SEQUENCES, MAX_SEQUENCE_LEN>;

    fn ctx_with(sequences: &[&[u16]]) -> Ctx {
        match sequences.len() {
            1 => Context::from_config(Config {
                sequences: Slice::from_slice(&[SequenceIndices::from_slice(sequences[0])]),
                ..Config::new()
            }),
            2 => Context::from_config(Config {
                sequences: Slice::from_slice(&[
                    SequenceIndices::from_slice(sequences[0]),
                    SequenceIndices::from_slice(sequences[1]),
                ]),
                ..Config::new()
            }),
            _ => Context::from_config(Config::new()),
        }
    }

    #[test]
    fn start_arms_mode() {
        let mut ctx = Ctx::from_config(Config::new());
        let _ = ctx.handle_event(key::Event::key_event(0, Event::Arm));
        assert!(ctx.is_armed());
    }

    #[test]
    fn two_step_resolves() {
        // Assemble: context with sequence [0, 1]
        let mut ctx = ctx_with(&[&[0, 1]]);

        // Act: start sequence, input 0, input 1.
        let _ = ctx.handle_event(key::Event::key_event(9, Event::Arm));
        assert!(ctx.is_armed());
        let _ = ctx.handle_event(key::Event::Input(input::Event::Press { keymap_index: 0 }));
        assert_eq!(ctx.last_press_outcome(), PressOutcome::Continue);
        let _ = ctx.handle_event(key::Event::Input(input::Event::Press { keymap_index: 1 }));

        // Assert: should resolve
        assert_eq!(ctx.last_press_outcome(), PressOutcome::Resolved(0));
        assert!(!ctx.is_armed());
    }

    #[test]
    fn unknown_aborts() {
        // Assemble: context with sequence [0, 1]
        let mut ctx = ctx_with(&[&[0, 1]]);

        // Act: start sequence, press 0, press 9 (not part of any sequence)
        let _ = ctx.handle_event(key::Event::key_event(9, Event::Arm));
        let _ = ctx.handle_event(key::Event::Input(input::Event::Press { keymap_index: 0 }));
        let _ = ctx.handle_event(key::Event::Input(input::Event::Press { keymap_index: 9 }));

        // Assert: should abort the sequence key
        assert_eq!(ctx.last_press_outcome(), PressOutcome::Aborted);
        assert!(!ctx.is_armed());
    }

    #[test]
    fn timeout_aborts_strict_prefix() {
        // Assemble: context with sequence [0, 1, 2]
        let mut ctx = ctx_with(&[&[0, 1, 2]]);

        // Act: start sequence, press 0; wait
        let _ = ctx.handle_event(key::Event::key_event(9, Event::Arm));
        let _ = ctx.handle_event(key::Event::Input(input::Event::Press { keymap_index: 0 }));
        let gen = ctx.timeout_generation;
        let _ = ctx.handle_event(key::Event::key_event(0, Event::Timeout(gen)));

        // Assert: should abort the sequence key
        assert_eq!(ctx.last_press_outcome(), PressOutcome::Aborted);
        assert!(!ctx.is_armed());
    }
}
