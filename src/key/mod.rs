use crate::input;

pub mod simple;
pub mod tap_hold;

pub mod composite;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Event<T> {
    Input(input::Event),
    Key(T),
}

impl<T> From<input::Event> for Event<T> {
    fn from(event: input::Event) -> Self {
        Event::Input(event)
    }
}

#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Schedule {
    Immediate,
    After(u16),
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct ScheduledEvent<T> {
    pub schedule: Schedule,
    pub event: Event<T>,
}

impl<T> ScheduledEvent<T> {
    #[allow(unused)]
    pub fn immediate(event: Event<T>) -> Self {
        ScheduledEvent {
            schedule: Schedule::Immediate,
            event,
        }
    }
    pub fn after(delay: u16, event: Event<T>) -> Self {
        ScheduledEvent {
            schedule: Schedule::After(delay),
            event,
        }
    }
}
