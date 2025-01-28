//! # RP2040 RTIC Smart Keyboard
//!
//! This crate has support code for writing
//!  keyboard firmware RTIC apps for RP2040
//!  which use the [smart_keymap] crate.
//!
//! See the `examples/` for an example of Rust firmware using this crate.

#![no_main]
#![no_std]

/// Items related to app initialization.
pub mod app_init;

/// Items which are useful when writing keyboard firmware.
pub mod app_prelude;

/// Common USB device and class definitions.
pub mod common;

/// Type aliases related to input for RP2040 HAL types.
pub mod input;
