//! Full-profile, Vec-backed composite [`key_system`] for std consumers.
//!
//! This crate isolates Nickel codegen of the universal composite shell so the
//! core [`smart_keymap`] package does not need `nickel` for default builds
//! (including `cargo doc`).
//!
//! Intended consumers: Cucumber suite, struct-size tooling, and other std-only
//! harnesses that need every registry family with runtime-sized key data.

#![warn(missing_docs)]

/// Size constants used by the generated full-profile shell.
///
/// Matches the generous defaults of [`smart_keymap::init`] when no custom
/// keymap is configured. Generated `key_system` code references
/// `crate::init::…` for const generics.
pub mod init {
    pub use smart_keymap::init::{
        AUTOMATION_INSTRUCTION_COUNT, CHORDED_MAX_CHORDS, CHORDED_MAX_CHORD_SIZE,
        CHORDED_MAX_OVERLAPPING_CHORD_SIZE, LAYERED_LAYER_COUNT, TAP_DANCE_MAX_DEFINITIONS,
    };
}

include!(concat!(env!("OUT_DIR"), "/composite_full_vec.rs"));
