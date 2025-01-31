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
//! The heart of the library is the [key] module, and its
//! [key::Key], [key::Context], [key::PressedKey] traits.
//!
//! These provide the interface with which 'smart keys' are implemented.

#![cfg_attr(not(feature = "std"), no_std)]

/// Structs for input to the keymap.
pub mod input;
/// Smart key interface and implementations.
///
/// The core interface for the smart keymap library is [key::Key],
///  and its associated [key::Context] and [key::PressedKeyState] types.
/// Together, these are used to define smart key behaviour.
pub mod key;
/// Keymap implementation.
pub mod keymap;
/// Keys1, Keys2, etc. tuple structs for defining keymaps.
pub mod tuples;

/// Types and initial data used for constructing a [keymap::Keymap].
/// cbindgen:ignore
#[cfg(not(custom_keymap))]
pub mod init {
    use crate::key::{composite, keyboard};
    use crate::tuples::Keys1;

    /// Config used to construct initial context.
    pub const CONFIG: crate::key::composite::Config = crate::key::composite::DEFAULT_CONFIG;

    /// Number of layers supported by the [crate::key::layered] implementation.
    pub const LAYER_COUNT: usize = 8;

    /// Alias for Context type; i.e. [composite::Context].
    pub type Context = composite::Context;

    /// Alias for Event type; i.e. [composite::Event].
    pub type Event = composite::Event;

    /// Alias for PressedKey type; i.e. [composite::PressedKey].
    pub type PressedKey = composite::PressedKey;

    /// Alias for keys.
    pub type Key = keyboard::Key;

    /// Initial [Context] value.
    pub const CONTEXT: Context = composite::Context::from_config(CONFIG);

    /// Alias for a tuples KeysN type. Without a custom keymap, just a single [composite::Key].
    pub type KeyDefinitionsType = Keys1<Key, Context, Event, PressedKey>;

    /// A tuples KeysN value with keys. Without a custom keymap, just the letter 'A'.
    pub const KEY_DEFINITIONS: KeyDefinitionsType = Keys1::new((keyboard::Key::new(0x04),));
}

#[cfg(custom_keymap)]
include!(concat!(env!("OUT_DIR"), "/keymap.rs"));
