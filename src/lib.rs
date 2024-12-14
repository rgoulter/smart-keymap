#![cfg_attr(not(feature = "std"), no_std)]

mod keymap;

pub use keymap::KeyDefinition;

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
