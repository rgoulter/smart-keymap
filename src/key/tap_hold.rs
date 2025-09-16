// #![doc = include_str!("doc_de_tap_hold.md")]

use core::fmt::Debug;
use core::ops::Index;

use serde::Deserialize;

use crate::input;
use crate::key;
use crate::keymap;

/// Reference for a tap_hold key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Ref(pub u8);

/// A key with tap-hold functionality.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key<R> {
    /// The 'tap' key.
    pub tap: R,
    /// The 'hold' key.
    pub hold: R,
}

impl<R> Key<R> {
    /// Constructs a new tap-hold key.
    pub const fn new(tap: R, hold: R) -> Key<R> {
        Key { tap, hold }
    }
}

#[cfg(feature = "std")]
impl<R: Default> Default for Key<R> {
    fn default() -> Self {
        Key {
            tap: R::default(),
            hold: R::default(),
        }
    }
}

/// How the tap hold key should respond to interruptions (input events from other keys).
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum InterruptResponse {
    /// The tap-hold key ignores other key presses/taps.
    /// (Only resolves to hold on timeout).
    Ignore,
    /// The tap-hold key resolves as "hold" when interrupted by a key press.
    HoldOnKeyPress,
    /// The tap-hold key resolves as "hold" when interrupted by a key tap.
    /// (Another key was pressed and released).
    HoldOnKeyTap,
}

/// Configuration settings for tap hold keys.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Config {
    /// The timeout (in number of milliseconds) for a tap-hold key to resolve as hold.
    #[serde(default = "default_timeout")]
    pub timeout: u16,

    /// How the tap-hold key should respond to interruptions.
    #[serde(default = "default_interrupt_response")]
    pub interrupt_response: InterruptResponse,

    /// Amount of time (in milliseconds) the keymap must have been idle
    ///  in order for tap hold to support 'hold' functionality.
    ///
    /// This reduces disruption from unexpected hold resolutions
    ///  when typing quickly.
    pub required_idle_time: Option<u16>,
}

fn default_timeout() -> u16 {
    DEFAULT_CONFIG.timeout
}

fn default_interrupt_response() -> InterruptResponse {
    DEFAULT_CONFIG.interrupt_response
}

/// Default tap hold config.
pub const DEFAULT_CONFIG: Config = Config {
    timeout: 200,
    interrupt_response: InterruptResponse::Ignore,
    required_idle_time: None,
};

impl Default for Config {
    /// Returns the default context.
    fn default() -> Self {
        DEFAULT_CONFIG
    }
}

/// Context for [Key].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context {
    config: Config,
    idle_time_ms: u32,
}

/// Default context.
pub const DEFAULT_CONTEXT: Context = Context::from_config(DEFAULT_CONFIG);

impl Context {
    /// Constructs a context from the given config
    pub const fn from_config(config: Config) -> Context {
        Context {
            config,
            idle_time_ms: 0,
        }
    }

    /// Updates the context with the given keymap context.
    pub fn update_keymap_context(
        &mut self,
        keymap::KeymapContext { idle_time_ms, .. }: &keymap::KeymapContext,
    ) {
        self.idle_time_ms = *idle_time_ms;
    }
}

/// The state of a tap-hold key.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TapHoldState {
    /// Resolved as tap.
    Tap,
    /// Resolved as hold.
    Hold,
}

/// Events emitted by a tap-hold key.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// Event indicating the key has been held long enough to resolve as hold.
    TapHoldTimeout,
}

/// The state of a pressed tap-hold key.
#[derive(Debug, Clone, PartialEq)]
pub struct PendingKeyState {
    // For tracking 'tap' interruptions
    other_pressed_keymap_index: Option<u16>,
}

impl PendingKeyState {
    /// Constructs the initial pressed key state
    fn new() -> PendingKeyState {
        PendingKeyState {
            other_pressed_keymap_index: None,
        }
    }

    /// Compute whether the tap-hold key should resolve as tap or hold,
    ///  given the tap hold config, the current state, and the key event.
    fn hold_resolution(
        &self,
        interrupt_response: InterruptResponse,
        keymap_index: u16,
        event: key::Event<Event>,
    ) -> Option<TapHoldState> {
        match interrupt_response {
            InterruptResponse::HoldOnKeyPress => {
                match event {
                    key::Event::Input(input::Event::Press { .. }) => {
                        // TapHold: any interruption resolves pending TapHold as Hold.
                        Some(TapHoldState::Hold)
                    }
                    key::Event::Input(input::Event::Release { keymap_index: ki }) => {
                        if keymap_index == ki {
                            // TapHold: not interrupted; resolved as tap.
                            Some(TapHoldState::Tap)
                        } else {
                            None
                        }
                    }
                    key::Event::Key {
                        key_event: Event::TapHoldTimeout,
                        ..
                    } => {
                        // Key held long enough to resolve as hold.
                        Some(TapHoldState::Hold)
                    }
                    _ => None,
                }
            }
            InterruptResponse::HoldOnKeyTap => {
                match event {
                    key::Event::Input(input::Event::Release { keymap_index: ki }) => {
                        if keymap_index == ki {
                            // TapHold: not interrupted; resolved as tap.
                            Some(TapHoldState::Tap)
                        } else if Some(ki) == self.other_pressed_keymap_index {
                            // TapHold: interrupted by key tap (press + release); resolved as hold.
                            Some(TapHoldState::Hold)
                        } else {
                            None
                        }
                    }
                    key::Event::Key {
                        key_event: Event::TapHoldTimeout,
                        ..
                    } => {
                        // Key held long enough to resolve as hold.
                        Some(TapHoldState::Hold)
                    }
                    _ => None,
                }
            }
            InterruptResponse::Ignore => {
                match event {
                    key::Event::Input(input::Event::Release { keymap_index: ki }) => {
                        if keymap_index == ki {
                            // TapHold: not interrupted; resolved as tap.
                            Some(TapHoldState::Tap)
                        } else {
                            None
                        }
                    }
                    key::Event::Key {
                        key_event: Event::TapHoldTimeout,
                        ..
                    } => {
                        // Key held long enough to resolve as hold.
                        Some(TapHoldState::Hold)
                    }
                    _ => None,
                }
            }
        }
    }

    /// Returns at most 2 events
    pub fn handle_event(
        &mut self,
        context: &Context,
        keymap_index: u16,
        event: key::Event<Event>,
    ) -> Option<TapHoldState> {
        // Check for interrupting taps
        // (track other key press)
        if let key::Event::Input(input::Event::Press { keymap_index: ki }) = event {
            self.other_pressed_keymap_index = Some(ki);
        }

        // Resolve tap-hold state per the event.
        let Context { config, .. } = context;
        self.hold_resolution(config.interrupt_response, keymap_index, event)
    }
}

/// Key state for tap_hold keys. (Not used).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState;

/// The [key::System] implementation for keyboard keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct System<R, Keys: Index<usize, Output = Key<R>>> {
    keys: Keys,
}

impl<R, Keys: Index<usize, Output = Key<R>>> System<R, Keys> {
    /// Constructs a new [System] with the given key data.
    pub const fn new(key_data: Keys) -> Self {
        Self { keys: key_data }
    }

    fn new_pending_key(
        &self,
        context: &Context,
        keymap_index: u16,
    ) -> (PendingKeyState, key::ScheduledEvent<Event>) {
        let timeout_ev = Event::TapHoldTimeout;
        (
            PendingKeyState::new(),
            key::ScheduledEvent::after(
                context.config.timeout,
                key::Event::key_event(keymap_index, timeout_ev),
            ),
        )
    }
}

impl<R: Copy + Debug, Keys: Debug + Index<usize, Output = Key<R>>> key::System<R>
    for System<R, Keys>
{
    type Ref = Ref;
    type Context = Context;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        keymap_index: u16,
        context: &Self::Context,
        Ref(key_index): Ref,
    ) -> (
        key::PressedKeyResult<R, Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        match context.config.required_idle_time {
            Some(required_idle_time) => {
                if context.idle_time_ms >= required_idle_time as u32 {
                    // Keymap has been idle long enough; use pending tap-hold key state.
                    let (th_pks, sch_ev) = self.new_pending_key(context, keymap_index);
                    let pk = key::PressedKeyResult::Pending(th_pks);
                    let pke = key::KeyEvents::scheduled_event(sch_ev.into_scheduled_event());
                    (pk, pke)
                } else {
                    // Keymap has not been idle for long enough;
                    // immediately resolve as tap.
                    let Key {
                        tap: tap_key_ref, ..
                    } = self.keys[key_index as usize];
                    (
                        key::PressedKeyResult::NewPressedKey(key::NewPressedKey::key(tap_key_ref)),
                        key::KeyEvents::no_events(),
                    )
                }
            }
            None => {
                // Idle time not considered. Use pending tap-hold key state.
                let (th_pks, sch_ev) = self.new_pending_key(context, keymap_index);
                let pk = key::PressedKeyResult::Pending(th_pks);
                let pke = key::KeyEvents::scheduled_event(sch_ev.into_scheduled_event());
                (pk, pke)
            }
        }
    }

    fn update_pending_state(
        &self,
        pending_state: &mut Self::PendingKeyState,
        keymap_index: u16,
        context: &Self::Context,
        Ref(key_index): Ref,
        event: key::Event<Self::Event>,
    ) -> (Option<key::NewPressedKey<R>>, key::KeyEvents<Self::Event>) {
        let th_state = pending_state.handle_event(context.into(), keymap_index, event);
        if let Some(th_state) = th_state {
            let Key { tap, hold } = self.keys[key_index as usize];
            let new_key_ref = match th_state {
                key::tap_hold::TapHoldState::Tap => tap,
                key::tap_hold::TapHoldState::Hold => hold,
            };

            (
                Some(key::NewPressedKey::key(new_key_ref)),
                key::KeyEvents::no_events(),
            )
        } else {
            (None, key::KeyEvents::no_events())
        }
    }

    fn update_state(
        &self,
        _key_state: &mut Self::KeyState,
        _ref: &Self::Ref,
        _context: &Self::Context,
        _keymap_index: u16,
        _event: key::Event<Self::Event>,
    ) -> key::KeyEvents<Self::Event> {
        panic!() // tap_hold has no key state
    }

    fn key_output(
        &self,
        _key_ref: &Self::Ref,
        _key_state: &Self::KeyState,
    ) -> Option<key::KeyOutput> {
        panic!() // tap_hold has no key state
    }
}
