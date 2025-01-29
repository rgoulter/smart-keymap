#![doc = include_str!("doc_de_tap_hold.md")]

use core::fmt::Debug;

use serde::Deserialize;

use crate::input;
use crate::key;

use super::PressedKey as _;

use crate::init::CONFIG;

const TAP_HOLD_CONFIG: Config = CONFIG.tap_hold;

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
    pub timeout: u16,

    /// How the tap-hold key should respond to interruptions.
    pub interrupt_response: InterruptResponse,
}

/// Default tap hold config.
pub const DEFAULT_CONFIG: Config = Config {
    timeout: 200,
    interrupt_response: InterruptResponse::Ignore,
};

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

impl<K: key::Key> key::Key for Key<K> {
    type Context = key::ModifierKeyContext<Context, K::Context>;
    type ContextEvent = ();
    type Event = key::ModifierKeyEvent<Event, K::Event>;
    type PressedKeyState = PressedKeyState<K>;

    fn new_pressed_key(
        &self,
        _context: Self::Context,
        keymap_index: u16,
    ) -> (
        input::PressedKey<Self, Self::PressedKeyState>,
        key::PressedKeyEvents<Self::Event>,
    ) {
        let timeout_ev = Event::TapHoldTimeout;
        (
            input::PressedKey {
                keymap_index,
                key: *self,
                pressed_key_state: PressedKeyState::new(),
            },
            key::PressedKeyEvents::scheduled_event(
                TAP_HOLD_CONFIG.timeout,
                key::Event::key_event(keymap_index, key::ModifierKeyEvent::Modifier(timeout_ev)),
            ),
        )
    }
}

/// Context for [Key].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Context {}

/// Default context.
pub const DEFAULT_CONTEXT: Context = Context::from_config(DEFAULT_CONFIG);

impl Context {
    /// Constructs a context from the given config
    pub const fn from_config(_config: Config) -> Context {
        Context {}
    }
}

/// The state of a tap-hold key.
#[derive(Debug, Clone, Copy)]
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
#[derive(Debug, Clone, Copy)]
pub struct PressedKeyState<K: key::Key> {
    state: TapHoldState,
    pressed_key: Option<input::PressedKey<K, K::PressedKeyState>>,
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
        event: key::Event<key::ModifierKeyEvent<Event, <K as key::Key>::Event>>,
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
                                key_event: key::ModifierKeyEvent::Modifier(Event::TapHoldTimeout),
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
                                key_event: key::ModifierKeyEvent::Modifier(Event::TapHoldTimeout),
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
                                key_event: key::ModifierKeyEvent::Modifier(Event::TapHoldTimeout),
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
}

impl<K: key::Key> key::PressedKeyState<Key<K>> for PressedKeyState<K> {
    type Event = key::ModifierKeyEvent<Event, K::Event>;

    /// Returns at most 2 events
    fn handle_event_for(
        &mut self,
        key::ModifierKeyContext {
            context: _,
            inner_context,
        }: <Key<K> as key::Key>::Context,
        keymap_index: u16,
        key: &Key<K>,
        event: key::Event<Self::Event>,
    ) -> key::PressedKeyEvents<Self::Event> {
        let mut pke = key::PressedKeyEvents::no_events();

        // Add events from inner pk handle_event
        if let Ok(inner_ev) = event.try_into_key_event(|mke| mke.try_into_inner()) {
            if let Some(pk) = &mut self.pressed_key {
                let pk_ev = pk
                    .pressed_key_state
                    .handle_event_for(inner_context, keymap_index, &pk.key, inner_ev)
                    .map_events(|ev| key::ModifierKeyEvent::Inner(ev));
                pke.extend(pk_ev);
            }
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
        match self.hold_resolution(TAP_HOLD_CONFIG.interrupt_response, keymap_index, event) {
            Some(TapHoldState::Hold) => {
                self.resolve(TapHoldState::Hold);

                let (hold_pk, hold_pke) = key.hold.new_pressed_key(inner_context, keymap_index);
                self.pressed_key = Some(hold_pk);
                pke.extend(hold_pke.map_events(|ev| key::ModifierKeyEvent::Inner(ev)));
            }
            Some(TapHoldState::Tap) => {
                self.resolve(TapHoldState::Tap);

                let (tap_pk, tap_pke) = key.tap.new_pressed_key(inner_context, keymap_index);
                self.pressed_key = Some(tap_pk);
                pke.extend(tap_pke.map_events(|ev| key::ModifierKeyEvent::Inner(ev)));
            }
            _ => {}
        }

        match event {
            key::Event::Input(input::Event::Release { keymap_index: ki }) if keymap_index == ki => {
                match (self.state, &self.pressed_key) {
                    // Tap Hold key released, and the tap hold key is "tap";
                    //  so, we send virtual key tap (press, scheduled release) with the
                    //  pressed key's output.
                    (TapHoldState::Tap, Some(pk)) => {
                        if let Some(key_output) = pk.key_output().to_option() {
                            let key_code = key_output.key_code();
                            let press_ev = input::Event::VirtualKeyPress {
                                key_code,
                                pressed_keymap_index: keymap_index,
                            };
                            let release_ev = input::Event::VirtualKeyRelease { key_code };
                            let mut events: key::PressedKeyEvents<Self::Event> =
                                key::PressedKeyEvents::event(press_ev.into());
                            events.schedule_event(10, release_ev.into());
                            pke.extend(events);
                        }

                        pke
                    }
                    _ => pke,
                }
            }
            _ => pke,
        }
    }

    fn key_output(&self, _key: &Key<K>) -> key::KeyOutputState {
        match &self.pressed_key {
            Some(pk) => pk.key_output(),
            None => key::KeyOutputState::pending(),
        }
    }
}
