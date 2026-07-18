use core::fmt::Debug;
use core::marker::PhantomData;
use core::ops::Index;

use serde::Deserialize;

use crate::key;

/// Mouse action (button, cursor movement, or wheel).
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Action {
    /// A mouse button. (Value is button number, 1-8).
    Button(u8),
    /// Move cursor left.
    CursorLeft,
    /// Move cursor right.
    CursorRight,
    /// Move cursor up.
    CursorUp,
    /// Move cursor down.
    CursorDown,
    /// Scroll wheel up.
    WheelUp,
    /// Scroll wheel down.
    WheelDown,
    /// Scroll wheel left.
    WheelLeft,
    /// Scroll wheel right.
    WheelRight,
}

/// Reference for a mouse key.
///
/// Simple actions without keyboard modifiers are represented directly.
/// Keys that include keyboard modifiers are an index into [System] key data.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum Ref {
    /// A mouse button. (Value is button number, 1-8).
    Button(u8),
    /// Move cursor left.
    CursorLeft,
    /// Move cursor right.
    CursorRight,
    /// Move cursor up.
    CursorUp,
    /// Move cursor down.
    CursorDown,
    /// Scroll wheel up.
    WheelUp,
    /// Scroll wheel down.
    WheelDown,
    /// Scroll wheel left.
    WheelLeft,
    /// Scroll wheel right.
    WheelRight,
    /// Index into the key data array of [System] for a [Key] (action + modifiers).
    Key(u8),
}

/// A mouse key: an [Action] with optional keyboard modifiers.
///
/// A modifiers value of zero is equivalent to no modifiers.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key {
    /// The mouse action.
    pub action: Action,
    /// Keyboard modifiers.
    #[serde(default)]
    pub modifiers: key::KeyboardModifiers,
}

impl Key {
    /// Constructs a key with the given action and no modifiers.
    pub const fn new(action: Action) -> Self {
        Self {
            action,
            modifiers: key::KeyboardModifiers::new(),
        }
    }

    /// Constructs a key with the given action and modifiers.
    pub const fn new_with_modifiers(action: Action, modifiers: key::KeyboardModifiers) -> Self {
        Self { action, modifiers }
    }
}

/// Context for mouse keys. (No context).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context;

/// The event type for mouse keys. (No events).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Event;

/// The pending key state type for mouse keys. (No pending state).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PendingKeyState;

/// Key state used by [System].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState;

/// The [key::System] implementation for mouse keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct System<R: Debug, Keys: Index<usize, Output = Key>> {
    keys: Keys,
    marker: PhantomData<R>,
}

impl<R: Debug, Keys: Index<usize, Output = Key>> System<R, Keys> {
    /// Constructs a new [System] with the given key data.
    ///
    /// The key data is for mouse keys that include keyboard modifiers.
    pub const fn new(keys: Keys) -> Self {
        Self {
            keys,
            marker: PhantomData,
        }
    }
}

fn mouse_output_for_action(action: Action) -> key::MouseOutput {
    const MOVE_AMOUNT: i8 = 5;
    match action {
        Action::Button(b) => key::MouseOutput {
            pressed_buttons: 1 << (b - 1),
            ..key::MouseOutput::NO_OUTPUT
        },
        Action::CursorLeft => key::MouseOutput {
            x: -MOVE_AMOUNT,
            ..key::MouseOutput::NO_OUTPUT
        },
        Action::CursorRight => key::MouseOutput {
            x: MOVE_AMOUNT,
            ..key::MouseOutput::NO_OUTPUT
        },
        Action::CursorUp => key::MouseOutput {
            y: -MOVE_AMOUNT,
            ..key::MouseOutput::NO_OUTPUT
        },
        Action::CursorDown => key::MouseOutput {
            y: MOVE_AMOUNT,
            ..key::MouseOutput::NO_OUTPUT
        },
        Action::WheelUp => key::MouseOutput {
            vertical_scroll: 1,
            ..key::MouseOutput::NO_OUTPUT
        },
        Action::WheelDown => key::MouseOutput {
            vertical_scroll: -1,
            ..key::MouseOutput::NO_OUTPUT
        },
        Action::WheelLeft => key::MouseOutput {
            horizontal_scroll: -1,
            ..key::MouseOutput::NO_OUTPUT
        },
        Action::WheelRight => key::MouseOutput {
            horizontal_scroll: 1,
            ..key::MouseOutput::NO_OUTPUT
        },
    }
}

fn action_for_simple_ref(key_ref: Ref) -> Option<Action> {
    match key_ref {
        Ref::Button(b) => Some(Action::Button(b)),
        Ref::CursorLeft => Some(Action::CursorLeft),
        Ref::CursorRight => Some(Action::CursorRight),
        Ref::CursorUp => Some(Action::CursorUp),
        Ref::CursorDown => Some(Action::CursorDown),
        Ref::WheelUp => Some(Action::WheelUp),
        Ref::WheelDown => Some(Action::WheelDown),
        Ref::WheelLeft => Some(Action::WheelLeft),
        Ref::WheelRight => Some(Action::WheelRight),
        Ref::Key(_) => None,
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
        _keymap_index: u16,
        _context: &Self::Context,
        _key_ref: Ref,
    ) -> (
        key::PressedKeyResult<R, Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        (
            key::PressedKeyResult::Resolved(KeyState),
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

    fn update_state(
        &self,
        _key_state: &mut Self::KeyState,
        _ref: &Self::Ref,
        _context: &Self::Context,
        _keymap_index: u16,
        _event: key::Event<Self::Event>,
    ) -> key::KeyEvents<Self::Event> {
        key::KeyEvents::no_events()
    }

    fn key_output(
        &self,
        key_ref: &Self::Ref,
        _key_state: &Self::KeyState,
    ) -> Option<key::KeyOutput> {
        match key_ref {
            Ref::Key(idx) => {
                let Key {
                    action,
                    modifiers,
                } = self.keys[*idx as usize];
                Some(key::KeyOutput::from_usage_with_modifiers(
                    key::KeyUsage::Mouse(mouse_output_for_action(action)),
                    modifiers,
                ))
            }
            simple => {
                let action = action_for_simple_ref(*simple).unwrap();
                Some(key::KeyOutput::from_mouse_output(mouse_output_for_action(
                    action,
                )))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sizeof_ref() {
        // Action-like variants and Key(u8) index: 2 bytes.
        assert_eq!(2, core::mem::size_of::<Ref>());
    }

    #[test]
    fn test_sizeof_event() {
        assert_eq!(0, core::mem::size_of::<Event>());
    }
}
