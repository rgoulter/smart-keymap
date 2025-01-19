use core::fmt::Debug;

use serde::Deserialize;

use crate::input;

/// HID Keyboard keys.
pub mod keyboard;
/// Layered keys. (Layering functionality).
pub mod layered;
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
    pub fn map_events<F>(&self, f: fn(E) -> F) -> PressedKeyEvents<F> {
        PressedKeyEvents(
            self.0
                .as_slice()
                .iter()
                .map(|sch_ev| sch_ev.map_scheduled_event(f))
                .collect(),
        )
    }

    /// Maps the PressedKeyEvents to a new type.
    pub fn into_events<F>(&self) -> PressedKeyEvents<F>
    where
        E: Into<F>,
    {
        PressedKeyEvents(
            self.0
                .as_slice()
                .iter()
                .map(|sch_ev| sch_ev.map_scheduled_event(|ev| ev.into()))
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

impl<MC, IC> ModifierKeyContext<MC, IC> {
    /// Constructs a ModifierKeyContext from the given context, using the provided functions for context/inner_context.
    pub fn from_context<FC: Copy>(
        fc: FC,
        f: fn(FC) -> MC,
        g: fn(FC) -> IC,
    ) -> ModifierKeyContext<MC, IC> {
        ModifierKeyContext {
            context: f(fc),
            inner_context: g(fc),
        }
    }
}

/// Bool flags for each of the modifier keys (left ctrl, etc.).
#[derive(Deserialize, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardModifiers {
    left_ctrl: bool,
    left_shift: bool,
    left_alt: bool,
    left_gui: bool,
    right_ctrl: bool,
    right_shift: bool,
    right_alt: bool,
    right_gui: bool,
}

impl KeyboardModifiers {
    /// Constructs with modifiers defaulting to false.
    pub const fn new() -> Self {
        KeyboardModifiers {
            left_ctrl: false,
            left_shift: false,
            left_alt: false,
            left_gui: false,
            right_ctrl: false,
            right_shift: false,
            right_alt: false,
            right_gui: false,
        }
    }

    /// Predicate for whether the key code is a modifier key code.
    pub const fn is_modifier_key_code(key_code: u8) -> bool {
        match key_code {
            0xE0..=0xE7 => true,
            _ => false,
        }
    }

    /// Constructs a Vec of key codes from the modifiers.
    pub fn as_key_codes(&self) -> heapless::Vec<u8, 8> {
        let mut key_codes = heapless::Vec::new();

        if self.left_ctrl {
            key_codes.push(0xE0).unwrap();
        }
        if self.left_shift {
            key_codes.push(0xE1).unwrap();
        }
        if self.left_alt {
            key_codes.push(0xE2).unwrap();
        }
        if self.left_gui {
            key_codes.push(0xE3).unwrap();
        }
        if self.right_ctrl {
            key_codes.push(0xE4).unwrap();
        }
        if self.right_shift {
            key_codes.push(0xE5).unwrap();
        }
        if self.right_alt {
            key_codes.push(0xE6).unwrap();
        }
        if self.right_gui {
            key_codes.push(0xE7).unwrap();
        }

        key_codes
    }

    /// Union of two KeyboardModifiers, taking "or" of each modifier.
    pub const fn union(&self, other: &KeyboardModifiers) -> KeyboardModifiers {
        KeyboardModifiers {
            left_ctrl: self.left_ctrl || other.left_ctrl,
            left_shift: self.left_shift || other.left_shift,
            left_alt: self.left_alt || other.left_alt,
            left_gui: self.left_gui || other.left_gui,
            right_ctrl: self.right_ctrl || other.right_ctrl,
            right_shift: self.right_shift || other.right_shift,
            right_alt: self.right_alt || other.right_alt,
            right_gui: self.right_gui || other.right_gui,
        }
    }
}

/// Struct for the output from [PressedKey].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyOutput {
    key_code: u8,
    key_modifiers: KeyboardModifiers,
}

impl KeyOutput {
    /// Constructs a [KeyOutput] from a key code.
    pub fn from_key_code(key_code: u8) -> Self {
        let key_modifiers = KeyboardModifiers::new();
        KeyOutput {
            key_code,
            key_modifiers,
        }
    }

    /// Constructs a [KeyOutput] from a key code with the given keyboard modifiers.
    pub fn from_key_code_with_modifiers(key_code: u8, key_modifiers: KeyboardModifiers) -> Self {
        KeyOutput {
            key_code,
            key_modifiers,
        }
    }

    /// Constructs a [KeyOutput] for just the given keyboard modifiers.
    pub fn from_key_modifiers(key_modifiers: KeyboardModifiers) -> Self {
        KeyOutput {
            key_code: 0x00,
            key_modifiers,
        }
    }

    /// Returns the key code value.
    pub fn key_code(&self) -> u8 {
        self.key_code
    }

    /// Returns the keyboard modifiers of the key output.
    pub fn key_modifiers(&self) -> KeyboardModifiers {
        self.key_modifiers
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
    pub fn into_key_event<U>(&self) -> Event<U>
    where
        T: Into<U>,
    {
        self.map_key_event(|ke| ke.into())
    }

    /// Maps the Event into a new type.
    pub fn try_into_key_event<U>(&self, f: fn(T) -> EventResult<U>) -> EventResult<Event<U>> {
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

impl<T: Copy, U: Copy> ModifierKeyEvent<T, U> {
    /// Applies a function to either Modifier or Inner.
    pub fn map_events<V>(self, f: fn(T) -> V, g: fn(U) -> V) -> V {
        match self {
            ModifierKeyEvent::Modifier(t) => f(t),
            ModifierKeyEvent::Inner(u) => g(u),
        }
    }

    /// Tries to construct either variant.
    pub fn try_from<V: Copy>(
        v: V,
        f: fn(V) -> EventResult<T>,
        g: fn(V) -> EventResult<U>,
    ) -> EventResult<ModifierKeyEvent<T, U>> {
        if let Ok(t) = f(v) {
            Ok(ModifierKeyEvent::Modifier(t))
        } else if let Ok(u) = g(v) {
            Ok(ModifierKeyEvent::Inner(u))
        } else {
            Err(EventError::UnmappableEvent)
        }
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

    /// Maps the Event of the ScheduledEvent into a new type.
    pub fn map_scheduled_event<U>(&self, f: fn(T) -> U) -> ScheduledEvent<U> {
        ScheduledEvent {
            event: self.event.map_key_event(f),
            schedule: self.schedule,
        }
    }

    /// Maps the ScheduledEvent into a new type.
    pub fn into_scheduled_event<U>(&self) -> ScheduledEvent<U>
    where
        T: Into<U>,
    {
        self.map_scheduled_event(|e| e.into())
    }
}
