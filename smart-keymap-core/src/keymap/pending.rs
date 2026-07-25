//! Pending-key resolution helpers for [`Keymap`](super::Keymap).
//!
//! Some keys (tap-hold, chorded, layered) start in an unresolved "pending" state.
//! While pending,
//!  the keymap paces physical inputs one-per-tick through
//!  [`PendingState::ingest_queue`] (the **delay line**)
//!  and records each processed input in [`PendingState::queued_events`] (the **session log**)
//!  for replay when the key resolves.
//!
//! Concretely:
//! - The **delay line** while pending is [`PendingState::ingest_queue`].
//!   Those inputs have not yet affected the pending key,
//!   so they are *not* part of replay.
//!   On resolve, any remaining delay-line inputs are transferred to the
//!   keymap's global `input_queue` tail for post-resolve pacing.
//! - The **session log** is [`PendingState::queued_events`]:
//!   the inputs and key events that were already paced, observed,
//!   and applied while the key was pending.
//!
//! When the pending key resolves or transitions to nested pending,
//!  the session log is replayed according to a [`KeyResolution`].

use core::fmt::Debug;

use crate::input;
use crate::key::{self, pending_resolution_events};

use super::event_scheduler::EventScheduler;
use super::input_event_queue::InputEventQueue;
use super::MAX_QUEUED_INPUT_EVENTS;

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
    /// Physical inputs waiting to be paced while this key is pending.
    pub ingest_queue: InputEventQueue<{ MAX_QUEUED_INPUT_EVENTS }>,
}

impl<R, Ev, PKS> PendingState<R, Ev, PKS> {
    /// Construct a pending session with the delay line already armed
    ///  so the next physical input is deferred by one tick.
    pub fn new(keymap_index: u16, key_ref: R, pending_key_state: PKS) -> Self {
        let mut ingest_queue = InputEventQueue::new();
        ingest_queue.set_delay();
        Self {
            keymap_index,
            key_ref,
            pending_key_state,
            queued_events: heapless::Vec::new(),
            ingest_queue,
        }
    }

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
    ///  All session-log inputs are re-queued in chronological order ahead of the
    ///  current pending delay line so the new pending state re-observes them.
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    /// Resolve keeps other-key inputs and the last self event,
    ///  then prepends them *before* any existing delay-line tail.
    ///
    /// Motivating smart keys: **tap-hold** (esp. rolling `HoldOnKeyTap`)
    ///  and **chorded** over tap-hold — without prepend-before-tail,
    ///  `tests/rust/tap_hold/hold_on_interrupt_tap` and
    ///  `tests/rust/chorded/{tap_hold,tap_hold_over_tap_hold,over_layered_tap_hold}`
    ///  fail.
    #[test]
    fn resolve_replay_prepends_filtered_inputs_before_delay_line_tail() {
        // Assemble -- session log: Press(0), Press(1), Release(0);
        //  delay-line tail: Press(9).
        let mut queued: heapless::Vec<key::Event<()>, 8> = heapless::Vec::new();
        queued.push(input::Event::press(0).into()).unwrap();
        queued.push(input::Event::press(1).into()).unwrap();
        queued.push(input::Event::release(0).into()).unwrap();

        let mut input_queue = InputEventQueue::<8>::new();
        input_queue.push_back(input::Event::press(9)).unwrap();

        let mut event_scheduler = EventScheduler::new();

        // Act -- resolve key 0 → keep Press(1) + last self event Release(0).
        dispatch_replayed_events(
            KeyResolution::Resolved { keymap_index: 0 },
            &mut queued,
            &mut input_queue,
            &mut event_scheduler,
        );

        // Assert -- replay batch, then original tail.
        assert_eq!(
            input_queue.pop_front_if_ready(),
            Some(input::Event::press(1))
        );
        assert_eq!(
            input_queue.pop_front_if_ready(),
            Some(input::Event::release(0))
        );
        assert_eq!(
            input_queue.pop_front_if_ready(),
            Some(input::Event::press(9))
        );
        assert!(input_queue.is_empty());
    }

    /// Non-input session-log events are scheduled with staggered delay,
    ///  not prepended to the delay line.
    ///
    /// Motivating smart keys: any pending key that records non-input
    ///  `Event::Key` traffic in the session log before resolve
    ///  (e.g. **tap-hold** / **chorded** composites that schedule key events).
    ///  Stagger keeps HID reports one tick apart after resolve.
    #[test]
    fn resolve_replay_schedules_non_input_events_with_staggered_delay() {
        // Assemble
        let mut queued: heapless::Vec<key::Event<()>, 8> = heapless::Vec::new();
        queued.push(input::Event::press(1).into()).unwrap();
        queued.push(key::Event::key_event(2, ())).unwrap();
        queued.push(key::Event::key_event(3, ())).unwrap();

        let mut input_queue = InputEventQueue::<8>::new();
        let mut event_scheduler = EventScheduler::new();

        // Act
        dispatch_replayed_events(
            KeyResolution::Resolved { keymap_index: 0 },
            &mut queued,
            &mut input_queue,
            &mut event_scheduler,
        );

        // Assert -- other-key input prepended; key events scheduled.
        assert_eq!(
            input_queue.pop_front_if_ready(),
            Some(input::Event::press(1))
        );
        assert!(input_queue.is_empty());
        assert!(event_scheduler.next_event_time().is_some());
        event_scheduler.tick(1);
        assert_eq!(
            event_scheduler.dequeue(),
            Some(key::Event::key_event(2, ()))
        );
        event_scheduler.tick(1);
        assert_eq!(
            event_scheduler.dequeue(),
            Some(key::Event::key_event(3, ()))
        );
    }

    /// Nested-pending policy: chronological (FIFO) over input events only
    ///  (key events dropped),
    ///  prepended before any existing delay-line tail
    ///  (the current pending session's `ingest_queue`).
    ///
    /// Motivating smart keys: **chorded → passthrough pending**
    ///  (e.g. chorded over tap-hold) and other pending→pending
    ///  transitions (layered outer resolving into an inner pending key).
    #[test]
    fn nested_pending_replay_prepends_inputs_fifo_before_delay_line_tail() {
        // Assemble -- log: Press(0), KeyEvent, Release(0), Press(1);
        //  delay-line tail: Press(9).
        let mut queued: heapless::Vec<key::Event<()>, 8> = heapless::Vec::new();
        queued.push(input::Event::press(0).into()).unwrap();
        queued.push(key::Event::key_event(0, ())).unwrap();
        queued.push(input::Event::release(0).into()).unwrap();
        queued.push(input::Event::press(1).into()).unwrap();

        let mut input_queue = InputEventQueue::<8>::new();
        input_queue.push_back(input::Event::press(9)).unwrap();

        let mut event_scheduler = EventScheduler::new();

        // Act
        dispatch_replayed_events(
            KeyResolution::Pending,
            &mut queued,
            &mut input_queue,
            &mut event_scheduler,
        );

        // Assert -- chronological inputs, then original tail; no key-event scheduling.
        assert_eq!(
            input_queue.pop_front_if_ready(),
            Some(input::Event::press(0))
        );
        assert_eq!(
            input_queue.pop_front_if_ready(),
            Some(input::Event::release(0))
        );
        assert_eq!(
            input_queue.pop_front_if_ready(),
            Some(input::Event::press(1))
        );
        assert_eq!(
            input_queue.pop_front_if_ready(),
            Some(input::Event::press(9))
        );
        assert!(input_queue.is_empty());
        assert!(queued.is_empty());
        assert!(event_scheduler.next_event_time().is_none());
    }
}
