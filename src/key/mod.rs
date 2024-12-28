use core::fmt::Debug;

use crate::input;

pub mod layered;
pub mod simple;
pub mod tap_hold;

pub mod composite;

pub trait Key<PK: Key = Self>
where
    Self::ContextEvent: From<Self::Event>,
{
    type Context: Context<Event = Self::ContextEvent>;
    type ContextEvent;
    type PressedKey: PressedKey<PK, Event = Self::Event> + Debug;
    type Event: Copy + Debug + Ord;

    fn new_pressed_key(
        &self,
        context: &Self::Context,
        keymap_index: u16,
    ) -> (Self::PressedKey, Option<ScheduledEvent<Self::Event>>);
}

pub trait Context {
    type Event;
    fn handle_event(&mut self, event: Self::Event);
}

impl Context for () {
    type Event = ();
    fn handle_event(&mut self, _event: Self::Event) {}
}

pub trait PressedKey<K: Key> {
    type Event;
    fn handle_event(
        &mut self,
        key_definition: &K,
        event: Event<Self::Event>,
    ) -> impl IntoIterator<Item = Event<Self::Event>>;
    fn key_code(&self, key_definition: &K) -> Option<u8>;
}

#[allow(unused)]
pub enum EventError {
    UnmappableEvent,
}

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
