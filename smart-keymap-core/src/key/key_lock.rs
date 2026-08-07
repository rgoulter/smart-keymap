//! Key Lock: hold the next key until it is pressed again.
//!
//! After arming with `Key::KeyLock`, the next resolved keyboard key output is
//! kept held via a virtual key press. Pressing a key that produces the same
//! output again releases the lock.

use core::fmt::Debug;
use core::marker::PhantomData;

use serde::Deserialize;

use crate::input;
use crate::key;
use crate::keymap;

/// Maximum number of simultaneously locked key outputs.
pub const MAX_LOCKED_KEYS: usize = 4;

/// Reference for a key lock key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Ref(pub Key);

/// Whether a resolved [key::KeyOutput] can be locked.
///
/// Only non-empty keyboard usage outputs are lockable.
pub fn is_lockable(key_output: &key::KeyOutput) -> bool {
    if *key_output == key::KeyOutput::NO_OUTPUT {
        return false;
    }
    matches!(key_output.key_code(), key::KeyUsage::Keyboard(_))
}

/// Key awaiting physical release before its virtual lock is applied.
#[derive(Debug, Clone, Copy, PartialEq)]
struct PendingLock {
    keymap_index: u16,
    key_output: key::KeyOutput,
}

/// Key Lock context: watching arm and active virtual locks.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context {
    watching: bool,
    /// Next key to lock once its physical key is released (avoids duplicate HID codes).
    pending_lock: Option<PendingLock>,
    locked: [Option<key::KeyOutput>; MAX_LOCKED_KEYS],
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
            locked: [None; MAX_LOCKED_KEYS],
        }
    }

    /// Clear watching and all locked keys (does not emit virtual releases).
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// Whether key lock is watching for the next key to lock.
    pub fn is_watching(&self) -> bool {
        self.watching
    }

    /// Whether `key_output` is currently locked.
    pub fn is_locked(&self, key_output: &key::KeyOutput) -> bool {
        self.locked
            .iter()
            .any(|slot| slot.as_ref() == Some(key_output))
    }

    fn find_locked_slot(&self, key_output: &key::KeyOutput) -> Option<usize> {
        self.locked
            .iter()
            .position(|slot| slot.as_ref() == Some(key_output))
    }

    fn find_free_slot(&self) -> Option<usize> {
        self.locked.iter().position(|slot| slot.is_none())
    }

    fn lock(&mut self, key_output: key::KeyOutput) -> bool {
        if self.is_locked(&key_output) {
            return true;
        }
        if let Some(i) = self.find_free_slot() {
            self.locked[i] = Some(key_output);
            true
        } else {
            false
        }
    }

    fn unlock(&mut self, key_output: &key::KeyOutput) -> bool {
        if let Some(i) = self.find_locked_slot(key_output) {
            self.locked[i] = None;
            true
        } else {
            false
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
            }) => {
                if self.watching {
                    self.watching = false;
                    if is_lockable(&key_output) {
                        // Defer virtual press until physical release so the HID
                        // report does not carry a duplicate key code.
                        self.pending_lock = Some(PendingLock {
                            keymap_index,
                            key_output,
                        });
                    }
                    // Non-lockable keys cancel watching without locking.
                    key::KeyEvents::no_events()
                } else if is_lockable(&key_output) && self.unlock(&key_output) {
                    let vk_ev = input::Event::VirtualKeyRelease { key_output };
                    key::KeyEvents::event(key::Event::Input(vk_ev))
                } else {
                    key::KeyEvents::no_events()
                }
            }
            key::Event::Input(input::Event::Release { keymap_index }) => {
                if let Some(PendingLock {
                    keymap_index: pending_index,
                    key_output,
                }) = self.pending_lock
                {
                    if pending_index == keymap_index {
                        self.pending_lock = None;
                        if self.lock(key_output) {
                            let vk_ev = input::Event::VirtualKeyPress { key_output };
                            return key::KeyEvents::event(key::Event::Input(vk_ev));
                        }
                    }
                }
                key::KeyEvents::no_events()
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
    fn toggle_watching() {
        let mut ctx = Context::new();
        assert!(!ctx.is_watching());

        let _ =
            key::Context::handle_event(&mut ctx, key::Event::key_event(0, Event::ToggleWatching));
        assert!(ctx.is_watching());

        let _ =
            key::Context::handle_event(&mut ctx, key::Event::key_event(0, Event::ToggleWatching));
        assert!(!ctx.is_watching());
    }

    #[test]
    fn watching_defers_lock_until_physical_release() {
        let mut ctx = Context::new();
        let _ =
            key::Context::handle_event(&mut ctx, key::Event::key_event(0, Event::ToggleWatching));

        let key_output = key::KeyOutput::from_key_code(0x04);
        let pke = key::Context::handle_event(
            &mut ctx,
            key::Event::Keymap(keymap::KeymapEvent::ResolvedKeyOutput {
                keymap_index: 1,
                key_output,
            }),
        );

        assert!(!ctx.is_watching());
        assert!(!ctx.is_locked(&key_output));
        assert_eq!(0, pke.into_iter().count());
        assert_eq!(
            Some(PendingLock {
                keymap_index: 1,
                key_output,
            }),
            ctx.pending_lock
        );

        let pke = key::Context::handle_event(
            &mut ctx,
            key::Event::Input(input::Event::Release { keymap_index: 1 }),
        );

        assert!(ctx.is_locked(&key_output));
        assert_eq!(None, ctx.pending_lock);
        let events: heapless::Vec<_, 4> = pke.into_iter().collect();
        assert_eq!(1, events.len());
        assert_eq!(
            key::Event::Input(input::Event::VirtualKeyPress { key_output }),
            events[0].event
        );
    }

    #[test]
    fn second_press_unlocks_and_emits_vk_release() {
        let mut ctx = Context::new();
        let key_output = key::KeyOutput::from_key_code(0x04);
        assert!(ctx.lock(key_output));

        let pke = key::Context::handle_event(
            &mut ctx,
            key::Event::Keymap(keymap::KeymapEvent::ResolvedKeyOutput {
                keymap_index: 1,
                key_output,
            }),
        );

        assert!(!ctx.is_locked(&key_output));
        let events: heapless::Vec<_, 4> = pke.into_iter().collect();
        assert_eq!(1, events.len());
        assert_eq!(
            key::Event::Input(input::Event::VirtualKeyRelease { key_output }),
            events[0].event
        );
    }

    #[test]
    fn non_lockable_cancels_watching() {
        let mut ctx = Context::new();
        ctx.watching = true;

        let key_output = key::KeyOutput::from_consumer_code(0x01);
        let pke = key::Context::handle_event(
            &mut ctx,
            key::Event::Keymap(keymap::KeymapEvent::ResolvedKeyOutput {
                keymap_index: 1,
                key_output,
            }),
        );

        assert!(!ctx.is_watching());
        assert!(!ctx.is_locked(&key_output));
        assert_eq!(None, ctx.pending_lock);
        assert_eq!(0, pke.into_iter().count());
    }

    #[test]
    fn is_lockable_keyboard_only() {
        assert!(is_lockable(&key::KeyOutput::from_key_code(0x04)));
        assert!(is_lockable(&key::KeyOutput::from_key_code(0xE1))); // Left Shift
        assert!(!is_lockable(&key::KeyOutput::NO_OUTPUT));
        assert!(!is_lockable(&key::KeyOutput::from_consumer_code(1)));
    }
}
