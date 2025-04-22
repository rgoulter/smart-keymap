use core::cmp::PartialEq;
use core::fmt::Debug;
use core::marker::Copy;

use crate::key;

use key::Event;

pub(crate) const MAX_PENDING_EVENTS: usize = 32;
pub(crate) const MAX_SCHEDULED_EVENTS: usize = 32;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) struct ScheduledEvent<E: Debug> {
    time: u32,
    event: Event<E>,
}

#[derive(Debug)]
pub(crate) struct EventScheduler<E: Debug> {
    pub(crate) pending_events: heapless::spsc::Queue<Event<E>, { MAX_PENDING_EVENTS }>,
    pub(crate) scheduled_events: heapless::Vec<ScheduledEvent<E>, { MAX_SCHEDULED_EVENTS }>,
    pub(crate) schedule_counter: u32,
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

    pub fn tick(&mut self, delta_ms: u8) {
        self.schedule_counter += delta_ms as u32;
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
