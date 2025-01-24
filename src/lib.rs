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
//! The most user friendly way to do this is to use `ncl/keymap-codegen.ncl`
//!  to produce a `keymap.rs` file from a `keymap.json` file.
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

#[allow(unused)]
use key::{composite, keyboard, tap_hold};

/// Types and initial data used for constructing [KEYMAP].
/// cbindgen:ignore
#[cfg(not(custom_keymap))]
pub mod init {
    use crate::key::{composite, keyboard, layered};
    use crate::tuples::Keys1;

    const NUM_LAYERS: usize = 0;

    /// Alias for layers impl.
    pub type LayersImpl = layered::ArrayImpl<NUM_LAYERS>;

    /// Alias for the NestedKey used for the [Context].
    pub type NestedKey = composite::DefaultNestableKey;

    /// Types used in Composite keys.
    pub type CompositeImpl = composite::CompositeImpl<LayersImpl, NestedKey>;

    /// Alias for Context type; i.e. [crate::key::context::Context] with generics.
    pub type Context = composite::Context<LayersImpl>;

    /// Alias for Event type; i.e. [crate::key::context::Event].
    pub type Event = composite::Event;

    /// Alias for keys.
    pub type Key = composite::Key<CompositeImpl>;

    /// Initial [Context] value.
    pub const CONTEXT: Context = composite::Context {
        layer_context: layered::Context {
            active_layers: [false; NUM_LAYERS],
        },
    };

    /// Alias for a [tuples] KeysN type. Without a custom keymap, just a single [key::composite::Key].
    pub type KeyDefinitionsType = Keys1<Key, Context, Event>;

    /// A [tuples] KeysN value with keys. Without a custom keymap, just the letter 'A'.
    pub const KEY_DEFINITIONS: KeyDefinitionsType =
        Keys1::new((Key::keyboard(keyboard::Key::new(0x04)),));
}

#[cfg(custom_keymap)]
include!(concat!(env!("OUT_DIR"), "/keymap.rs"));

static mut KEYMAP: keymap::Keymap<init::KeyDefinitionsType, init::LayersImpl> =
    keymap::Keymap::new(init::KEY_DEFINITIONS, init::CONTEXT);

/// Initialize the global keymap instance.
#[allow(static_mut_refs)]
#[no_mangle]
pub extern "C" fn keymap_init() {
    unsafe {
        KEYMAP.init();
    }
}

/// Register a keypress event to the global keymap instance.
#[allow(static_mut_refs)]
#[no_mangle]
pub extern "C" fn keymap_register_input_keypress(keymap_index: u16) {
    unsafe {
        KEYMAP.handle_input(input::Event::Press { keymap_index });
    }
}

/// Register a keyrelease event to the global keymap instance.
#[allow(static_mut_refs)]
#[no_mangle]
pub extern "C" fn keymap_register_input_keyrelease(keymap_index: u16) {
    unsafe {
        KEYMAP.handle_input(input::Event::Release { keymap_index });
    }
}

/// Run Keymap processing.
///
/// Should be called every ms.
///
/// The HID keyboard report is copied to the given buffer.
///
/// The `buf` report is for the HID boot keyboard.
///
/// # Safety
///
/// `buf` must be a valid pointer to a buffer of at least 8 bytes.
#[allow(static_mut_refs)]
#[no_mangle]
pub unsafe extern "C" fn keymap_tick(buf: *mut u8) {
    if buf.is_null() {
        return;
    }

    unsafe {
        KEYMAP.tick();

        let report = KEYMAP.report_output().as_hid_boot_keyboard_report();
        core::ptr::copy_nonoverlapping(report.as_ptr(), buf, report.len());
    }
}

/// Copy the HID keyboard report to the given buffer.
///
/// It's better to use [keymap_tick] copy the report to the buffer,
///  because the report won't change between [keymap_tick] calls.
///
/// # Safety
///
/// `buf` must be a valid pointer to a buffer of at least 8 bytes.
#[allow(static_mut_refs)]
#[no_mangle]
pub unsafe extern "C" fn copy_hid_boot_keyboard_report(buf: *mut u8) {
    if buf.is_null() {
        return;
    }

    unsafe {
        let report = keymap::KeymapOutput::new(KEYMAP.pressed_keys()).as_hid_boot_keyboard_report();
        core::ptr::copy_nonoverlapping(report.as_ptr(), buf, report.len());
    }
}

#[cfg(all(not(feature = "std"), feature = "staticlib"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
