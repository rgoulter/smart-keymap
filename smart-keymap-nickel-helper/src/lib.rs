use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::{Mutex, OnceLock};

/// Identifies a cached Nickel JSON export (keymap, inputs, or HID report).
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
enum NickelJsonExport {
    Keymap {
        import_path: String,
        keymap_ncl: String,
    },
    Inputs {
        import_path: String,
        keymap_ncl: String,
        inputs_ncl: String,
    },
    HidReport {
        import_path: String,
        hid_report_ncl: String,
    },
}

impl NickelJsonExport {
    fn keymap(import_path: &str, keymap_ncl: &str) -> Self {
        Self::Keymap {
            import_path: import_path.to_owned(),
            keymap_ncl: keymap_ncl.to_owned(),
        }
    }

    fn inputs(import_path: &str, keymap_ncl: &str, inputs_ncl: &str) -> Self {
        Self::Inputs {
            import_path: import_path.to_owned(),
            keymap_ncl: keymap_ncl.to_owned(),
            inputs_ncl: inputs_ncl.to_owned(),
        }
    }

    fn hid_report(import_path: &str, hid_report_ncl: &str) -> Self {
        Self::HidReport {
            import_path: import_path.to_owned(),
            hid_report_ncl: hid_report_ncl.to_owned(),
        }
    }
}

mod eval_cache {
    use super::{HashMap, Mutex, NickelJsonExport, OnceLock};

    fn cache() -> &'static Mutex<HashMap<NickelJsonExport, String>> {
        static CACHE: OnceLock<Mutex<HashMap<NickelJsonExport, String>>> = OnceLock::new();
        CACHE.get_or_init(|| Mutex::new(HashMap::new()))
    }

    pub fn get(key: &NickelJsonExport) -> Option<String> {
        cache().lock().unwrap().get(key).cloned()
    }

    pub fn insert(key: NickelJsonExport, value: String) {
        cache().lock().unwrap().insert(key, value);
    }

    pub fn clear() {
        cache().lock().unwrap().clear();
    }
}

/// Clears the in-process Nickel JSON eval cache (for tests).
pub fn clear_nickel_eval_cache() {
    eval_cache::clear();
}

/// Inputs for Nickel evaluation.
pub struct NickelEvalInputs<'a> {
    /// The Nickel import path to use for the evaluation.
    pub ncl_import_path: &'a str,
    /// Path to a Nickel file to evaluate.
    pub input_path: &'a Path,
}

/// Inputs for Nickel code generation. (e.g. board.rs, keymap.rs).
pub struct CodegenInputs<'a> {
    /// The environment variable to for the codegen input.
    pub env_var: &'a str,
    /// The name of the conditional-compilation flag.
    pub cfg_name: &'a str,
    /// The base name for the custom module. (e.g. "keymap.rs", "board.rs")
    pub module_basename: &'a str,
    /// The Nickel import path to use for the evaluation.
    pub ncl_import_path: &'a str,
    /// The Nickel evaluation function.
    pub nickel_eval_fn: fn(NickelEvalInputs) -> NickelResult,
}

/// Likely reasons why running `nickel` may fail.
pub enum NickelError {
    NickelNotFound,
    EvalError(String),
}

/// Result of Nickel evaluation.
pub type NickelResult = Result<String, NickelError>;

/// Evaluates the Nickel expr for a keymap, returning the keymap.rs contents.
pub fn nickel_keymap_rs_for_keymap_path(
    NickelEvalInputs {
        ncl_import_path,
        input_path,
    }: NickelEvalInputs,
) -> NickelResult {
    let spawn_nickel_result = Command::new("nickel")
        .args([
            "export",
            "--format=raw",
            format!("--import-path={}", ncl_import_path).as_ref(),
            "--field=keymap_rs",
            "keymap-codegen.ncl",
            "keymap-ncl-to-json.ncl",
            input_path.to_str().unwrap(),
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

/// Evaluates the Nickel expr for a keymap, returning the keymap expression.
pub fn nickel_keymap_expr_for_keymap_ncl(ncl_import_path: &str, keymap_ncl: &str) -> NickelResult {
    let spawn_nickel_result = Command::new("nickel")
        .args([
            "export",
            "--format=raw",
            &format!("--import-path={}", ncl_import_path),
            "--field=rust_expressions.keymap",
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
                    format!(
                        r#"(import "keymap-codegen.ncl") & (import "keymap-ncl-to-json.ncl") & ({})"#,
                        keymap_ncl
                    )
                    .as_bytes(),
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

/// Evaluates the Nickel expr for a board, returning the board.rs contents.
pub fn nickel_board_rs_for_board_path(
    NickelEvalInputs {
        ncl_import_path,
        input_path,
    }: NickelEvalInputs,
) -> NickelResult {
    let spawn_nickel_result = Command::new("nickel")
        .args([
            "export",
            "--format=raw",
            format!("--import-path={}", ncl_import_path).as_ref(),
            "--field=board_rs",
            "codegen.ncl",
            input_path.to_str().unwrap(),
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
pub fn nickel_json_value_for_keymap(ncl_import_path: String, keymap_ncl: &str) -> NickelResult {
    let cache_key = NickelJsonExport::keymap(&ncl_import_path, keymap_ncl);
    if let Some(json) = eval_cache::get(&cache_key) {
        return Ok(json);
    }

    let result = nickel_json_value_for_keymap_uncached(&ncl_import_path, keymap_ncl);
    if let Ok(ref json) = result {
        eval_cache::insert(cache_key, json.clone());
    }
    result
}

fn nickel_json_value_for_keymap_uncached(ncl_import_path: &str, keymap_ncl: &str) -> NickelResult {
    let spawn_nickel_result = Command::new("nickel")
        .args([
            "export",
            "--format=json",
            format!("--import-path={ncl_import_path}").as_ref(),
            "--field=json_deserializable_keymap",
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
                    format!(r#"(import "keymap-codegen.ncl") & (import "keymap-ncl-to-json.ncl") & ({keymap_ncl})"#)
                        .as_bytes(),
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

/// Evaluates the Nickel expr for inputs, with a given keymap ncl, returning the json serialization.
pub fn nickel_json_value_for_inputs(
    ncl_import_path: String,
    keymap_ncl: &str,
    inputs_ncl: &str,
) -> NickelResult {
    let cache_key = NickelJsonExport::inputs(&ncl_import_path, keymap_ncl, inputs_ncl);
    if let Some(json) = eval_cache::get(&cache_key) {
        return Ok(json);
    }

    let result = nickel_json_value_for_inputs_uncached(&ncl_import_path, keymap_ncl, inputs_ncl);
    if let Ok(ref json) = result {
        eval_cache::insert(cache_key, json.clone());
    }
    result
}

fn nickel_json_value_for_inputs_uncached(
    ncl_import_path: &str,
    keymap_ncl: &str,
    inputs_ncl: &str,
) -> NickelResult {
    let spawn_nickel_result = Command::new("nickel")
        .args([
            "export",
            "--format=json",
            format!("--import-path={ncl_import_path}").as_ref(),
            "--field=inputs_as_json_value_input_events",
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
                    format!(
                        r#"
                           (import "keymap-codegen.ncl")
                           & (import "keymap-ncl-to-json.ncl")
                           & (import "inputs-to-json.ncl")
                           & ({keymap_ncl})
                           & ({{
                                 inputs =
                                    let K = import "keys.ncl" in
                                    let {{
                                      press,
                                      press_keymap_index,
                                      release,
                                      release_keymap_index,
                                      tap,
                                      tap_keymap_index,
                                      wait,
                                      ..
                                    }} = import "inputs.ncl" in
                                    {inputs_ncl},
                              }})
                        "#,
                    )
                    .as_bytes(),
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
    hid_report_ncl: &str,
) -> io::Result<String> {
    let cache_key = NickelJsonExport::hid_report(&ncl_import_path, hid_report_ncl);
    if let Some(json) = eval_cache::get(&cache_key) {
        return Ok(json);
    }

    let json = nickel_to_json_for_hid_report_uncached(&ncl_import_path, hid_report_ncl)?;
    eval_cache::insert(cache_key, json.clone());
    Ok(json)
}

fn nickel_to_json_for_hid_report_uncached(
    ncl_import_path: &str,
    hid_report_ncl: &str,
) -> io::Result<String> {
    let mut nickel_command = Command::new("nickel")
        .args([
            "export",
            "--format=json",
            format!("--import-path={ncl_import_path}").as_ref(),
            "--field=as_bytes",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let child_stdin = nickel_command.stdin.as_mut().unwrap();
    child_stdin.write_all(
        format!(
            r#"
                (import "hid-report.ncl")
                & (
                    let K = import "hid-usage-keyboard.ncl" in
                    {hid_report_ncl}
                )
            "#,
        )
        .as_bytes(),
    )?;

    let output = nickel_command.wait_with_output()?;

    String::from_utf8(output.stdout).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
}

/// Emits the full-profile composite `key_system` module with Vec storage.
///
/// Used by the library `build.rs` under `feature = "std"` (cucumber / runtime serde).
/// Source of truth: `ncl/key_system/` (merge full profile + vec data,
/// then `composite.system.rust_mod`).
pub fn nickel_composite_full_vec_rs(ncl_import_path: &str) -> NickelResult {
    let spawn_nickel_result = Command::new("nickel")
        .args([
            "export",
            "--format=raw",
            format!("--import-path={ncl_import_path}").as_ref(),
            "--field=composite.system.rust_mod",
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
                    br#"
  (import "keymap-codegen.ncl")
  & { composite, composite.profile = composite.full_profile }
  & { composite.data = 'Vec }
"#,
                )
                .unwrap();

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

/// Generates the code for the given module.
pub fn codegen_rust_module(
    CodegenInputs {
        env_var,
        cfg_name,
        module_basename,
        ncl_import_path,
        nickel_eval_fn,
    }: CodegenInputs,
) {
    println!("cargo:rerun-if-env-changed={}", env_var);
    println!("cargo::rustc-check-cfg=cfg({})", cfg_name);
    if let Some(custom_module_path) = env::var(env_var).ok().filter(|s| !s.is_empty()) {
        let out_dir = env::var("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join(module_basename);
        println!("cargo:rerun-if-changed={}", dest_path.to_str().unwrap());

        if custom_module_path.ends_with(".rs") {
            println!("cargo:rustc-cfg={}", cfg_name);

            // Copy the custom module file to the output directory
            fs::copy(custom_module_path, &dest_path).unwrap();
        } else if custom_module_path.ends_with(".ncl") {
            println!("cargo:rustc-cfg={}", cfg_name);

            // Evaluate the custom keymap file with Nickel
            let input_path = Path::new(&custom_module_path);
            match nickel_eval_fn(NickelEvalInputs {
                ncl_import_path,
                input_path,
            }) {
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
            panic!("Unsupported {}: {}", env_var, custom_module_path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{clear_nickel_eval_cache, eval_cache, NickelJsonExport};

    #[test]
    fn nickel_json_export_distinguishes_eval_kinds() {
        let keymap = || NickelJsonExport::keymap("/ncl", "{ keys = [] }");
        assert_eq!(keymap(), keymap());
        assert_ne!(
            keymap(),
            NickelJsonExport::inputs("/ncl", "{ keys = [] }", "[]")
        );
    }

    #[test]
    fn clear_nickel_eval_cache_empties_entries() {
        let key = NickelJsonExport::keymap("/ncl", "{ keys = [] }");
        eval_cache::insert(key, "value".into());
        clear_nickel_eval_cache();
        assert!(eval_cache::get(&NickelJsonExport::keymap("/ncl", "{ keys = [] }")).is_none());
    }
}
