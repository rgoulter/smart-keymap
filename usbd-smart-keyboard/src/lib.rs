//! # USBD Smart Keyboard
//!
//! This crate provides functionality for using [smart_keymap]
//!  with USB HID keyboards built for the [usb_device] crate.
//!
//! See the `examples/` under `rp2040-rtic-smart-keyboard`
//!  for an example of Rust firmware using this crate.

#![no_main]
#![no_std]

/// Items which are useful to have imported in usbd keyboard firmware.
pub mod app_prelude;

/// Useful traits and structs related to input.
pub mod input;

/// A matrix scanning impl. with delay.
pub mod matrix;

/// UART-based split pressed key communication.
pub mod split;

/// Common USB device/class definitions.
pub mod common;
