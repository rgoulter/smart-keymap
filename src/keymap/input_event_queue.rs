use core::fmt::Debug;

use crate::input;

/// Fixed-capacity FIFO queue for [input::Event]s with tick-based processing delay.
///
/// Backed by a [`heapless::Deque`] so events can be prepended when a pending key
/// state needs to replay buffered input without draining and rebuilding the queue.
#[derive(Debug)]
pub(crate) struct InputEventQueue<const N: usize> {
    events: heapless::Deque<input::Event, N>,
    delay_counter: u8,
}

#[allow(dead_code)]
impl<const N: usize> InputEventQueue<N> {
    pub const fn new() -> Self {
        Self {
            events: heapless::Deque::new(),
            delay_counter: 0,
        }
    }

    pub fn clear(&mut self) {
        self.events.clear();
        self.delay_counter = 0;
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

    pub fn set_delay_counter(&mut self, delay_counter: u8) {
        self.delay_counter = delay_counter;
    }

    pub fn tick_delay(&mut self) {
        if self.delay_counter > 0 {
            self.delay_counter -= 1;
        }
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
        core::mem::replace(&mut self.events, heapless::Deque::new())
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
    /// preserving the historical behaviour of the pending-state re-queue path.
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
        !self.is_empty() && self.delay_counter == 0
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
    use super::super::INPUT_QUEUE_TICK_DELAY;
    use super::*;

    const CAPACITY: usize = 4;

    fn press(index: u16) -> input::Event {
        input::Event::Press {
            keymap_index: index,
        }
    }

    fn release(index: u16) -> input::Event {
        input::Event::Release {
            keymap_index: index,
        }
    }

    #[test]
    fn push_back_and_pop_front_if_ready_respects_delay() {
        let mut queue = InputEventQueue::<CAPACITY>::new();

        queue.push_back(press(0)).unwrap();
        assert_eq!(queue.pop_front_if_ready(), Some(press(0)));
        queue.set_delay_counter(INPUT_QUEUE_TICK_DELAY);

        queue.push_back(release(0)).unwrap();
        assert_eq!(queue.pop_front_if_ready(), None);

        queue.tick_delay();
        assert_eq!(queue.pop_front_if_ready(), Some(release(0)));
    }

    #[test]
    fn take_all_drains_queue_preserving_order() {
        let mut queue = InputEventQueue::<CAPACITY>::new();
        queue.push_back(press(0)).unwrap();
        queue.push_back(release(0)).unwrap();

        let mut taken = queue.take_all();
        assert!(queue.is_empty());
        assert_eq!(taken.pop_front(), Some(press(0)));
        assert_eq!(taken.pop_front(), Some(release(0)));
        assert_eq!(taken.pop_front(), None);
    }

    #[test]
    fn append_all_extends_back_in_order() {
        let mut queue = InputEventQueue::<CAPACITY>::new();
        queue.push_back(press(0)).unwrap();

        let mut other = heapless::Deque::new();
        other.push_back(release(0)).unwrap();
        other.push_back(press(1)).unwrap();

        queue.append_all(&mut other);
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.pop_front_if_ready(), Some(press(0)));
        queue.set_delay_counter(0);
        assert_eq!(queue.pop_front_if_ready(), Some(release(0)));
        assert_eq!(queue.pop_front_if_ready(), Some(press(1)));
    }

    #[test]
    fn prepend_inserts_events_at_front_in_order() {
        let mut queue = InputEventQueue::<CAPACITY>::new();
        queue.push_back(press(1)).unwrap();
        queue.prepend(&[press(0), release(0)]);

        assert_eq!(queue.pop_front_if_ready(), Some(press(0)));
        assert_eq!(queue.pop_front_if_ready(), Some(release(0)));
        assert_eq!(queue.pop_front_if_ready(), Some(press(1)));
    }

    #[test]
    fn prepend_pending_input_events_replays_lifo_then_restores_tail() {
        let mut queue = InputEventQueue::<CAPACITY>::new();
        queue.push_back(press(9)).unwrap();

        let mut pending_events: heapless::Vec<key::Event<()>, 4> = heapless::Vec::new();
        pending_events.push(key::Event::Input(press(0))).unwrap();
        pending_events.push(key::Event::Input(release(0))).unwrap();
        pending_events
            .push(key::Event::Key {
                keymap_index: 0,
                key_event: (),
            })
            .unwrap();

        queue.prepend_pending_input_events(&mut pending_events);

        assert_eq!(queue.pop_front_if_ready(), Some(release(0)));
        assert_eq!(queue.pop_front_if_ready(), Some(press(0)));
        assert_eq!(queue.pop_front_if_ready(), Some(press(9)));
        assert!(queue.is_empty());
    }

    #[test]
    fn is_full_when_at_capacity() {
        let mut queue = InputEventQueue::<CAPACITY>::new();
        for index in 0..CAPACITY {
            queue.push_back(press(index as u16)).unwrap();
        }
        assert!(queue.is_full());
        assert!(queue.push_back(press(99)).is_err());
    }

    #[test]
    fn push_back_or_ignore_drops_when_at_capacity() {
        let mut queue = InputEventQueue::<2>::new();
        queue.push_back(press(0)).unwrap();
        queue.push_back_or_ignore(press(1));
        queue.push_back_or_ignore(press(2));

        assert_eq!(queue.pop_front_if_ready(), Some(press(0)));
        assert_eq!(queue.pop_front_if_ready(), Some(press(1)));
        assert_eq!(queue.pop_front_if_ready(), None);
    }

    #[test]
    fn push_front_or_ignore_drops_when_at_capacity() {
        let mut queue = InputEventQueue::<2>::new();
        queue.push_back(press(0)).unwrap();
        queue.push_front_or_ignore(press(1));
        queue.push_front_or_ignore(press(2));

        assert_eq!(queue.pop_front_if_ready(), Some(press(1)));
        assert_eq!(queue.pop_front_if_ready(), Some(press(0)));
        assert_eq!(queue.pop_front_if_ready(), None);
    }
}
