//! USB HID boot keyboard usage codes referenced by Rust integration tests.
//!
//! Values match `tests/ceedling/test/support/hid_keycodes.h` and `ncl/hid-usage-keyboard.ncl`.

#![allow(dead_code)]

pub const KC_A: u8 = 0x04;
pub const KC_B: u8 = 0x05;
pub const KC_C: u8 = 0x06;
pub const KC_D: u8 = 0x07;
pub const KC_E: u8 = 0x08;
pub const KC_F: u8 = 0x09;
pub const KC_G: u8 = 0x0A;
pub const KC_K: u8 = 0x0E;
pub const KC_L: u8 = 0x0F;
pub const KC_M: u8 = 0x10;
pub const KC_N: u8 = 0x11;
pub const KC_O: u8 = 0x12;
pub const KC_P: u8 = 0x13;
pub const KC_SPACE: u8 = 0x2C;
pub const KC_SLASH: u8 = 0x38;

/// Modifier bits in HID boot keyboard report byte 0.
pub const MOD_LCTL: u8 = 0x01;
pub const MOD_LSHFT: u8 = 0x02;
pub const MOD_LCTL_LSHFT: u8 = MOD_LCTL | MOD_LSHFT;
