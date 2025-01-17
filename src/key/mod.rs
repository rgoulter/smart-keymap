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

/// Events emitted when a [Key] is pressed.
#[derive(Debug, PartialEq, Eq)]
pub struct PressedKeyEvents<E, const M: usize = 2>(heapless::Vec<ScheduledEvent<E>, M>);

impl<E: Copy + Debug> PressedKeyEvents<E> {
    /// Constructs a [PressedKeyEvents] with no events scheduled.
    pub fn no_events() -> Self {
        PressedKeyEvents(None.into_iter().collect())
    }

    /// Constructs a [PressedKeyEvents] with an immediate [Event].
    pub fn event(event: Event<E>) -> Self {
        PressedKeyEvents(Some(ScheduledEvent::immediate(event)).into_iter().collect())
    }

    /// Constructs a [PressedKeyEvents] with an [Event] scheduled after a delay.
    pub fn scheduled_event(delay: u16, event: Event<E>) -> Self {
        PressedKeyEvents(
            Some(ScheduledEvent::after(delay, event))
                .into_iter()
                .collect(),
        )
    }

    /// Adds an event with the schedule to the [PressedKeyEvents].
    pub fn schedule_event(&mut self, delay: u16, event: Event<E>) {
        self.0.push(ScheduledEvent::after(delay, event)).unwrap();
    }

    /// Adds events from the other [PressedKeyEvents] to the [PressedKeyEvents].
    pub fn extend(&mut self, other: PressedKeyEvents<E>) {
        self.0.extend(other.0);
    }

    /// Maps over the PressedKeyEvents.
    pub fn map_events<F>(&self, f: fn(Event<E>) -> Event<F>) -> PressedKeyEvents<F> {
        PressedKeyEvents(
            self.0
                .as_slice()
                .iter()
                .map(|scheduled_event| ScheduledEvent {
                    schedule: scheduled_event.schedule,
                    event: f(scheduled_event.event),
                })
                .collect(),
        )
    }

    /// Maps the PressedKeyEvents to a new type.
    pub fn into_events<F>(&self) -> PressedKeyEvents<F>
    where
        Event<F>: From<Event<E>>,
    {
        PressedKeyEvents(
            self.0
                .as_slice()
                .iter()
                .map(|scheduled_event| ScheduledEvent {
                    schedule: scheduled_event.schedule,
                    event: scheduled_event.event.into(),
                })
                .collect(),
        )
    }
}

impl<E: Debug, const M: usize> IntoIterator for PressedKeyEvents<E, M> {
    type Item = ScheduledEvent<E>;
    type IntoIter = <heapless::Vec<ScheduledEvent<E>, M> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// The interface for `Key` behaviour.
///
/// A `Key` has an associated [Context], `Event`, and [PressedKeyState].
///
/// The generic `PK` is used as the type of the `PressedKey` that the `Key`
///  produces.
/// (e.g. [layered::LayeredKey]'s pressed key state passes-through to
///  the keys of its layers).
pub trait Key: Copy + Debug + PartialEq {
    /// The associated [Context] is used to provide state that
    ///  may affect behaviour when pressing the key.
    /// (e.g. the behaviour of [layered::LayeredKey] depends on which
    ///  layers are active in [layered::Context]).
    type Context;
    /// The event used by the [Key]'s associated [Context].
    type ContextEvent;
    /// The associated `Event` is to be handled by the associated [Context],
    ///  and any active [PressedKeyState]s.
    type Event: Copy + Debug + PartialEq;
    /// The associated [PressedKeyState] implements functionality
    ///  for the pressed key.
    /// (e.g. [tap_hold::PressedKeyState] implements behaviour resolving
    ///  the pressed tap hold key as either 'tap' or 'hold').
    type PressedKeyState: PressedKeyState<Self, Event = Self::Event>;

    /// [Key::new_pressed_key] produces a pressed key value, and may
    ///  yield some [ScheduledEvent]s.
    /// (e.g. [tap_hold::Key] schedules a [tap_hold::Event::TapHoldTimeout]
    ///  so that holding the key resolves as a hold).
    fn new_pressed_key(
        &self,
        context: Self::Context,
        keymap_index: u16,
    ) -> (
        input::PressedKey<Self, Self::PressedKeyState>,
        PressedKeyEvents<Self::Event>,
    );
}

/// Used to provide state that may affect behaviour when pressing the key.
///
/// e.g. the behaviour of [layered::LayeredKey] depends on which
///  layers are active in [layered::Context].
pub trait Context: Clone + Copy {
    /// The type of `Event` the context handles.
    type Event;

    /// Used to update the [Context]'s state.
    fn handle_event(&mut self, event: Self::Event);
}

impl Context for () {
    type Event = ();
    fn handle_event(&mut self, _event: Self::Event) {}
}

/// Context struct for use by "modifier" keys.
/// (Keys which modify the behaviour of some key,
///  e.g. [key::layered::LayeredKey]).
pub struct ModifierKeyContext<Ctx, NCtx> {
    /// The [Context] for the modifier key.
    pub context: Ctx,
    /// The [Context] for the modified key.
    pub inner_context: NCtx,
}

/// Struct for the output from [PressedKey].
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
pub struct KeyOutput(u8);

impl KeyOutput {
    /// Constructs a [KeyOutput] from a key code.
    pub fn from_key_code(key_code: u8) -> Self {
        KeyOutput(key_code)
    }

    /// Returns the key code value.
    pub fn key_code(&self) -> u8 {
        self.0
    }
}

/// [PressedKeyState] for a stateful pressed key value.
pub trait PressedKey {
    /// The type of `Context` the pressed key handles.
    type Context;
    /// The type of `Event` the pressed key handles.
    type Event;

    /// Used to update the [PressedKey]'s state, and possibly yield event(s).
    fn handle_event(
        &mut self,
        context: Self::Context,
        event: Event<Self::Event>,
    ) -> PressedKeyEvents<Self::Event>;

    /// Output for the pressed key.
    fn key_output(&self) -> Option<KeyOutput>;
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
        context: K::Context,
        keymap_index: u16,
        key: &K,
        event: Event<Self::Event>,
    ) -> PressedKeyEvents<Self::Event>;

    /// Output for the pressed key state.
    fn key_output(&self, key: &K) -> Option<KeyOutput>;
}

/// Errors for [TryFrom] implementations.
#[allow(unused)]
pub enum EventError {
    /// Error when mapping isn't possible.
    ///
    /// e.g. trying to map variants of [composite::Event] to [tap_hold::Event].
    UnmappableEvent,
}

/// Convenience alias for a [Result] with an [EventError].
type EventResult<T> = Result<T, EventError>;

/// Events which are either input, or for a particular [Key::Event].
///
/// It's useful for [Key] implementations to use [Event] with [Key::Event],
///  and map [Key::Event] to and partially from [composite::Event].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event<T> {
    /// Keymap input events, such as physical key presses.
    Input(input::Event),
    /// [Key] implementation specific events.
    Key {
        /// The keymap index the event was generated from.
        keymap_index: u16,
        /// A [Key::Event] event.
        key_event: T,
    },
}

impl<T: Copy> Event<T> {
    /// Constructs an [Event] from an [Key::Event].
    pub fn key_event(keymap_index: u16, key_event: T) -> Self {
        Event::Key {
            keymap_index,
            key_event,
        }
    }

    /// Maps the Event into a new type.
    pub fn map_key_event<U>(&self, f: fn(T) -> U) -> Event<U> {
        match self {
            Event::Input(event) => Event::Input(*event),
            Event::Key {
                key_event,
                keymap_index,
            } => Event::Key {
                key_event: f(*key_event),
                keymap_index: *keymap_index,
            },
        }
    }

    /// Maps the Event into a new type.
    pub fn try_map_key_event<U>(&self, f: fn(T) -> EventResult<U>) -> EventResult<Event<U>> {
        match self {
            Event::Input(event) => Ok(Event::Input(*event)),
            Event::Key {
                key_event,
                keymap_index,
            } => f(*key_event).map(|key_event| Event::Key {
                key_event,
                keymap_index: *keymap_index,
            }),
        }
    }
}

impl<T> From<input::Event> for Event<T> {
    fn from(event: input::Event) -> Self {
        Event::Input(event)
    }
}

/// Sum type for events which modify other keys.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModifierKeyEvent<T, U> {
    /// The modifier's event type.
    Modifier(T),
    /// The inner key's event type.
    Inner(U),
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScheduledEvent<T> {
    /// Whether to handle the event immediately, or after some delay.
    pub schedule: Schedule,
    /// The event.
    pub event: Event<T>,
}

impl<T: Copy> ScheduledEvent<T> {
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

    /// Maps the ScheduledEvent into a new type.
    pub fn into_scheduled_event<U>(&self) -> ScheduledEvent<U>
    where
        Event<U>: From<Event<T>>,
    {
        ScheduledEvent {
            event: self.event.into(),
            schedule: self.schedule,
        }
    }
}
