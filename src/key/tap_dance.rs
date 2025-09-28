use core::fmt::Debug;
use core::ops::Index;

use serde::Deserialize;

use crate::input;
use crate::key;

pub use crate::init::MAX_TAP_DANCE_DEFINITIONS;

/// Reference for a tap dance key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Ref(pub u8);

/// Configuration settings for tap dance keys.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Config {
    /// The timeout (in number of milliseconds) for the next press of the tap-dance.
    #[serde(default = "default_timeout")]
    pub timeout: u16,
}

fn default_timeout() -> u16 {
    DEFAULT_CONFIG.timeout
}

/// Default tap dance config.
pub const DEFAULT_CONFIG: Config = Config { timeout: 200 };

impl Default for Config {
    /// Returns the default context.
    fn default() -> Self {
        DEFAULT_CONFIG
    }
}

/// A key with tap-dance functionality.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key<R> {
    /// Tap-Dance definitions.
    definitions: [Option<R>; MAX_TAP_DANCE_DEFINITIONS],
}

impl<R: Copy> Key<R> {
    /// Constructs a new tap-dance key.
    pub const fn new(definitions: [Option<R>; MAX_TAP_DANCE_DEFINITIONS]) -> Key<R> {
        Key { definitions }
    }

    /// Construct the tap-dance key from the given slice of keys.
    pub const fn from_definitions(defs: &[R]) -> Self {
        let mut definitions: [Option<R>; MAX_TAP_DANCE_DEFINITIONS] =
            [None; MAX_TAP_DANCE_DEFINITIONS];
        let mut idx = 0;
        while idx < definitions.len() && idx < defs.len() {
            definitions[idx] = Some(defs[idx]);
            idx += 1;
        }
        Self::new(definitions)
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

/// Resolution of a tap-dance key. (Index of the tap-dance definition).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TapDanceResolution(u8);

/// Events emitted by a tap-dance key.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// Timed out waiting for the next press of the tap-dance key.
    NextPressTimeout(u8),
}

/// The state of a pressed tap-dance key.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PendingKeyState {
    press_count: u8,
}

impl PendingKeyState {
    /// Constructs the initial pressed key state
    fn new() -> PendingKeyState {
        PendingKeyState { press_count: 0 }
    }

    fn handle_event(
        &mut self,
        context: &Context,
        keymap_index: u16,
        event: key::Event<Event>,
    ) -> (Option<TapDanceResolution>, key::KeyEvents<Event>) {
        match event {
            key::Event::Key {
                key_event: Event::NextPressTimeout(press_timed_out),
                keymap_index: ev_kmi,
            } if ev_kmi == keymap_index && press_timed_out == self.press_count => (
                Some(TapDanceResolution(self.press_count)),
                key::KeyEvents::no_events(),
            ),

            key::Event::Input(input::Event::Press {
                keymap_index: ev_kmi,
            }) if ev_kmi == keymap_index => {
                self.press_count += 1;

                let Context { config } = context;
                let timeout_ev = Event::NextPressTimeout(self.press_count);

                let key_ev = key::Event::Key {
                    keymap_index,
                    key_event: timeout_ev,
                };
                let pke = key::KeyEvents::scheduled_event(key::ScheduledEvent::after(
                    config.timeout,
                    key_ev,
                ));

                (None, pke)
            }

            _ => (None, key::KeyEvents::no_events()),
        }
    }
}

/// The key state for System. (No state).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState;

/// The [key::System] implementation for tap dance keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct System<R, Keys: Index<usize, Output = Key<R>>> {
    keys: Keys,
}

impl<R, Keys: Index<usize, Output = Key<R>>> System<R, Keys> {
    /// Constructs a new [System] with the given key data.
    pub const fn new(key_data: Keys) -> Self {
        Self { keys: key_data }
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
        _key_ref: Ref,
    ) -> (
        key::PressedKeyResult<R, Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        let td_pks = PendingKeyState::new();
        let pk = key::PressedKeyResult::Pending(td_pks);

        let timeout_ev = Event::NextPressTimeout(0);
        let key_ev = key::Event::Key {
            keymap_index,
            key_event: timeout_ev,
        };
        let pke = key::KeyEvents::scheduled_event(key::ScheduledEvent::after(
            context.config.timeout,
            key_ev,
        ));

        (pk, pke)
    }

    fn update_pending_state(
        &self,
        pending_state: &mut Self::PendingKeyState,
        keymap_index: u16,
        context: &Self::Context,
        Ref(key_index): Ref,
        event: key::Event<Self::Event>,
    ) -> (Option<key::NewPressedKey<R>>, key::KeyEvents<Self::Event>) {
        let key = &self.keys[key_index as usize];
        let (maybe_resolution, pke) = pending_state.handle_event(context, keymap_index, event);

        if let Some(TapDanceResolution(idx)) = maybe_resolution {
            let new_key_ref = key.definitions[idx as usize].unwrap();

            (
                Some(key::NewPressedKey::key(new_key_ref)),
                pke.into_events(),
            )
        } else {
            // check pending_state press_count against key definitions
            let definition_count = key.definitions.iter().filter(|o| o.is_some()).count();
            if pending_state.press_count as usize >= definition_count - 1 {
                let idx = definition_count - 1;
                let new_key_ref = key.definitions[idx].unwrap();

                (
                    Some(key::NewPressedKey::key(new_key_ref)),
                    pke.into_events(),
                )
            } else {
                (None, pke.into_events())
            }
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
        panic!() // tap dance has no key state
    }

    fn key_output(
        &self,
        _key_ref: &Self::Ref,
        _key_state: &Self::KeyState,
    ) -> Option<key::KeyOutput> {
        panic!() // tap dance has no key state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sizeof_ref() {
        assert_eq!(1, core::mem::size_of::<Ref>());
    }
}
