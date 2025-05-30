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

use smart_keymap::{init, input, key, keymap, split};

/// Length of a buffer for serializing/deserializing split keyboard events.
pub const MESSAGE_BUFFER_LEN: usize = 4;

/// Callback ID for "reset keyboard"
pub const KEYMAP_CALLBACK_RESET: u8 = 0;
/// Callback ID for "enter bootloader mode"
pub const KEYMAP_CALLBACK_BOOTLOADER: u8 = 1;

/// Input event type.
#[repr(C)]
pub enum KeymapInputEventType {
    /// Key Press event.
    KeymapEventPress = 0,
    /// Key Release event.
    KeymapEventRelease = 1,
    /// Virtual Key Press event.
    KeymapEventVirtualPress = 2,
    /// Virtual Key Release event.
    KeymapEventVirtualRelease = 3,
}

/// Input event.
///
/// LIMITATION: split transport for virtual keys only works for basic key codes.
#[repr(C)]
pub struct KeymapInputEvent {
    /// Whether the event is a press or a release.
    pub event_type: KeymapInputEventType,
    /// The keymap index of the event.
    pub value: u16,
}

impl From<KeymapInputEvent> for input::Event {
    fn from(KeymapInputEvent { event_type, value }: KeymapInputEvent) -> Self {
        match event_type {
            KeymapInputEventType::KeymapEventPress => input::Event::Press {
                keymap_index: value,
            },
            KeymapInputEventType::KeymapEventRelease => input::Event::Release {
                keymap_index: value,
            },
            KeymapInputEventType::KeymapEventVirtualPress => {
                let key_output = key::KeyOutput::from_key_code(value as u8);
                input::Event::VirtualKeyPress { key_output }
            }
            KeymapInputEventType::KeymapEventVirtualRelease => {
                let key_output = key::KeyOutput::from_key_code(value as u8);
                input::Event::VirtualKeyRelease { key_output }
            }
        }
    }
}

impl From<input::Event> for KeymapInputEvent {
    fn from(ev: input::Event) -> Self {
        match ev {
            input::Event::Press {
                keymap_index: value,
            } => KeymapInputEvent {
                event_type: KeymapInputEventType::KeymapEventPress,
                value,
            },
            input::Event::Release {
                keymap_index: value,
            } => KeymapInputEvent {
                event_type: KeymapInputEventType::KeymapEventRelease,
                value,
            },
            // LIMITATION: split transport for virtual keys not implemented
            input::Event::VirtualKeyPress { .. } => todo!(),
            // LIMITATION: split transport for virtual keys not implemented
            input::Event::VirtualKeyRelease { .. } => todo!(),
        }
    }
}

/// HID report.
#[repr(C)]
pub struct KeymapHidReport {
    /// HID Boot keyboard report.
    pub keyboard: [u8; 8],
    /// Reported `Custom` codes. (Implementation defined).
    pub custom: [u8; 6],
}

static mut KEYMAP: init::Keymap = init::Keymap::new(init::KEY_DEFINITIONS, init::CONTEXT);

/// Initialize the global keymap instance.
#[allow(static_mut_refs)]
#[no_mangle]
pub extern "C" fn keymap_init() {
    unsafe {
        KEYMAP.init();
    }
}

/// Sets the number of milliseconds between calls to `keymap_tick`.
#[allow(static_mut_refs)]
#[no_mangle]
pub extern "C" fn keymap_set_ms_per_tick(ms_per_tick: u8) {
    unsafe {
        KEYMAP.set_ms_per_tick(ms_per_tick);
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
pub unsafe extern "C" fn keymap_tick(report: &mut KeymapHidReport) {
    unsafe {
        KEYMAP.tick();

        let keymap_output = KEYMAP.report_output();

        let keyboard_report = keymap_output.as_hid_boot_keyboard_report();
        core::ptr::copy_nonoverlapping(
            keyboard_report.as_ptr(),
            report.keyboard.as_mut_ptr(),
            keyboard_report.len(),
        );

        let zeros = [0; 6];
        core::ptr::copy_nonoverlapping(zeros.as_ptr(), report.custom.as_mut_ptr(), zeros.len());
        let custom_codes = &keymap_output.pressed_custom_codes();
        core::ptr::copy_nonoverlapping(
            custom_codes.as_ptr(),
            report.custom.as_mut_ptr(),
            6.min(custom_codes.len()),
        );
    }
}

/// Registers a callback with the keymap.
///
/// callback_id should be one of:
/// - KEYMAP_CALLBACK_RESET
/// - KEYMAP_CALLBACK_BOOTLOADER
#[allow(static_mut_refs)]
#[no_mangle]
pub unsafe extern "C" fn keymap_register_callback(
    callback_id: u8,
    callback_fn: extern "C" fn() -> (),
) {
    if let Some(callback_id) = match callback_id {
        _ if callback_id == KEYMAP_CALLBACK_RESET => Some(keymap::KeymapCallback::Reset),
        _ if callback_id == KEYMAP_CALLBACK_BOOTLOADER => {
            Some(keymap::KeymapCallback::ResetToBootloader)
        }
        _ => None,
    } {
        unsafe {
            KEYMAP.set_callback_extern(callback_id, callback_fn);
        }
    }
}

/// Registers a custom callback with the keymap.
#[allow(static_mut_refs)]
#[no_mangle]
pub unsafe extern "C" fn keymap_register_custom_callback(
    custom_0: u8,
    custom_1: u8,
    callback_fn: extern "C" fn() -> (),
) {
    unsafe {
        let callback_id = keymap::KeymapCallback::Custom(custom_0, custom_1);
        KEYMAP.set_callback_extern(callback_id, callback_fn);
    }
}

/// Serializes the given event into the given buffer.
#[no_mangle]
pub unsafe extern "C" fn keymap_serialize_event(buf: *mut u8, event: KeymapInputEvent) {
    unsafe {
        let message = split::Message::new(event.into());
        let message_bytes = message.serialize();
        core::ptr::copy_nonoverlapping(message_bytes.as_ptr(), buf, message_bytes.len());
    }
}

/// Deserializes the given bytes into the given pointer;
/// returns true if successful, false if fails.
#[no_mangle]
pub unsafe extern "C" fn keymap_message_buffer_receive_byte(
    buf: &mut [u8; MESSAGE_BUFFER_LEN],
    recv_byte: u8,
    event: *mut KeymapInputEvent,
) -> bool {
    let res = split::receive_byte(buf, recv_byte);
    match res {
        Ok(message) => {
            *event = message.input_event.into();
            true
        }
        Err(_) => false,
    }
}

// When built with "std", a panic handler is provided.
#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
