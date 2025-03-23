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
//! This crate is to be built as a C library.
//!
//! # Usage as a C library
//!
//! ## Custom Keymap
//!
//! When used as a C library, the library should be built by setting
//!  the environment variable `SMART_KEYMAP_CUSTOM_KEYMAP` to the path
//!  of a custom keymap file.
//!
//! `SMART_KEYMAP_CUSTOM_KEYMAP` can be set either to a `.ncl` file,
//!  or to a `.rs` file (generated using the scripts under `ncl/`).
//!
//! ## Keyboard Firmware Implementation
//!
//! When used as a C library, the firmware should call to
//!  `keymap_init`, `keymap_register_input_keypress`, `keymap_register_input_keyrelease`,
//!  and `keymap_tick` functions.
//! The `keymap_tick` function should be called every ms, and should copy the
//!  HID keyboard report to the given buffer.

#![cfg_attr(not(feature = "std"), no_std)]

use smart_keymap::{init, input, keymap};

/// Input event type.
#[repr(C)]
pub enum KeymapInputEventType {
    /// Key Press event.
    KeymapEventPress = 0,
    /// Key Release event.
    KeymapEventRelease = 1,
}

/// Input event.
#[repr(C)]
pub struct KeymapInputEvent {
    /// Whether the event is a press or a release.
    pub event_type: KeymapInputEventType,
    /// The keymap index of the event.
    pub value: u16,
}

impl From<KeymapInputEvent> for input::Event {
    fn from(
        KeymapInputEvent {
            event_type,
            value: keymap_index,
        }: KeymapInputEvent,
    ) -> Self {
        match event_type {
            KeymapInputEventType::KeymapEventPress => input::Event::Press { keymap_index },
            KeymapInputEventType::KeymapEventRelease => input::Event::Release { keymap_index },
        }
    }
}

static mut KEYMAP: keymap::Keymap<init::KeyDefinitionsType> =
    keymap::Keymap::new(init::KEY_DEFINITIONS, init::CONTEXT);

/// Initialize the global keymap instance.
#[allow(static_mut_refs)]
#[no_mangle]
pub extern "C" fn keymap_init() {
    unsafe {
        KEYMAP.init();
    }
}

/// Register an input event to the global keymap instance.
#[allow(static_mut_refs)]
#[no_mangle]
pub extern "C" fn keymap_register_input_event(event: KeymapInputEvent) {
    unsafe {
        KEYMAP.handle_input(event.into());
    }
}

/// Register a keypress event to the global keymap instance.
#[allow(static_mut_refs)]
#[no_mangle]
pub extern "C" fn keymap_register_input_keypress(keymap_index: u16) {
    keymap_register_input_event(KeymapInputEvent {
        event_type: KeymapInputEventType::KeymapEventPress,
        value: keymap_index,
    });
}

/// Register a keyrelease event to the global keymap instance.
#[allow(static_mut_refs)]
#[no_mangle]
pub extern "C" fn keymap_register_input_keyrelease(keymap_index: u16) {
    keymap_register_input_event(KeymapInputEvent {
        event_type: KeymapInputEventType::KeymapEventRelease,
        value: keymap_index,
    });
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

// When built with "std", a panic handler is provided.
#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
