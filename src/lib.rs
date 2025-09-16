#![warn(missing_docs)]

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
//! Implementors are then aggregated by [key::composite],
//! which is used by the [keymap] module.

#![cfg_attr(not(feature = "std"), no_std)]

/// Structs for input to the keymap.
pub mod input;
/// Smart key interface and implementations.
///
/// The core interface for the smart keymap library is [key::Key],
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
/// cbindgen:ignore
#[cfg(not(custom_keymap))]
pub mod init {
    use crate::key::composite;
    use crate::key::keyboard;
    use crate::keymap;

    use composite as key_system;

    /// Config used to construct initial context.
    pub const CONFIG: key_system::Config = key_system::DEFAULT_CONFIG;

    /// Number of layers supported by the [crate::key::layered] implementation.
    pub const LAYER_COUNT: usize = 8;

    /// The maximum number of keys in a chord.
    pub const MAX_CHORD_SIZE: usize = 16;

    /// The maximum number of chords.
    pub const MAX_CHORDS: usize = 4;

    /// The maximum number of overlapping chords for a chorded key.
    pub const MAX_OVERLAPPING_CHORD_SIZE: usize = 16;

    /// The tap-dance definitions.
    pub const MAX_TAP_DANCE_DEFINITIONS: usize = 3;

    pub use key_system::Ref;

    pub use key_system::Context;

    pub use key_system::Event;

    pub use key_system::PendingKeyState;

    pub use key_system::KeyState;

    pub use key_system::System;

    /// Max number of data entries for each system.
    pub const DATA_LEN: usize = 32;

    /// Initial [Context] value.
    pub const CONTEXT: Context = key_system::Context::from_config(CONFIG);

    /// The number of keys in the keymap.
    pub const KEY_COUNT: usize = 1;

    /// Alias for the [keymap::Keymap] type.
    pub type Keymap = keymap::Keymap<
        [Ref; KEY_COUNT],
        Ref,
        Context,
        Event,
        PendingKeyState,
        KeyState,
        System<
            crate::key::composite::KeyArrays<
                DATA_LEN,
                DATA_LEN,
                DATA_LEN,
                DATA_LEN,
                DATA_LEN,
                DATA_LEN,
                DATA_LEN,
            >,
        >,
    >;

    /// A tuples KeysN value with keys. Without a custom keymap, just the letter 'A'.
    pub const KEY_DEFINITIONS: [Ref; KEY_COUNT] = [Ref::Keyboard(keyboard::Ref::KeyCode(0x04))];
}

#[cfg(custom_keymap)]
include!(concat!(env!("OUT_DIR"), "/keymap.rs"));
