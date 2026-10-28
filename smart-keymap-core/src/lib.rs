#![warn(missing_docs)]
#![warn(clippy::unwrap_used)]
#![warn(clippy::expect_used)]

//! Core smart-keymap engine.
//!
//! This crate provides the key family implementations
//!  and the [keymap::Keymap] engine.
//!
//! Firmware and other consumers that need a compiled keymap instance
//!  use the `smart-keymap` facade package,
//! which re-exports these modules
//!  and adds `init` / `new_keymap()`
//!  (via `SMART_KEYMAP_CUSTOM_KEYMAP`).

#![cfg_attr(not(feature = "std"), no_std)]

/// Structs for input to the keymap.
pub mod input;
/// Smart key interface and implementations.
///
/// The core interface is [key::System], and its associated [key::Context],
/// `PendingKeyState`, and [key::KeyState] types.
pub mod key;
/// Keymap implementation.
pub mod keymap;

/// Split keyboard support.
pub mod split;

/// A helper value type for Copy-able slices.
pub mod slice;
