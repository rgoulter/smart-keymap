use core::fmt::Debug;

use crate::input;

/// Fixed-capacity FIFO queue for [input::Event]s with tick-based processing delay.
///
/// Backed by a [`heapless::Deque`] so events can be prepended
///  when a pending key state needs to replay buffered input
///  without draining and rebuilding the queue.
///
/// After an event is processed, [`set_delay`] marks the queue so the next
///  event waits until [`tick_delay`] (called from `Keymap::tick`).
#[derive(Debug)]
pub(crate) struct InputEventQueue<const N: usize> {
    events: heapless::Deque<input::Event, N>,
    /// When true, the next event must wait for a tick before processing.
    delay: bool,
}

#[allow(dead_code)]
impl<const N: usize> InputEventQueue<N> {
    pub const fn new() -> Self {
        Self {
            events: heapless::Deque::new(),
            delay: false,
        }
    }

    pub fn clear(&mut self) {
        self.events.clear();
        self.delay = false;
    }

    pub fn is_full(&self) -> bool {
        self.events.is_full()
    }

    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    /// Arms the one-tick pacing gate so the next event waits for [`tick_delay`].
    pub fn set_delay(&mut self) {
        self.delay = true;
    }

    /// Returns whether the queue is currently waiting for a tick before processing.
    pub fn delay(&self) -> bool {
        self.delay
    }

    /// Clears the pacing gate (one tick of delay has elapsed).
    pub fn tick_delay(&mut self) {
        self.delay = false;
    }

    pub fn push_back(&mut self, event: input::Event) -> Result<(), input::Event> {
        self.events.push_back(event)
    }

    pub fn push_back_or_ignore(&mut self, event: input::Event) {
        let _ = self.push_back(event);
    }

    pub fn push_front(&mut self, event: input::Event) -> Result<(), input::Event> {
        self.events.push_front(event)
    }

    pub fn push_front_or_ignore(&mut self, event: input::Event) {
        let _ = self.push_front(event);
    }

    /// Prepends events so the first item in `events` is processed next.
    pub fn prepend(&mut self, events: &[input::Event]) {
        for event in events.iter().rev() {
            self.push_front_or_ignore(*event);
        }
    }

    /// Removes and returns every queued event, leaving the queue empty.
    pub fn take_all(&mut self) -> heapless::Deque<input::Event, N> {
        core::mem::take(&mut self.events)
    }

    /// Appends every event from `other` to the back of this queue.
    pub fn append_all(&mut self, other: &mut heapless::Deque<input::Event, N>) {
        while let Some(event) = other.pop_front() {
            self.push_back_or_ignore(event);
        }
    }

    /// Replays input events from a pending key's buffer ahead of any already-queued events.
    ///
    /// Events are popped LIFO from `pending_events` and appended to the front of the queue,
    ///  preserving the historical behaviour of the pending-state re-queue path.
    pub fn prepend_pending_input_events<Ev: Debug, const M: usize>(
        &mut self,
        pending_events: &mut heapless::Vec<key::Event<Ev>, M>,
    ) {
        let mut pending_inputs = heapless::Vec::<input::Event, M>::new();
        while let Some(event) = pending_events.pop() {
            if let key::Event::Input(input_event) = event {
                // If the temporary buffer is full, drop the current pending input.
                let _ = pending_inputs.push(input_event);
            }
        }
        self.prepend(&pending_inputs);
    }

    pub fn ready_to_process(&self) -> bool {
        !self.is_empty() && !self.delay
    }

    pub fn pop_front_if_ready(&mut self) -> Option<input::Event> {
        if self.ready_to_process() {
            self.events.pop_front()
        } else {
            None
        }
    }
}

use crate::key;

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    const CAPACITY: usize = 4;

    #[test]
    fn push_back_and_pop_front_if_ready_respects_delay() {
        let mut queue = InputEventQueue::<CAPACITY>::new();

        queue.push_back(input::Event::press(0)).unwrap();
        assert_eq!(queue.pop_front_if_ready(), Some(input::Event::press(0)));
        queue.set_delay();

        queue.push_back(input::Event::release(0)).unwrap();
        assert_eq!(queue.pop_front_if_ready(), None);

        queue.tick_delay();
        assert_eq!(queue.pop_front_if_ready(), Some(input::Event::release(0)));
    }

    #[test]
    fn take_all_drains_queue_preserving_order() {
        let mut queue = InputEventQueue::<CAPACITY>::new();
        queue.push_back(input::Event::press(0)).unwrap();
        queue.push_back(input::Event::release(0)).unwrap();

        let mut taken = queue.take_all();
        assert!(queue.is_empty());
        assert_eq!(taken.pop_front(), Some(input::Event::press(0)));
        assert_eq!(taken.pop_front(), Some(input::Event::release(0)));
        assert_eq!(taken.pop_front(), None);
    }

    #[test]
    fn append_all_extends_back_in_order() {
        let mut queue = InputEventQueue::<CAPACITY>::new();
        queue.push_back(input::Event::press(0)).unwrap();

        let mut other = heapless::Deque::new();
        other.push_back(input::Event::release(0)).unwrap();
        other.push_back(input::Event::press(1)).unwrap();

        queue.append_all(&mut other);
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.pop_front_if_ready(), Some(input::Event::press(0)));
        // No delay set: remaining events are ready immediately.
        assert_eq!(queue.pop_front_if_ready(), Some(input::Event::release(0)));
        assert_eq!(queue.pop_front_if_ready(), Some(input::Event::press(1)));
    }

    #[test]
    fn prepend_inserts_events_at_front_in_order() {
        let mut queue = InputEventQueue::<CAPACITY>::new();
        queue.push_back(input::Event::press(1)).unwrap();
        queue.prepend(&[input::Event::press(0), input::Event::release(0)]);

        assert_eq!(queue.pop_front_if_ready(), Some(input::Event::press(0)));
        assert_eq!(queue.pop_front_if_ready(), Some(input::Event::release(0)));
        assert_eq!(queue.pop_front_if_ready(), Some(input::Event::press(1)));
    }

    #[test]
    fn prepend_pending_input_events_replays_lifo_then_restores_tail() {
        let mut queue = InputEventQueue::<CAPACITY>::new();
        queue.push_back(input::Event::press(9)).unwrap();

        let mut pending_events: heapless::Vec<key::Event<()>, 4> = heapless::Vec::new();
        pending_events.push(input::Event::press(0).into()).unwrap();
        pending_events
            .push(input::Event::release(0).into())
            .unwrap();
        pending_events.push(key::Event::key_event(0, ())).unwrap();

        queue.prepend_pending_input_events(&mut pending_events);

        assert_eq!(queue.pop_front_if_ready(), Some(input::Event::release(0)));
        assert_eq!(queue.pop_front_if_ready(), Some(input::Event::press(0)));
        assert_eq!(queue.pop_front_if_ready(), Some(input::Event::press(9)));
        assert!(queue.is_empty());
    }

    #[test]
    fn is_full_when_at_capacity() {
        let mut queue = InputEventQueue::<CAPACITY>::new();
        for index in 0..CAPACITY {
            queue.push_back(input::Event::press(index as u16)).unwrap();
        }
        assert!(queue.is_full());
        assert!(queue.push_back(input::Event::press(99)).is_err());
    }

    #[test]
    fn push_back_or_ignore_drops_when_at_capacity() {
        let mut queue = InputEventQueue::<2>::new();
        queue.push_back(input::Event::press(0)).unwrap();
        queue.push_back_or_ignore(input::Event::press(1));
        queue.push_back_or_ignore(input::Event::press(2));

        assert_eq!(queue.pop_front_if_ready(), Some(input::Event::press(0)));
        assert_eq!(queue.pop_front_if_ready(), Some(input::Event::press(1)));
        assert_eq!(queue.pop_front_if_ready(), None);
    }

    #[test]
    fn push_front_or_ignore_drops_when_at_capacity() {
        let mut queue = InputEventQueue::<2>::new();
        queue.push_back(input::Event::press(0)).unwrap();
        queue.push_front_or_ignore(input::Event::press(1));
        queue.push_front_or_ignore(input::Event::press(2));

        assert_eq!(queue.pop_front_if_ready(), Some(input::Event::press(1)));
        assert_eq!(queue.pop_front_if_ready(), Some(input::Event::press(0)));
        assert_eq!(queue.pop_front_if_ready(), None);
    }
}
