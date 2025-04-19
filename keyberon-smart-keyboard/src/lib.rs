//! # USBD Smart Keyboard
//!
//! This crate provides functionality for using [smart_keymap]
//!  as part of Rust keyboard firmware.
//!
//! See the `examples/` under `rp2040-rtic-smart-keyboard`
//!  for an example of Rust firmware using this crate.

#![no_main]
#![no_std]

/// Useful traits and structs related to input.
pub mod input;

/// A matrix scanning impl. with delay.
pub mod matrix;

/// UART-based split pressed key communication.
pub mod split;
