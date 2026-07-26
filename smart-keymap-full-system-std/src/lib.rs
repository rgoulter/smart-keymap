//! Full-profile, Vec-backed composite [`key_system`] for std consumers.
//!
//! This crate isolates Nickel codegen of the universal composite shell so the
//! core [`smart_keymap`] package does not need `nickel` for default builds
//! (including `cargo doc`).
//!
//! Intended consumers: Cucumber suite, struct-size tooling, and other std-only
//! harnesses that need every registry family with runtime-sized key data.

#![warn(missing_docs)]

/// Size constants and the generated full-profile shell.
///
/// Matches the generous defaults of [`smart_keymap::init`] when no custom
/// keymap is configured. Generated `key_system` is nested here and refers to
/// size consts via `super::…` (same nested-shell convention as firmware
/// `init` / `keymap!`).
pub mod init {
    pub use smart_keymap::init::{
        AUTOMATION_INSTRUCTION_COUNT, CHORDED_MAX_CHORDS, CHORDED_MAX_CHORD_SIZE,
        CHORDED_MAX_OVERLAPPING_CHORD_SIZE, CONDITIONAL_LAYER_COUNT, LAYERED_LAYER_COUNT,
        SEQUENCE_MAX_OVERLAPPING, SEQUENCE_MAX_SEQUENCES, SEQUENCE_MAX_SEQUENCE_LEN,
        TAP_DANCE_MAX_DEFINITIONS,
    };

    include!(concat!(env!("OUT_DIR"), "/composite_full_vec.rs"));
}

/// Full-profile, Vec-backed composite key system (re-export from [`init`]).
pub use init::key_system;
