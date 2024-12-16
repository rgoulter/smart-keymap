#![cfg_attr(not(feature = "std"), no_std)]

mod keymap;

#[no_mangle]
pub extern "C" fn keymap_init() {
}

#[no_mangle]
pub extern "C" fn keymap_register_input_keypress(index: u8) {
}

#[no_mangle]
pub extern "C" fn keymap_register_input_keyrelease(index: u8) {
}

#[no_mangle]
pub extern "C" fn copy_hid_boot_keyboard_report(buf: *mut u8) {
    if buf.is_null() {
        return;
    }

    unsafe {
        core::ptr::write_bytes(buf, 0, 8);
    }
}

#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
