use core::fmt::Debug;
use core::marker::PhantomData;

use serde::Deserialize;

use crate::input;
use crate::key;
use crate::keymap;

/// Reference for a caps word key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Ref(pub Key);

/// Caps Word context.
#[derive(Debug, Clone, Copy)]
pub struct Context {
    is_active: bool,
}

impl Context {
    /// Constructs a new [Context].
    pub const fn new() -> Self {
        Context { is_active: false }
    }

    /// Updates the context with the given event.
    fn handle_event(&mut self, event: key::Event<Event>) -> key::KeyEvents<Event> {
        match event {
            key::Event::Keymap(keymap::KeymapEvent::ResolvedKeyOutput {
                key_output:
                    key::KeyOutput {
                        key_code: key::KeyUsage::Keyboard(key_code),
                        key_modifiers,
                    },
                ..
            }) if self.is_active => {
                // CapsWord is deactivated for key presses other than:
                //   - A-Z
                //   - 0-9
                //   - Backspace, Delete
                //   - Minus, Underscore
                let is_shifted = key_modifiers.has_modifiers(
                    &key::KeyboardModifiers::LEFT_SHIFT.union(&key::KeyboardModifiers::RIGHT_SHIFT),
                );
                let exit_caps_word = match key_code {
                    0x04..=0x1D => false,                // A-Z
                    0x1E..=0x27 if !is_shifted => false, // 0-9
                    0x2A => false,                       // Backspace
                    0x2D => false,                       // `-` minus
                    0x4C => false,                       // Delete
                    0xE1 => false,                       // Left Shift
                    0xE5 => false,                       // Right Shift
                    0x00 => false,                       // No key code (modifier)
                    _ => true,
                };

                if exit_caps_word {
                    self.is_active = false;

                    let key_code = 0xE1;
                    let vk_ev = input::Event::VirtualKeyRelease {
                        key_output: key::KeyOutput::from_key_code(key_code),
                    };
                    key::KeyEvents::event(key::Event::Input(vk_ev))
                } else {
                    key::KeyEvents::no_events()
                }
            }
            key::Event::Key { key_event, .. } => match key_event {
                Event::EnableCapsWord => {
                    self.is_active = true;

                    let key_code = 0xE1;
                    let vk_ev = input::Event::VirtualKeyPress {
                        key_output: key::KeyOutput::from_key_code(key_code),
                    };
                    key::KeyEvents::event(key::Event::Input(vk_ev))
                }
                Event::DisableCapsWord => {
                    self.is_active = false;

                    let key_code = 0xE1;
                    let vk_ev = input::Event::VirtualKeyRelease {
                        key_output: key::KeyOutput::from_key_code(key_code),
                    };
                    key::KeyEvents::event(key::Event::Input(vk_ev))
                }
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
}

/// Caps Word events.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// Enables Caps Word.
    EnableCapsWord,
    /// Disables Caps Word.
    DisableCapsWord,
}

/// A key for HID Keyboard usage codes.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Key {
    /// Enters/Exits CapsWord mode.
    ToggleCapsWord,
}

impl Key {
    /// Constructs a key with the given key_code.
    pub const fn new() -> Self {
        Key::ToggleCapsWord
    }

    /// Constructs a pressed key state
    pub fn new_pressed_key(&self, context: &Context, keymap_index: u16) -> key::KeyEvents<Event> {
        let key_event = match self {
            Key::ToggleCapsWord => {
                if context.is_active {
                    Event::DisableCapsWord
                } else {
                    Event::EnableCapsWord
                }
            }
        };
        key::KeyEvents::event(key::Event::key_event(keymap_index, key_event))
    }
}

impl Default for Key {
    fn default() -> Self {
        Self::new()
    }
}

/// The pending key state type for caps word keys. (No pending state).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PendingKeyState;

/// Key state used by [System].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState;

/// The [key::System] implementation for caps word keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct System<R>(PhantomData<R>);

impl<R> System<R> {
    /// Constructs a new [System] with the given key data.
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
        context: &Self::Context,
        Ref(key): Ref,
    ) -> (
        key::PressedKeyResult<R, Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        let pke = key.new_pressed_key(context, keymap_index);
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
}
