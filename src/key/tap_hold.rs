#![doc = include_str!("doc_de_tap_hold.md")]

use core::fmt::Debug;

use serde::Deserialize;

use crate::input;
use crate::key;

use super::PressedKey as _;

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
    /// The timeout (in number of ticks) for a tap-hold key to resolve as hold.
    #[serde(default = "default_timeout")]
    pub timeout: u16,

    /// How the tap-hold key should respond to interruptions.
    #[serde(default = "default_interrupt_response")]
    pub interrupt_response: InterruptResponse,
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
};

impl Default for Config {
    /// Returns the default context.
    fn default() -> Self {
        DEFAULT_CONFIG
    }
}

/// A key with tap-hold functionality.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key<K: key::Key> {
    /// The 'tap' key.
    pub tap: K,
    /// The 'hold' key.
    pub hold: K,
}

impl<K: key::Key> Key<K> {
    /// Constructs a new tap-hold key.
    pub const fn new(tap: K, hold: K) -> Key<K> {
        Key { tap, hold }
    }

    /// Maps the Key of the Key into a new type.
    pub fn map_key<T: key::Key>(self, f: fn(K) -> T) -> Key<T> {
        let Key { tap, hold } = self;
        Key {
            tap: f(tap),
            hold: f(hold),
        }
    }

    /// Maps the Key of the Key into a new type.
    pub fn into_key<T: key::Key>(self) -> Key<T>
    where
        K: Into<T>,
    {
        self.map_key(|k| k.into())
    }
}

impl<K: key::Key> Key<K> {
    /// Constructs a new pressed key state and a scheduled event for the tap-hold key.
    pub fn new_pressed_key(
        &self,
        context: Context,
        keymap_index: u16,
    ) -> (PressedKeyState<K>, key::ScheduledEvent<Event>) {
        let timeout_ev = Event::TapHoldTimeout;
        (
            PressedKeyState::new(),
            key::ScheduledEvent::after(
                context.config.timeout,
                key::Event::key_event(keymap_index, timeout_ev),
            ),
        )
    }
}

/// Context for [Key].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context {
    config: Config,
}

/// Default context.
pub const DEFAULT_CONTEXT: Context = Context::from_config(DEFAULT_CONFIG);

impl Context {
    /// Constructs a context from the given config
    pub const fn from_config(config: Config) -> Context {
        Context { config }
    }
}

/// The state of a tap-hold key.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TapHoldState {
    /// Not yet resolved as tap or hold.
    Pending,
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
#[derive(Debug, PartialEq)]
pub struct PressedKeyState<K: key::Key> {
    state: TapHoldState,
    pressed_key: Option<K::PressedKey>,
    // For tracking 'tap' interruptions
    other_pressed_keymap_index: Option<u16>,
}

/// Convenience type for a pressed tap-hold key.
pub type PressedKey<K> = input::PressedKey<Key<K>, PressedKeyState<K>>;

impl<K: key::Key> PressedKeyState<K> {
    /// Constructs the initial pressed key state
    fn new() -> PressedKeyState<K> {
        PressedKeyState {
            state: TapHoldState::Pending,
            pressed_key: None,
            other_pressed_keymap_index: None,
        }
    }

    /// Maps the Key of the PressedKeyState into a new type.
    pub fn map_pressed_key<T: key::Key>(
        self,
        f: fn(K::PressedKey) -> T::PressedKey,
    ) -> PressedKeyState<T> {
        let PressedKeyState {
            state,
            pressed_key,
            other_pressed_keymap_index,
        } = self;
        PressedKeyState {
            state,
            pressed_key: pressed_key.map(f),
            other_pressed_keymap_index,
        }
    }

    /// Maps the Key of the PressedKeyState into a new type.
    pub fn into_pressed_key<T: key::Key>(self) -> PressedKeyState<T>
    where
        T::PressedKey: From<K::PressedKey>,
    {
        self.map_pressed_key(|pk| pk.into())
    }

    /// Resolves the state of the key, unless it has already been resolved.
    fn resolve(&mut self, state: TapHoldState) {
        if let TapHoldState::Pending = self.state {
            self.state = state;
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
        match self.state {
            TapHoldState::Pending => {
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
            _ => None,
        }
    }

    /// Returns the key output state.
    pub fn key_output(&self) -> key::KeyOutputState {
        match &self.pressed_key {
            Some(pk) => pk.key_output(),
            None => key::KeyOutputState::pending(),
        }
    }
}

impl<K: key::Key> PressedKeyState<K>
where
    K::Context: Into<Context>,
    K::Event: TryInto<Event>,
    K::Event: From<Event>,
{
    /// Returns at most 2 events
    pub fn handle_event_for(
        &mut self,
        context: K::Context,
        keymap_index: u16,
        key: &Key<K>,
        event: key::Event<K::Event>,
    ) -> key::PressedKeyEvents<K::Event> {
        let mut pke = key::PressedKeyEvents::no_events();

        // Add events from inner pk handle_event
        if let Some(pk) = &mut self.pressed_key {
            let pk_ev = pk.handle_event(context, event);
            pke.extend(pk_ev);
        }

        // Check for interrupting taps
        // (track other key press, if this PKS has not resolved)
        match self.state {
            TapHoldState::Pending => match event {
                key::Event::Input(input::Event::Press { keymap_index: ki }) => {
                    self.other_pressed_keymap_index = Some(ki);
                }
                _ => {}
            },
            _ => {}
        }

        // Resolve tap-hold state per the event.
        let Context { config, .. } = context.into();
        if let Ok(ev) = event.try_into_key_event(|e| e.try_into()) {
            match self.hold_resolution(config.interrupt_response, keymap_index, ev) {
                Some(TapHoldState::Hold) => {
                    self.resolve(TapHoldState::Hold);

                    let (hold_pk, hold_pke) = key.hold.new_pressed_key(context, keymap_index);
                    self.pressed_key = Some(hold_pk);
                    pke.extend(hold_pke);
                }
                Some(TapHoldState::Tap) => {
                    self.resolve(TapHoldState::Tap);

                    let (tap_pk, tap_pke) = key.tap.new_pressed_key(context, keymap_index);
                    self.pressed_key = Some(tap_pk);
                    pke.extend(tap_pke);
                }
                _ => {}
            }
        }

        pke
    }
}
