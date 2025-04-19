use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

use smart_keymap_nickel_helper::{nickel_keymap_rs_for_keymap_path, rustfmt, NickelError};

fn main() {
    println!("cargo:rerun-if-env-changed=SMART_KEYMAP_CUSTOM_KEYMAP");
    println!("cargo::rustc-check-cfg=cfg(custom_keymap)");
    if let Some(custom_keymap_path) = env::var("SMART_KEYMAP_CUSTOM_KEYMAP")
        .ok()
        .filter(|s| !s.is_empty())
    {
        let out_dir = env::var("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("keymap.rs");
        println!("cargo:rerun-if-changed={}", dest_path.to_str().unwrap());

        if custom_keymap_path.ends_with(".rs") {
            println!("cargo:rustc-cfg=custom_keymap");

            // Copy the custom keymap file to the output directory
            fs::copy(custom_keymap_path, &dest_path).unwrap();
        } else if custom_keymap_path.ends_with(".ncl") {
            println!("cargo:rustc-cfg=custom_keymap");

            // Evaluate the custom keymap file with Nickel
            let keymap_path = Path::new(&custom_keymap_path);
            match nickel_keymap_rs_for_keymap_path(
                format!("{}/ncl", env!("CARGO_MANIFEST_DIR")),
                keymap_path,
            ) {
                Ok(keymap_rs) => {
                    let mut file = fs::File::create(&dest_path).unwrap();
                    let formatted = rustfmt(keymap_rs);
                    file.write_all(formatted.as_bytes()).unwrap();
                }
                Err(NickelError::NickelNotFound) => {
                    panic!("`nickel` not found in PATH");
                }
                Err(NickelError::EvalError(e)) => {
                    panic!("Nickel evaluation failed: {}", e);
                }
            }
        } else {
            panic!("Unsupported custom keymap path: {}", custom_keymap_path);
        }
    }
}
