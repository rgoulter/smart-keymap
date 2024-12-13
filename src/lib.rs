mod keymap;

pub use keymap::KeyDefinition;

#[no_mangle]
pub extern "C" fn copy_hid_boot_keyboard_report(buf: *mut u8) {
    if buf.is_null() {
        return;
    }

    unsafe {
        std::ptr::write_bytes(buf, 0, 8);
    }
}
