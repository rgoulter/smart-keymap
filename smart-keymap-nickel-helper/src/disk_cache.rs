//! Content-addressed on-disk cache for Nickel → JSON exports.
//!
//! ## Functional core / imperative shell
//!
//! - **Core** ([`content_digest`], [`entry_rel_path`], [`parse_disk_cache_mode`],
//!   [`parse_cache_log_enabled`]): pure; no filesystem, no process, no env.
//! - **Shell** ([`ncl_tree_digest`], [`read_entry`], [`write_entry_atomic`],
//!   [`configured_cache_dir`], [`nickel_version_string`]): I/O and process.
//!
//! Entries are only written for successful JSON exports. Errors/timeouts are
//! never stored. Cache key includes export kind, Nickel version, docstring
//! bodies, and a digest of the entire `ncl` import tree (coarse invalidation).

use std::env;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::{Mutex, OnceLock};

use sha2::{Digest, Sha256};

use crate::NickelJsonExport;

fn sha256_hash(data: &[u8]) -> [u8; 32] {
    Sha256::digest(data).into()
}

/// Directory layout version prefix under the cache root.
pub const CACHE_LAYOUT_VERSION: &str = "v1";

/// Domain separator baked into digests (algorithm / layout changes bump this).
const DIGEST_DOMAIN: &[u8] = b"smart-keymap-nickel-json-cache\0v1\0";

/// Env: `0` / `false` / `off` / `no` / `disabled` disables disk cache;
/// unset or other values enable.
pub const NICKEL_JSON_CACHE_ENV: &str = "SMART_KEYMAP_NICKEL_JSON_CACHE";

/// Env: absolute or relative path for the cache root directory.
pub const NICKEL_JSON_CACHE_DIR_ENV: &str = "SMART_KEYMAP_NICKEL_JSON_CACHE_DIR";

/// Env: `1` / `true` / `yes` / `on` / `enabled` logs hit/miss/store to stderr.
pub const NICKEL_JSON_CACHE_LOG_ENV: &str = "SMART_KEYMAP_NICKEL_JSON_CACHE_LOG";

// ---------------------------------------------------------------------------
// Functional core
// ---------------------------------------------------------------------------

/// Shared truthy values for boolean-ish env flags (`1`, `true`, `yes`, `on`, `enabled`).
fn is_env_true(s: &str) -> bool {
    s.eq_ignore_ascii_case("1")
        || s.eq_ignore_ascii_case("true")
        || s.eq_ignore_ascii_case("yes")
        || s.eq_ignore_ascii_case("on")
        || s.eq_ignore_ascii_case("enabled")
}

/// Shared falsy values for boolean-ish env flags (`0`, `false`, `no`, `off`, `disabled`).
fn is_env_false(s: &str) -> bool {
    s.eq_ignore_ascii_case("0")
        || s.eq_ignore_ascii_case("false")
        || s.eq_ignore_ascii_case("no")
        || s.eq_ignore_ascii_case("off")
        || s.eq_ignore_ascii_case("disabled")
}

/// Whether the disk layer should be used (pure parse of the enable flag).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DiskCacheMode {
    Enabled,
    Disabled,
}

/// Parse `SMART_KEYMAP_NICKEL_JSON_CACHE` (value only; presence of dir is separate).
///
/// - missing / empty → enabled (default on)
/// - falsy (`0` / `false` / `off` / `no` / `disabled`) → disabled
/// - anything else (including truthy) → enabled (forward-compatible)
pub fn parse_disk_cache_mode(raw: Option<&str>) -> DiskCacheMode {
    match raw.map(str::trim) {
        None | Some("") => DiskCacheMode::Enabled,
        Some(s) if is_env_false(s) => DiskCacheMode::Disabled,
        Some(_) => DiskCacheMode::Enabled,
    }
}

/// Parse `SMART_KEYMAP_NICKEL_JSON_CACHE_LOG` (default off).
///
/// - missing / empty → disabled
/// - truthy (`1` / `true` / `yes` / `on` / `enabled`) → enabled
/// - anything else → disabled
pub fn parse_cache_log_enabled(raw: Option<&str>) -> bool {
    match raw.map(str::trim) {
        None | Some("") => false,
        Some(s) => is_env_true(s),
    }
}

/// Field name passed to `nickel export --field=…` for this export kind.
pub fn export_field_name(key: &NickelJsonExport) -> &'static str {
    match key {
        NickelJsonExport::Keymap { .. } => "json_deserializable_keymap",
        NickelJsonExport::Inputs { .. } => "inputs_as_json_value_input_events",
        NickelJsonExport::HidReport { .. } => "as_bytes",
    }
}

/// Kind tag included in the digest (distinct from field name for stability).
fn kind_tag(key: &NickelJsonExport) -> &'static str {
    match key {
        NickelJsonExport::Keymap { .. } => "keymap",
        NickelJsonExport::Inputs { .. } => "inputs",
        NickelJsonExport::HidReport { .. } => "hid",
    }
}

/// Length-prefixed UTF-8 chunk for domain-separated hashing.
fn push_lp(buf: &mut Vec<u8>, s: &str) {
    let bytes = s.as_bytes();
    buf.extend_from_slice(&(bytes.len() as u64).to_le_bytes());
    buf.extend_from_slice(bytes);
}

/// Pure content digest for a JSON export cache entry.
///
/// Inputs:
/// - `key` — export kind + docstring bodies (not import path string)
/// - `nickel_version` — `nickel --version` line
/// - `ncl_tree_digest` — hash of files under the import root (relative paths)
///
/// Import path absolute strings are intentionally omitted so the same tree +
/// docstrings share digests across machines/CI checkouts.
pub fn content_digest(
    key: &NickelJsonExport,
    nickel_version: &str,
    ncl_tree_digest: &[u8; 32],
) -> [u8; 32] {
    let mut buf = Vec::with_capacity(256);
    buf.extend_from_slice(DIGEST_DOMAIN);
    push_lp(&mut buf, kind_tag(key));
    push_lp(&mut buf, export_field_name(key));
    push_lp(&mut buf, "json");
    push_lp(&mut buf, nickel_version);
    buf.extend_from_slice(ncl_tree_digest);

    match key {
        NickelJsonExport::Keymap { keymap_ncl, .. } => {
            push_lp(&mut buf, keymap_ncl);
        }
        NickelJsonExport::Inputs {
            keymap_ncl,
            inputs_ncl,
            ..
        } => {
            push_lp(&mut buf, keymap_ncl);
            push_lp(&mut buf, inputs_ncl);
        }
        NickelJsonExport::HidReport { hid_report_ncl, .. } => {
            push_lp(&mut buf, hid_report_ncl);
        }
    }

    sha256_hash(&buf)
}

/// Hex-encode a 32-byte digest (lowercase).
pub fn digest_hex(digest: &[u8; 32]) -> String {
    let mut s = String::with_capacity(64);
    for b in digest {
        s.push_str(&format!("{b:02x}"));
    }
    s
}

/// Relative path under the cache root: `v1/ab/abcd…ef.json`.
pub fn entry_rel_path(hex: &str) -> PathBuf {
    debug_assert_eq!(hex.len(), 64);
    let prefix = &hex[..2];
    PathBuf::from(CACHE_LAYOUT_VERSION)
        .join(prefix)
        .join(format!("{hex}.json"))
}

// ---------------------------------------------------------------------------
// Imperative shell
// ---------------------------------------------------------------------------

/// Resolve cache directory from env, or `None` if disk cache is disabled.
///
/// Default root: `$CARGO_TARGET_DIR/nickel-json-cache` if set, else
/// `target/nickel-json-cache` relative to the process cwd (workspace root for
/// normal `cargo test` / cucumber).
pub fn configured_cache_dir() -> Option<PathBuf> {
    let mode = parse_disk_cache_mode(env::var(NICKEL_JSON_CACHE_ENV).ok().as_deref());
    if mode == DiskCacheMode::Disabled {
        return None;
    }
    if let Ok(dir) = env::var(NICKEL_JSON_CACHE_DIR_ENV) {
        let dir = dir.trim();
        if !dir.is_empty() {
            return Some(PathBuf::from(dir));
        }
    }
    if let Ok(target) = env::var("CARGO_TARGET_DIR") {
        return Some(Path::new(&target).join("nickel-json-cache"));
    }
    Some(PathBuf::from("target/nickel-json-cache"))
}

fn cache_log_enabled() -> bool {
    parse_cache_log_enabled(env::var(NICKEL_JSON_CACHE_LOG_ENV).ok().as_deref())
}

fn log_cache(event: &str, hex: &str) {
    if cache_log_enabled() {
        eprintln!("smart-keymap nickel-json-cache {event} {hex}");
    }
}

/// Log a disk miss when [`NICKEL_JSON_CACHE_LOG_ENV`] is enabled.
pub(crate) fn log_miss(digest: &[u8; 32]) {
    log_cache("miss", &digest_hex(digest));
}

/// `nickel --version` stdout (trimmed), memoized per process.
pub fn nickel_version_string() -> String {
    static VERSION: OnceLock<String> = OnceLock::new();
    VERSION
        .get_or_init(|| {
            Command::new("nickel")
                .arg("--version")
                .output()
                .ok()
                .and_then(|o| {
                    if o.status.success() {
                        String::from_utf8(o.stdout).ok()
                    } else {
                        None
                    }
                })
                .map(|s| s.trim().to_owned())
                .unwrap_or_else(|| "nickel-unavailable".to_owned())
        })
        .clone()
}

/// Digest of all files under `ncl_import_path` (sorted relative paths + contents).
///
/// Memoized per absolute/normalized path string for the process lifetime.
pub fn ncl_tree_digest(ncl_import_path: &str) -> [u8; 32] {
    static CACHE: OnceLock<Mutex<std::collections::HashMap<String, [u8; 32]>>> = OnceLock::new();
    let map = CACHE.get_or_init(|| Mutex::new(std::collections::HashMap::new()));

    {
        let guard = map.lock().unwrap();
        if let Some(d) = guard.get(ncl_import_path) {
            return *d;
        }
    }

    let digest = ncl_tree_digest_uncached(Path::new(ncl_import_path));
    map.lock()
        .unwrap()
        .insert(ncl_import_path.to_owned(), digest);
    digest
}

/// Walk `root` and hash relative paths + file bytes (no memo).
pub fn ncl_tree_digest_uncached(root: &Path) -> [u8; 32] {
    let mut files: Vec<(String, Vec<u8>)> = Vec::new();
    if root.is_dir() {
        collect_files(root, root, &mut files);
    }
    files.sort_by(|a, b| a.0.cmp(&b.0));

    let mut hasher = Sha256::new();
    hasher.update(DIGEST_DOMAIN);
    hasher.update(b"ncl-tree\0");
    for (rel, bytes) in files {
        let rb = rel.as_bytes();
        hasher.update((rb.len() as u64).to_le_bytes());
        hasher.update(rb);
        hasher.update((bytes.len() as u64).to_le_bytes());
        hasher.update(&bytes);
    }
    hasher.finalize().into()
}

fn collect_files(root: &Path, dir: &Path, out: &mut Vec<(String, Vec<u8>)>) {
    let read = match fs::read_dir(dir) {
        Ok(r) => r,
        Err(_) => return,
    };
    for entry in read.flatten() {
        let path = entry.path();
        let Ok(meta) = entry.metadata() else {
            continue;
        };
        if meta.is_dir() {
            collect_files(root, &path, out);
        } else if meta.is_file() {
            let rel = path
                .strip_prefix(root)
                .map(|p| p.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|_| path.to_string_lossy().into_owned());
            if let Ok(bytes) = fs::read(&path) {
                out.push((rel, bytes));
            }
        }
    }
}

fn entry_path(cache_dir: &Path, hex: &str) -> PathBuf {
    cache_dir.join(entry_rel_path(hex))
}

/// Read a cache entry if present and non-empty.
pub fn read_entry(cache_dir: &Path, digest: &[u8; 32]) -> Option<String> {
    let hex = digest_hex(digest);
    let path = entry_path(cache_dir, &hex);
    let mut file = File::open(&path).ok()?;
    let mut buf = String::new();
    file.read_to_string(&mut buf).ok()?;
    if buf.is_empty() {
        return None;
    }
    log_cache("hit", &hex);
    Some(buf)
}

/// Atomically write `json` for `digest` under `cache_dir`.
///
/// Uses a unique temp name per write so concurrent writers for the same digest
/// cannot clobber each other's partial content before rename.
pub fn write_entry_atomic(cache_dir: &Path, digest: &[u8; 32], json: &str) -> io::Result<()> {
    let hex = digest_hex(digest);
    let final_path = entry_path(cache_dir, &hex);
    if let Some(parent) = final_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let tmp_name = format!(
        "{hex}.{}.{}.json.tmp",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .unwrap_or(0)
    );
    let tmp_path = final_path.parent().unwrap_or(cache_dir).join(tmp_name);
    {
        let mut f = File::create(&tmp_path)?;
        f.write_all(json.as_bytes())?;
        f.sync_all()?;
    }
    fs::rename(&tmp_path, &final_path)?;
    log_cache("store", &hex);
    Ok(())
}

/// Import path string for tree digest (from the export key).
pub fn import_path_of(key: &NickelJsonExport) -> &str {
    match key {
        NickelJsonExport::Keymap { import_path, .. }
        | NickelJsonExport::Inputs { import_path, .. }
        | NickelJsonExport::HidReport { import_path, .. } => import_path.as_str(),
    }
}

/// Compute digest for `key` using live Nickel version + ncl tree (shell).
pub fn content_digest_for_key(key: &NickelJsonExport) -> [u8; 32] {
    let ncl = ncl_tree_digest(import_path_of(key));
    content_digest(key, &nickel_version_string(), &ncl)
}

/// Look up disk then, on miss, leave to caller; used by tests with fixed digests.
#[cfg(test)]
pub fn try_read(
    cache_dir: &Path,
    key: &NickelJsonExport,
    nickel_version: &str,
    ncl: &[u8; 32],
) -> Option<String> {
    let digest = content_digest(key, nickel_version, ncl);
    read_entry(cache_dir, &digest)
}

/// Store under the digest for `key` / version / ncl tree.
#[cfg(test)]
pub fn try_store(
    cache_dir: &Path,
    key: &NickelJsonExport,
    nickel_version: &str,
    ncl: &[u8; 32],
    json: &str,
) -> io::Result<()> {
    let digest = content_digest(key, nickel_version, ncl);
    write_entry_atomic(cache_dir, &digest, json)
}

// ---------------------------------------------------------------------------
// Tests (core + shell with temp dirs; no Nickel binary required)
// ---------------------------------------------------------------------------

/// Serialize env mutations that touch disk-cache configuration across parallel tests.
#[cfg(test)]
pub(crate) fn test_env_lock() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    LOCK.lock().unwrap_or_else(|e| e.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NickelJsonExport;

    #[test]
    fn parse_mode_defaults_enabled() {
        assert_eq!(parse_disk_cache_mode(None), DiskCacheMode::Enabled);
        assert_eq!(parse_disk_cache_mode(Some("")), DiskCacheMode::Enabled);
        for s in ["1", "true", "YES", "On", "enabled", "whatever"] {
            assert_eq!(
                parse_disk_cache_mode(Some(s)),
                DiskCacheMode::Enabled,
                "{s}"
            );
        }
    }

    #[test]
    fn parse_mode_disables() {
        for s in ["0", "false", "OFF", "No", "disabled", " DISABLED "] {
            assert_eq!(
                parse_disk_cache_mode(Some(s)),
                DiskCacheMode::Disabled,
                "{s}"
            );
        }
    }

    #[test]
    fn parse_log_defaults_off() {
        assert!(!parse_cache_log_enabled(None));
        assert!(!parse_cache_log_enabled(Some("")));
        assert!(!parse_cache_log_enabled(Some("0")));
        assert!(!parse_cache_log_enabled(Some("false")));
        assert!(!parse_cache_log_enabled(Some("nope")));
    }

    #[test]
    fn parse_log_enables() {
        for s in ["1", "true", "YES", "On", "enabled", " ENABLED "] {
            assert!(parse_cache_log_enabled(Some(s)), "{s}");
        }
    }

    #[test]
    fn entry_rel_path_layout() {
        let hex = "ab".to_owned() + &"cd".repeat(31);
        assert_eq!(hex.len(), 64);
        let p = entry_rel_path(&hex);
        assert_eq!(
            p,
            PathBuf::from("v1").join("ab").join(format!("{hex}.json"))
        );
    }

    #[test]
    fn content_digest_stable_and_kind_sensitive() {
        let ncl = [7u8; 32];
        let ver = "nickel-lang-cli nickel 1.16.0";
        let km = NickelJsonExport::keymap("/any", "{ keys = [] }");
        let d1 = content_digest(&km, ver, &ncl);
        let d2 = content_digest(&km, ver, &ncl);
        assert_eq!(d1, d2);

        let inputs = NickelJsonExport::inputs("/any", "{ keys = [] }", "[]");
        assert_ne!(content_digest(&inputs, ver, &ncl), d1);

        let km2 = NickelJsonExport::keymap("/any", "{ keys = [1] }");
        assert_ne!(content_digest(&km2, ver, &ncl), d1);

        let ncl2 = [8u8; 32];
        assert_ne!(content_digest(&km, ver, &ncl2), d1);
        assert_ne!(content_digest(&km, "other-ver", &ncl), d1);
    }

    #[test]
    fn content_digest_ignores_import_path_string() {
        let ncl = [1u8; 32];
        let ver = "v";
        let a = NickelJsonExport::keymap("/home/a/ncl", "{ keys = [] }");
        let b = NickelJsonExport::keymap("/ci/ncl", "{ keys = [] }");
        assert_eq!(content_digest(&a, ver, &ncl), content_digest(&b, ver, &ncl));
    }

    #[test]
    fn ncl_tree_digest_changes_when_file_changes() {
        let dir = tempfile_dir("ncl-tree");
        fs::write(dir.join("a.ncl"), "1").unwrap();
        let d1 = ncl_tree_digest_uncached(&dir);
        fs::write(dir.join("a.ncl"), "2").unwrap();
        let d2 = ncl_tree_digest_uncached(&dir);
        assert_ne!(d1, d2);
        fs::write(dir.join("b.ncl"), "x").unwrap();
        let d3 = ncl_tree_digest_uncached(&dir);
        assert_ne!(d2, d3);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn atomic_write_then_read_roundtrip() {
        let dir = tempfile_dir("cache-rw");
        let digest = [0xab; 32];
        write_entry_atomic(&dir, &digest, "{\"ok\":true}").unwrap();
        let got = read_entry(&dir, &digest).unwrap();
        assert_eq!(got, "{\"ok\":true}");
        // empty rejected
        let empty = [0xcd; 32];
        let path = entry_path(&dir, &digest_hex(&empty));
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(&path, "").unwrap();
        assert!(read_entry(&dir, &empty).is_none());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn try_store_try_read_with_key() {
        let dir = tempfile_dir("cache-key");
        let ncl = ncl_tree_digest_uncached(&dir);
        let key = NickelJsonExport::keymap(dir.to_str().unwrap(), "let K = 1 in {}");
        let ver = "test-nickel";
        assert!(try_read(&dir, &key, ver, &ncl).is_none());
        try_store(&dir, &key, ver, &ncl, "[1,2,3]").unwrap();
        assert_eq!(try_read(&dir, &key, ver, &ncl).as_deref(), Some("[1,2,3]"));
        // docstring change → miss
        let key2 = NickelJsonExport::keymap(dir.to_str().unwrap(), "let K = 2 in {}");
        assert!(try_read(&dir, &key2, ver, &ncl).is_none());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn configured_dir_respects_disable_env() {
        let _g = test_env_lock();
        let prev = env::var_os(NICKEL_JSON_CACHE_ENV);
        env::set_var(NICKEL_JSON_CACHE_ENV, "0");
        assert!(configured_cache_dir().is_none());
        match prev {
            Some(v) => env::set_var(NICKEL_JSON_CACHE_ENV, v),
            None => env::remove_var(NICKEL_JSON_CACHE_ENV),
        }
    }

    #[test]
    fn configured_dir_respects_dir_env() {
        let _g = test_env_lock();
        let prev_mode = env::var_os(NICKEL_JSON_CACHE_ENV);
        let prev_dir = env::var_os(NICKEL_JSON_CACHE_DIR_ENV);
        env::remove_var(NICKEL_JSON_CACHE_ENV);
        env::set_var(NICKEL_JSON_CACHE_DIR_ENV, "/tmp/sk-ncl-json-cache-test");
        assert_eq!(
            configured_cache_dir().as_deref(),
            Some(Path::new("/tmp/sk-ncl-json-cache-test"))
        );
        match prev_mode {
            Some(v) => env::set_var(NICKEL_JSON_CACHE_ENV, v),
            None => env::remove_var(NICKEL_JSON_CACHE_ENV),
        }
        match prev_dir {
            Some(v) => env::set_var(NICKEL_JSON_CACHE_DIR_ENV, v),
            None => env::remove_var(NICKEL_JSON_CACHE_DIR_ENV),
        }
    }

    fn tempfile_dir(label: &str) -> PathBuf {
        let mut p = env::temp_dir();
        p.push(format!("sk-ncl-cache-{}-{}", label, std::process::id()));
        // unique-ish
        p.push(format!(
            "{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&p).unwrap();
        p
    }
}
