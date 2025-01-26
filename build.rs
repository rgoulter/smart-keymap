use std::env;
use std::fs;
use std::path::Path;

use std::io::{self, Write};
use std::process::{Command, Stdio};

/// Likely reasons why running `nickel` may fail.
pub enum NickelError {
    NickelNotFound,
    EvalError(String),
}

/// Result of Nickel evaluation.
pub type NickelResult = Result<String, NickelError>;

/// Evaluates the Nickel expr for a keymap, returning the json serialization.
pub fn nickel_keymap_rs_for_keymap_path(keymap_path: &Path) -> NickelResult {
    let spawn_nickel_result = Command::new("nickel")
        .args([
            "export",
            "--format=raw",
            format!("--import-path={}/ncl", env!("CARGO_MANIFEST_DIR")).as_ref(),
            "--field=keymap_rs",
            "keymap-codegen.ncl",
            "keymap-ncl-to-json.ncl",
            keymap_path.to_str().unwrap(),
        ])
        .stdin(Stdio::null())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => return Err(NickelError::NickelNotFound),
            _ => panic!("Failed to spawn nickel: {:?}", e),
        });

    match spawn_nickel_result {
        Ok(nickel_command) => match nickel_command.wait_with_output() {
            Ok(output) => {
                if output.status.success() {
                    String::from_utf8(output.stdout)
                        .map_err(|e| panic!("Failed to decode UTF-8: {:?}", e))
                } else {
                    let nickel_error_message = String::from_utf8(output.stderr)
                        .unwrap_or_else(|e| panic!("Failed to decode UTF-8: {:?}", e));
                    Err(NickelError::EvalError(nickel_error_message))
                }
            }
            Err(io_e) => {
                panic!("Unhandled IO error: {:?}", io_e)
            }
        },
        Err(e) => Err(e?),
    }
}

/// Tries running the given source through `rustfmt`.
pub fn rustfmt(rust_src: String) -> String {
    let spawn_rustfmt_result = Command::new("rustfmt")
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn();

    match spawn_rustfmt_result {
        Ok(mut rustfmt_child) => {
            let child_stdin = rustfmt_child.stdin.as_mut().unwrap();
            child_stdin.write_all(rust_src.as_bytes()).unwrap();

            match rustfmt_child.wait_with_output() {
                Ok(output) => {
                    if output.status.success() {
                        String::from_utf8(output.stdout).unwrap_or(rust_src)
                    } else {
                        rust_src
                    }
                }
                Err(_) => rust_src,
            }
        }
        Err(_) => return rust_src,
    }
}

fn main() {
    println!("cargo:rerun-if-env-changed=SMART_KEYMAP_CUSTOM_KEYMAP");
    println!("cargo::rustc-check-cfg=cfg(custom_keymap)");
    if let Ok(custom_keymap_path) = env::var("SMART_KEYMAP_CUSTOM_KEYMAP") {
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
            match nickel_keymap_rs_for_keymap_path(keymap_path) {
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
