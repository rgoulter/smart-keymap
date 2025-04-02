use serde::Deserialize;

use crate::input;
use crate::key;
use crate::keymap;

/// When the sticky modifiers activate.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "std", derive(Deserialize))]
pub enum StickyKeyActivation {
    /// Sticky modifiers activate when the sticky key is released.
    OnStickyKeyRelease,
    // TODO: add another config option for "on next key press"
    // /// Sticky modifiers activate when the next key is pressed.
    // OnNextKeyPress,
}

/// Sticky Key configuration.
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "std", derive(Deserialize))]
pub struct Config {
    /// The sticky key activation mode.
    pub activation: StickyKeyActivation,
}

/// The default [Config].
pub const DEFAULT_CONFIG: Config = Config {
    activation: StickyKeyActivation::OnStickyKeyRelease,
};

impl Default for Config {
    /// Returns the default context.
    fn default() -> Self {
        DEFAULT_CONFIG
    }
}

/// Sticky Modifiers context.
#[derive(Debug, Clone, Copy)]
pub struct Context {
    /// The sticky modifier key configuration.
    pub config: Config,
    /// Sticky modifiers.
    pub active_modifiers: Option<key::KeyboardModifiers>,
    /// Index of the next output resolved once a sticky key has been released.
    pub pressed_keymap_index: Option<u16>,
}

/// The default [Context].
pub const DEFAULT_CONTEXT: Context = Context {
    config: DEFAULT_CONFIG,
    active_modifiers: None,
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
    pub fn handle_event<E>(&mut self, event: key::Event<E>) -> key::KeyEvents<E>
    where
        Event: TryFrom<E>,
        E: core::fmt::Debug + core::marker::Copy,
    {
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
        //
        // TODO: update key doc with this; disambiguate "active" vs "virtual modifier pressed"

        match self.active_modifiers {
            Some(active_modifiers) => {
                match event {
                    key::Event::Keymap(keymap::KeymapEvent::ResolvedKeyOutput {
                        keymap_index,
                        key_output:
                            key::KeyOutput {
                                key_code,
                                key_modifiers,
                            },
                    }) => {
                        let mut pke = key::KeyEvents::no_events();

                        // The sticky key deactivates (releases)
                        //  once the modified key releases.
                        //
                        // Track the keymap index that resolved the key state.
                        self.pressed_keymap_index = Some(keymap_index);

                        // if matches!(self.config.activation, StickyKeyActivation::OnNextKeyPress) {
                        //     // TODO: if the config is to activate on key press, send the VK here
                        //     todo!()
                        // }

                        pke
                    }
                    // TODO: other key release
                    // TODO: can also do *other* key press to release the sticky key (c.f. zmk quick-release)
                    key::Event::Key { key_event, .. } => {
                        if let Ok(ev) = key_event.try_into() {
                            match ev {
                                Event::ActivateModifiers(mods) => {
                                    // TODO: activate the modifiers
                                    todo!()
                                }
                            }
                        } else {
                            key::KeyEvents::no_events()
                        }
                    }
                    _ => key::KeyEvents::no_events(),
                }
            }
            None => key::KeyEvents::no_events(),
        }
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
        KeyState::new(self.sticky_modifiers)
    }
}

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
    sticky_modifiers: key::KeyboardModifiers,
    behavior: Behavior,
}

impl KeyState {
    /// Constructs a new key state with the given sticky modifiers.
    pub fn new(sticky_modifiers: key::KeyboardModifiers) -> Self {
        KeyState {
            sticky_modifiers,
            behavior: Behavior::Sticky,
        }
    }
}

impl key::KeyState for KeyState {
    type Context = key::composite::Context;
    type Event = key::composite::Event;

    fn handle_event(
        &mut self,
        context: Self::Context,
        keymap_index: u16,
        event: key::Event<Self::Event>,
    ) -> key::KeyEvents<Self::Event> {
        // TODO:
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
                    let sticky_ctx: Context = todo!(); // context.into();
                    match sticky_ctx.config.activation {
                        StickyKeyActivation::OnStickyKeyRelease => {
                            let sticky_ev = Event::ActivateModifiers(self.sticky_modifiers);
                            let k_ev = key::Event::key_event(keymap_index, sticky_ev);
                            // LIMITATION: Assume sticky mod is for ONE modifier only.
                            let sticky_key_code = self.sticky_modifiers.as_key_codes()[0];
                            todo!()
                            // let vk_ev = key::Event::Input(input::Event::VirtualKeyPress { key_code: sticky_key_code });
                            // key::KeyEvents::event(k_ev)
                        }
                    }
                }
                _ => key::KeyEvents::no_events(),
            },
            Behavior::Regular => key::KeyEvents::no_events(),
        }
    }

    fn key_output(&self) -> Option<key::KeyOutput> {
        match self.behavior {
            Behavior::Sticky => None,
            Behavior::Regular => Some(key::KeyOutput::from_key_modifiers(self.sticky_modifiers)),
        }
    }
}
