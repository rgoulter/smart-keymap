//! This module implements the `keymap::Key` for a 'composite' key,
//!  which can be any of the other key definitions,
//!  and is the default Key for the `keymap::KeyMap` implementation.

use serde::Deserialize;

use core::fmt::Debug;

use crate::key;
use key::{simple, tap_hold};

#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Key {
    Simple(simple::Key),
    TapHold(tap_hold::Key),
}

impl key::Key for Key {
    type Context = Context;
    type Event = Event;
    type PressedKey = PressedKey;

    fn new_pressed_key(
        &self,
        _context: &Self::Context,
        keymap_index: u16,
    ) -> (PressedKey, Option<key::ScheduledEvent<Event>>) {
        match self {
            Key::Simple(k) => {
                let pressed_key = simple::PressedKey::new(k.key_code());
                (pressed_key.into(), None)
            }
            Key::TapHold(_) => {
                let (pressed_key, new_event) = tap_hold::PressedKey::new(keymap_index);
                (pressed_key.into(), Some(new_event.into()))
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Context {}

impl Context {
    pub const fn new() -> Self {
        Self {}
    }
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl key::Context for Context {
    type Event = Event;
    fn handle_event(&mut self, _event: Self::Event) {}
}

#[derive(Debug, Clone, Copy)]
pub enum PressedKey {
    Simple(simple::PressedKey),
    TapHold(tap_hold::PressedKey),
}

impl From<simple::PressedKey> for PressedKey {
    fn from(pk: simple::PressedKey) -> Self {
        PressedKey::Simple(pk)
    }
}

impl From<tap_hold::PressedKey> for PressedKey {
    fn from(pk: tap_hold::PressedKey) -> Self {
        PressedKey::TapHold(pk)
    }
}

impl key::PressedKey for PressedKey {
    type Event = Event;
    type Key = Key;

    fn key_code(&self, key_definition: &Key) -> Option<u8> {
        match self {
            PressedKey::Simple(pk) => match key_definition {
                Key::Simple(key_def) => Some(pk.key_code(key_def)),
                _ => None,
            },

            PressedKey::TapHold(pk) => match key_definition {
                Key::TapHold(key_def) => pk.key_code(key_def),
                _ => None,
            },
        }
    }

    fn handle_event(
        &mut self,
        key_definition: &Key,
        event: key::Event<Self::Event>,
    ) -> impl IntoIterator<Item = key::Event<Self::Event>> {
        if let PressedKey::TapHold(tap_hold) = self {
            if let Key::TapHold(key_def) = key_definition {
                if let Ok(ev) = key::Event::try_from(event) {
                    let events: heapless::Vec<key::Event<tap_hold::Event>, 2> =
                        tap_hold.handle_event(key_def, ev);
                    events.into_iter().map(|ev| ev.into()).collect()
                } else {
                    heapless::Vec::<key::Event<Self::Event>, 2>::new()
                }
            } else {
                heapless::Vec::new()
            }
        } else {
            heapless::Vec::new()
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub enum Event {
    TapHold(tap_hold::Event),
}

impl From<key::Event<tap_hold::Event>> for key::Event<Event> {
    fn from(ev: key::Event<tap_hold::Event>) -> Self {
        match ev {
            key::Event::Input(ev) => key::Event::Input(ev),
            key::Event::Key(ev) => key::Event::Key(Event::TapHold(ev)),
        }
    }
}

impl From<key::ScheduledEvent<tap_hold::Event>> for key::ScheduledEvent<Event> {
    fn from(ev: key::ScheduledEvent<tap_hold::Event>) -> Self {
        Self {
            schedule: ev.schedule,
            event: ev.event.into(),
        }
    }
}

impl TryFrom<key::Event<Event>> for key::Event<tap_hold::Event> {
    type Error = key::EventError;

    fn try_from(ev: key::Event<Event>) -> Result<Self, Self::Error> {
        match ev {
            key::Event::Input(ev) => Ok(key::Event::Input(ev)),
            key::Event::Key(Event::TapHold(ev)) => Ok(key::Event::Key(ev)),
        }
    }
}
