#![cfg_attr(not(feature = "std"), no_std)]

pub mod input;
pub mod key;
pub mod keymap;

#[allow(unused)]
use composite::{Event, Key};
#[allow(unused)]
use key::{composite, simple, tap_hold};

#[cfg(not(custom_keymap))]
pub const KEY_DEFINITIONS: [Key; 1] = [
    Key::Simple(simple::Key(0x04)), // A
];
#[cfg(custom_keymap)]
include!(env!("SMART_KEYMAP_CUSTOM_KEYMAP"));

static mut KEYMAP: keymap::Keymap<[Key; KEY_DEFINITIONS.len()], Key, Event> =
    keymap::Keymap::new(KEY_DEFINITIONS, key::composite::Context::new());

#[allow(static_mut_refs)]
#[no_mangle]
pub extern "C" fn keymap_init() {
    unsafe {
        KEYMAP.init();
    }
}

#[allow(static_mut_refs)]
#[no_mangle]
pub extern "C" fn keymap_register_input_keypress(keymap_index: u16) {
    unsafe {
        KEYMAP.handle_input(input::Event::Press { keymap_index });
    }
}

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

        let report = KEYMAP.boot_keyboard_report();
        core::ptr::copy_nonoverlapping(report.as_ptr(), buf, report.len());
    }
}

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
        let report = KEYMAP.boot_keyboard_report();
        core::ptr::copy_nonoverlapping(report.as_ptr(), buf, report.len());
    }
}

#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
