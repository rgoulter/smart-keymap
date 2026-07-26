//! History keys: behaviours that depend on previously resolved key output.
//!
//! v1 provides [Key::Repeat](crate::key::history::Key::Repeat), which re-emits
//! the last remembered [crate::key::KeyOutput] as the pressed key's own output
//! while held.

use core::fmt::Debug;
use core::marker::PhantomData;

use serde::Deserialize;

use crate::key;
use crate::keymap;

/// Reference for a history key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Ref(pub Key);

/// History key kinds.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Key {
    /// Re-emit the last remembered key output while pressed.
    Repeat,
}

impl Key {
    /// Constructs a [Key::Repeat].
    pub const fn new_repeat() -> Self {
        Key::Repeat
    }
}

/// Whether a resolved [key::KeyOutput] should become the remembered last output.
///
/// Empty / no-op outputs are ignored so that pressing Repeat with no history
/// (or keys that resolve without output) does not clear a prior memory.
pub fn is_rememberable(key_output: &key::KeyOutput) -> bool {
    *key_output != key::KeyOutput::NO_OUTPUT
}

/// Context for history keys: tracks the last rememberable resolved output.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context {
    last: Option<key::KeyOutput>,
}

impl Default for Context {
    fn default() -> Self {
        Self::new()
    }
}

impl Context {
    /// Constructs a new [Context].
    pub const fn new() -> Self {
        Context { last: None }
    }

    /// Clear remembered history.
    pub fn reset(&mut self) {
        *self = Self::new();
    }

    /// The last rememberable resolved key output, if any.
    pub fn last(&self) -> Option<key::KeyOutput> {
        self.last
    }

    fn handle_event(&mut self, event: key::Event<Event>) -> key::KeyEvents<Event> {
        if let key::Event::Keymap(keymap::KeymapEvent::ResolvedKeyOutput { key_output, .. }) = event
        {
            if is_rememberable(&key_output) {
                self.last = Some(key_output);
            }
        }
        key::KeyEvents::no_events()
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

/// Events for history keys. (None for v1.)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Event;

/// Pending key state type for history keys. (No pending state.)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PendingKeyState;

/// Pressed state: the output being repeated (if any) for the hold duration.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState {
    output: Option<key::KeyOutput>,
}

impl KeyState {
    /// Constructs a key state with the given output.
    pub const fn new(output: Option<key::KeyOutput>) -> Self {
        Self { output }
    }

    /// The output this pressed history key contributes, if any.
    pub const fn output(&self) -> Option<key::KeyOutput> {
        self.output
    }
}

/// The [key::System] implementation for history keys.
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
        _keymap_index: u16,
        context: &Self::Context,
        Ref(key): Ref,
    ) -> (
        key::PressedKeyResult<R, Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        let output = match key {
            Key::Repeat => context.last(),
        };
        (
            key::PressedKeyResult::Resolved(KeyState::new(output)),
            key::KeyEvents::no_events(),
        )
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

    fn key_output(
        &self,
        _key_ref: &Self::Ref,
        key_state: &Self::KeyState,
    ) -> Option<key::KeyOutput> {
        key_state.output()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::key::System as _;

    #[test]
    fn test_sizeof_ref() {
        assert_eq!(0, core::mem::size_of::<Ref>());
    }

    #[test]
    fn test_sizeof_event() {
        assert_eq!(0, core::mem::size_of::<Event>());
    }

    #[test]
    fn context_remembers_resolved_keyboard_output() {
        let mut ctx = Context::new();
        assert_eq!(None, ctx.last());

        let key_output = key::KeyOutput::from_key_code(0x04);
        let _ = key::Context::handle_event(
            &mut ctx,
            key::Event::Keymap(keymap::KeymapEvent::ResolvedKeyOutput {
                keymap_index: 0,
                key_output,
            }),
        );

        assert_eq!(Some(key_output), ctx.last());
    }

    #[test]
    fn context_ignores_empty_output() {
        let mut ctx = Context::new();
        let remembered = key::KeyOutput::from_key_code(0x04);
        ctx.last = Some(remembered);

        let _ = key::Context::handle_event(
            &mut ctx,
            key::Event::Keymap(keymap::KeymapEvent::ResolvedKeyOutput {
                keymap_index: 0,
                key_output: key::KeyOutput::NO_OUTPUT,
            }),
        );

        assert_eq!(Some(remembered), ctx.last());
    }

    #[test]
    fn repeat_pressed_key_uses_context_last() {
        let system = System::<()>::new();
        let mut ctx = Context::new();
        let key_output = key::KeyOutput::from_key_code(0x05);
        ctx.last = Some(key_output);

        let (pkr, _) = system.new_pressed_key(0, &ctx, Ref(Key::Repeat));
        let ks = pkr.unwrap_resolved();
        assert_eq!(Some(key_output), system.key_output(&Ref(Key::Repeat), &ks));
    }
}
