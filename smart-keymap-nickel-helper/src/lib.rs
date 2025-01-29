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

/// Evaluates the Nickel expr for a keymap, returning the keymap.rs contents.
pub fn nickel_keymap_rs_for_keymap_path(
    ncl_import_path: String,
    keymap_path: &Path,
) -> NickelResult {
    let spawn_nickel_result = Command::new("nickel")
        .args([
            "export",
            "--format=raw",
            format!("--import-path={}", ncl_import_path).as_ref(),
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
            io::ErrorKind::NotFound => Err(NickelError::NickelNotFound),
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

/// Evaluates the Nickel expr for a board, returning the board.rs contents.
pub fn nickel_board_rs_for_board_path(ncl_import_path: String, board_path: &Path) -> NickelResult {
    let spawn_nickel_result = Command::new("nickel")
        .args([
            "export",
            "--format=raw",
            format!("--import-path={}", ncl_import_path).as_ref(),
            "--field=board_rs",
            "codegen.ncl",
            board_path.to_str().unwrap(),
        ])
        .stdin(Stdio::null())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => Err(NickelError::NickelNotFound),
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
        Err(_) => rust_src,
    }
}

/// Evaluates the Nickel expr for a keymap, returning the json serialization.
pub fn nickel_json_serialization_for_keymap(
    ncl_import_path: String,
    keymap_ncl: &str,
) -> NickelResult {
    let spawn_nickel_result = Command::new("nickel")
        .args([
            "export",
            "--format=json",
            format!("--import-path={}", ncl_import_path).as_ref(),
            "--field=serialized_json_composite_keys",
        ])
        .stdin(Stdio::piped())
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| match e.kind() {
            io::ErrorKind::NotFound => Err(NickelError::NickelNotFound),
            _ => panic!("Failed to spawn nickel: {:?}", e),
        });

    match spawn_nickel_result {
        Ok(mut nickel_command) => {
            let child_stdin = nickel_command.stdin.as_mut().unwrap();
            child_stdin
                .write_all(
                    format!(r#"(import "keymap-ncl-to-json.ncl") & ({})"#, keymap_ncl).as_bytes(),
                )
                .unwrap_or_else(|e| panic!("Failed to write to stdin: {:?}", e));

            match nickel_command.wait_with_output() {
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
            }
        }
        Err(e) => Err(e?),
    }
}

/// Evaluates the Nickel expr for an HID, returning the json serialization.
pub fn nickel_to_json_for_hid_report(
    ncl_import_path: String,
    keymap_ncl: &str,
) -> io::Result<String> {
    let mut nickel_command = Command::new("nickel")
        .args([
            "export",
            "--format=json",
            format!("--import-path={}", ncl_import_path).as_ref(),
            "--field=as_bytes",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let child_stdin = nickel_command.stdin.as_mut().unwrap();
    child_stdin.write_all(format!(r#"(import "hid-report.ncl") & ({})"#, keymap_ncl).as_bytes())?;

    let output = nickel_command.wait_with_output()?;

    String::from_utf8(output.stdout).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}
