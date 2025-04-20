//! # Keyberon Smart Keyboard
//!
//! This crate provides functionality for using [smart_keymap]
//!  as part of Rust keyboard firmware.
//!
//! See the `examples/` under `rp2040-rtic-smart-keyboard`
//!  for an example of Rust firmware using this crate.
//!
//! ## Migrating from Keyberon
//!
//! Roughly, where Keyberon code is typically:
//!
//! ```ignore
//! // Gather events from keyboard matrix
//! let key_presses = matrix.get().unwrap();
//! let debounced_events = debouncer.events(key_presses).collect();
//! chording.tick(debounced_events)
//!
//! // Compute key codes from those events
//! for event in chording.events() {
//!     layout.event(event);
//! }
//! let keycodes = layout.keycodes();
//! ```
//!
//! With this crate, [input::Keyboard] manages scanning/debouncing events
//!  from the keyboard matrix. Chording is handled by smart keymap.
//!
//! For computing the key codes, this uses [input::smart_keymap::KeyboardBackend]:
//!
//! ```ignore
//! // Compute key codes from those events
//! for event in keyboard.events() {
//!     if let Some(event) = keymap_index_of(&KEYMAP_INDICES, event) {
//!         backend.event(event);
//!     }
//! }
//! backend.tick();
//! let key_codes = backend
//!   .keymap_output()
//!   .pressed_key_codes();
//! ```

#![no_main]
#![no_std]

/// Useful traits and structs related to input.
pub mod input;

/// A matrix scanning impl. with delay.
pub mod matrix;

/// UART-based split pressed key communication.
pub mod split;
