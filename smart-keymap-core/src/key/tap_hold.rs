use core::fmt::Debug;
use core::ops::Index;

use serde::Deserialize;

use crate::input;
use crate::key;
use crate::keymap;

/// Reference for a tap_hold key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Ref(pub u8);

/// Bitmask of keymap indices (0..128) used for positional hold triggers.
///
/// Empty mask means "no restriction" only when wrapped in [`Option::None`] on
/// [Key]; a `Some(empty)` mask matches no positions.
///
/// JSON accepts an array of keymap indices (e.g. `[2, 3, 18]`), matching the
/// Nickel authoring surface `hold_trigger_key_positions`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct KeyPositionMask {
    /// Bits for keymap indices 0..63.
    pub lo: u64,
    /// Bits for keymap indices 64..127.
    pub hi: u64,
}

impl<'de> Deserialize<'de> for KeyPositionMask {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = KeyPositionMask;

            fn expecting(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
                f.write_str("an array of keymap indices")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut mask = KeyPositionMask::EMPTY;
                while let Some(idx) = seq.next_element::<u16>()? {
                    mask = mask.with(idx);
                }
                Ok(mask)
            }
        }

        deserializer.deserialize_seq(Visitor)
    }
}

impl KeyPositionMask {
    /// Empty mask (matches no indices).
    pub const EMPTY: Self = Self { lo: 0, hi: 0 };

    /// Whether `keymap_index` is set in the mask.
    pub const fn contains(self, keymap_index: u16) -> bool {
        if keymap_index < 64 {
            (self.lo & (1u64 << keymap_index)) != 0
        } else if keymap_index < 128 {
            (self.hi & (1u64 << (keymap_index - 64))) != 0
        } else {
            false
        }
    }

    /// Returns a copy with `keymap_index` set (indices ≥ 128 are ignored).
    pub const fn with(mut self, keymap_index: u16) -> Self {
        if keymap_index < 64 {
            self.lo |= 1u64 << keymap_index;
        } else if keymap_index < 128 {
            self.hi |= 1u64 << (keymap_index - 64);
        }
        self
    }

    /// Build a mask from a list of keymap indices.
    pub const fn from_indices(indices: &[u16]) -> Self {
        let mut mask = Self::EMPTY;
        let mut i = 0;
        while i < indices.len() {
            mask = mask.with(indices[i]);
            i += 1;
        }
        mask
    }
}

/// A key with tap-hold functionality.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key<R> {
    /// The 'tap' key.
    pub tap: R,
    /// The 'hold' key.
    pub hold: R,
    /// When set, only these keymap indices may resolve an interrupt as hold
    /// (ZMK `hold-trigger-key-positions` / opposite-hand HRM polish).
    ///
    /// `None` means any other key may trigger hold (default).
    ///
    /// JSON field name matches the Nickel authoring surface
    ///  (`hold_trigger_key_positions`).
    #[serde(default, rename = "hold_trigger_key_positions")]
    pub hold_trigger_positions: Option<KeyPositionMask>,
}

impl<R> Key<R> {
    /// Constructs a new tap-hold key (no positional hold restriction).
    pub const fn new(tap: R, hold: R) -> Key<R> {
        Key {
            tap,
            hold,
            hold_trigger_positions: None,
        }
    }

    /// Constructs a tap-hold key that only resolves as hold when interrupted
    /// by a key whose keymap index is in `hold_trigger_positions`.
    pub const fn with_hold_triggers(
        tap: R,
        hold: R,
        hold_trigger_positions: KeyPositionMask,
    ) -> Key<R> {
        Key {
            tap,
            hold,
            hold_trigger_positions: Some(hold_trigger_positions),
        }
    }

    /// Whether `other_keymap_index` is allowed to resolve this key as hold.
    pub const fn allows_hold_trigger(&self, other_keymap_index: u16) -> bool {
        match self.hold_trigger_positions {
            None => true,
            Some(mask) => mask.contains(other_keymap_index),
        }
    }
}

#[cfg(feature = "std")]
impl<R: Default> Default for Key<R> {
    fn default() -> Self {
        Key {
            tap: R::default(),
            hold: R::default(),
            hold_trigger_positions: None,
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
    ///
    /// When `None`, the tap/hold decision does not timeout;
    /// the key resolves only on release (as tap) or interruption
    /// (depending on [InterruptResponse]).
    #[serde(default = "default_timeout")]
    pub timeout: Option<u16>,

    /// How the tap-hold key should respond to interruptions.
    #[serde(default = "default_interrupt_response")]
    pub interrupt_response: InterruptResponse,

    /// Amount of time (in milliseconds) the keymap must have been idle
    ///  in order for tap hold to support 'hold' functionality.
    ///
    /// This reduces disruption from unexpected hold resolutions
    ///  when typing quickly.
    pub required_idle_time: Option<u16>,

    /// When true, timeout alone never resolves as hold (ZMK `retro-tap`).
    ///
    /// Hold activates only when another key interrupts (per
    /// [InterruptResponse]); releasing the key alone always yields tap,
    /// even after the timeout has elapsed.
    #[serde(default)]
    pub retro_tap: bool,
}

/// The default timeout.
pub const DEFAULT_TIMEOUT: u16 = 200;

/// The default interrupt response.
pub const DEFAULT_INTERRUPT_RESPONSE: InterruptResponse = InterruptResponse::Ignore;

fn default_timeout() -> Option<u16> {
    Some(DEFAULT_TIMEOUT)
}

fn default_interrupt_response() -> InterruptResponse {
    DEFAULT_INTERRUPT_RESPONSE
}

/// Default tap hold config.
pub const DEFAULT_CONFIG: Config = Config {
    timeout: Some(DEFAULT_TIMEOUT),
    interrupt_response: DEFAULT_INTERRUPT_RESPONSE,
    required_idle_time: None,
    retro_tap: false,
};

impl Config {
    /// Constructs a new default [Config].
    pub const fn new() -> Self {
        DEFAULT_CONFIG
    }
}

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

impl Context {
    /// Constructs a context from the given config
    pub const fn from_config(config: Config) -> Context {
        Context {
            config,
            idle_time_ms: 0,
        }
    }

    /// Re-construct from context's [Config], clearing idle-time tracking.
    pub fn reset(&mut self) {
        *self = Self::from_config(self.config);
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
        retro_tap: bool,
        keymap_index: u16,
        event: key::Event<Event>,
        allows_hold_trigger: impl Fn(u16) -> bool,
    ) -> Option<TapHoldState> {
        match interrupt_response {
            InterruptResponse::HoldOnKeyPress => {
                match event {
                    key::Event::Input(input::Event::Press { keymap_index: ki }) => {
                        // TapHold: interruption resolves as Hold when positional
                        //  filter (if any) allows the interrupting key.
                        if allows_hold_trigger(ki) {
                            Some(TapHoldState::Hold)
                        } else {
                            None
                        }
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
                        if retro_tap {
                            // Stay pending until interrupt or self-release.
                            None
                        } else {
                            Some(TapHoldState::Hold)
                        }
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
                        } else if Some(ki) == self.other_pressed_keymap_index
                            && allows_hold_trigger(ki)
                        {
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
                        if retro_tap {
                            None
                        } else {
                            Some(TapHoldState::Hold)
                        }
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
                        if retro_tap {
                            None
                        } else {
                            Some(TapHoldState::Hold)
                        }
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
        allows_hold_trigger: impl Fn(u16) -> bool,
    ) -> Option<TapHoldState> {
        // Check for interrupting taps
        // (track other key press)
        if let key::Event::Input(input::Event::Press { keymap_index: ki }) = event {
            self.other_pressed_keymap_index = Some(ki);
        }

        // Resolve tap-hold state per the event.
        let Context { config, .. } = context;
        self.hold_resolution(
            config.interrupt_response,
            config.retro_tap,
            keymap_index,
            event,
            allows_hold_trigger,
        )
    }
}

/// Key state for tap_hold keys. (Not used).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState;

/// The [key::System] implementation for tap hold keys.
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
    ) -> (PendingKeyState, Option<key::ScheduledEvent<Event>>) {
        let pending = PendingKeyState::new();
        let scheduled = context.config.timeout.map(|timeout| {
            key::ScheduledEvent::after(
                timeout,
                key::Event::key_event(keymap_index, Event::TapHoldTimeout),
            )
        });
        (pending, scheduled)
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
                    let (th_pks, maybe_sch_ev) = self.new_pending_key(context, keymap_index);
                    let pk = key::PressedKeyResult::Pending(th_pks);
                    let pke = match maybe_sch_ev {
                        Some(sch_ev) => {
                            key::KeyEvents::scheduled_event(sch_ev.into_scheduled_event())
                        }
                        None => key::KeyEvents::no_events(),
                    };
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
                let (th_pks, maybe_sch_ev) = self.new_pending_key(context, keymap_index);
                let pk = key::PressedKeyResult::Pending(th_pks);
                let pke = match maybe_sch_ev {
                    Some(sch_ev) => key::KeyEvents::scheduled_event(sch_ev.into_scheduled_event()),
                    None => key::KeyEvents::no_events(),
                };
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
        let key_def = &self.keys[key_index as usize];
        let th_state = pending_state.handle_event(context, keymap_index, event, |ki| {
            key_def.allows_hold_trigger(ki)
        });
        if let Some(th_state) = th_state {
            let Key { tap, hold, .. } = *key_def;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sizeof_ref() {
        assert_eq!(1, core::mem::size_of::<Ref>());
    }

    #[test]
    fn test_sizeof_event() {
        assert_eq!(0, core::mem::size_of::<Event>());
    }

    #[test]
    fn test_key_position_mask_contains() {
        let mask = KeyPositionMask::from_indices(&[0, 5, 64, 127]);
        assert!(mask.contains(0));
        assert!(mask.contains(5));
        assert!(mask.contains(64));
        assert!(mask.contains(127));
        assert!(!mask.contains(1));
        assert!(!mask.contains(63));
        assert!(!mask.contains(65));
    }

    #[test]
    fn test_allows_hold_trigger_default() {
        let key = Key::new(0u8, 1u8);
        assert!(key.allows_hold_trigger(0));
        assert!(key.allows_hold_trigger(99));
    }

    #[test]
    fn test_allows_hold_trigger_restricted() {
        let key = Key::with_hold_triggers(0u8, 1u8, KeyPositionMask::from_indices(&[2, 3]));
        assert!(!key.allows_hold_trigger(0));
        assert!(key.allows_hold_trigger(2));
        assert!(key.allows_hold_trigger(3));
    }
}
