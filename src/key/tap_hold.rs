#![doc = include_str!("doc_de_tap_hold.md")]

use core::fmt::Debug;

use serde::Deserialize;

use crate::input;
use crate::key;

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
}

impl<K: key::Key> Key<K> {
    /// Constructs a new pressed key state and a scheduled event for the tap-hold key.
    fn new_pressed_key(
        &self,
        context: Context,
        key_path: key::KeyPath,
    ) -> (PendingKeyState, key::ScheduledEvent<Event>) {
        let keymap_index: u16 = key_path[0];
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

impl<
        K: key::Key<
            Context = crate::init::Context,
            Event = crate::init::Event,
            PendingKeyState = crate::init::PendingKeyState,
            KeyState = crate::init::KeyState,
        >,
    > key::Key for Key<K>
{
    type Context = crate::init::Context;
    type Event = crate::init::Event;
    type PendingKeyState = crate::init::PendingKeyState;
    type KeyState = crate::init::KeyState;

    fn new_pressed_key(
        &self,
        context: Self::Context,
        key_path: key::KeyPath,
    ) -> (
        key::PressedKeyResult<Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        let (th_pks, sch_ev) = self.new_pressed_key(context.into(), key_path.clone());
        let pk =
            key::PressedKeyResult::Pending(key_path, crate::init::PendingKeyState::TapHold(th_pks));
        let pke = key::KeyEvents::scheduled_event(sch_ev.into_scheduled_event());
        (pk, pke)
    }

    fn handle_event(
        &self,
        pending_state: &mut Self::PendingKeyState,
        context: Self::Context,
        key_path: key::KeyPath,
        event: key::Event<Self::Event>,
    ) -> (
        Option<key::PressedKeyResult<Self::PendingKeyState, Self::KeyState>>,
        key::KeyEvents<Self::Event>,
    ) {
        let keymap_index = key_path[0];
        match pending_state {
            crate::init::PendingKeyState::TapHold(th_pks) => {
                if let Ok(th_ev) = event.try_into_key_event(|e| e.try_into()) {
                    let th_state = th_pks.handle_event(context.into(), keymap_index, th_ev);
                    if let Some(th_state) = th_state {
                        let (i, nk) = match th_state {
                            key::tap_hold::TapHoldState::Tap => (0, &self.tap),
                            key::tap_hold::TapHoldState::Hold => (1, &self.hold),
                        };
                        let (pkr, pke) = nk.new_pressed_key(context, key_path);
                        // PRESSED KEY PATH: add Tap Hold item (0 = tap, 1 = hold)
                        let pkr = pkr.add_path_item(i);

                        (Some(pkr), pke)
                    } else {
                        (None, key::KeyEvents::no_events())
                    }
                } else {
                    (None, key::KeyEvents::no_events())
                }
            }
            _ => (None, key::KeyEvents::no_events()),
        }
    }

    fn lookup(
        &self,
        path: &[u16],
    ) -> &dyn key::Key<
        Context = Self::Context,
        Event = Self::Event,
        PendingKeyState = Self::PendingKeyState,
        KeyState = Self::KeyState,
    > {
        match path {
            [] => self,
            // 0 = tap, 1 = hold
            [0, path @ ..] => self.tap.lookup(path),
            [1, path @ ..] => self.hold.lookup(path),
            _ => panic!(),
        }
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
        context: Context,
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
