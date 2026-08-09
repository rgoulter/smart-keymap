use core::fmt::Debug;
use core::ops::Index;

use serde::Deserialize;

use crate::input;
use crate::key;
use crate::keymap;
use crate::slice::Slice;

/// Reference for a tap_hold key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Ref(pub u8);

/// Maximum number of *extra* tap-hold profiles beyond the default (profile 0).
///
/// Profile indices on keys are `0` ([`Config::default_profile`]) then
/// `1..` into [`Config::profiles`]. Total usable profiles = `1 + MAX_EXTRA_PROFILES`.
pub const MAX_EXTRA_PROFILES: usize = 7;

/// One tap-hold behavior profile (timeout, interrupt flavor, idle gate).
///
/// Profile 0 is [`Config::default_profile`]; extra profiles live in
/// [`Config::profiles`] and are selected per key via [`Key::profile`].
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Profile {
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
    #[serde(default)]
    pub required_idle_time: Option<u16>,
}

impl Profile {
    /// Constructs a new default [Profile].
    pub const fn new() -> Self {
        DEFAULT_PROFILE
    }
}

impl Default for Profile {
    fn default() -> Self {
        DEFAULT_PROFILE
    }
}

/// A key with tap-hold functionality.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key<R> {
    /// The 'tap' key.
    pub tap: R,
    /// The 'hold' key.
    pub hold: R,
    /// Behavior profile index: `0` = [`Config::default_profile`]; `1..` = [`Config::profiles`].
    #[serde(default)]
    pub profile: u8,
}

impl<R> Key<R> {
    /// Constructs a new tap-hold key using the default profile (0).
    pub const fn new(tap: R, hold: R) -> Key<R> {
        Key {
            tap,
            hold,
            profile: 0,
        }
    }

    /// Constructs a tap-hold key that uses the given behavior profile index.
    pub const fn with_profile(tap: R, hold: R, profile: u8) -> Key<R> {
        Key { tap, hold, profile }
    }
}

#[cfg(feature = "std")]
impl<R: Default> Default for Key<R> {
    fn default() -> Self {
        Key {
            tap: R::default(),
            hold: R::default(),
            profile: 0,
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
///
/// [`Config::default_profile`] is **profile 0**. Optional [`Config::profiles`]
/// are extra behaviors at indices `1..`. Keys select a profile via [`Key::profile`].
///
/// Nickel authoring keeps profile-0 knobs flat on `config.tap_hold`
/// (`timeout`, `interrupt_response`, …); lowering nests them as
/// `default_profile` in JSON (no `serde(flatten)` — that needs alloc).
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Config {
    /// Default behavior (profile index 0).
    #[serde(default = "default_profile_value")]
    pub default_profile: Profile,

    /// Extra behavior profiles (indices `1..=len`).
    #[serde(default)]
    pub profiles: Slice<Profile, MAX_EXTRA_PROFILES>,
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

fn default_profile_value() -> Profile {
    DEFAULT_PROFILE
}

/// Default profile (also the contents of [`DEFAULT_CONFIG`]'s default profile).
pub const DEFAULT_PROFILE: Profile = Profile {
    timeout: Some(DEFAULT_TIMEOUT),
    interrupt_response: DEFAULT_INTERRUPT_RESPONSE,
    required_idle_time: None,
};

/// Default tap hold config.
pub const DEFAULT_CONFIG: Config = Config {
    default_profile: DEFAULT_PROFILE,
    profiles: Slice::from_slice(&[]),
};

impl Config {
    /// Constructs a new default [Config].
    pub const fn new() -> Self {
        DEFAULT_CONFIG
    }

    /// Resolves the behavior profile for `profile_id`.
    ///
    /// - `0` → [`Self::default_profile`]
    /// - `1..` → [`Self::profiles`] entry `id - 1`
    ///
    /// Out-of-range ids fall back to the default profile.
    pub const fn profile(&self, profile_id: u8) -> Profile {
        if profile_id == 0 {
            return self.default_profile;
        }
        let idx = (profile_id - 1) as usize;
        let extras = self.profiles.as_slice();
        if idx < extras.len() {
            extras[idx]
        } else {
            self.default_profile
        }
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

    /// Returns the resolved [Profile] for `profile_id`.
    pub const fn profile(&self, profile_id: u8) -> Profile {
        self.config.profile(profile_id)
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
        profile: &Profile,
        keymap_index: u16,
        event: key::Event<Event>,
    ) -> Option<TapHoldState> {
        // Check for interrupting taps
        // (track other key press)
        if let key::Event::Input(input::Event::Press { keymap_index: ki }) = event {
            self.other_pressed_keymap_index = Some(ki);
        }

        // Resolve tap-hold state per the event.
        self.hold_resolution(profile.interrupt_response, keymap_index, event)
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
        profile: &Profile,
        keymap_index: u16,
    ) -> (PendingKeyState, Option<key::ScheduledEvent<Event>>) {
        let pending = PendingKeyState::new();
        let scheduled = profile.timeout.map(|timeout| {
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
        let key_def = &self.keys[key_index as usize];
        let profile = context.profile(key_def.profile);

        match profile.required_idle_time {
            Some(required_idle_time) => {
                if context.idle_time_ms >= required_idle_time as u32 {
                    // Keymap has been idle long enough; use pending tap-hold key state.
                    let (th_pks, maybe_sch_ev) = self.new_pending_key(&profile, keymap_index);
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
                    } = *key_def;
                    (
                        key::PressedKeyResult::NewPressedKey(key::NewPressedKey::key(tap_key_ref)),
                        key::KeyEvents::no_events(),
                    )
                }
            }
            None => {
                // Idle time not considered. Use pending tap-hold key state.
                let (th_pks, maybe_sch_ev) = self.new_pending_key(&profile, keymap_index);
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
        let profile = context.profile(key_def.profile);
        let th_state = pending_state.handle_event(&profile, keymap_index, event);
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

    use crate::key::System as _;
    use crate::keymap::KeymapContext;

    const TAP: u8 = 1;
    const HOLD: u8 = 2;
    const KEYMAP_INDEX: u16 = 0;
    const OTHER_INDEX: u16 = 1;

    fn context_with(config: Config) -> Context {
        Context::from_config(config)
    }

    fn default_context() -> Context {
        context_with(Config::new())
    }

    fn config(
        timeout: Option<u16>,
        interrupt_response: InterruptResponse,
        required_idle_time: Option<u16>,
    ) -> Config {
        Config {
            default_profile: Profile {
                timeout,
                interrupt_response,
                required_idle_time,
            },
            profiles: Slice::from_slice(&[]),
        }
    }

    fn system() -> System<u8, [Key<u8>; 1]> {
        System::new([Key::new(TAP, HOLD)])
    }

    fn timeout_event(keymap_index: u16) -> key::Event<Event> {
        key::Event::key_event(keymap_index, Event::TapHoldTimeout)
    }

    fn press(keymap_index: u16) -> key::Event<Event> {
        key::Event::Input(input::Event::Press { keymap_index })
    }

    fn release(keymap_index: u16) -> key::Event<Event> {
        key::Event::Input(input::Event::Release { keymap_index })
    }

    // --- sizeof ---

    #[test]
    fn test_sizeof_ref() {
        assert_eq!(1, core::mem::size_of::<Ref>());
    }

    #[test]
    fn test_sizeof_event() {
        assert_eq!(0, core::mem::size_of::<Event>());
    }

    // --- PendingKeyState: Ignore ---

    #[test]
    fn ignore_own_release_resolves_as_tap() {
        // Assemble
        let ctx = context_with(config(
            Some(DEFAULT_TIMEOUT),
            InterruptResponse::Ignore,
            None,
        ));
        let mut pks = PendingKeyState::new();

        // Act
        let resolution = pks.handle_event(&ctx.profile(0), KEYMAP_INDEX, release(KEYMAP_INDEX));

        // Assert
        assert_eq!(Some(TapHoldState::Tap), resolution);
    }

    #[test]
    fn ignore_timeout_resolves_as_hold() {
        // Assemble
        let ctx = context_with(config(
            Some(DEFAULT_TIMEOUT),
            InterruptResponse::Ignore,
            None,
        ));
        let mut pks = PendingKeyState::new();

        // Act
        let resolution =
            pks.handle_event(&ctx.profile(0), KEYMAP_INDEX, timeout_event(KEYMAP_INDEX));

        // Assert
        assert_eq!(Some(TapHoldState::Hold), resolution);
    }

    #[test]
    fn ignore_other_press_does_not_resolve() {
        // Assemble
        let ctx = context_with(config(
            Some(DEFAULT_TIMEOUT),
            InterruptResponse::Ignore,
            None,
        ));
        let mut pks = PendingKeyState::new();

        // Act
        let resolution = pks.handle_event(&ctx.profile(0), KEYMAP_INDEX, press(OTHER_INDEX));

        // Assert
        assert_eq!(None, resolution);
    }

    #[test]
    fn ignore_other_release_does_not_resolve() {
        // Assemble
        let ctx = context_with(config(
            Some(DEFAULT_TIMEOUT),
            InterruptResponse::Ignore,
            None,
        ));
        let mut pks = PendingKeyState::new();

        // Act
        let resolution = pks.handle_event(&ctx.profile(0), KEYMAP_INDEX, release(OTHER_INDEX));

        // Assert
        assert_eq!(None, resolution);
    }

    // --- PendingKeyState: HoldOnKeyPress ---

    #[test]
    fn hold_on_key_press_other_press_resolves_as_hold() {
        // Assemble
        let ctx = context_with(config(
            Some(DEFAULT_TIMEOUT),
            InterruptResponse::HoldOnKeyPress,
            None,
        ));
        let mut pks = PendingKeyState::new();

        // Act
        let resolution = pks.handle_event(&ctx.profile(0), KEYMAP_INDEX, press(OTHER_INDEX));

        // Assert
        assert_eq!(Some(TapHoldState::Hold), resolution);
    }

    #[test]
    fn hold_on_key_press_own_release_resolves_as_tap() {
        // Assemble
        let ctx = context_with(config(
            Some(DEFAULT_TIMEOUT),
            InterruptResponse::HoldOnKeyPress,
            None,
        ));
        let mut pks = PendingKeyState::new();

        // Act
        let resolution = pks.handle_event(&ctx.profile(0), KEYMAP_INDEX, release(KEYMAP_INDEX));

        // Assert
        assert_eq!(Some(TapHoldState::Tap), resolution);
    }

    #[test]
    fn hold_on_key_press_timeout_resolves_as_hold() {
        // Assemble
        let ctx = context_with(config(
            Some(DEFAULT_TIMEOUT),
            InterruptResponse::HoldOnKeyPress,
            None,
        ));
        let mut pks = PendingKeyState::new();

        // Act
        let resolution =
            pks.handle_event(&ctx.profile(0), KEYMAP_INDEX, timeout_event(KEYMAP_INDEX));

        // Assert
        assert_eq!(Some(TapHoldState::Hold), resolution);
    }

    #[test]
    fn hold_on_key_press_other_release_does_not_resolve() {
        // Assemble
        let ctx = context_with(config(
            Some(DEFAULT_TIMEOUT),
            InterruptResponse::HoldOnKeyPress,
            None,
        ));
        let mut pks = PendingKeyState::new();

        // Act
        let resolution = pks.handle_event(&ctx.profile(0), KEYMAP_INDEX, release(OTHER_INDEX));

        // Assert
        assert_eq!(None, resolution);
    }

    // --- PendingKeyState: HoldOnKeyTap ---

    #[test]
    fn hold_on_key_tap_own_release_resolves_as_tap() {
        // Assemble
        let ctx = context_with(config(
            Some(DEFAULT_TIMEOUT),
            InterruptResponse::HoldOnKeyTap,
            None,
        ));
        let mut pks = PendingKeyState::new();

        // Act
        let resolution = pks.handle_event(&ctx.profile(0), KEYMAP_INDEX, release(KEYMAP_INDEX));

        // Assert
        assert_eq!(Some(TapHoldState::Tap), resolution);
    }

    #[test]
    fn hold_on_key_tap_timeout_resolves_as_hold() {
        // Assemble
        let ctx = context_with(config(
            Some(DEFAULT_TIMEOUT),
            InterruptResponse::HoldOnKeyTap,
            None,
        ));
        let mut pks = PendingKeyState::new();

        // Act
        let resolution =
            pks.handle_event(&ctx.profile(0), KEYMAP_INDEX, timeout_event(KEYMAP_INDEX));

        // Assert
        assert_eq!(Some(TapHoldState::Hold), resolution);
    }

    #[test]
    fn hold_on_key_tap_other_press_does_not_resolve() {
        // Assemble
        let ctx = context_with(config(
            Some(DEFAULT_TIMEOUT),
            InterruptResponse::HoldOnKeyTap,
            None,
        ));
        let mut pks = PendingKeyState::new();

        // Act
        let resolution = pks.handle_event(&ctx.profile(0), KEYMAP_INDEX, press(OTHER_INDEX));

        // Assert
        assert_eq!(None, resolution);
    }

    #[test]
    fn hold_on_key_tap_other_tap_resolves_as_hold() {
        // Assemble
        let ctx = context_with(config(
            Some(DEFAULT_TIMEOUT),
            InterruptResponse::HoldOnKeyTap,
            None,
        ));
        let mut pks = PendingKeyState::new();
        let _ = pks.handle_event(&ctx.profile(0), KEYMAP_INDEX, press(OTHER_INDEX));

        // Act
        let resolution = pks.handle_event(&ctx.profile(0), KEYMAP_INDEX, release(OTHER_INDEX));

        // Assert
        assert_eq!(Some(TapHoldState::Hold), resolution);
    }

    #[test]
    fn hold_on_key_tap_unmatched_other_release_does_not_resolve() {
        // Assemble: no prior press of OTHER_INDEX tracked
        let ctx = context_with(config(
            Some(DEFAULT_TIMEOUT),
            InterruptResponse::HoldOnKeyTap,
            None,
        ));
        let mut pks = PendingKeyState::new();

        // Act
        let resolution = pks.handle_event(&ctx.profile(0), KEYMAP_INDEX, release(OTHER_INDEX));

        // Assert
        assert_eq!(None, resolution);
    }

    #[test]
    fn hold_on_key_tap_release_of_different_key_does_not_resolve() {
        // Assemble: track press of index 1, then release index 2
        let ctx = context_with(config(
            Some(DEFAULT_TIMEOUT),
            InterruptResponse::HoldOnKeyTap,
            None,
        ));
        let mut pks = PendingKeyState::new();
        let _ = pks.handle_event(&ctx.profile(0), KEYMAP_INDEX, press(OTHER_INDEX));

        // Act
        let resolution = pks.handle_event(&ctx.profile(0), KEYMAP_INDEX, release(2));

        // Assert
        assert_eq!(None, resolution);
    }

    // --- System::new_pressed_key ---

    #[test]
    fn new_pressed_key_is_pending_by_default() {
        // Assemble
        let system = system();
        let ctx = default_context();

        // Act
        let (pkr, _) = system.new_pressed_key(KEYMAP_INDEX, &ctx, Ref(0));

        // Assert
        assert!(matches!(pkr, key::PressedKeyResult::Pending(_)));
    }

    #[test]
    fn new_pressed_key_schedules_default_timeout() {
        // Assemble
        let system = system();
        let ctx = default_context();

        // Act
        let (_, events) = system.new_pressed_key(KEYMAP_INDEX, &ctx, Ref(0));

        // Assert
        let expected = key::KeyEvents::scheduled_event(key::ScheduledEvent::after(
            DEFAULT_TIMEOUT,
            timeout_event(KEYMAP_INDEX),
        ));
        assert_eq!(expected, events);
    }

    #[test]
    fn new_pressed_key_with_no_timeout_schedules_nothing() {
        // Assemble
        let system = system();
        let ctx = context_with(config(None, InterruptResponse::Ignore, None));

        // Act
        let (_, events) = system.new_pressed_key(KEYMAP_INDEX, &ctx, Ref(0));

        // Assert
        assert_eq!(key::KeyEvents::no_events(), events);
    }

    #[test]
    fn new_pressed_key_with_no_timeout_is_still_pending() {
        // Assemble
        let system = system();
        let ctx = context_with(config(None, InterruptResponse::Ignore, None));

        // Act
        let (pkr, _) = system.new_pressed_key(KEYMAP_INDEX, &ctx, Ref(0));

        // Assert
        assert!(matches!(pkr, key::PressedKeyResult::Pending(_)));
    }

    #[test]
    fn new_pressed_key_resolves_as_tap_when_idle_time_insufficient() {
        // Assemble
        let system = system();
        let mut ctx = context_with(config(
            Some(DEFAULT_TIMEOUT),
            InterruptResponse::Ignore,
            Some(100),
        ));
        ctx.idle_time_ms = 50;

        // Act
        let (pkr, _) = system.new_pressed_key(KEYMAP_INDEX, &ctx, Ref(0));

        // Assert
        assert_eq!(
            key::PressedKeyResult::NewPressedKey(key::NewPressedKey::key(TAP)),
            pkr
        );
    }

    #[test]
    fn new_pressed_key_insufficient_idle_emits_no_events() {
        // Assemble
        let system = system();
        let mut ctx = context_with(config(
            Some(DEFAULT_TIMEOUT),
            InterruptResponse::Ignore,
            Some(100),
        ));
        ctx.idle_time_ms = 50;

        // Act
        let (_, events) = system.new_pressed_key(KEYMAP_INDEX, &ctx, Ref(0));

        // Assert
        assert_eq!(key::KeyEvents::no_events(), events);
    }

    #[test]
    fn new_pressed_key_is_pending_when_idle_time_sufficient() {
        // Assemble
        let system = system();
        let mut ctx = context_with(config(
            Some(DEFAULT_TIMEOUT),
            InterruptResponse::Ignore,
            Some(100),
        ));
        ctx.idle_time_ms = 100;

        // Act
        let (pkr, _) = system.new_pressed_key(KEYMAP_INDEX, &ctx, Ref(0));

        // Assert
        assert!(matches!(pkr, key::PressedKeyResult::Pending(_)));
    }

    #[test]
    fn new_pressed_key_schedules_timeout_when_idle_time_sufficient() {
        // Assemble
        let system = system();
        let mut ctx = context_with(config(
            Some(DEFAULT_TIMEOUT),
            InterruptResponse::Ignore,
            Some(100),
        ));
        ctx.idle_time_ms = 100;

        // Act
        let (_, events) = system.new_pressed_key(KEYMAP_INDEX, &ctx, Ref(0));

        // Assert
        let expected = key::KeyEvents::scheduled_event(key::ScheduledEvent::after(
            DEFAULT_TIMEOUT,
            timeout_event(KEYMAP_INDEX),
        ));
        assert_eq!(expected, events);
    }

    // --- System::update_pending_state ---

    #[test]
    fn update_pending_own_release_resolves_to_tap_ref() {
        // Assemble
        let system = system();
        let ctx = default_context();
        let mut pks = PendingKeyState::new();

        // Act
        let (resolved, _) = system.update_pending_state(
            &mut pks,
            KEYMAP_INDEX,
            &ctx,
            Ref(0),
            release(KEYMAP_INDEX),
        );

        // Assert
        assert_eq!(Some(key::NewPressedKey::key(TAP)), resolved);
    }

    #[test]
    fn update_pending_timeout_resolves_to_hold_ref() {
        // Assemble
        let system = system();
        let ctx = default_context();
        let mut pks = PendingKeyState::new();

        // Act
        let (resolved, _) = system.update_pending_state(
            &mut pks,
            KEYMAP_INDEX,
            &ctx,
            Ref(0),
            timeout_event(KEYMAP_INDEX),
        );

        // Assert
        assert_eq!(Some(key::NewPressedKey::key(HOLD)), resolved);
    }

    #[test]
    fn update_pending_hold_on_press_interrupt_resolves_to_hold_ref() {
        // Assemble
        let system = system();
        let ctx = context_with(config(
            Some(DEFAULT_TIMEOUT),
            InterruptResponse::HoldOnKeyPress,
            None,
        ));
        let mut pks = PendingKeyState::new();

        // Act
        let (resolved, _) =
            system.update_pending_state(&mut pks, KEYMAP_INDEX, &ctx, Ref(0), press(OTHER_INDEX));

        // Assert
        assert_eq!(Some(key::NewPressedKey::key(HOLD)), resolved);
    }

    #[test]
    fn update_pending_non_resolving_event_yields_none() {
        // Assemble
        let system = system();
        let ctx = default_context();
        let mut pks = PendingKeyState::new();

        // Act
        let (resolved, _) =
            system.update_pending_state(&mut pks, KEYMAP_INDEX, &ctx, Ref(0), press(OTHER_INDEX));

        // Assert
        assert_eq!(None, resolved);
    }

    #[test]
    fn update_pending_resolution_emits_no_events() {
        // Assemble
        let system = system();
        let ctx = default_context();
        let mut pks = PendingKeyState::new();

        // Act
        let (_, events) = system.update_pending_state(
            &mut pks,
            KEYMAP_INDEX,
            &ctx,
            Ref(0),
            release(KEYMAP_INDEX),
        );

        // Assert
        assert_eq!(key::KeyEvents::no_events(), events);
    }

    // --- Context ---

    #[test]
    fn context_update_keymap_context_sets_idle_time() {
        // Assemble
        let mut ctx = default_context();
        let km_ctx = KeymapContext {
            idle_time_ms: 42,
            ..KeymapContext::new()
        };

        // Act
        ctx.update_keymap_context(&km_ctx);

        // Assert
        assert_eq!(42, ctx.idle_time_ms);
    }

    #[test]
    fn context_reset_clears_idle_time() {
        // Assemble
        let mut ctx = default_context();
        ctx.idle_time_ms = 99;

        // Act
        ctx.reset();

        // Assert
        assert_eq!(0, ctx.idle_time_ms);
    }

    #[test]
    fn context_reset_preserves_config() {
        // Assemble
        let config = config(None, InterruptResponse::HoldOnKeyPress, Some(50));
        let mut ctx = context_with(config);
        ctx.idle_time_ms = 99;

        // Act
        ctx.reset();

        // Assert
        assert_eq!(config, ctx.config);
    }

    // --- profiles ---

    #[test]
    fn test_profile_zero_is_default_fields() {
        let config = Config {
            default_profile: Profile {
                timeout: Some(50),
                interrupt_response: InterruptResponse::HoldOnKeyPress,
                required_idle_time: Some(10),
            },
            profiles: Slice::from_slice(&[]),
        };
        let p = config.profile(0);
        assert_eq!(p, config.default_profile);
        assert_eq!(p.timeout, Some(50));
        assert_eq!(p.interrupt_response, InterruptResponse::HoldOnKeyPress);
        assert_eq!(p.required_idle_time, Some(10));
    }

    #[test]
    fn test_profile_extra_lookup() {
        let extra = Profile {
            timeout: Some(300),
            interrupt_response: InterruptResponse::HoldOnKeyTap,
            required_idle_time: None,
        };
        let config = Config {
            default_profile: Profile {
                timeout: Some(200),
                interrupt_response: InterruptResponse::Ignore,
                required_idle_time: None,
            },
            profiles: Slice::from_slice(&[extra]),
        };
        assert_eq!(config.profile(1).timeout, Some(300));
        assert_eq!(
            config.profile(1).interrupt_response,
            InterruptResponse::HoldOnKeyTap
        );
        // OOR falls back to default
        assert_eq!(config.profile(9).timeout, Some(200));
        assert_eq!(config.profile(9), config.default_profile);
    }

    #[test]
    fn test_key_default_profile_is_zero() {
        let key = Key::new(0u8, 1u8);
        assert_eq!(key.profile, 0);
        let key = Key::with_profile(0u8, 1u8, 2);
        assert_eq!(key.profile, 2);
    }
}
