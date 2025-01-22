use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-env-changed=SMART_KEYMAP_CUSTOM_KEYMAP");
    println!("cargo::rustc-check-cfg=cfg(custom_keymap)");
    if let Ok(custom_keymap_path) = env::var("SMART_KEYMAP_CUSTOM_KEYMAP") {
        let out_dir = env::var("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("keymap.rs");

        if custom_keymap_path.ends_with(".rs") {
            println!("cargo:rustc-cfg=custom_keymap");

            // Copy the custom keymap file to the output directory
            fs::copy(custom_keymap_path, &dest_path).unwrap();
        } else {
            panic!("Unsupported custom keymap path: {}", custom_keymap_path);
        }
    }
}
