//! Pending-key resolution helpers for [`Keymap`](super::Keymap).
//!
//! Some keys (tap-hold, chorded, layered) start in an unresolved "pending" state.
//! While pending,
//!  the keymap paces inputs one-per-tick through `input_queue` (the ! **delay line**)
//!  and records each processed input in [`PendingState::queued_events`] (the **session log**)
//!  for replay when the key resolves.
//!
//! Concretely:
//! - The **delay line** is the backlog of physical inputs still waiting in
//!   [`InputEventQueue`](super::input_event_queue::InputEventQueue). Those inputs have
//!   not yet affected the pending key, so they are *not* part of replay.
//! - The **session log** is [`PendingState::queued_events`]: the inputs and key events
//!   that were already paced, observed, and applied while the key was pending.
//!
//! When the pending key resolves or transitions to nested pending, the session log
//! is replayed according to a [`ReplayCase`].

use core::fmt::Debug;

use crate::input;
use crate::key::{self, pending_resolution_events};

use super::event_scheduler::EventScheduler;
use super::input_event_queue::InputEventQueue;

/// Session state while a key's output is still undecided (tap-hold, chorded, etc.).
///
/// **Example:**
///  A tap-hold key is pressed.
///  `new_pressed_key` returns `PressedKeyResult::Pending`
///   and the keymap stores a `PendingState` with the tap-hold's inner state.
///  Each subsequent paced input is appended to `queued_events`
///   until the key resolves as tap or hold.
#[derive(Debug)]
pub(crate) struct PendingState<R, Ev, PKS> {
    pub keymap_index: u16,
    pub key_ref: R,
    pub pending_key_state: PKS,
    /// Inputs already paced and applied during this pending session; replayed on resolve.
    pub queued_events: heapless::Vec<key::Event<Ev>, { super::MAX_PRESSED_KEYS }>,
}

impl<R, Ev, PKS> PendingState<R, Ev, PKS> {
    /// Append a paced input to the session log for replay on resolve.
    pub fn record_input(&mut self, ev: input::Event) {
        let _ = self.queued_events.push(ev.into());
    }
}

/// Which replay behaviour to apply to a pending key's session log.
pub(crate) enum KeyResolution {
    /// The pending key resolved to a pressed key state.
    ///
    /// **Example:**
    ///  Tap-hold resolves as tap after a quick release.
    ///  The session log might contain `[Press(0), Release(0), Press(1)]`.
    ///  Only the last event targeting key 0 (`Release(0)`) is kept;
    ///   other-key inputs are prepended to the delay line;
    ///   any key events are scheduled with staggered delays.
    Resolved { keymap_index: u16 },
    /// The pending key transitioned to a *different* pending state (nested pending).
    ///
    /// **Example:**
    ///  A layered key resolves the outer layer
    ///   but the inner key is still pending.
    ///  All session-log inputs are prepended LIFO ahead of the delay line
    ///   so pacing continues correctly for the new pending state.
    Pending,
}

fn dispatch_resolved_key_replayed_events<Ev: Copy + Debug, const N: usize, const Q: usize>(
    keymap_index: u16,
    queued_events: &mut heapless::Vec<key::Event<Ev>, N>,
    input_queue: &mut InputEventQueue<Q>,
    event_scheduler: &mut EventScheduler<Ev>,
) {
    let mut schedule_delay = 1;
    let mut input_events_to_prepend = heapless::Vec::<input::Event, N>::new();

    for ev in pending_resolution_events(queued_events, keymap_index).iter() {
        match ev {
            key::Event::Input(ie) => {
                let _ = input_events_to_prepend.push(*ie);
            }
            _ => {
                event_scheduler.schedule_after(schedule_delay, *ev);
                schedule_delay += 1;
            }
        }
    }

    input_queue.prepend(&input_events_to_prepend);
}

fn dispatch_pending_replayed_events<Ev: Copy + Debug, const N: usize, const Q: usize>(
    queued_events: &mut heapless::Vec<key::Event<Ev>, N>,
    input_queue: &mut InputEventQueue<Q>,
) {
    input_queue.prepend_pending_input_events(queued_events);
}

/// Routes replayed session-log events to the input queue and/or event scheduler.
pub(crate) fn dispatch_replayed_events<Ev: Copy + Debug, const N: usize, const Q: usize>(
    replay_case: KeyResolution,
    queued_events: &mut heapless::Vec<key::Event<Ev>, N>,
    input_queue: &mut InputEventQueue<Q>,
    event_scheduler: &mut EventScheduler<Ev>,
) {
    match replay_case {
        KeyResolution::Resolved { keymap_index } => {
            dispatch_resolved_key_replayed_events(
                keymap_index,
                queued_events,
                input_queue,
                event_scheduler,
            );
        }
        KeyResolution::Pending => {
            dispatch_pending_replayed_events(queued_events, input_queue);
        }
    }
}
