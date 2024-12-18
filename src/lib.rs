#![cfg_attr(not(feature = "std"), no_std)]

mod input;
mod keymap;


static mut KEYMAP: keymap::Keymap<4> = keymap::Keymap::new(keymap::KEY_DEFINITIONS);

#[no_mangle]
pub extern "C" fn keymap_init() {
    unsafe {
        KEYMAP.init();
    }
}

#[no_mangle]
pub extern "C" fn keymap_register_input_keypress(keymap_index: u16) {
    unsafe {
        KEYMAP.handle_input(input::Event::Press { keymap_index });
    }
}

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
#[no_mangle]
pub extern "C" fn keymap_tick(buf: *mut u8) {
    if buf.is_null() {
        return;
    }

    unsafe {
        KEYMAP.tick();

        let report = KEYMAP.boot_keyboard_report();
        core::ptr::copy_nonoverlapping(report.as_ptr(), buf, report.len());
    }
}

#[no_mangle]
pub extern "C" fn copy_hid_boot_keyboard_report(buf: *mut u8) {
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
