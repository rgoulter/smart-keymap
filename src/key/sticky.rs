use core::fmt::Debug;
use core::marker::Copy;
use core::marker::PhantomData;
use core::ops::Index;

use serde::Deserialize;

use crate::input;
use crate::key;
use crate::keymap;

/// Reference for a sticky modifier key.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Ref(pub u8);

/// When the sticky modifiers activate.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum StickyKeyActivation {
    /// Sticky modifiers activate when the sticky key is released.
    OnStickyKeyRelease,
    // TODO: add another config option for "on next key press"
    // /// Sticky modifiers activate when the next key is pressed.
    // OnNextKeyPress,
}

/// When the sticky modifiers release.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum StickyKeyRelease {
    /// Sticky modifiers release when the modified key is released.
    OnModifiedKeyRelease,
    /// Sticky modifiers release when a key is pressed after the modified key.
    OnNextKeyPress,
}

/// Sticky Key configuration.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Config {
    /// The sticky key activation mode.
    #[serde(default = "default_activation")]
    pub activation: StickyKeyActivation,
    /// When the sticky modifiers release.
    #[serde(default = "default_release")]
    pub release: StickyKeyRelease,
}

fn default_activation() -> StickyKeyActivation {
    DEFAULT_CONFIG.activation
}

fn default_release() -> StickyKeyRelease {
    DEFAULT_CONFIG.release
}

/// The default [Config].
pub const DEFAULT_CONFIG: Config = Config {
    activation: StickyKeyActivation::OnStickyKeyRelease,
    release: StickyKeyRelease::OnModifiedKeyRelease,
};

impl Default for Config {
    /// Returns the default context.
    fn default() -> Self {
        DEFAULT_CONFIG
    }
}

const MAX_STICKY_MODIFIERS: u8 = 4;

/// Sticky Modifiers context.
#[derive(Debug, Clone, Copy)]
pub struct Context {
    /// The sticky modifier key configuration.
    pub config: Config,
    /// Sticky modifiers.
    pub active_modifiers: [key::KeyboardModifiers; MAX_STICKY_MODIFIERS as usize],
    /// Number af active sticky modifiers.
    pub active_modifier_count: u8,
    /// Index of the next output resolved once a sticky key has been released.
    pub pressed_keymap_index: Option<u16>,
}

/// The default [Context].
pub const DEFAULT_CONTEXT: Context = Context {
    config: DEFAULT_CONFIG,
    active_modifiers: [key::KeyboardModifiers::NONE; MAX_STICKY_MODIFIERS as usize],
    active_modifier_count: 0,
    pressed_keymap_index: None,
};

impl Context {
    /// Constructs a context from the given config
    pub const fn from_config(config: Config) -> Context {
        Context {
            config,
            ..DEFAULT_CONTEXT
        }
    }

    /// Updates the context with the given event.
    fn handle_event(&mut self, event: key::Event<Event>) -> key::KeyEvents<Event> {
        // Cases:
        //
        // - No sticky key has been pressed.
        //   - Event: any, when active_modifiers is None.
        //   - Ctx same as default.
        // - Sticky key has been tapped,
        //    (pressed, released, without interruption),
        //   - Event: Event::ActivateModifiers
        //   - Ctx has sticky key active.
        //     - Virtual Key modifier is pressed (if config StickyKeyActivation::OnStickyKeyRelease)
        //   - add the activated modifiers to self.activated_modifiers
        // - Next key has been pressed
        //   ("modified key")
        //   - Event: Keymap::ResolvedKeyOutput (active modifiers is Some(), pressed_keymap_index is None)
        //   - Virtual Key modifier is pressed  (if config StickyKeyActivation::OnNextKeyPress)
        //   - Ctx still has the sticky key active.
        // - Same next key has been released
        //   ("modified key")
        //   - Event: input::Event::KeyRelease, with the same keymap index.
        //   - Ctx deactivates the sticky key.
        //     - Virtual Key modifier is released.
        // - Another key has been pressed,
        //    after sticky modifiers are pressed.
        //   - Event: Keymap::ResolvedKeyOutput (active modifiers is Some(), pressed_keymap_index is Some())
        //   - c.f. ZMK's quick-release.

        match (self.active_modifier_count, event) {
            // Case:
            //  - a sticky key has been released.
            (
                0,
                key::Event::Key {
                    key_event: Event::ActivateModifiers(mods),
                    ..
                },
            ) => {
                self.active_modifiers[0] = mods;
                self.active_modifier_count = 1;

                key::KeyEvents::no_events()
            }
            // Case:
            //  - another sticky key has been released.
            (
                active_modifier_count,
                key::Event::Key {
                    key_event: Event::ActivateModifiers(mods),
                    ..
                },
            ) => {
                if active_modifier_count < MAX_STICKY_MODIFIERS {
                    self.active_modifiers[active_modifier_count as usize] = mods;
                    self.active_modifier_count += 1;
                }

                key::KeyEvents::no_events()
            }
            // Case:
            //  - Next key has been pressed, this is the "modified key";
            //     this key gets modified until it is released.
            (
                active_modifier_count,
                key::Event::Keymap(keymap::KeymapEvent::ResolvedKeyOutput { keymap_index, .. }),
            ) if active_modifier_count > 0 => {
                let pke = key::KeyEvents::no_events();

                // The sticky key deactivates (releases)
                //  once the modified key releases.
                //
                // Track the keymap index that resolved the key state.
                self.pressed_keymap_index = Some(keymap_index);

                // if matches!(self.config.activation, StickyKeyActivation::OnNextKeyPress) {
                //     // TODO: if the config is to activate on key press, send the VK here
                // }

                pke
            }
            // Case:
            //  - Modified key is released.
            (
                active_modifier_count,
                key::Event::Input(input::Event::Release {
                    keymap_index: ev_kmi,
                }),
            ) if Some(ev_kmi) == self.pressed_keymap_index && active_modifier_count > 0 => {
                // Modified key has been released; release the VK.
                let mut pke = key::KeyEvents::no_events();

                self.active_modifiers[..active_modifier_count as usize]
                    .iter()
                    .for_each(|&m| {
                        let sticky_key_output = key::KeyOutput::from_key_modifiers(m);
                        let vk_ev = key::Event::Input(input::Event::VirtualKeyRelease {
                            key_output: sticky_key_output,
                        });
                        pke.add_event(key::ScheduledEvent::immediate(vk_ev));
                    });

                self.active_modifier_count = 0;
                self.pressed_keymap_index = None;

                pke
            }
            // Case: after the sticky key modifiers are modifying a key,
            //        another key is pressed,
            //        & the config.release is OnNextKeyPress.
            //  - Modified key is released.
            (active_modifier_count, key::Event::Input(input::Event::Press { .. }))
                if self.pressed_keymap_index.is_some()
                    && self.config.release == StickyKeyRelease::OnNextKeyPress =>
            {
                // Another key has been pressed (& config is to release sticky modifiers);
                //  release the VK.
                let mut pke = key::KeyEvents::no_events();

                self.active_modifiers[..active_modifier_count as usize]
                    .iter()
                    .for_each(|&m| {
                        let sticky_key_output = key::KeyOutput::from_key_modifiers(m);
                        let vk_ev = key::Event::Input(input::Event::VirtualKeyRelease {
                            key_output: sticky_key_output,
                        });
                        pke.add_event(key::ScheduledEvent::immediate(vk_ev));
                    });

                self.active_modifier_count = 0;
                self.pressed_keymap_index = None;

                pke
            }
            _ => key::KeyEvents::no_events(),
        }
    }
}

impl key::Context for Context {
    type Event = Event;

    fn handle_event(&mut self, event: key::Event<Self::Event>) -> key::KeyEvents<Self::Event> {
        self.handle_event(event)
    }
}

/// Sticky Modifier key events.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    /// Activates the given modifier(s) as "sticky"
    ActivateModifiers(key::KeyboardModifiers),
}

/// A key for HID Keyboard usage codes.
#[derive(Deserialize, Debug, Clone, Copy, PartialEq)]
pub struct Key {
    /// The sticky key modifiers.
    pub sticky_modifiers: key::KeyboardModifiers,
}

impl Key {
    /// Constructs a key with the given key_code.
    pub const fn new(sticky_modifiers: key::KeyboardModifiers) -> Self {
        Key { sticky_modifiers }
    }

    /// Constructs a pressed key state
    pub fn new_pressed_key(&self) -> KeyState {
        KeyState::new()
    }
}

/// The pending key state type for sticky modifier keys. (No pending state).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PendingKeyState;

/// Whether the pressed Sticky modifier key is "sticky" or "regular".
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Behavior {
    /// Key state is "sticky". (Will activate sticky modifier when released).
    Sticky,
    /// Key state is "regular". (No sticky modifiers activated when released).
    Regular,
}

/// Key state for sticky modifier keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyState {
    behavior: Behavior,
}

impl KeyState {
    /// Constructs a new key state with the given sticky modifiers.
    pub fn new() -> Self {
        KeyState {
            behavior: Behavior::Sticky,
        }
    }
}

impl Default for KeyState {
    /// Returns the default key state.
    fn default() -> Self {
        Self::new()
    }
}

impl KeyState {
    /// Handle the given event.
    pub fn update_state(
        &mut self,
        key: &Key,
        context: &Context,
        keymap_index: u16,
        event: key::Event<Event>,
    ) -> key::KeyEvents<Event> {
        //  - If another key is *pressed*, then we're no longer a sticky key.
        //  - If this key is released & it's a sticky key
        //     (& the config is for "eager sticky mod"),
        //     then emit a VK with the mods; emit event "activate".
        match self.behavior {
            Behavior::Sticky => match event {
                key::Event::Keymap(keymap::KeymapEvent::ResolvedKeyOutput { .. }) => {
                    // Another key has been pressed.
                    // The sticky modifier key acts as a regular key.
                    self.behavior = Behavior::Regular;

                    key::KeyEvents::no_events()
                }
                key::Event::Input(input::Event::Release {
                    keymap_index: released_index,
                }) if released_index == keymap_index => {
                    // The sticky key has been released.
                    match context.config.activation {
                        StickyKeyActivation::OnStickyKeyRelease => {
                            let sticky_ev = Event::ActivateModifiers(key.sticky_modifiers);
                            let k_ev = key::Event::key_event(keymap_index, sticky_ev);

                            let sticky_key_output =
                                key::KeyOutput::from_key_modifiers(key.sticky_modifiers);
                            let vk_ev = key::Event::Input(input::Event::VirtualKeyPress {
                                key_output: sticky_key_output,
                            });

                            let mut pke = key::KeyEvents::event(k_ev);
                            pke.add_event(key::ScheduledEvent::immediate(vk_ev));
                            pke
                        }
                    }
                }
                _ => key::KeyEvents::no_events(),
            },
            Behavior::Regular => key::KeyEvents::no_events(),
        }
    }

    /// Key output for the pressed key state.
    pub fn key_output(&self, key: &Key) -> Option<key::KeyOutput> {
        match self.behavior {
            Behavior::Sticky => None,
            Behavior::Regular => Some(key::KeyOutput::from_key_modifiers(key.sticky_modifiers)),
        }
    }
}

/// The [key::System] implementation for keyboard keys.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct System<R, Keys: Index<usize, Output = Key>> {
    keys: Keys,
    marker: PhantomData<R>,
}

impl<R, Keys: Index<usize, Output = Key>> System<R, Keys> {
    /// Constructs a new [System] with the given key data.
    pub const fn new(keys: Keys) -> Self {
        Self {
            keys,
            marker: PhantomData,
        }
    }
}

impl<R: Debug, Keys: Debug + Index<usize, Output = Key>> key::System<R> for System<R, Keys> {
    type Ref = Ref;
    type Context = Context;
    type Event = Event;
    type PendingKeyState = PendingKeyState;
    type KeyState = KeyState;

    fn new_pressed_key(
        &self,
        _keymap_index: u16,
        _context: &Self::Context,
        Ref(key_index): Ref,
    ) -> (
        key::PressedKeyResult<R, Self::PendingKeyState, Self::KeyState>,
        key::KeyEvents<Self::Event>,
    ) {
        let key = &self.keys[key_index as usize];
        let ks = key.new_pressed_key();
        let pks = key::PressedKeyResult::Resolved(ks);
        let pke = key::KeyEvents::no_events();
        (pks, pke)
    }

    fn update_pending_state(
        &self,
        _pending_state: &mut Self::PendingKeyState,
        _keymap_index: u16,
        _context: &Self::Context,
        _key_ref: Ref,
        _event: key::Event<Self::Event>,
    ) -> (Option<key::NewPressedKey<R>>, key::KeyEvents<Self::Event>) {
        panic!()
    }

    fn update_state(
        &self,
        key_state: &mut Self::KeyState,
        Ref(key_index): &Self::Ref,
        context: &Self::Context,
        keymap_index: u16,
        event: key::Event<Self::Event>,
    ) -> key::KeyEvents<Self::Event> {
        let key = &self.keys[*key_index as usize];
        key_state.update_state(key, context, keymap_index, event)
    }

    fn key_output(
        &self,
        Ref(key_index): &Self::Ref,
        key_state: &Self::KeyState,
    ) -> Option<key::KeyOutput> {
        let key = &self.keys[*key_index as usize];
        key_state.key_output(key)
    }
}
