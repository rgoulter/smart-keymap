use core::fmt::Debug;

use serde::Deserialize;

use crate::key;

/// Reference for a mouse key.
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
}

/// Context for mouse keys. (No context).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context;

impl key::Context for Context {
    type Event = Event;

    /// Used to update the [Context]'s state.
    fn handle_event(&mut self, _event: key::Event<Self::Event>) -> key::KeyEvents<Self::Event> {
        key::KeyEvents::no_events()
    }
}

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
pub struct System<R: Debug> {
    _marker: core::marker::PhantomData<R>,
}

impl<R: Debug> System<R> {
    /// Constructs a new [System].
    pub const fn new() -> Self {
        Self {
            _marker: core::marker::PhantomData,
        }
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
        const MOVE_AMOUNT: i8 = 5;
        let mouse_output = match key_ref {
            Ref::Button(b) => key::MouseOutput {
                pressed_buttons: 1 << (b - 1),
                x: 0,
                y: 0,
                vertical_scroll: 0,
                horizontal_scroll: 0,
            },
            Ref::CursorLeft => key::MouseOutput {
                pressed_buttons: 0,
                x: -MOVE_AMOUNT,
                y: 0,
                vertical_scroll: 0,
                horizontal_scroll: 0,
            },
            Ref::CursorRight => key::MouseOutput {
                pressed_buttons: 0,
                x: MOVE_AMOUNT,
                y: 0,
                vertical_scroll: 0,
                horizontal_scroll: 0,
            },
            Ref::CursorUp => key::MouseOutput {
                pressed_buttons: 0,
                x: 0,
                y: -MOVE_AMOUNT,
                vertical_scroll: 0,
                horizontal_scroll: 0,
            },
            Ref::CursorDown => key::MouseOutput {
                pressed_buttons: 0,
                x: 0,
                y: MOVE_AMOUNT,
                vertical_scroll: 0,
                horizontal_scroll: 0,
            },
            Ref::WheelUp => key::MouseOutput {
                pressed_buttons: 0,
                x: 0,
                y: 0,
                vertical_scroll: 1,
                horizontal_scroll: 0,
            },
            Ref::WheelDown => key::MouseOutput {
                pressed_buttons: 0,
                x: 0,
                y: 0,
                vertical_scroll: -1,
                horizontal_scroll: 0,
            },
            Ref::WheelLeft => key::MouseOutput {
                pressed_buttons: 0,
                x: 0,
                y: 0,
                vertical_scroll: 0,
                horizontal_scroll: -1,
            },
            Ref::WheelRight => key::MouseOutput {
                pressed_buttons: 0,
                x: 0,
                y: 0,
                vertical_scroll: 0,
                horizontal_scroll: 1,
            },
        };
        Some(key::KeyOutput::from_mouse_output(mouse_output))
    }
}
