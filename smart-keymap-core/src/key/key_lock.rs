//! Key Lock: hold the next key until it is pressed again.
//!
//! After arming with `Key::KeyLock`, the next resolved keyboard key output is
//! kept held via a virtual key press. Pressing a key that produces the same
//! output again releases the lock.
//!
//! Only one output is locked at a time (QMK-style). Arming again and locking a
//! different key replaces the previous lock. Simultaneous multi-lock can be
//! added later if needed.

use core::fmt::Debug;
use core::marker::PhantomData;

use serde::Deserialize;

use crate::input;
use crate::key;
use crate::keymap;

/// Reference for a key lock key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Ref(pub Key);

/// Whether a resolved [key::KeyOutput] can be locked.
///
/// Only non-empty keyboard usage outputs are lockable.
pub fn is_lockable(key_output: &key::KeyOutput) -> bool {
    *key_output != key::KeyOutput::NO_OUTPUT
        && matches!(key_output.key_code(), key::KeyUsage::Keyboard(_))
}

/// Key awaiting physical release before its virtual lock is applied.
#[derive(Debug, Clone, Copy, PartialEq)]
struct PendingLock {
    keymap_index: u16,
    key_output: key::KeyOutput,
}

/// Key Lock context: watching arm and at most one active virtual lock.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context {
    watching: bool,
    /// Next key to lock once its physical key is released (avoids duplicate HID codes).
    pending_lock: Option<PendingLock>,
    locked: Option<key::KeyOutput>,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    /// Constructs a new [Context].
    pub const fn new() -> Self {
        Context {
            watching: false,
            pending_lock: None,
            locked: None,
        }
    }

    /// Clear watching and the locked key (does not emit virtual releases).
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Whether key lock is watching for the next key to lock.
    pub fn is_watching(&self) -> bool {
        self.watching
    }

    /// Whether `key_output` is currently locked.
    pub fn is_locked(&self, key_output: &key::KeyOutput) -> bool {
        self.locked.as_ref() == Some(key_output)
    }

    /// Apply a pending lock after physical release.
    ///
    /// Replaces any previously locked output (releasing it virtually first).
    fn commit_lock(&mut self, key_output: key::KeyOutput) -> key::KeyEvents<Event> {
        match self.locked {
            Some(existing) if existing == key_output => key::KeyEvents::no_events(),
            Some(existing) => {
                self.locked = Some(key_output);
                let mut pke =
                    key::KeyEvents::event(key::Event::Input(input::Event::VirtualKeyRelease {
                        key_output: existing,
                    }));
                pke.add_event(key::Event::Input(input::Event::VirtualKeyPress {
                    key_output,
                }));
                pke
            }
            None => {
                self.locked = Some(key_output);
                key::KeyEvents::event(key::Event::Input(input::Event::VirtualKeyPress {
                    key_output,
                }))
            }
        }
    }

    /// Clear the lock if it matches `key_output`; returns whether it was unlocked.
    fn unlock_if_matches(&mut self, key_output: &key::KeyOutput) -> bool {
        match self.locked {
            Some(locked) if locked == *key_output => {
                self.locked = None;
                true
            }
            _ => false,
        }
    }

    fn handle_event(&mut self, event: key::Event<Event>) -> key::KeyEvents<Event> {
        match event {
            key::Event::Key {
                key_event: Event::ToggleWatching,
                ..
            } => {
                self.watching = !self.watching;
                // Cancelling watch also drops a pending lock that was never committed.
                if !self.watching {
                    self.pending_lock = None;
                }
                key::KeyEvents::no_events()
            }
            key::Event::Keymap(keymap::KeymapEvent::ResolvedKeyOutput {
                keymap_index,
                key_output,
            }) => match (self.watching, is_lockable(&key_output)) {
                (true, true) => {
                    // Defer virtual press until physical release so the HID
                    // report does not carry a duplicate key code.
                    self.watching = false;
                    self.pending_lock = Some(PendingLock {
                        keymap_index,
                        key_output,
                    });
                    key::KeyEvents::no_events()
                }
                (true, false) => {
                    // Non-lockable keys cancel watching without locking.
                    self.watching = false;
                    key::KeyEvents::no_events()
                }
                (false, true) if self.unlock_if_matches(&key_output) => {
                    key::KeyEvents::event(key::Event::Input(input::Event::VirtualKeyRelease {
                        key_output,
                    }))
                }
                (false, _) => key::KeyEvents::no_events(),
            },
            key::Event::Input(input::Event::Release { keymap_index }) => match self.pending_lock {
                Some(PendingLock {
                    keymap_index: pending_index,
                    key_output,
                }) if pending_index == keymap_index => {
                    self.pending_lock = None;
                    self.commit_lock(key_output)
                }
                _ => key::KeyEvents::no_events(),
            },
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

/// Key Lock events.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// Toggle whether the next resolved key should be locked.
    ToggleWatching,
}

/// A key that arms/disarms key lock watching.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Key {
    /// Enter/exit watching mode for the next key to lock.
    KeyLock,
}

impl Key {
    /// Constructs a [Key::KeyLock].
    pub const fn new() -> Self {
        Key::KeyLock
    }

    /// Constructs pressed-key events for this key.
    pub fn new_pressed_key(&self, keymap_index: u16) -> key::KeyEvents<Event> {
        match self {
            Key::KeyLock => {
                key::KeyEvents::event(key::Event::key_event(keymap_index, Event::ToggleWatching))
            }
        }
    }
}

impl Default for Key {
    fn default() -> Self {
        Self::new()
    }
}

/// Pending key state type for key lock keys. (No pending state.)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PendingKeyState;

/// Key state used by [System]. (No per-key state; behaviour is on [Context].)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState;

/// The [key::System] implementation for key lock keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct System<R>(PhantomData<R>);

impl<R> System<R> {
    /// Constructs a new [System].
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}

impl<R> Default for System<R> {
    fn default() -> Self {
        Self::new()
    }
}

impl<R: Debug> key::System<R> for System<R> {
    type Ref = Ref;
    type Context = Context;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        keymap_index: u16,
        _context: &Self::Context,
        Ref(key): Ref,
    ) -> (
        key::PressedKeyResult<R, Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        let pke = key.new_pressed_key(keymap_index);
        let pkr = key::PressedKeyResult::NewPressedKey(key::NewPressedKey::NoOp);
        (pkr, pke.into_events())
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
        assert_eq!(0, core::mem::size_of::<Ref>());
    }

    #[test]
    fn test_sizeof_event() {
        assert_eq!(0, core::mem::size_of::<Event>());
    }

    #[test]
    fn is_lockable_accepts_keyboard_keycode() {
        assert!(is_lockable(&key::KeyOutput::from_key_code(0x04)));
    }

    #[test]
    fn is_lockable_accepts_modifier() {
        assert!(is_lockable(&key::KeyOutput::from_key_code(0xE1)));
    }

    #[test]
    fn is_lockable_rejects_no_output() {
        assert!(!is_lockable(&key::KeyOutput::NO_OUTPUT));
    }

    #[test]
    fn is_lockable_rejects_consumer() {
        assert!(!is_lockable(&key::KeyOutput::from_consumer_code(1)));
    }

    #[test]
    fn toggle_watching_arms() {
        let mut ctx = Context::new();
        let _ =
            key::Context::handle_event(&mut ctx, key::Event::key_event(0, Event::ToggleWatching));
        assert!(ctx.is_watching());
    }

    #[test]
    fn toggle_watching_twice_disarms() {
        let mut ctx = Context::new();
        let _ =
            key::Context::handle_event(&mut ctx, key::Event::key_event(0, Event::ToggleWatching));
        let _ =
            key::Context::handle_event(&mut ctx, key::Event::key_event(0, Event::ToggleWatching));
        assert!(!ctx.is_watching());
    }

    #[test]
    fn non_lockable_cancels_watching() {
        let mut ctx = Context::new();
        let _ =
            key::Context::handle_event(&mut ctx, key::Event::key_event(0, Event::ToggleWatching));
        let _ = key::Context::handle_event(
            &mut ctx,
            key::Event::Keymap(keymap::KeymapEvent::ResolvedKeyOutput {
                keymap_index: 1,
                key_output: key::KeyOutput::from_consumer_code(0x01),
            }),
        );
        assert!(!ctx.is_watching());
    }
}
