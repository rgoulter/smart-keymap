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
//! ```
//! use embedded_hal::digital::{InputPin, OutputPin};
//! use keyberon::chording::Chording;
//! use keyberon::debounce::Debouncer;
//! use keyberon::layout::CustomEvent;
//! use keyberon::layout::Event;
//! use keyberon::layout::Layout;
//! use keyberon::matrix::Matrix;
//!
//! // Gather events from keyboard matrix
//! fn keyboard_events<C, R, E, const COLS: usize, const ROWS: usize, const CHORD_COUNT: usize>(
//!     matrix: &mut Matrix<C, R, COLS, ROWS>,
//!     debouncer: &mut Debouncer<[[bool; COLS]; ROWS]>,
//!     chording: &mut Chording<CHORD_COUNT>,
//! ) -> heapless::Vec<Event, 8>
//! where
//!     C: InputPin<Error = E>,
//!     R: OutputPin<Error = E>,
//!     E: core::fmt::Debug,
//! {
//!     let key_presses: [[bool; COLS]; ROWS] = matrix.get().unwrap();
//!     let debounced_events: heapless::Vec<Event, 8> = debouncer.events(key_presses).collect();
//!     chording.tick(debounced_events)
//! }
//!
//! // Compute key codes from those events
//! fn key_codes_from_events<
//!     T: 'static,
//!     K: Copy + 'static,
//!     const COLS: usize,
//!     const ROWS: usize,
//!     const LAYER_COUNT: usize,
//! >(
//!     events: heapless::Vec<Event, 8>,
//!     layout: &mut Layout<COLS, ROWS, LAYER_COUNT, T, K>,
//! ) -> heapless::Vec<K, 8> {
//!     for event in events {
//!         layout.event(event);
//!     }
//!     let _custom_event: CustomEvent<T> = layout.tick();
//!     layout.keycodes().collect()
//! }
//! ```
//!
//! With this crate, [input::Keyboard] manages scanning/debouncing events
//!  from the keyboard matrix. Chording is handled by smart keymap.
//!
//! For computing the key codes, this uses [input::smart_keymap::KeyboardBackend]:
//!
//! ```
//! use keyberon::layout::Event;
//!
//! use keyberon_smart_keyboard::input::Keyboard;
//! use keyberon_smart_keyboard::input::MatrixScanner;
//! use keyberon_smart_keyboard::input::PressedKeys;
//! use keyberon_smart_keyboard::input::smart_keymap::KeyboardBackend;
//! use keyberon_smart_keyboard::input::smart_keymap::keymap_index_of;
//!
//! // Gather events from keyboard matrix
//! fn keyboard_events<M: MatrixScanner<COLS, ROWS>, const COLS: usize, const ROWS: usize>(
//!     keyboard: &mut Keyboard<COLS, ROWS, M>,
//! ) -> heapless::Vec<Event, 8>
//! {
//!     keyboard.events()
//! }
//!
//! // Compute key codes from those events
//! fn key_codes_from_events<const COLS: usize, const ROWS: usize>(
//!     keymap_indices: &[[Option<u16>; COLS]; ROWS],
//!     events: heapless::Vec<Event, 8>,
//!     backend: &mut KeyboardBackend,
//! ) -> heapless::Vec<u8, 24> {
//!     for event in events {
//!         if let Some(event) = keymap_index_of(keymap_indices, event) {
//!             backend.event(event);
//!         }
//!     }
//!     backend.tick();
//!     backend.keymap_output().pressed_key_codes()
//! }
//! ```

#![no_main]
#![no_std]

/// Useful traits and structs related to input.
pub mod input;

/// A matrix scanning impl. with delay.
pub mod matrix;

/// UART-based split pressed key communication.
pub mod split;
