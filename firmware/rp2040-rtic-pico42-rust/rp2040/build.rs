use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

use smart_keymap_nickel_helper::{nickel_board_rs_for_board_path, rustfmt, NickelError};

fn main() {
    println!("cargo:rerun-if-env-changed=SMART_KEYBOARD_CUSTOM_BOARD");
    println!("cargo::rustc-check-cfg=cfg(custom_board)");
    if let Ok(custom_board_path) = env::var("SMART_KEYBOARD_CUSTOM_BOARD") {
        let out_dir = env::var("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join("board.rs");
        println!("cargo:rerun-if-changed={}", dest_path.to_str().unwrap());

        if custom_board_path.ends_with(".rs") {
            println!("cargo:rustc-cfg=custom_board");

            // Copy the custom keymap file to the output directory
            fs::copy(custom_board_path, &dest_path).unwrap();
        } else if custom_board_path.ends_with(".ncl") {
            println!("cargo:rustc-cfg=custom_board");

            // Evaluate the custom keymap file with Nickel
            let keymap_path = Path::new(&custom_board_path);
            match nickel_board_rs_for_board_path(
                format!("{}/ncl", env!("CARGO_MANIFEST_DIR")),
                keymap_path,
            ) {
                Ok(board_rs) => {
                    let mut file = fs::File::create(&dest_path).unwrap();
                    let formatted = rustfmt(board_rs);
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
            panic!("Unsupported custom board path: {}", custom_board_path);
        }
    }
}
