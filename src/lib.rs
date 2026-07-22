#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

//! Smart Keymap library.
//!
//! A "smart keyboard" is a keyboard where the keys can perform
//!  multiple actions, depending on the context.
//! Features such as layering, tap-hold, and tap-dance are
//!  examples of smart keyboard functionality.
//!
//! The smart keymap library provides an interface for the "smart keymap"
//!  part of smart keyboard firmware.
//! i.e. the part that takes key presses and releases as input,
//!  and outputs HID keyboard reports (or other smart keyboard outputs).
//!
//! This crate can be used directly with Rust, or built as a C library.
//!
//! # Usage as a C library
//!
//! ## Custom Keymap
//!
//! When used as a C library, the library should be built by setting
//!  the environment variable `SMART_KEYMAP_CUSTOM_KEYMAP` to the path
//!  of a custom keymap file.
//!
//! `SMART_KEYMAP_CUSTOM_KEYMAP` can be set either to a `.ncl` file,
//!  or to a `.rs` file (generated using the scripts under `ncl/`).
//!
//! ## Keyboard Firmware Implementation
//!
//! When used as a C library, the firmware should call to
//!  `keymap_init`, `keymap_register_input_keypress`, `keymap_register_input_keyrelease`,
//!  and `keymap_tick` functions.
//! The `keymap_tick` function should be called every ms, and should copy the
//!  HID keyboard report to the given buffer.
//!
//! # Implementation Overview
//!
//! The heart of the library is the [key] module,
//! and its [key::System] trait.
//!
//! Per-keymap aggregation is produced by Nickel codegen
//!  as `init::key_system` (custom keymap / `keymap!`),
//!  or as `key::key_system` for std/cucumber.
//! Without a custom keymap,
//!  [init] provides a trivial keyboard-only shell
//!  so [`new_keymap`] still type-checks.

#![cfg_attr(not(feature = "std"), no_std)]

/// Structs for input to the keymap.
pub mod input;
/// Smart key interface and implementations.
///
/// The core interface for the smart keymap library is [key::System],
///  and its associated [key::Context], `PendingKeyState`, and [key::KeyState] types.
/// Together, these are used to define smart key behaviour.
pub mod key;
/// Keymap implementation.
pub mod keymap;

/// Split keyboard support.
pub mod split;

/// A helper value type for Copy-able slices.
pub mod slice;

/// Types and initial data used for constructing a [keymap::Keymap].
///
/// Without `SMART_KEYMAP_CUSTOM_KEYMAP`, this is a **keyboard-only** dummy map
/// (letter `A`) plus generous size constants used by composite shells /
/// cucumber. With a custom keymap, build codegen replaces this module.
/// cbindgen:ignore
#[cfg(not(custom_keymap))]
pub mod init {
    use crate as smart_keymap;

    /// Number of instructions used by the [crate::key::automation] implementation.
    pub const AUTOMATION_INSTRUCTION_COUNT: usize = 1024;

    /// Number of layers supported by the [crate::key::layered] implementation.
    pub const LAYERED_LAYER_COUNT: usize = 8;

    /// The maximum number of keys in a chord.
    pub const CHORDED_MAX_CHORD_SIZE: usize = 16;

    /// The maximum number of chords.
    pub const CHORDED_MAX_CHORDS: usize = 4;

    /// The maximum number of overlapping chords for a chorded key.
    pub const CHORDED_MAX_OVERLAPPING_CHORD_SIZE: usize = 16;

    /// The tap-dance definitions.
    pub const TAP_DANCE_MAX_DEFINITIONS: usize = 3;

    /// Trivial composite shell: keyboard family only (matches codegen shape).
    pub mod key_system {
        use crate as smart_keymap;
        use smart_keymap::key;
        use smart_keymap::keymap;

        /// Aggregate key reference.
        #[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq)]
        pub enum Ref {
            /// [smart_keymap::key::keyboard] variant.
            Keyboard(smart_keymap::key::keyboard::Ref),
        }

        /// Aggregate config (no configurable families in this default map).
        #[derive(serde::Deserialize, Debug, Clone, Copy, PartialEq)]
        pub struct Config {}

        impl Default for Config {
            fn default() -> Self {
                Self::new()
            }
        }

        impl Config {
            /// Constructs a new [Config] with defaults.
            pub const fn new() -> Self {
                Self {}
            }
        }

        /// Aggregate context.
        #[derive(Debug, Clone, Copy)]
        pub struct Context {
            keymap_context: smart_keymap::keymap::KeymapContext,
            keyboard: smart_keymap::key::keyboard::Context,
        }

        impl Context {
            /// Constructs a [Context] from the given [Config].
            pub const fn from_config(config: Config) -> Self {
                let _ = &config;
                Self {
                    keymap_context: smart_keymap::keymap::KeymapContext::new(),
                    keyboard: smart_keymap::key::keyboard::Context,
                }
            }
        }

        impl Default for Context {
            fn default() -> Self {
                Self::from_config(Config::new())
            }
        }

        impl key::Context for Context {
            type Event = Event;

            fn handle_event(
                &mut self,
                _event: key::Event<Self::Event>,
            ) -> key::KeyEvents<Self::Event> {
                key::KeyEvents::no_events()
            }
        }

        impl keymap::SetKeymapContext for Context {
            fn set_keymap_context(&mut self, context: keymap::KeymapContext) {
                self.keymap_context = context;
            }
        }

        /// Aggregate event.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum Event {
            /// [smart_keymap::key::keyboard] variant.
            Keyboard(smart_keymap::key::keyboard::Event),
        }

        impl From<smart_keymap::key::keyboard::Event> for Event {
            fn from(v: smart_keymap::key::keyboard::Event) -> Self {
                Event::Keyboard(v)
            }
        }

        impl TryFrom<Event> for smart_keymap::key::keyboard::Event {
            type Error = smart_keymap::key::EventError;

            fn try_from(v: Event) -> Result<Self, Self::Error> {
                match v {
                    Event::Keyboard(v) => Ok(v),
                }
            }
        }

        /// Aggregate pending key state.
        #[derive(Debug, Clone, PartialEq)]
        #[allow(clippy::large_enum_variant)]
        pub enum PendingKeyState {
            /// [smart_keymap::key::keyboard] variant.
            Keyboard(smart_keymap::key::keyboard::PendingKeyState),
        }

        impl From<smart_keymap::key::keyboard::PendingKeyState> for PendingKeyState {
            fn from(pks: smart_keymap::key::keyboard::PendingKeyState) -> Self {
                PendingKeyState::Keyboard(pks)
            }
        }

        /// Aggregate key state.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub enum KeyState {
            /// No-op key state (e.g. auxiliary chorded keys).
            NoOp,
            /// [smart_keymap::key::keyboard] key state.
            Keyboard(smart_keymap::key::keyboard::KeyState),
        }

        impl From<key::NoOpKeyState> for KeyState {
            fn from(_: key::NoOpKeyState) -> Self {
                KeyState::NoOp
            }
        }

        impl From<smart_keymap::key::keyboard::KeyState> for KeyState {
            fn from(ks: smart_keymap::key::keyboard::KeyState) -> Self {
                KeyState::Keyboard(ks)
            }
        }

        /// Aggregate [key::System] for the default keyboard-only map.
        #[derive(Debug, Clone, Copy, PartialEq)]
        pub struct System {
            keyboard:
                smart_keymap::key::keyboard::System<Ref, [smart_keymap::key::keyboard::Key; 0]>,
        }

        impl System {
            /// Constructs the system from the keyboard subsystem.
            pub const fn new(
                keyboard: smart_keymap::key::keyboard::System<
                    Ref,
                    [smart_keymap::key::keyboard::Key; 0],
                >,
            ) -> Self {
                Self { keyboard }
            }
        }

        impl key::System<Ref> for System {
            type Ref = Ref;
            type Context = Context;
            type Event = Event;
            type PendingKeyState = PendingKeyState;
            type KeyState = KeyState;

            fn new_pressed_key(
                &self,
                keymap_index: u16,
                context: &Self::Context,
                key_ref: Ref,
            ) -> (
                key::PressedKeyResult<Ref, Self::PendingKeyState, Self::KeyState>,
                key::KeyEvents<Self::Event>,
            ) {
                match key_ref {
                    Ref::Keyboard(key_ref) => {
                        let (pkr, pke) =
                            self.keyboard
                                .new_pressed_key(keymap_index, &context.keyboard, key_ref);
                        (pkr.into_result(), pke.into_events())
                    }
                }
            }

            fn update_pending_state(
                &self,
                pending_state: &mut Self::PendingKeyState,
                keymap_index: u16,
                context: &Self::Context,
                key_ref: Ref,
                event: key::Event<Self::Event>,
            ) -> (Option<key::NewPressedKey<Ref>>, key::KeyEvents<Self::Event>) {
                let _ = (pending_state, keymap_index, context, key_ref, event);
                panic!("no pending key systems in this key_system")
            }

            fn update_state(
                &self,
                key_state: &mut Self::KeyState,
                key_ref: &Self::Ref,
                context: &Self::Context,
                keymap_index: u16,
                event: key::Event<Self::Event>,
            ) -> key::KeyEvents<Self::Event> {
                match (key_ref, key_state) {
                    (Ref::Keyboard(key_ref), KeyState::Keyboard(key_state)) => {
                        if let Ok(event) = event.try_into_key_event() {
                            self.keyboard
                                .update_state(
                                    key_state,
                                    key_ref,
                                    &context.keyboard,
                                    keymap_index,
                                    event,
                                )
                                .into_events()
                        } else {
                            key::KeyEvents::no_events()
                        }
                    }
                    (_, _) => key::KeyEvents::no_events(),
                }
            }

            fn key_output(
                &self,
                key_ref: &Self::Ref,
                key_state: &Self::KeyState,
            ) -> Option<key::KeyOutput> {
                match (key_ref, key_state) {
                    (Ref::Keyboard(r), KeyState::Keyboard(ks)) => self.keyboard.key_output(r, ks),
                    (_, _) => None,
                }
            }
        }
    }

    pub use key_system::Context;
    pub use key_system::Event;
    pub use key_system::KeyState;
    pub use key_system::PendingKeyState;
    pub use key_system::Ref;
    pub use key_system::System;

    /// The number of keys in the keymap.
    pub const KEY_COUNT: usize = 1;

    /// Without a custom keymap, just the letter 'A'.
    pub const KEY_REFS: [Ref; KEY_COUNT] = [Ref::Keyboard(
        smart_keymap::key::keyboard::Ref::KeyCode(0x04),
    )];

    /// Config used to construct initial context.
    pub const CONFIG: key_system::Config = key_system::Config::new();

    /// Initial [Context] value.
    pub const CONTEXT: Context = key_system::Context::from_config(CONFIG);

    /// Initial [System] value.
    pub const SYSTEM: System =
        key_system::System::new(smart_keymap::key::keyboard::System::new([]));

    /// Alias for the [crate::keymap::Keymap] type.
    pub type Keymap = smart_keymap::keymap::Keymap<
        [Ref; KEY_COUNT],
        Ref,
        Context,
        Event,
        PendingKeyState,
        KeyState,
        System,
    >;
}

#[cfg(custom_keymap)]
include!(concat!(env!("OUT_DIR"), "/keymap.rs"));

pub use init::{Keymap, CONTEXT, KEY_REFS, SYSTEM};

/// Constructs a new keymap.
pub const fn new_keymap() -> Keymap {
    Keymap::new(KEY_REFS, CONTEXT, SYSTEM)
}
