use std::env;

fn main() {
    println!("cargo:rerun-if-env-changed=SMART_KEYMAP_CUSTOM_KEYMAP");
    println!("cargo::rustc-check-cfg=cfg(custom_keymap)");
    if env::var("SMART_KEYMAP_CUSTOM_KEYMAP").is_ok() {
        println!("cargo:rustc-cfg=custom_keymap");
    }
}
