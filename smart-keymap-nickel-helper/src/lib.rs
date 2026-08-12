use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::{Child, Command, Output, Stdio};
use std::sync::{Mutex, OnceLock};
use std::thread;
use std::time::{Duration, Instant};

/// Environment variable controlling Nickel wall-clock timeout (seconds).
///
/// - unset → default ([`DEFAULT_NICKEL_TIMEOUT_SECS`])
/// - `0` → no timeout
/// - positive integer → timeout in seconds
/// - unparsable → default
pub const NICKEL_TIMEOUT_ENV: &str = "SMART_KEYMAP_NICKEL_TIMEOUT_SECS";

/// Default Nickel subprocess timeout when [`NICKEL_TIMEOUT_ENV`] is unset.
pub const DEFAULT_NICKEL_TIMEOUT_SECS: u64 = 60;

mod disk_cache;

pub use disk_cache::{NICKEL_JSON_CACHE_DIR_ENV, NICKEL_JSON_CACHE_ENV, NICKEL_JSON_CACHE_LOG_ENV};

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

/// RAM cache, then optional content-addressed disk cache, then `eval`.
///
/// Successful JSON is stored in RAM and (when enabled) on disk. Errors are not.
fn get_or_eval_json(key: NickelJsonExport, eval: impl FnOnce() -> NickelResult) -> NickelResult {
    if let Some(json) = eval_cache::get(&key) {
        return Ok(json);
    }

    if let Some(cache_dir) = disk_cache::configured_cache_dir() {
        let digest = disk_cache::content_digest_for_key(&key);
        if let Some(json) = disk_cache::read_entry(&cache_dir, &digest) {
            eval_cache::insert(key, json.clone());
            return Ok(json);
        }
        disk_cache::log_miss(&digest);

        let result = eval();
        if let Ok(ref json) = result {
            eval_cache::insert(key, json.clone());
            // Best-effort: a failed disk write must not fail the export.
            let _ = disk_cache::write_entry_atomic(&cache_dir, &digest, json);
        }
        return result;
    }

    let result = eval();
    if let Ok(ref json) = result {
        eval_cache::insert(key, json.clone());
    }
    result
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
#[derive(Debug)]
pub enum NickelError {
    NickelNotFound,
    EvalError(String),
    /// Nickel did not finish within the configured wall-clock limit.
    Timeout {
        timeout_secs: u64,
    },
}

/// Result of Nickel evaluation.
pub type NickelResult = Result<String, NickelError>;

/// Wall-clock timeout for Nickel subprocesses.
///
/// Controlled by [`NICKEL_TIMEOUT_ENV`] (`SMART_KEYMAP_NICKEL_TIMEOUT_SECS`):
/// unset → [`DEFAULT_NICKEL_TIMEOUT_SECS`]; `0` → no timeout; positive → seconds.
pub fn nickel_timeout() -> Option<Duration> {
    match env::var(NICKEL_TIMEOUT_ENV) {
        Ok(raw) => {
            let raw = raw.trim();
            if raw.is_empty() {
                return Some(Duration::from_secs(DEFAULT_NICKEL_TIMEOUT_SECS));
            }
            match raw.parse::<u64>() {
                Ok(0) => None,
                Ok(secs) => Some(Duration::from_secs(secs)),
                Err(_) => Some(Duration::from_secs(DEFAULT_NICKEL_TIMEOUT_SECS)),
            }
        }
        Err(_) => Some(Duration::from_secs(DEFAULT_NICKEL_TIMEOUT_SECS)),
    }
}

/// Wait for a child, optionally killing it after `timeout`.
///
/// Stdout/stderr are drained on background threads so a full pipe buffer cannot
/// deadlock the wait.
fn wait_with_optional_timeout(
    mut child: Child,
    timeout: Option<Duration>,
) -> Result<Output, NickelError> {
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let stdout_handle = thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(mut reader) = stdout {
            let _ = reader.read_to_end(&mut buf);
        }
        buf
    });
    let stderr_handle = thread::spawn(move || {
        let mut buf = Vec::new();
        if let Some(mut reader) = stderr {
            let _ = reader.read_to_end(&mut buf);
        }
        buf
    });

    let status = match timeout {
        None => child
            .wait()
            .unwrap_or_else(|e| panic!("Failed to wait on nickel: {:?}", e)),
        Some(limit) => {
            let start = Instant::now();
            loop {
                match child.try_wait() {
                    Ok(Some(status)) => break status,
                    Ok(None) => {
                        if start.elapsed() >= limit {
                            let _ = child.kill();
                            let _ = child.wait();
                            let _ = stdout_handle.join();
                            let _ = stderr_handle.join();
                            return Err(NickelError::Timeout {
                                timeout_secs: limit.as_secs().max(1),
                            });
                        }
                        thread::sleep(Duration::from_millis(20));
                    }
                    Err(e) => panic!("Failed to wait on nickel: {:?}", e),
                }
            }
        }
    };

    let stdout = stdout_handle
        .join()
        .unwrap_or_else(|_| panic!("nickel stdout reader panicked"));
    let stderr = stderr_handle
        .join()
        .unwrap_or_else(|_| panic!("nickel stderr reader panicked"));

    Ok(Output {
        status,
        stdout,
        stderr,
    })
}

/// Spawn `nickel` with the given args and optional stdin, applying the configured timeout.
fn run_nickel(args: &[&str], stdin_bytes: Option<&[u8]>) -> NickelResult {
    let mut command = Command::new("nickel");
    command
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    if stdin_bytes.is_some() {
        command.stdin(Stdio::piped());
    } else {
        command.stdin(Stdio::null());
    }

    let mut child = command.spawn().map_err(|e| match e.kind() {
        io::ErrorKind::NotFound => NickelError::NickelNotFound,
        _ => panic!("Failed to spawn nickel: {:?}", e),
    })?;

    if let Some(bytes) = stdin_bytes {
        let mut child_stdin = child
            .stdin
            .take()
            .unwrap_or_else(|| panic!("nickel stdin not piped"));
        child_stdin
            .write_all(bytes)
            .unwrap_or_else(|e| panic!("Failed to write to nickel stdin: {:?}", e));
        // Drop stdin so Nickel sees EOF.
    }

    let output = wait_with_optional_timeout(child, nickel_timeout())?;

    if output.status.success() {
        String::from_utf8(output.stdout).map_err(|e| panic!("Failed to decode UTF-8: {:?}", e))
    } else {
        let nickel_error_message = String::from_utf8(output.stderr)
            .unwrap_or_else(|e| panic!("Failed to decode UTF-8: {:?}", e));
        Err(NickelError::EvalError(nickel_error_message))
    }
}

/// Evaluates the Nickel expr for a keymap, returning the keymap.rs contents.
pub fn nickel_keymap_rs_for_keymap_path(
    NickelEvalInputs {
        ncl_import_path,
        input_path,
    }: NickelEvalInputs,
) -> NickelResult {
    let import_path_arg = format!("--import-path={}", ncl_import_path);
    run_nickel(
        &[
            "export",
            "--format=raw",
            import_path_arg.as_str(),
            "--field=keymap_rs",
            "keymap-codegen.ncl",
            "keymap-ncl-to-json.ncl",
            input_path.to_str().unwrap(),
        ],
        None,
    )
}

/// Evaluates the Nickel expr for a keymap, returning the keymap expression.
pub fn nickel_keymap_expr_for_keymap_ncl(ncl_import_path: &str, keymap_ncl: &str) -> NickelResult {
    let import_path_arg = format!("--import-path={}", ncl_import_path);
    let stdin = format!(
        r#"(import "keymap-codegen.ncl") & (import "keymap-ncl-to-json.ncl") & ({})"#,
        keymap_ncl
    );
    run_nickel(
        &[
            "export",
            "--format=raw",
            import_path_arg.as_str(),
            "--field=rust_expressions.keymap",
        ],
        Some(stdin.as_bytes()),
    )
}

/// Evaluates the Nickel expr for a board, returning the board.rs contents.
pub fn nickel_board_rs_for_board_path(
    NickelEvalInputs {
        ncl_import_path,
        input_path,
    }: NickelEvalInputs,
) -> NickelResult {
    let import_path_arg = format!("--import-path={}", ncl_import_path);
    run_nickel(
        &[
            "export",
            "--format=raw",
            import_path_arg.as_str(),
            "--field=board_rs",
            "codegen.ncl",
            input_path.to_str().unwrap(),
        ],
        None,
    )
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
    get_or_eval_json(cache_key, || {
        nickel_json_value_for_keymap_uncached(&ncl_import_path, keymap_ncl)
    })
}

fn nickel_json_value_for_keymap_uncached(ncl_import_path: &str, keymap_ncl: &str) -> NickelResult {
    let import_path_arg = format!("--import-path={ncl_import_path}");
    let stdin = format!(
        r#"(import "keymap-codegen.ncl") & (import "keymap-ncl-to-json.ncl") & ({keymap_ncl})"#
    );
    run_nickel(
        &[
            "export",
            "--format=json",
            import_path_arg.as_str(),
            "--field=json_deserializable_keymap",
        ],
        Some(stdin.as_bytes()),
    )
}

/// Evaluates the Nickel expr for inputs, with a given keymap ncl, returning the json serialization.
pub fn nickel_json_value_for_inputs(
    ncl_import_path: String,
    keymap_ncl: &str,
    inputs_ncl: &str,
) -> NickelResult {
    let cache_key = NickelJsonExport::inputs(&ncl_import_path, keymap_ncl, inputs_ncl);
    get_or_eval_json(cache_key, || {
        nickel_json_value_for_inputs_uncached(&ncl_import_path, keymap_ncl, inputs_ncl)
    })
}

fn nickel_json_value_for_inputs_uncached(
    ncl_import_path: &str,
    keymap_ncl: &str,
    inputs_ncl: &str,
) -> NickelResult {
    let import_path_arg = format!("--import-path={ncl_import_path}");
    let stdin = format!(
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
    );
    run_nickel(
        &[
            "export",
            "--format=json",
            import_path_arg.as_str(),
            "--field=inputs_as_json_value_input_events",
        ],
        Some(stdin.as_bytes()),
    )
}

/// Evaluates the Nickel expr for an HID, returning the json serialization.
pub fn nickel_to_json_for_hid_report(
    ncl_import_path: String,
    hid_report_ncl: &str,
) -> NickelResult {
    let cache_key = NickelJsonExport::hid_report(&ncl_import_path, hid_report_ncl);
    get_or_eval_json(cache_key, || {
        nickel_to_json_for_hid_report_uncached(&ncl_import_path, hid_report_ncl)
    })
}

fn nickel_to_json_for_hid_report_uncached(
    ncl_import_path: &str,
    hid_report_ncl: &str,
) -> NickelResult {
    let import_path_arg = format!("--import-path={ncl_import_path}");
    let stdin = format!(
        r#"
                (import "hid-report.ncl")
                & (
                    let K = import "hid-usage-keyboard.ncl" in
                    {hid_report_ncl}
                )
            "#,
    );
    run_nickel(
        &[
            "export",
            "--format=json",
            import_path_arg.as_str(),
            "--field=as_bytes",
        ],
        Some(stdin.as_bytes()),
    )
}

/// Emits the full-profile composite `key_system` module with Vec storage.
///
/// Used by the `smart-keymap-full-system-std` package build script (cucumber /
/// std harnesses). Source of truth: `ncl/key_system/` (merge full profile +
/// vec data, then `composite.system.rust_mod`).
///
/// Generated code assumes a nested-shell host: include under a parent module
/// that defines size consts (referenced as `super::…`), and resolve engine
/// paths via the `smart_keymap` crate name.
pub fn nickel_composite_full_vec_rs(ncl_import_path: &str) -> NickelResult {
    let import_path_arg = format!("--import-path={ncl_import_path}");
    run_nickel(
        &[
            "export",
            "--format=raw",
            import_path_arg.as_str(),
            "--field=composite.system.rust_mod",
        ],
        Some(
            br#"
  (import "keymap-codegen.ncl")
  & { composite, composite.profile = 'FullProfile }
  & { composite.data = 'Vec }
"#,
        ),
    )
}

/// Generates the code for the given module.
///
/// Cargo invalidation edges are:
/// - the env value path itself (`SMART_KEYMAP_CUSTOM_KEYMAP` / board env)
/// - for `.ncl` inputs, the Nickel import tree used by codegen
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
    println!("cargo:rerun-if-env-changed={}", NICKEL_TIMEOUT_ENV);
    println!("cargo::rustc-check-cfg=cfg({})", cfg_name);
    if let Some(custom_module_path) = env::var(env_var).ok().filter(|s| !s.is_empty()) {
        let out_dir = env::var("OUT_DIR").unwrap();
        let dest_path = Path::new(&out_dir).join(module_basename);

        // Input edges — source path (and ncl tree for Nickel eval), not OUT_DIR.
        println!("cargo:rerun-if-changed={}", custom_module_path);
        if custom_module_path.ends_with(".ncl") {
            // Coarse: codegen pulls keymap-codegen.ncl, smart_keys, etc.
            println!("cargo:rerun-if-changed={}", ncl_import_path);
        }

        if custom_module_path.ends_with(".rs") {
            println!("cargo:rustc-cfg={}", cfg_name);

            // Copy the custom module file to the output directory
            fs::copy(&custom_module_path, &dest_path).unwrap();
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
                Err(NickelError::Timeout { timeout_secs }) => {
                    panic!(
                        "Nickel evaluation timed out after {}s (set {}=0 to disable, or raise the limit)",
                        timeout_secs, NICKEL_TIMEOUT_ENV
                    );
                }
            }
        } else {
            panic!("Unsupported {}: {}", env_var, custom_module_path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        clear_nickel_eval_cache, disk_cache, eval_cache, get_or_eval_json, nickel_timeout,
        wait_with_optional_timeout, NickelError, NickelJsonExport, DEFAULT_NICKEL_TIMEOUT_SECS,
        NICKEL_TIMEOUT_ENV,
    };
    use std::process::{Command, Stdio};
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::{Duration, Instant};

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

    #[test]
    fn nickel_timeout_env_parsing() {
        // Serialize env mutations: cargo runs lib tests in parallel by default.
        static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _guard = ENV_LOCK.lock().unwrap();

        let prev = std::env::var_os(NICKEL_TIMEOUT_ENV);

        std::env::remove_var(NICKEL_TIMEOUT_ENV);
        assert_eq!(
            nickel_timeout(),
            Some(Duration::from_secs(DEFAULT_NICKEL_TIMEOUT_SECS))
        );

        std::env::set_var(NICKEL_TIMEOUT_ENV, "0");
        assert_eq!(nickel_timeout(), None);

        std::env::set_var(NICKEL_TIMEOUT_ENV, "12");
        assert_eq!(nickel_timeout(), Some(Duration::from_secs(12)));

        std::env::set_var(NICKEL_TIMEOUT_ENV, "not-a-number");
        assert_eq!(
            nickel_timeout(),
            Some(Duration::from_secs(DEFAULT_NICKEL_TIMEOUT_SECS))
        );

        match prev {
            Some(v) => std::env::set_var(NICKEL_TIMEOUT_ENV, v),
            None => std::env::remove_var(NICKEL_TIMEOUT_ENV),
        }
    }

    #[test]
    fn wait_timeout_kills_long_running_child() {
        let child = Command::new("sleep")
            .arg("30")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .expect("spawn sleep");

        let start = Instant::now();
        let err = wait_with_optional_timeout(child, Some(Duration::from_millis(200)))
            .expect_err("expected timeout");
        let elapsed = start.elapsed();

        match err {
            NickelError::Timeout { timeout_secs } => {
                // Sub-second limits report at least 1s in the error.
                assert_eq!(timeout_secs, 1);
            }
            other => panic!("unexpected error: {:?}", other),
        }
        assert!(
            elapsed < Duration::from_secs(5),
            "kill path should return promptly, elapsed {:?}",
            elapsed
        );
    }

    fn temp_cache_and_ncl(label: &str) -> (std::path::PathBuf, String) {
        let dir = std::env::temp_dir().join(format!(
            "sk-ncl-goe-{}-{}-{}",
            label,
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let ncl_root = dir.join("ncl");
        std::fs::create_dir_all(&ncl_root).unwrap();
        std::fs::write(ncl_root.join("x.ncl"), "1").unwrap();
        let ncl_path = ncl_root.to_str().unwrap().to_owned();
        (dir, ncl_path)
    }

    fn restore_cache_env(
        prev_dir: Option<std::ffi::OsString>,
        prev_mode: Option<std::ffi::OsString>,
    ) {
        match prev_dir {
            Some(v) => std::env::set_var(disk_cache::NICKEL_JSON_CACHE_DIR_ENV, v),
            None => std::env::remove_var(disk_cache::NICKEL_JSON_CACHE_DIR_ENV),
        }
        match prev_mode {
            Some(v) => std::env::set_var(disk_cache::NICKEL_JSON_CACHE_ENV, v),
            None => std::env::remove_var(disk_cache::NICKEL_JSON_CACHE_ENV),
        }
        clear_nickel_eval_cache();
    }

    /// Live Nickel: cold export fills disk; second process-clear (RAM only)
    /// should hit disk. Requires `nickel` and workspace `ncl/` cwd layout.
    ///
    /// ```text
    /// cargo test -p smart-keymap-nickel-helper --lib live_keymap_disk_cache -- --ignored --nocapture
    /// ```
    #[test]
    #[ignore = "requires nickel + ncl/; run manually for smoke timing"]
    fn live_keymap_disk_cache_cold_then_warm() {
        use super::nickel_json_value_for_keymap;

        let _g = disk_cache::test_env_lock();
        let nickel_ok = Command::new("nickel")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        assert!(nickel_ok, "nickel not available");

        let ncl = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../ncl");
        assert!(ncl.is_dir(), "expected ncl next to crate: {:?}", ncl);
        let ncl = ncl.canonicalize().unwrap();
        let ncl_s = ncl.to_str().unwrap().to_owned();

        let cache_dir = std::env::temp_dir().join(format!(
            "sk-ncl-live-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        std::fs::create_dir_all(&cache_dir).unwrap();

        let prev_dir = std::env::var_os(disk_cache::NICKEL_JSON_CACHE_DIR_ENV);
        let prev_mode = std::env::var_os(disk_cache::NICKEL_JSON_CACHE_ENV);
        let prev_log = std::env::var_os(disk_cache::NICKEL_JSON_CACHE_LOG_ENV);
        std::env::set_var(disk_cache::NICKEL_JSON_CACHE_DIR_ENV, &cache_dir);
        std::env::remove_var(disk_cache::NICKEL_JSON_CACHE_ENV);
        std::env::set_var(disk_cache::NICKEL_JSON_CACHE_LOG_ENV, "1");

        // Minimal valid keymap docstring (matches feature style).
        let keymap_ncl = r#"
            let K = import "keys.ncl" in
            { keys = [ K.A, K.B ] }
        "#;

        clear_nickel_eval_cache();
        let t0 = Instant::now();
        let json1 = nickel_json_value_for_keymap(ncl_s.clone(), keymap_ncl)
            .unwrap_or_else(|e| panic!("cold eval failed: {:?}", e));
        let cold = t0.elapsed();
        assert!(
            json1.contains('['),
            "expected JSON array-ish keymap: {json1}"
        );

        clear_nickel_eval_cache(); // force disk path, not RAM
        let t1 = Instant::now();
        let json2 = nickel_json_value_for_keymap(ncl_s, keymap_ncl)
            .unwrap_or_else(|e| panic!("warm eval failed: {:?}", e));
        let warm = t1.elapsed();
        assert_eq!(json1, json2);
        eprintln!("live nickel-json disk cache: cold={cold:?} warm={warm:?}");
        assert!(
            warm < cold || warm.as_millis() < 50,
            "warm path should beat cold Nickel (cold={cold:?}, warm={warm:?})"
        );

        match prev_log {
            Some(v) => std::env::set_var(disk_cache::NICKEL_JSON_CACHE_LOG_ENV, v),
            None => std::env::remove_var(disk_cache::NICKEL_JSON_CACHE_LOG_ENV),
        }
        restore_cache_env(prev_dir, prev_mode);
        let _ = std::fs::remove_dir_all(&cache_dir);
    }

    #[test]
    fn get_or_eval_disk_hit_skips_eval() {
        let _g = disk_cache::test_env_lock();
        let (dir, ncl_path) = temp_cache_and_ncl("hit");

        let prev_dir = std::env::var_os(disk_cache::NICKEL_JSON_CACHE_DIR_ENV);
        let prev_mode = std::env::var_os(disk_cache::NICKEL_JSON_CACHE_ENV);
        std::env::set_var(disk_cache::NICKEL_JSON_CACHE_DIR_ENV, &dir);
        std::env::remove_var(disk_cache::NICKEL_JSON_CACHE_ENV);

        let key = NickelJsonExport::keymap(&ncl_path, "{ keys = [] }");
        let digest = disk_cache::content_digest_for_key(&key);
        disk_cache::write_entry_atomic(&dir, &digest, r#"{"from":"disk"}"#).unwrap();

        clear_nickel_eval_cache();
        let calls = AtomicUsize::new(0);
        let got = get_or_eval_json(key, || {
            calls.fetch_add(1, Ordering::SeqCst);
            panic!("eval must not run on disk hit");
        })
        .unwrap();
        assert_eq!(got, r#"{"from":"disk"}"#);
        assert_eq!(calls.load(Ordering::SeqCst), 0);

        // RAM should now serve without disk/eval.
        let got2 = get_or_eval_json(NickelJsonExport::keymap(&ncl_path, "{ keys = [] }"), || {
            panic!("eval must not run on ram hit");
        })
        .unwrap();
        assert_eq!(got2, r#"{"from":"disk"}"#);

        restore_cache_env(prev_dir, prev_mode);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn get_or_eval_does_not_store_errors() {
        let _g = disk_cache::test_env_lock();
        let (dir, ncl_path) = temp_cache_and_ncl("err");

        let prev_dir = std::env::var_os(disk_cache::NICKEL_JSON_CACHE_DIR_ENV);
        let prev_mode = std::env::var_os(disk_cache::NICKEL_JSON_CACHE_ENV);
        std::env::set_var(disk_cache::NICKEL_JSON_CACHE_DIR_ENV, &dir);
        std::env::remove_var(disk_cache::NICKEL_JSON_CACHE_ENV);
        clear_nickel_eval_cache();

        let key = NickelJsonExport::keymap(&ncl_path, "{ keys = [error] }");
        let digest = disk_cache::content_digest_for_key(&key);
        let calls = AtomicUsize::new(0);

        let err = get_or_eval_json(key.clone(), || {
            calls.fetch_add(1, Ordering::SeqCst);
            Err(NickelError::EvalError("boom".into()))
        })
        .expect_err("error");
        match err {
            NickelError::EvalError(m) => assert_eq!(m, "boom"),
            other => panic!("unexpected: {:?}", other),
        }
        assert!(disk_cache::read_entry(&dir, &digest).is_none());
        assert_eq!(calls.load(Ordering::SeqCst), 1);

        // Second call still invokes eval (nothing cached).
        let _ = get_or_eval_json(key, || {
            calls.fetch_add(1, Ordering::SeqCst);
            Err(NickelError::EvalError("boom".into()))
        });
        assert_eq!(calls.load(Ordering::SeqCst), 2);

        restore_cache_env(prev_dir, prev_mode);
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn get_or_eval_stores_ok_on_disk() {
        let _g = disk_cache::test_env_lock();
        let (dir, ncl_path) = temp_cache_and_ncl("store");

        let prev_dir = std::env::var_os(disk_cache::NICKEL_JSON_CACHE_DIR_ENV);
        let prev_mode = std::env::var_os(disk_cache::NICKEL_JSON_CACHE_ENV);
        std::env::set_var(disk_cache::NICKEL_JSON_CACHE_DIR_ENV, &dir);
        std::env::remove_var(disk_cache::NICKEL_JSON_CACHE_ENV);
        clear_nickel_eval_cache();

        let key = NickelJsonExport::keymap(&ncl_path, "{ keys = [ok] }");
        let digest = disk_cache::content_digest_for_key(&key);
        let calls = AtomicUsize::new(0);

        let got = get_or_eval_json(key.clone(), || {
            calls.fetch_add(1, Ordering::SeqCst);
            Ok(r#"{"ok":1}"#.into())
        })
        .unwrap();
        assert_eq!(got, r#"{"ok":1}"#);
        assert_eq!(calls.load(Ordering::SeqCst), 1);
        assert_eq!(
            disk_cache::read_entry(&dir, &digest).as_deref(),
            Some(r#"{"ok":1}"#)
        );

        clear_nickel_eval_cache();
        let got2 = get_or_eval_json(key, || {
            calls.fetch_add(1, Ordering::SeqCst);
            panic!("should hit disk");
        })
        .unwrap();
        assert_eq!(got2, r#"{"ok":1}"#);
        assert_eq!(calls.load(Ordering::SeqCst), 1);

        restore_cache_env(prev_dir, prev_mode);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
