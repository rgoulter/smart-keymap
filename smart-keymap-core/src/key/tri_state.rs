//! Tri-state keys: start / continue / interrupt.
//!
//! A tri-state key owns a session across later presses of the same key.
//! The first press *starts*
//!  (typically virtual-hold a modifier and tap a key),
//! later presses of the same key *continue*
//!  (tap again while the hold stays),
//! and any other resolved key *interrupts*
//!  (releases the hold).
//!
//! The physical key is a [`crate::key::NewPressedKey::NoOp`]; HID output is
//!  injected with [`crate::input::Event::VirtualKeyPress`] /
//!  [`crate::input::Event::VirtualKeyRelease`].
//! Hold is pressed before tap so the first report is the chord, not a
//!  naked tap.
//!
//! Classic use is Alt-Tab (a "swapper"): start holds Left Alt and taps Tab,
//!  continue taps Tab, interrupt releases Left Alt.

use core::fmt::Debug;
use core::marker::PhantomData;
use core::ops::Index;

use serde::Deserialize;

use crate::input;
use crate::key;
use crate::keymap;

/// Reference for a tri-state key (index into [System] key data).
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Ref(pub u8);

/// A tri-state key: virtual-hold `hold` across taps of `tap`.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key {
    /// Output held for the life of the session (start press, interrupt release).
    pub hold: key::KeyOutput,
    /// Output tapped on start and on each continue.
    pub tap: key::KeyOutput,
}

impl Key {
    /// Constructs a tri-state key.
    pub const fn new(hold: key::KeyOutput, tap: key::KeyOutput) -> Self {
        Self { hold, tap }
    }
}

/// Armed session: `hold` is virtually pressed until interrupt.
#[derive(Debug, Clone, Copy, PartialEq)]
struct Session {
    keymap_index: u16,
    hold: key::KeyOutput,
    tap: key::KeyOutput,
    tap_held: bool,
}

/// Tri-state context: at most one session is armed.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context {
    session: Option<Session>,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    /// Constructs an idle context.
    pub const fn new() -> Self {
        Context { session: None }
    }

    /// Clear the session without emitting virtual releases.
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Whether a session is currently armed.
    pub fn is_active(&self) -> bool {
        self.session.is_some()
    }

    /// Whether the armed session belongs to `keymap_index`.
    pub fn is_session_for(&self, keymap_index: u16) -> bool {
        matches!(self.session, Some(s) if s.keymap_index == keymap_index)
    }

    fn end_session(&mut self) -> key::KeyEvents<Event> {
        match self.session.take() {
            None => key::KeyEvents::no_events(),
            Some(session) => {
                let mut pke = key::KeyEvents::no_events();
                if session.tap_held {
                    pke.add_event(key::Event::Input(input::Event::VirtualKeyRelease {
                        key_output: session.tap,
                    }));
                }
                pke.add_event(key::Event::Input(input::Event::VirtualKeyRelease {
                    key_output: session.hold,
                }));
                pke
            }
        }
    }

    fn handle_event(&mut self, event: key::Event<Event>) -> key::KeyEvents<Event> {
        match event {
            key::Event::Key {
                keymap_index,
                key_event: Event::Start { hold, tap },
            } => {
                let mut pke = self.end_session();
                self.session = Some(Session {
                    keymap_index,
                    hold,
                    tap,
                    tap_held: true,
                });
                pke.add_event(key::Event::Input(input::Event::VirtualKeyPress {
                    key_output: hold,
                }));
                pke.add_event(key::Event::Input(input::Event::VirtualKeyPress {
                    key_output: tap,
                }));
                pke
            }
            key::Event::Key {
                keymap_index,
                key_event: Event::Continue,
            } => match self.session.as_mut() {
                Some(session) if session.keymap_index == keymap_index => {
                    session.tap_held = true;
                    key::KeyEvents::event(key::Event::Input(input::Event::VirtualKeyPress {
                        key_output: session.tap,
                    }))
                }
                _ => key::KeyEvents::no_events(),
            },
            key::Event::Input(input::Event::Release { keymap_index }) => {
                match self.session.as_mut() {
                    Some(session) if session.keymap_index == keymap_index && session.tap_held => {
                        session.tap_held = false;
                        key::KeyEvents::event(key::Event::Input(input::Event::VirtualKeyRelease {
                            key_output: session.tap,
                        }))
                    }
                    _ => key::KeyEvents::no_events(),
                }
            }
            key::Event::Keymap(keymap::KeymapEvent::ResolvedKeyOutput { keymap_index, .. }) => {
                match self.session {
                    Some(session) if session.keymap_index != keymap_index => self.end_session(),
                    _ => key::KeyEvents::no_events(),
                }
            }
            _ => key::KeyEvents::no_events(),
        }
    }
}

impl key::Context for Context {
    type Event = Event;

    fn handle_event(&mut self, event: key::Event<Self::Event>) -> key::KeyEvents<Self::Event> {
        self.handle_event(event)
    }

    fn reset(&mut self) {
        Context::reset(self);
    }
}

/// Tri-state events.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// First press (or a different tri-state key): arm `hold` and tap `tap`.
    Start {
        /// Output to virtual-hold for the session.
        hold: key::KeyOutput,
        /// Output to tap now.
        tap: key::KeyOutput,
    },
    /// Re-press of the armed key: tap again; `hold` stays.
    Continue,
}

/// Pending key state type for tri-state keys. (No pending state.)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PendingKeyState;

/// Key state used by [System]. (No per-key state; behaviour is on [Context].)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState;

/// The [key::System] implementation for tri-state keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct System<R, Keys: Index<usize, Output = Key>> {
    keys: Keys,
    marker: PhantomData<R>,
}

impl<R, Keys: Index<usize, Output = Key>> System<R, Keys> {
    /// Constructs a new [System] with the given key data.
    pub const fn new(keys: Keys) -> Self {
        Self {
            keys,
            marker: PhantomData,
        }
    }
}

impl<R: Debug, Keys: Debug + Index<usize, Output = Key>> key::System<R> for System<R, Keys> {
    type Ref = Ref;
    type Context = Context;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        keymap_index: u16,
        context: &Self::Context,
        Ref(key_index): Ref,
    ) -> (
        key::PressedKeyResult<R, Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        let Key { hold, tap } = self.keys[key_index as usize];
        let key_event = if context.is_session_for(keymap_index) {
            Event::Continue
        } else {
            Event::Start { hold, tap }
        };
        let pkr = key::PressedKeyResult::NewPressedKey(key::NewPressedKey::NoOp);
        let pke = key::KeyEvents::event(key::Event::key_event(keymap_index, key_event));
        (pkr, pke)
    }

    fn update_pending_state(
        &self,
        _pending_state: &mut Self::PendingKeyState,
        _keymap_index: u16,
        _context: &Self::Context,
        _key_ref: Ref,
        _event: key::Event<Self::Event>,
    ) -> (Option<key::NewPressedKey<R>>, key::KeyEvents<Self::Event>) {
        panic!()
    }

    fn update_state(
        &self,
        _key_state: &mut Self::KeyState,
        _ref: &Self::Ref,
        _context: &Self::Context,
        _keymap_index: u16,
        _event: key::Event<Self::Event>,
    ) -> key::KeyEvents<Self::Event> {
        panic!()
    }

    fn key_output(
        &self,
        _key_ref: &Self::Ref,
        _key_state: &Self::KeyState,
    ) -> Option<key::KeyOutput> {
        panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sizeof_ref() {
        assert_eq!(1, core::mem::size_of::<Ref>());
    }

    #[test]
    fn test_sizeof_event() {
        assert_eq!(14, core::mem::size_of::<Event>());
    }

    #[test]
    fn start_arms_session() {
        let mut ctx = Context::new();
        let hold = key::KeyOutput::from_key_code(key::KeyboardModifiers::HID_LEFT_ALT);
        let tap = key::KeyOutput::from_key_code(0x2B);
        let _ = key::Context::handle_event(
            &mut ctx,
            key::Event::key_event(0, Event::Start { hold, tap }),
        );
        assert!(ctx.is_active());
        assert!(ctx.is_session_for(0));
    }

    #[test]
    fn other_resolved_output_ends_session() {
        let mut ctx = Context::new();
        let hold = key::KeyOutput::from_key_code(key::KeyboardModifiers::HID_LEFT_ALT);
        let tap = key::KeyOutput::from_key_code(0x2B);
        let _ = key::Context::handle_event(
            &mut ctx,
            key::Event::key_event(0, Event::Start { hold, tap }),
        );
        let _ = key::Context::handle_event(
            &mut ctx,
            key::Event::Keymap(keymap::KeymapEvent::ResolvedKeyOutput {
                keymap_index: 1,
                key_output: key::KeyOutput::from_key_code(0x04),
            }),
        );
        assert!(!ctx.is_active());
    }

    #[test]
    fn own_resolved_output_does_not_end_session() {
        let mut ctx = Context::new();
        let hold = key::KeyOutput::from_key_code(key::KeyboardModifiers::HID_LEFT_ALT);
        let tap = key::KeyOutput::from_key_code(0x2B);
        let _ = key::Context::handle_event(
            &mut ctx,
            key::Event::key_event(0, Event::Start { hold, tap }),
        );
        let _ = key::Context::handle_event(
            &mut ctx,
            key::Event::Keymap(keymap::KeymapEvent::ResolvedKeyOutput {
                keymap_index: 0,
                key_output: tap,
            }),
        );
        assert!(ctx.is_active());
    }
}
