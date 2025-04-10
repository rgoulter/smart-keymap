use core::fmt::Debug;
use core::marker::PhantomData;

use serde::{Deserialize, Serialize};

use crate::input;

/// Keymap Callback keys
pub mod callback;
/// CapsWord key(s).
pub mod caps_word;
/// Chorded keys. (Chording functionality).
pub mod chorded;
/// HID Keyboard keys.
pub mod keyboard;
/// Layered keys. (Layering functionality).
pub mod layered;
/// Sticky Modifier keys.
pub mod sticky;
/// Tap-Hold keys.
pub mod tap_hold;

/// "Composite" keys; an aggregate type used for a common context and event.
pub mod composite;

/// The maximum number of key events that are emitted [Key] or [KeyState].
pub const MAX_KEY_EVENTS: usize = 4;

/// The maximum length of a key path.
pub const MAX_KEY_PATH_LEN: usize = 4;

/// Sequence of indices into a key map.
pub type KeyPath = heapless::Vec<u16, MAX_KEY_PATH_LEN>;

/// Events emitted when a [Key] is pressed.
#[derive(Debug, PartialEq, Eq)]
pub struct KeyEvents<E, const M: usize = { MAX_KEY_EVENTS }>(heapless::Vec<ScheduledEvent<E>, M>);

impl<E: Copy + Debug> KeyEvents<E> {
    /// Constructs a [KeyEvents] with no events scheduled.
    pub fn no_events() -> Self {
        KeyEvents(None.into_iter().collect())
    }

    /// Constructs a [KeyEvents] with an immediate [Event].
    pub fn event(event: Event<E>) -> Self {
        KeyEvents(Some(ScheduledEvent::immediate(event)).into_iter().collect())
    }

    /// Constructs a [KeyEvents] with an [Event] scheduled after a delay.
    pub fn scheduled_event(sch_event: ScheduledEvent<E>) -> Self {
        KeyEvents(Some(sch_event).into_iter().collect())
    }

    /// Adds an event with the schedule to the [KeyEvents].
    pub fn schedule_event(&mut self, delay: u16, event: Event<E>) {
        self.0.push(ScheduledEvent::after(delay, event)).unwrap();
    }

    /// Adds events from the other [KeyEvents] to the [KeyEvents].
    pub fn extend(&mut self, other: KeyEvents<E>) {
        other.0.into_iter().for_each(|ev| self.0.push(ev).unwrap());
    }

    /// Adds an event from to the [KeyEvents].
    pub fn add_event(&mut self, ev: ScheduledEvent<E>) {
        self.0.push(ev).unwrap();
    }

    /// Maps over the KeyEvents.
    pub fn map_events<F>(&self, f: fn(E) -> F) -> KeyEvents<F> {
        KeyEvents(
            self.0
                .as_slice()
                .iter()
                .map(|sch_ev| sch_ev.map_scheduled_event(f))
                .collect(),
        )
    }

    /// Maps the KeyEvents to a new type.
    pub fn into_events<F>(&self) -> KeyEvents<F>
    where
        E: Into<F>,
    {
        KeyEvents(
            self.0
                .as_slice()
                .iter()
                .map(|sch_ev| sch_ev.map_scheduled_event(|ev| ev.into()))
                .collect(),
        )
    }
}

impl<E: Debug, const M: usize> IntoIterator for KeyEvents<E, M> {
    type Item = ScheduledEvent<E>;
    type IntoIter = <heapless::Vec<ScheduledEvent<E>, M> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

/// Pressed Key which may be pending, or a resolved key state.
pub enum PressedKeyResult<PKS, KS> {
    /// Unresolved key state. (e.g. tap-hold or chorded keys when first pressed).
    Pending(KeyPath, PKS),
    /// Resolved key state.
    Resolved(KS),
}

/// Constructs key path with the given keymap index.
pub fn key_path(keymap_index: u16) -> KeyPath {
    let mut key_path = KeyPath::new();
    key_path.push(keymap_index).unwrap();
    key_path
}

impl<PKS, KS> PressedKeyResult<PKS, KS> {
    /// Returns the Resolved variant, or else panics.
    #[cfg(feature = "std")]
    pub fn unwrap_resolved(self) -> KS {
        match self {
            PressedKeyResult::Resolved(r) => r,
            _ => panic!("PressedKeyResult::unwrap_resolved: not Resolved"),
        }
    }

    /// Adds an item to the KeyPath if the pressed key result is pending.
    pub fn add_path_item(self, item: u16) -> Self {
        match self {
            PressedKeyResult::Pending(mut key_path, pks) => {
                key_path.push(item).unwrap();
                PressedKeyResult::Pending(key_path, pks)
            }
            pkr => pkr,
        }
    }
}

/// The interface for `Key` behaviour.
///
/// A `Key` has an associated [Context], `Event`, and [KeyState].
///
/// The generic `PK` is used as the type of the `PressedKey` that the `Key`
///  produces.
/// (e.g. [layered::LayeredKey]'s pressed key state passes-through to
///  the keys of its layers).
pub trait Key: Debug {
    /// The associated [Context] is used to provide state that
    ///  may affect behaviour when pressing the key.
    /// (e.g. the behaviour of [layered::LayeredKey] depends on which
    ///  layers are active in [layered::Context]).
    type Context: Copy;

    /// The associated `Event` is to be handled by the associated [Context],
    ///  pending key states, and key states.
    type Event: Copy + Debug + PartialEq;

    /// Associated pending key state.
    type PendingKeyState;

    /// Associated key state type.
    type KeyState;

    /// [Key::new_pressed_key] produces a pressed key value, and may
    ///  yield some [ScheduledEvent]s.
    /// (e.g. [tap_hold::Key] schedules a [tap_hold::Event::TapHoldTimeout]
    ///  so that holding the key resolves as a hold).
    fn new_pressed_key(
        &self,
        context: Self::Context,
        key_path: KeyPath,
    ) -> (
        PressedKeyResult<Self::PendingKeyState, Self::KeyState>,
        KeyEvents<Self::Event>,
    );

    /// Update the given pending key state with the given impl.
    fn handle_event(
        &self,
        pending_state: &mut Self::PendingKeyState,
        context: Self::Context,
        key_path: KeyPath,
        event: Event<Self::Event>,
    ) -> (
        Option<PressedKeyResult<Self::PendingKeyState, Self::KeyState>>,
        KeyEvents<Self::Event>,
    );

    /// Return a reference to the key for the given path.
    fn lookup(
        &self,
        path: &[u16],
    ) -> &dyn Key<
        Context = Self::Context,
        Event = Self::Event,
        PendingKeyState = Self::PendingKeyState,
        KeyState = Self::KeyState,
    >;
}

/// Used to provide state that may affect behaviour when pressing the key.
///
/// e.g. the behaviour of [layered::LayeredKey] depends on which
///  layers are active in [layered::Context].
pub trait Context: Clone + Copy {
    /// The type of `Event` the context handles.
    type Event;

    /// Used to update the [Context]'s state.
    fn handle_event(&mut self, event: Event<Self::Event>) -> KeyEvents<Self::Event>;
}

/// Bool flags for each of the modifier keys (left ctrl, etc.).
#[derive(Deserialize, Serialize, Default, Clone, Copy, PartialEq, Eq)]
pub struct KeyboardModifiers {
    #[serde(default)]
    left_ctrl: bool,
    #[serde(default)]
    left_shift: bool,
    #[serde(default)]
    left_alt: bool,
    #[serde(default)]
    left_gui: bool,
    #[serde(default)]
    right_ctrl: bool,
    #[serde(default)]
    right_shift: bool,
    #[serde(default)]
    right_alt: bool,
    #[serde(default)]
    right_gui: bool,
}

impl core::fmt::Debug for KeyboardModifiers {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("KeyboardModifiers");
        if self.left_ctrl {
            ds.field("left_ctrl", &true);
        }
        if self.left_shift {
            ds.field("left_shift", &true);
        }
        if self.left_alt {
            ds.field("left_alt", &true);
        }
        if self.left_gui {
            ds.field("left_gui", &true);
        }
        if self.right_ctrl {
            ds.field("right_ctrl", &true);
        }
        if self.right_shift {
            ds.field("right_shift", &true);
        }
        if self.right_alt {
            ds.field("right_alt", &true);
        }
        if self.right_gui {
            ds.field("right_gui", &true);
        }
        ds.finish_non_exhaustive()
    }
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

    /// Constructs with the given key_code.
    ///
    /// Returns None if the key_code is not a modifier key code.
    pub const fn from_key_code(key_code: u8) -> Option<Self> {
        match key_code {
            0xE0 => Some(Self::LEFT_CTRL),
            0xE1 => Some(Self::LEFT_SHIFT),
            0xE2 => Some(Self::LEFT_ALT),
            0xE3 => Some(Self::LEFT_GUI),
            0xE4 => Some(Self::RIGHT_CTRL),
            0xE5 => Some(Self::RIGHT_SHIFT),
            0xE6 => Some(Self::RIGHT_ALT),
            0xE7 => Some(Self::RIGHT_GUI),
            _ => None,
        }
    }

    /// Const for no modifiers
    pub const NONE: KeyboardModifiers = KeyboardModifiers {
        ..KeyboardModifiers::new()
    };

    /// Const for left ctrl.
    pub const LEFT_CTRL: KeyboardModifiers = KeyboardModifiers {
        left_ctrl: true,
        ..KeyboardModifiers::new()
    };

    /// Const for left shift.
    pub const LEFT_SHIFT: KeyboardModifiers = KeyboardModifiers {
        left_shift: true,
        ..KeyboardModifiers::new()
    };

    /// Const for left alt.
    pub const LEFT_ALT: KeyboardModifiers = KeyboardModifiers {
        left_alt: true,
        ..KeyboardModifiers::new()
    };

    /// Const for left gui.
    pub const LEFT_GUI: KeyboardModifiers = KeyboardModifiers {
        left_gui: true,
        ..KeyboardModifiers::new()
    };

    /// Const for right ctrl.
    pub const RIGHT_CTRL: KeyboardModifiers = KeyboardModifiers {
        right_ctrl: true,
        ..KeyboardModifiers::new()
    };

    /// Const for right shift.
    pub const RIGHT_SHIFT: KeyboardModifiers = KeyboardModifiers {
        right_shift: true,
        ..KeyboardModifiers::new()
    };

    /// Const for right alt.
    pub const RIGHT_ALT: KeyboardModifiers = KeyboardModifiers {
        right_alt: true,
        ..KeyboardModifiers::new()
    };

    /// Const for right gui.
    pub const RIGHT_GUI: KeyboardModifiers = KeyboardModifiers {
        right_gui: true,
        ..KeyboardModifiers::new()
    };

    /// Predicate for whether the key code is a modifier key code.
    pub const fn is_modifier_key_code(key_code: u8) -> bool {
        matches!(key_code, 0xE0..=0xE7)
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

    /// Constructs the byte for the modifiers of an HID keyboard report.
    pub fn as_byte(&self) -> u8 {
        self.as_key_codes()
            .iter()
            .fold(0u8, |acc, &kc| acc | (1 << (kc - 0xE0)))
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

    /// Whether this keyboard modifiers includes all the other modifiers.
    pub const fn has_modifiers(&self, other: &KeyboardModifiers) -> bool {
        (!other.left_ctrl || self.left_ctrl)
            && (!other.left_shift || self.left_shift)
            && (!other.left_alt || self.left_alt)
            && (!other.left_gui || self.left_gui)
            && (!other.right_ctrl || self.right_ctrl)
            && (!other.right_shift || self.right_shift)
            && (!other.right_alt || self.right_alt)
            && (!other.right_gui || self.right_gui)
    }
}

/// Struct for the output from [KeyState].
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyOutput {
    key_code: u8,
    key_modifiers: KeyboardModifiers,
}

impl KeyOutput {
    /// Constructs a [KeyOutput] from a key code.
    pub fn from_key_code(key_code: u8) -> Self {
        if let Some(key_modifiers) = KeyboardModifiers::from_key_code(key_code) {
            KeyOutput {
                key_code: 0x00,
                key_modifiers,
            }
        } else {
            KeyOutput {
                key_code,
                key_modifiers: KeyboardModifiers::new(),
            }
        }
    }

    /// Constructs a [KeyOutput] from a key code with the given keyboard modifiers.
    pub fn from_key_code_with_modifiers(key_code: u8, key_modifiers: KeyboardModifiers) -> Self {
        let KeyOutput {
            key_code,
            key_modifiers: km,
        } = Self::from_key_code(key_code);
        KeyOutput {
            key_code,
            key_modifiers: km.union(&key_modifiers),
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

/// Implements functionality for the pressed key.
pub trait KeyState: Debug {
    /// The type of `Context` the pressed key state handles.
    type Context;
    /// The type of `Event` the pressed key state handles.
    type Event: Copy + Debug;

    /// Used to update the [KeyState]'s state, and possibly yield event(s).
    fn handle_event(
        &mut self,
        _context: Self::Context,
        _keymap_index: u16,
        _event: Event<Self::Event>,
    ) -> KeyEvents<Self::Event> {
        KeyEvents::no_events()
    }

    /// Output for the pressed key state.
    fn key_output(&self) -> Option<KeyOutput> {
        None
    }
}

/// A NoOp key state, for keys which do nothing when pressed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoOpKeyState<Ctx, Ev>(PhantomData<(Ctx, Ev)>);

impl<Ctx, Ev> NoOpKeyState<Ctx, Ev> {
    /// Constructs a NoOpKeyState value.
    pub const fn new() -> Self {
        NoOpKeyState(PhantomData)
    }
}

impl<Ctx: Debug, Ev: Copy + Debug> KeyState for NoOpKeyState<Ctx, Ev> {
    type Context = Ctx;
    type Event = Ev;
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
    /// Invoke a keymap callback
    Keymap(crate::keymap::KeymapEvent),
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
    pub fn map_key_event<U>(self, f: fn(T) -> U) -> Event<U> {
        match self {
            Event::Input(event) => Event::Input(event),
            Event::Key {
                key_event,
                keymap_index,
            } => Event::Key {
                key_event: f(key_event),
                keymap_index,
            },
            Event::Keymap(cb) => Event::Keymap(cb),
        }
    }

    /// Maps the Event into a new type.
    pub fn into_key_event<U>(self) -> Event<U>
    where
        T: Into<U>,
    {
        self.map_key_event(|ke| ke.into())
    }

    /// Maps the Event into a new type.
    pub fn try_into_key_event<U, E>(self, f: fn(T) -> Result<U, E>) -> EventResult<Event<U>> {
        match self {
            Event::Input(event) => Ok(Event::Input(event)),
            Event::Key {
                key_event,
                keymap_index,
            } => f(key_event)
                .map(|key_event| Event::Key {
                    key_event,
                    keymap_index,
                })
                .map_err(|_| EventError::UnmappableEvent),
            Event::Keymap(cb) => Ok(Event::Keymap(cb)),
        }
    }
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
    pub fn map_scheduled_event<U>(self, f: fn(T) -> U) -> ScheduledEvent<U> {
        ScheduledEvent {
            event: self.event.map_key_event(f),
            schedule: self.schedule,
        }
    }

    /// Maps the ScheduledEvent into a new type.
    pub fn into_scheduled_event<U>(self) -> ScheduledEvent<U>
    where
        T: Into<U>,
    {
        self.map_scheduled_event(|e| e.into())
    }
}
