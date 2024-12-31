use core::fmt::Debug;
use core::marker::PhantomData;

use crate::{input, key};

use key::{Key as _, PressedKey as _};

use key::{composite, simple};

use super::ScheduledEvent;

/// A dyn-compatible Key trait.
pub trait Key<Ev, const N: usize = 2>: Debug
where
    Ev: Copy + Debug + Ord,
    Self::ContextEvent: From<Ev>,
{
    type Context: key::Context<Event = Self::ContextEvent>;
    type ContextEvent;

    /// Handles events in two cases:
    /// - an unpressed key will only receive [input::Event::Press]
    ///   for the keymap index for that key.
    ///   (i.e. an unpressed key won't receive [input::Event::Press] for other keys,
    ///    nor other [input::Event] types),
    /// - a pressed key will receive all kinds of [input::Event].
    fn handle_event(
        &mut self,
        context: &Self::Context,
        event: key::Event<Ev>,
    ) -> heapless::Vec<key::ScheduledEvent<Ev>, N>;

    fn key_code(&self) -> Option<u8>;
}

#[derive(Debug)]
pub struct DynamicKey<K: key::Key, Ctx, Ev> {
    key: K,
    pressed_key: Option<input::PressedKey<K, K::PressedKeyState>>,

    _context_type: PhantomData<Ctx>,
    _event_type: PhantomData<Ev>,
}

impl<K: key::Key, Ctx, Ev> DynamicKey<K, Ctx, Ev> {
    pub fn new(key: K) -> Self {
        Self {
            key,
            pressed_key: None,

            _context_type: PhantomData,
            _event_type: PhantomData,
        }
    }
}

impl<
        K: key::Key,
        Ctx: key::Context<Event = Ev> + Debug,
        Ev: Copy + Debug + Ord,
        const N: usize,
    > Key<Ev, N> for DynamicKey<K, Ctx, Ev>
where
    key::Event<K::Event>: TryFrom<key::Event<Ev>>,
    key::ScheduledEvent<Ev>: From<key::ScheduledEvent<K::Event>>,
    for<'c, 'k> &'k K::Context: From<&'c Ctx>,
{
    type Context = Ctx;
    type ContextEvent = Ev;

    fn handle_event(
        &mut self,
        context: &Self::Context,
        event: key::Event<Ev>,
    ) -> heapless::Vec<key::ScheduledEvent<Ev>, N> {
        let mut scheduled_events: heapless::Vec<key::ScheduledEvent<Ev>, N> = heapless::Vec::new();

        if let Some(ref mut pressed_key) = &mut self.pressed_key {
            if let Ok(event) = event.try_into() {
                scheduled_events.extend(
                    pressed_key
                        .handle_event(event)
                        .into_iter()
                        .map(|ev| ScheduledEvent::immediate(ev).into()),
                );
            }

            if let key::Event::Input(input::Event::Release { keymap_index }) = event {
                if keymap_index == pressed_key.keymap_index {
                    self.pressed_key = None;
                }
            }
        } else if let key::Event::Input(input::Event::Press { keymap_index }) = event {
            let (pressed_key, new_events) = self.key.new_pressed_key(context.into(), keymap_index);
            scheduled_events.extend(new_events.into_iter().map(|sch_ev| sch_ev.into()));
            self.pressed_key = Some(pressed_key);
        }

        scheduled_events
    }

    fn key_code(&self) -> Option<u8> {
        if let Some(pressed_key) = &self.pressed_key {
            pressed_key.key_code()
        } else {
            None
        }
    }
}
