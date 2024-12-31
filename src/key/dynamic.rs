use core::fmt::Debug;

use crate::{input, key};

use key::{Key as _, PressedKey as _};

use key::simple;

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
pub struct DynamicKey {
    key: simple::Key,
    pressed_key: Option<input::PressedKey<simple::Key, <simple::Key as key::Key>::PressedKeyState>>,
}

impl DynamicKey {
    pub fn new(key: simple::Key) -> Self {
        Self {
            key,
            pressed_key: None,
        }
    }
}

impl<const N: usize> Key<simple::Event, N> for DynamicKey {
    type Context = ();
    type ContextEvent = ();

    fn handle_event(
        &mut self,
        context: &Self::Context,
        event: key::Event<simple::Event>,
    ) -> heapless::Vec<key::ScheduledEvent<simple::Event>, N> {
        let mut scheduled_events = heapless::Vec::new();

        if let Some(mut pressed_key) = self.pressed_key {
            scheduled_events.extend(
                pressed_key
                    .handle_event(event)
                    .into_iter()
                    .map(|ev| ScheduledEvent::immediate(ev)),
            );

            if let key::Event::Input(input::Event::Release { keymap_index }) = event {
                if keymap_index == pressed_key.keymap_index {
                    self.pressed_key = None;
                }
            }
        } else {
            if let key::Event::Input(input::Event::Press { keymap_index }) = event {
                let (pressed_key, new_events) = self.key.new_pressed_key(context, keymap_index);
                scheduled_events.extend(new_events.into_iter());
                self.pressed_key = Some(pressed_key);
            }
        }

        scheduled_events
    }

    fn key_code(&self) -> Option<u8> {
        if let Some(pressed_key) = self.pressed_key {
            pressed_key.key_code()
        } else {
            None
        }
    }
}
