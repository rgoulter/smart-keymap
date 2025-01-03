use core::fmt::Debug;

use crate::input;

/// Layered keys. (Layering functionality).
pub mod layered;
/// Simple keys.
pub mod simple;
/// Tap-Hold keys.
pub mod tap_hold;

/// "Composite" keys; an aggregate type used for a common context and event.
pub mod composite;

/// dyn-compatible `Key` trait, and generic implementation.
pub mod dynamic;

/// The interface for `Key` behaviour.
///
/// A `Key` has an associated [Context], `Event`, and [PressedKeyState].
///
/// The generic `PK` is used as the type of the `PressedKey` that the `Key`
///  produces.
/// (e.g. [layered::LayeredKey]'s pressed key state passes-through to
///  the keys of its layers).
pub trait Key<PK: Key = Self>: Debug
where
    Self::ContextEvent: From<Self::Event>,
{
    /// The associated [Context] is used to provide state that
    ///  may affect behaviour when pressing the key.
    /// (e.g. the behaviour of [layered::LayeredKey] depends on which
    ///  layers are active in [layered::Context]).
    type Context: Context<Event = Self::ContextEvent>;
    /// The event used by the [Key]'s associated [Context].
    type ContextEvent;
    /// The associated `Event` is to be handled by the associated [Context],
    ///  and any active [PressedKeyState]s.
    type Event: Copy + Debug + Ord;
    /// The associated [PressedKeyState] implements functionality
    ///  for the pressed key.
    /// (e.g. [tap_hold::PressedKeyState] implements behaviour resolving
    ///  the pressed tap hold key as either 'tap' or 'hold').
    type PressedKeyState: PressedKeyState<PK, Event = Self::Event>;

    /// [Key::new_pressed_key] produces a pressed key value, and may
    ///  yield some [ScheduledEvent]s.
    /// (e.g. [tap_hold::Key] schedules a [tap_hold::Event::TapHoldTimeout]
    ///  so that holding the key resolves as a hold).
    fn new_pressed_key(
        &self,
        context: &Self::Context,
        keymap_index: u16,
    ) -> (
        input::PressedKey<PK, Self::PressedKeyState>,
        Option<ScheduledEvent<Self::Event>>,
    );
}

/// Used to provide state that may affect behaviour when pressing the key.
///
/// e.g. the behaviour of [layered::LayeredKey] depends on which
///  layers are active in [layered::Context].
pub trait Context {
    /// The type of `Event` the context handles.
    type Event;

    /// Used to update the [Context]'s state.
    fn handle_event(&mut self, event: Self::Event);
}

impl Context for () {
    type Event = ();
    fn handle_event(&mut self, _event: Self::Event) {}
}

/// [PressedKeyState] for a stateful pressed key value.
pub trait PressedKey {
    /// The type of `Event` the pressed key handles.
    type Event;

    /// Used to update the [PressedKey]'s state, and possibly yield event(s).
    fn handle_event(
        &mut self,
        event: Event<Self::Event>,
    ) -> impl IntoIterator<Item = Event<Self::Event>>;

    /// Output for the pressed key.
    fn key_code(&self) -> Option<u8>;
}

/// Implements functionality for the pressed key.
///
/// e.g. [tap_hold::PressedKeyState] implements behaviour resolving
///  the pressed tap hold key as either 'tap' or 'hold'.
pub trait PressedKeyState<K: Key>: Debug {
    /// The type of `Event` the pressed key state handles.
    type Event;

    /// Used to update the [PressedKeyState]'s state, and possibly yield event(s).
    fn handle_event_for(
        &mut self,
        keymap_index: u16,
        key: &K,
        event: Event<Self::Event>,
    ) -> impl IntoIterator<Item = Event<Self::Event>>;

    /// Output for the pressed key state.
    fn key_code(&self, key: &K) -> Option<u8>;
}

/// Errors for [TryFrom] implementations.
#[allow(unused)]
pub enum EventError {
    /// Error when mapping isn't possible.
    ///
    /// e.g. trying to map variants of [composite::Event] to [tap_hold::Event].
    UnmappableEvent,
}

/// Events which are either input, or for a particular [Key::Event].
///
/// It's useful for [Key] implementations to use [Event] with [Key::Event],
///  and map [Key::Event] to and partially from [composite::Event].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Event<T> {
    /// Keymap input events, such as physical key presses.
    Input(input::Event),
    /// [Key] implementation specific events.
    Key(T),
}

impl<T> From<input::Event> for Event<T> {
    fn from(event: input::Event) -> Self {
        Event::Input(event)
    }
}

/// Schedule for a [ScheduledEvent].
#[allow(unused)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub enum Schedule {
    /// Immediately.
    Immediate,
    /// After a given number of `tick`s.
    After(u16),
}

/// Schedules a given `T` with [Event], for some [Schedule].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct ScheduledEvent<T> {
    /// Whether to handle the event immediately, or after some delay.
    pub schedule: Schedule,
    /// The event.
    pub event: Event<T>,
}

impl<T> ScheduledEvent<T> {
    /// Constructs a [ScheduledEvent] with [Schedule::Immediate].
    #[allow(unused)]
    pub fn immediate(event: Event<T>) -> Self {
        ScheduledEvent {
            schedule: Schedule::Immediate,
            event,
        }
    }

    /// Constructs a [ScheduledEvent] with [Schedule::After].
    pub fn after(delay: u16, event: Event<T>) -> Self {
        ScheduledEvent {
            schedule: Schedule::After(delay),
            event,
        }
    }
}
