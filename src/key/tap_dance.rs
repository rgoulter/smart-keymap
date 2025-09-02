use core::fmt::Debug;

use serde::Deserialize;

use crate::input;
use crate::key;
use crate::keymap;

pub use crate::init::MAX_TAP_DANCE_DEFINITIONS;

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
pub struct Key<K: key::Key> {
    /// Tap-Dance definitions.
    definitions: [Option<K>; MAX_TAP_DANCE_DEFINITIONS],
}

impl<K: key::Key + Copy> Key<K> {
    /// Constructs a new tap-dance key.
    pub const fn new(definitions: [Option<K>; MAX_TAP_DANCE_DEFINITIONS]) -> Key<K> {
        Key { definitions }
    }

    /// Construct the tap-dance key from the given slice of keys.
    pub const fn from_definitions(defs: &[K]) -> Self {
        let mut definitions: [Option<K>; MAX_TAP_DANCE_DEFINITIONS] =
            [None; MAX_TAP_DANCE_DEFINITIONS];
        let mut idx = 0;
        while idx < definitions.len() && idx < defs.len() {
            definitions[idx] = Some(defs[idx]);
            idx += 1;
        }
        Self::new(definitions)
    }
}

impl<K: key::Key> Key<K> {
    fn new_pressed_key(
        &self,
        context: &K::Context,
        key_path: key::KeyPath,
    ) -> (
        key::PressedKeyResult<K::PendingKeyState, K::KeyState>,
        key::KeyEvents<K::Event>,
    )
    where
        for<'ctx> &'ctx K::Context: Into<&'ctx Context>,
        for<'ctx> &'ctx K::Context: Into<&'ctx keymap::KeymapContext>,
        Event: Into<K::Event>,
        PendingKeyState: Into<K::PendingKeyState>,
    {
        let keymap_index: u16 = key_path.keymap_index();

        let td_pks = PendingKeyState::new();
        let pk = key::PressedKeyResult::Pending(key_path, td_pks.into());

        let &Context { config, .. } = context.into();
        let timeout_ev = Event::NextPressTimeout(0);
        let key_ev = key::Event::Key {
            keymap_index,
            key_event: timeout_ev,
        };
        let pke =
            key::KeyEvents::scheduled_event(key::ScheduledEvent::after(config.timeout, key_ev));

        (pk, pke.into_events())
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
        context: &Self::Context,
        key_path: key::KeyPath,
    ) -> (
        key::PressedKeyResult<Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        self.new_pressed_key(context, key_path.clone())
    }

    fn handle_event(
        &self,
        pending_state: &mut Self::PendingKeyState,
        context: &Self::Context,
        key_path: key::KeyPath,
        event: key::Event<Self::Event>,
    ) -> (Option<key::NewPressedKey>, key::KeyEvents<Self::Event>) {
        let keymap_index = key_path.keymap_index();
        let td_pks_res: Result<&mut PendingKeyState, _> = pending_state.try_into();
        if let Ok(td_pks) = td_pks_res {
            if let Ok(td_ev) = event.try_into_key_event(|e| e.try_into()) {
                let (maybe_resolution, pke) =
                    td_pks.handle_event(context.into(), keymap_index, td_ev);

                if let Some(TapDanceResolution(idx)) = maybe_resolution {
                    // PRESSED KEY PATH: add Tap Dance item (index for the tap-dance definition)
                    let new_key_path = key_path.add_path_item(idx as u16);

                    (
                        Some(key::NewPressedKey::key_path(new_key_path)),
                        pke.into_events(),
                    )
                } else {
                    // check td_pks press_count against key definitions
                    let definition_count = self.definitions.iter().filter(|o| o.is_some()).count();
                    if td_pks.press_count as usize >= definition_count - 1 {
                        let idx = definition_count - 1;
                        // PRESSED KEY PATH: add Tap Dance item (index for the tap-dance definition)
                        let new_key_path = key_path.add_path_item(idx as u16);

                        (
                            Some(key::NewPressedKey::key_path(new_key_path)),
                            pke.into_events(),
                        )
                    } else {
                        (None, pke.into_events())
                    }
                }
            } else {
                (None, key::KeyEvents::no_events())
            }
        } else {
            (None, key::KeyEvents::no_events())
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
            // idx = definition index
            [idx, path @ ..] => match &self.definitions[*idx as usize] {
                Some(key) => key.lookup(path),
                None => panic!(),
            },
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
#[derive(Debug, Clone, PartialEq)]
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
