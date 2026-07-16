use core::fmt::Debug;

use serde::{Deserialize, Serialize};

use crate::input;

/// Automation (macro) keys.
pub mod automation;
/// Keymap Callback keys
pub mod callback;
/// CapsWord key(s).
pub mod caps_word;
/// Chorded keys. (Chording functionality).
pub mod chorded;
/// Consumer keys.
pub mod consumer;
/// Custom keys.
pub mod custom;
/// HID Keyboard keys.
pub mod keyboard;
/// Layered keys. (Layering functionality).
pub mod layered;
/// Mouse keys.
pub mod mouse;
/// Sticky Modifier keys.
pub mod sticky;
/// Tap-Dance keys.
pub mod tap_dance;
/// Tap-Hold keys.
pub mod tap_hold;

/// "Composite" keys; an aggregate type used for a common context and event.
pub mod composite;

/// The maximum number of key events that are emitted by [crate::key::System] implementations.
pub const MAX_KEY_EVENTS: usize = 4;

/// Events emitted when a key is pressed.
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
        let _ = self.0.push(ScheduledEvent::after(delay, event));
    }

    /// Adds events from the other [KeyEvents] to the [KeyEvents].
    pub fn extend(&mut self, other: KeyEvents<E>) {
        other.0.into_iter().for_each(|ev| {
            let _ = self.0.push(ev);
        });
    }

    /// Adds an event from to the [KeyEvents].
    pub fn add_event(&mut self, ev: ScheduledEvent<E>) {
        let _ = self.0.push(ev);
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

/// Newtype for invoking new_pressed_key on the key for the given ref.
#[derive(Debug, PartialEq)]
pub enum NewPressedKey<R> {
    /// Invoke new_pressed_key on the key at the given ref.
    Key(R),
    /// For keys which do nothing when pressed.
    NoOp,
}

impl<R> NewPressedKey<R> {
    /// Constructs a NewPressedKey value.
    pub fn key(key_ref: R) -> Self {
        NewPressedKey::Key(key_ref)
    }

    /// Constructs a NoOp NewPressedKey value.
    pub fn no_op() -> Self {
        NewPressedKey::NoOp
    }

    /// Maps the NewPressedKey into a new type.
    pub fn map<TR>(self, f: fn(R) -> TR) -> NewPressedKey<TR> {
        match self {
            NewPressedKey::Key(r) => NewPressedKey::Key(f(r)),
            NewPressedKey::NoOp => NewPressedKey::NoOp,
        }
    }
}

/// Pressed Key which may be pending, or a resolved key state.
#[derive(Debug, PartialEq)]
pub enum PressedKeyResult<R, PKS, KS> {
    /// Unresolved key state. (e.g. tap-hold or chorded keys when first pressed).
    Pending(PKS),
    /// Resolved as a new pressed key.
    NewPressedKey(NewPressedKey<R>),
    /// Resolved key state.
    Resolved(KS),
}

impl<R, PKS, KS> PressedKeyResult<R, PKS, KS> {
    /// Returns the Resolved variant, or else panics.
    #[cfg(feature = "std")]
    pub fn unwrap_resolved(self) -> KS {
        match self {
            PressedKeyResult::Resolved(r) => r,
            _ => panic!("PressedKeyResult::unwrap_resolved: not Resolved"),
        }
    }

    /// Maps the PressedKeyResult into a new type.
    pub fn map<TPKS, TKS>(
        self,
        f: fn(PKS) -> TPKS,
        g: fn(KS) -> TKS,
    ) -> PressedKeyResult<R, TPKS, TKS> {
        match self {
            PressedKeyResult::Pending(pks) => PressedKeyResult::Pending(f(pks)),
            PressedKeyResult::NewPressedKey(npk) => PressedKeyResult::NewPressedKey(npk),
            PressedKeyResult::Resolved(ks) => PressedKeyResult::Resolved(g(ks)),
        }
    }

    /// Maps the PressedKeyResult into a new type.
    pub fn into_result<TPKS, TKS>(self) -> PressedKeyResult<R, TPKS, TKS>
    where
        PKS: Into<TPKS>,
        KS: Into<TKS>,
    {
        self.map(|pks| pks.into(), |ks| ks.into())
    }
}

/// Outcome of [System::new_pressed_key].
pub type NewPressedKeyOutput<R, PKS, KS, E> = (PressedKeyResult<R, PKS, KS>, KeyEvents<E>);

/// The interface for key `System` behaviour.
///
/// A `System` has an associated `Ref`, [Context], `Event`, and [KeyState].
///
/// The generic `PK` is used as the type of the `PressedKey` that the `Key`
///  produces.
/// (e.g. [layered::LayeredKey]'s pressed key state passes-through to
///  the keys of its layers).
pub trait System<R>: Debug {
    /// Used to identify the key definition in the keymap.
    type Ref: Copy;

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

    /// Produces a pressed key value, and may
    ///  yield some [ScheduledEvent]s.
    /// (e.g. [tap_hold::Key] may schedule a [tap_hold::Event::TapHoldTimeout]
    ///  so that holding the key resolves as a hold,
    ///  when a timeout is configured).
    fn new_pressed_key(
        &self,
        keymap_index: u16,
        context: &Self::Context,
        key_ref: Self::Ref,
    ) -> NewPressedKeyOutput<R, Self::PendingKeyState, Self::KeyState, Self::Event>;

    /// Update the given pending key state with the given impl.
    fn update_pending_state(
        &self,
        pending_state: &mut Self::PendingKeyState,
        keymap_index: u16,
        context: &Self::Context,
        key_ref: Self::Ref,
        event: Event<Self::Event>,
    ) -> (Option<NewPressedKey<R>>, KeyEvents<Self::Event>);

    /// Used to update the [KeyState]'s state, and possibly yield event(s).
    fn update_state(
        &self,
        _key_state: &mut Self::KeyState,
        _ref: &Self::Ref,
        _context: &Self::Context,
        _keymap_index: u16,
        _event: Event<Self::Event>,
    ) -> KeyEvents<Self::Event> {
        KeyEvents::no_events()
    }

    /// Output for the pressed key state.
    fn key_output(&self, _ref: &Self::Ref, _key_state: &Self::KeyState) -> Option<KeyOutput> {
        None
    }
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
pub struct KeyboardModifiers(u8);

impl core::ops::Deref for KeyboardModifiers {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl core::fmt::Debug for KeyboardModifiers {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut ds = f.debug_struct("KeyboardModifiers");
        if self.0 & Self::LEFT_CTRL_U8 != 0 {
            ds.field("left_ctrl", &true);
        }
        if self.0 & Self::LEFT_SHIFT_U8 != 0 {
            ds.field("left_shift", &true);
        }
        if self.0 & Self::LEFT_ALT_U8 != 0 {
            ds.field("left_alt", &true);
        }
        if self.0 & Self::LEFT_GUI_U8 != 0 {
            ds.field("left_gui", &true);
        }
        if self.0 & Self::RIGHT_CTRL_U8 != 0 {
            ds.field("right_ctrl", &true);
        }
        if self.0 & Self::RIGHT_SHIFT_U8 != 0 {
            ds.field("right_shift", &true);
        }
        if self.0 & Self::RIGHT_ALT_U8 != 0 {
            ds.field("right_alt", &true);
        }
        if self.0 & Self::RIGHT_GUI_U8 != 0 {
            ds.field("right_gui", &true);
        }
        ds.finish_non_exhaustive()
    }
}

impl KeyboardModifiers {
    /// Byte value for left ctrl.
    pub const LEFT_CTRL_U8: u8 = 0x01;
    /// Byte value for left shift.
    pub const LEFT_SHIFT_U8: u8 = 0x02;
    /// Byte value for left alt.
    pub const LEFT_ALT_U8: u8 = 0x04;
    /// Byte value for left gui.
    pub const LEFT_GUI_U8: u8 = 0x08;
    /// Byte value for right ctrl.
    pub const RIGHT_CTRL_U8: u8 = 0x10;
    /// Byte value for right shift.
    pub const RIGHT_SHIFT_U8: u8 = 0x20;
    /// Byte value for right alt.
    pub const RIGHT_ALT_U8: u8 = 0x40;
    /// Byte value for right gui.
    pub const RIGHT_GUI_U8: u8 = 0x80;

    /// Constructs with modifiers defaulting to false.
    pub const fn new() -> Self {
        KeyboardModifiers(0x00)
    }

    /// Constructs with modifiers with the given byte.
    pub const fn from_byte(b: u8) -> Self {
        KeyboardModifiers(b)
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
    pub const LEFT_CTRL: KeyboardModifiers = KeyboardModifiers(Self::LEFT_CTRL_U8);

    /// Const for left shift.
    pub const LEFT_SHIFT: KeyboardModifiers = KeyboardModifiers(Self::LEFT_SHIFT_U8);

    /// Const for left alt.
    pub const LEFT_ALT: KeyboardModifiers = KeyboardModifiers(Self::LEFT_ALT_U8);

    /// Const for left gui.
    pub const LEFT_GUI: KeyboardModifiers = KeyboardModifiers(Self::LEFT_GUI_U8);

    /// Const for right ctrl.
    pub const RIGHT_CTRL: KeyboardModifiers = KeyboardModifiers(Self::RIGHT_CTRL_U8);

    /// Const for right shift.
    pub const RIGHT_SHIFT: KeyboardModifiers = KeyboardModifiers(Self::RIGHT_SHIFT_U8);

    /// Const for right alt.
    pub const RIGHT_ALT: KeyboardModifiers = KeyboardModifiers(Self::RIGHT_ALT_U8);

    /// Const for right gui.
    pub const RIGHT_GUI: KeyboardModifiers = KeyboardModifiers(Self::RIGHT_GUI_U8);

    /// Predicate for whether the key code is a modifier key code.
    pub const fn is_modifier_key_code(key_code: u8) -> bool {
        matches!(key_code, 0xE0..=0xE7)
    }

    /// Constructs a Vec of key codes from the modifiers.
    pub fn as_key_codes(&self) -> heapless::Vec<u8, 8> {
        let mut key_codes = heapless::Vec::new();

        if self.0 & Self::LEFT_CTRL_U8 != 0 {
            let _ = key_codes.push(0xE0);
        }
        if self.0 & Self::LEFT_SHIFT_U8 != 0 {
            let _ = key_codes.push(0xE1);
        }
        if self.0 & Self::LEFT_ALT_U8 != 0 {
            let _ = key_codes.push(0xE2);
        }
        if self.0 & Self::LEFT_GUI_U8 != 0 {
            let _ = key_codes.push(0xE3);
        }
        if self.0 & Self::RIGHT_CTRL_U8 != 0 {
            let _ = key_codes.push(0xE4);
        }
        if self.0 & Self::RIGHT_SHIFT_U8 != 0 {
            let _ = key_codes.push(0xE5);
        }
        if self.0 & Self::RIGHT_ALT_U8 != 0 {
            let _ = key_codes.push(0xE6);
        }
        if self.0 & Self::RIGHT_GUI_U8 != 0 {
            let _ = key_codes.push(0xE7);
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
        KeyboardModifiers(self.0 | other.0)
    }

    /// Whether this keyboard modifiers includes all the other modifiers.
    pub const fn has_modifiers(&self, other: &KeyboardModifiers) -> bool {
        self.0 & other.0 != 0
    }
}

/// Enum for the different types of key codes.
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyUsage {
    /// Key usage code.
    Keyboard(u8),
    /// Consumer usage code.
    Consumer(u8),
    /// Custom code. (Behaviour defined by firmware implementation).
    Custom(u8),
    /// Mouse usage.
    Mouse(MouseOutput),
}

impl KeyUsage {
    /// A key usage with no key code.
    pub const NO_USAGE: KeyUsage = KeyUsage::Keyboard(0x00);
}

impl Default for KeyUsage {
    fn default() -> Self {
        KeyUsage::NO_USAGE
    }
}

/// Struct for the output from [KeyState].
#[derive(Deserialize, Serialize, Clone, Copy, PartialEq, Eq)]
pub struct KeyOutput {
    #[serde(default)]
    key_code: KeyUsage,
    #[serde(default)]
    key_modifiers: KeyboardModifiers,
}

impl core::fmt::Debug for KeyOutput {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match (
            self.key_code != KeyUsage::NO_USAGE,
            self.key_modifiers != KeyboardModifiers::NONE,
        ) {
            (true, true) => f
                .debug_struct("KeyOutput")
                .field("key_code", &self.key_code)
                .field("key_modifiers", &self.key_modifiers)
                .finish(),
            (false, true) => f
                .debug_struct("KeyOutput")
                .field("key_modifiers", &self.key_modifiers)
                .finish(),
            _ => f
                .debug_struct("KeyOutput")
                .field("key_code", &self.key_code)
                .finish(),
        }
    }
}

impl KeyOutput {
    /// A key output with no key code and no modifiers.
    pub const NO_OUTPUT: KeyOutput = KeyOutput {
        key_code: KeyUsage::Keyboard(0x00),
        key_modifiers: KeyboardModifiers::new(),
    };

    /// Constructs a [KeyOutput] from a key usage.
    pub const fn from_usage(key_usage: KeyUsage) -> Self {
        match key_usage {
            KeyUsage::Keyboard(kc) => Self::from_key_code(kc),
            KeyUsage::Consumer(cc) => Self::from_consumer_code(cc),
            KeyUsage::Custom(cu) => Self::from_custom_code(cu),
            KeyUsage::Mouse(mo) => Self::from_mouse_output(mo),
        }
    }

    /// Constructs a [KeyOutput] from a key usage.
    pub const fn from_usage_with_modifiers(
        key_usage: KeyUsage,
        key_modifiers: KeyboardModifiers,
    ) -> Self {
        match key_usage {
            KeyUsage::Keyboard(kc) => {
                if let Some(usage_key_modifiers) = KeyboardModifiers::from_key_code(kc) {
                    KeyOutput {
                        key_code: KeyUsage::Keyboard(0x00),
                        key_modifiers: usage_key_modifiers.union(&key_modifiers),
                    }
                } else {
                    KeyOutput {
                        key_code: KeyUsage::Keyboard(kc),
                        key_modifiers,
                    }
                }
            }
            _ => KeyOutput {
                key_code: key_usage,
                key_modifiers,
            },
        }
    }

    /// Constructs a [KeyOutput] from a key code.
    pub const fn from_key_code(key_code: u8) -> Self {
        if let Some(key_modifiers) = KeyboardModifiers::from_key_code(key_code) {
            KeyOutput {
                key_code: KeyUsage::Keyboard(0x00),
                key_modifiers,
            }
        } else {
            KeyOutput {
                key_code: KeyUsage::Keyboard(key_code),
                key_modifiers: KeyboardModifiers::new(),
            }
        }
    }

    /// Constructs a [KeyOutput] from a key code with the given keyboard modifiers.
    pub const fn from_key_code_with_modifiers(
        key_code: u8,
        key_modifiers: KeyboardModifiers,
    ) -> Self {
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
    pub const fn from_key_modifiers(key_modifiers: KeyboardModifiers) -> Self {
        KeyOutput {
            key_code: KeyUsage::Keyboard(0x00),
            key_modifiers,
        }
    }

    /// Constructs a [KeyOutput] from a consumer code.
    pub const fn from_consumer_code(usage_code: u8) -> Self {
        KeyOutput {
            key_code: KeyUsage::Consumer(usage_code),
            key_modifiers: KeyboardModifiers::new(),
        }
    }

    /// Constructs a [KeyOutput] from a custom code.
    pub const fn from_custom_code(custom_code: u8) -> Self {
        KeyOutput {
            key_code: KeyUsage::Custom(custom_code),
            key_modifiers: KeyboardModifiers::new(),
        }
    }

    /// Constructs a [KeyOutput] from a mouse output.
    pub const fn from_mouse_output(mouse_output: MouseOutput) -> Self {
        KeyOutput {
            key_code: KeyUsage::Mouse(mouse_output),
            key_modifiers: KeyboardModifiers::new(),
        }
    }

    /// Returns the key code value.
    pub const fn key_code(&self) -> KeyUsage {
        self.key_code
    }

    /// Returns the keyboard modifiers of the key output.
    pub const fn key_modifiers(&self) -> KeyboardModifiers {
        self.key_modifiers
    }
}

/// Struct for the mouse output.
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
pub struct MouseOutput {
    /// Bitmask of pressed buttons.
    pub pressed_buttons: u8,
    /// X direction.
    pub x: i8,
    /// Y direction.
    pub y: i8,
    /// Vertical scroll.
    pub vertical_scroll: i8,
    /// Horizontal scroll.
    pub horizontal_scroll: i8,
}

impl MouseOutput {
    /// A mouse output with no buttons pressed and no movement.
    pub const NO_OUTPUT: MouseOutput = MouseOutput {
        pressed_buttons: 0,
        x: 0,
        y: 0,
        vertical_scroll: 0,
        horizontal_scroll: 0,
    };

    /// Combines two mouse output values into one.
    pub fn combine(&self, other: &Self) -> Self {
        Self {
            pressed_buttons: self.pressed_buttons | other.pressed_buttons,
            x: self.x.saturating_add(other.x),
            y: self.y.saturating_add(other.y),
            vertical_scroll: self.vertical_scroll.saturating_add(other.vertical_scroll),
            horizontal_scroll: self
                .horizontal_scroll
                .saturating_add(other.horizontal_scroll),
        }
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
        _context: &Self::Context,
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
pub struct NoOpKeyState;

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

/// Events which are either input, or for a particular [System::Event].
///
/// It's useful for key implementations to use [Event] with [System::Event],
///  and map [System::Event] to and partially from [composite::Event].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event<T> {
    /// Keymap input events, such as physical key presses.
    Input(input::Event),
    /// Key implementation specific events.
    Key {
        /// The keymap index the event was generated from.
        keymap_index: u16,
        /// A [System::Event] event.
        key_event: T,
    },
    /// Invoke a keymap callback
    Keymap(crate::keymap::KeymapEvent),
}

impl<T: Copy> Event<T> {
    /// Constructs an [Event] from an [System::Event].
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
    pub fn try_into_key_event<U, E>(self) -> EventResult<Event<U>>
    where
        T: TryInto<U, Error = E>,
    {
        match self {
            Event::Input(event) => Ok(Event::Input(event)),
            Event::Key {
                key_event,
                keymap_index,
            } => key_event
                .try_into()
                .map(|key_event| Event::Key {
                    key_event,
                    keymap_index,
                })
                .map_err(|_| EventError::UnmappableEvent),
            Event::Keymap(cb) => Ok(Event::Keymap(cb)),
        }
    }

    /// Whether this event targets the given `keymap_index`.
    ///
    /// Input press/release and key-specific events carry a `keymap_index`;
    /// keymap callbacks and other variants do not.
    pub(crate) fn targets_keymap_index(&self, keymap_index: u16) -> bool {
        match self {
            Event::Input(input::Event::Press {
                keymap_index: queued_kmi,
            })
            | Event::Input(input::Event::Release {
                keymap_index: queued_kmi,
            }) => *queued_kmi == keymap_index,
            Event::Key {
                keymap_index: queued_kmi,
                ..
            } => *queued_kmi == keymap_index,
            _ => false,
        }
    }
}

/// Returns the events that should be replayed when a pending key resolves.
///
/// Resolution filter:
/// - All queued events **not** targeting `keymap_index` are included.
/// - Only the **last** event targeting `keymap_index` is included (if any).
///
/// **Example:**
///  session log `[Press(1), Press(0), Release(0)]` for resolving key 0:
///   yields `[Press(1), Release(0)]`
///   — other-key inputs are kept,
///   - but only the final self-event (`Release(0)`) remains from key 0's own press/release pair.
pub(crate) fn pending_resolution_events<Ev: Copy, const N: usize>(
    queued_events: &heapless::Vec<Event<Ev>, N>,
    keymap_index: u16,
) -> heapless::Vec<Event<Ev>, N> {
    let (self_events, other_events): (heapless::Vec<Event<Ev>, N>, heapless::Vec<Event<Ev>, N>) =
        queued_events
            .iter()
            .partition(|ev| ev.targets_keymap_index(keymap_index));

    let mut result = heapless::Vec::new();
    for ev in other_events.iter().chain(self_events.last()) {
        let _ = result.push(*ev);
    }
    result
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

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests {
    use super::*;

    #[test]
    fn pending_resolution_events_empty_returns_empty() {
        let queued: heapless::Vec<Event<()>, 16> = heapless::Vec::new();
        let result = pending_resolution_events(&queued, 0);
        assert!(result.is_empty());
    }

    #[test]
    fn pending_resolution_events_other_key_events_all_included() {
        let mut queued: heapless::Vec<Event<()>, 16> = heapless::Vec::new();
        queued.push(input::Event::press(1).into()).unwrap();
        queued.push(input::Event::release(2).into()).unwrap();
        let result = pending_resolution_events(&queued, 0);
        assert_eq!(2, result.len());
    }

    #[test]
    fn pending_resolution_events_resolving_key_only_last_included() {
        let mut queued: heapless::Vec<Event<()>, 16> = heapless::Vec::new();
        queued.push(input::Event::press(0).into()).unwrap();
        queued.push(input::Event::release(0).into()).unwrap();
        let result = pending_resolution_events(&queued, 0);
        assert_eq!(1, result.len());
        assert_eq!(Event::from(input::Event::release(0)), result[0]);
    }

    #[test]
    fn pending_resolution_events_mix_other_and_resolving_key() {
        let mut queued: heapless::Vec<Event<()>, 16> = heapless::Vec::new();
        queued.push(input::Event::press(1).into()).unwrap();
        queued.push(input::Event::press(0).into()).unwrap();
        queued.push(input::Event::release(0).into()).unwrap();
        let result = pending_resolution_events(&queued, 0);
        assert_eq!(2, result.len());
        assert_eq!(Event::from(input::Event::press(1)), result[0]);
        assert_eq!(Event::from(input::Event::release(0)), result[1]);
    }
}
