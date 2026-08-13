//! Relative Nickel `import` graph for `cargo:rerun-if-changed`.
//!
//! ## Functional core / imperative shell
//!
//! - **Core** ([`ncl_static_import_paths`], [`relative_import_candidates`]):
//!   parse and join; no filesystem.
//! - **Shell** ([`collect_relative_ncl_imports`]): canonicalize, `is_file`,
//!   `read_to_string`.
//!
//! File imports are a string literal (`import "foo.ncl"`). Package imports
//! (`import gh`) have no quotes and are ignored. Paths that do not exist next
//! to the importer (e.g. `keys.ncl` from `--import-path`) are skipped; those
//! live in the codegen `ncl/` tree, which is watched separately.

use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Functional core
// ---------------------------------------------------------------------------

fn is_ident_cont(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

/// Byte index of the next `import` keyword at or after `from`.
fn find_import_keyword(src: &str, from: usize) -> Option<usize> {
    let mut i = from;
    while let Some(rel) = src[i..].find("import") {
        let start = i + rel;
        let after_kw = start + "import".len();
        let preceded_by_ident = src[..start].chars().next_back().is_some_and(is_ident_cont);
        let followed_by_ident = src[after_kw..].chars().next().is_some_and(is_ident_cont);
        if !preceded_by_ident && !followed_by_ident {
            return Some(start);
        }
        i = after_kw;
    }
    None
}

/// Leading quoted string: optional whitespace, then `"…"` or `'…'`.
///
/// Returns `(contents, rest_after_closing_quote)`.
fn take_quoted_string(s: &str) -> Option<(&str, &str)> {
    let s = s.trim_start();
    let quote = s.chars().next().filter(|c| *c == '"' || *c == '\'')?;
    let inner = &s[quote.len_utf8()..];
    let end = inner.find(quote)?;
    Some((&inner[..end], &inner[end + quote.len_utf8()..]))
}

/// Quoted paths from static `import "…"` / `import '…'` forms in Nickel source.
///
/// Dynamic or computed import expressions are a parse error in Nickel and are
/// not produced here. Identifier-adjacent matches (`imported`, `reimport`) and
/// package imports (`import gh`) are skipped.
pub fn ncl_static_import_paths(src: &str) -> Vec<&str> {
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(kw) = find_import_keyword(src, from) {
        let after_kw = kw + "import".len();
        match take_quoted_string(&src[after_kw..]) {
            Some((path, rest)) => {
                out.push(path);
                from = src.len() - rest.len();
            }
            None => from = after_kw,
        }
    }
    out
}

/// Paths to try for one quoted import, relative to the importing file's directory.
///
/// Does not check existence. A bare name also yields a `.ncl` sibling, matching
/// Nickel's optional-extension lookup.
pub fn relative_import_candidates(importer_dir: &Path, import: &str) -> Vec<PathBuf> {
    let mut out = vec![importer_dir.join(import)];
    if !import.ends_with(".ncl") {
        out.push(importer_dir.join(format!("{import}.ncl")));
    }
    out
}

// ---------------------------------------------------------------------------
// Imperative shell
// ---------------------------------------------------------------------------

fn first_existing_relative_import(importer_dir: &Path, import: &str) -> Option<PathBuf> {
    relative_import_candidates(importer_dir, import)
        .into_iter()
        .find_map(|candidate| candidate.canonicalize().ok().filter(|p| p.is_file()))
}

/// Files reachable from `root` via relative Nickel `import`s that exist on disk.
///
/// Includes `root` itself when it can be canonicalized.
pub fn collect_relative_ncl_imports(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut seen = HashSet::new();
    let mut stack = vec![root.to_path_buf()];

    while let Some(path) = stack.pop() {
        let Ok(canon) = path.canonicalize() else {
            continue;
        };
        if !seen.insert(canon.clone()) {
            continue;
        }
        out.push(canon.clone());

        let Ok(src) = fs::read_to_string(&canon) else {
            continue;
        };
        let Some(dir) = canon.parent() else {
            continue;
        };
        for import in ncl_static_import_paths(&src) {
            if let Some(next) = first_existing_relative_import(dir, import) {
                stack.push(next);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::{
        collect_relative_ncl_imports, ncl_static_import_paths, relative_import_candidates,
        take_quoted_string,
    };
    use std::path::{Path, PathBuf};

    #[test]
    fn take_quoted_string_double_and_single() {
        assert_eq!(
            take_quoted_string(r#"  "foo.ncl" rest"#),
            Some(("foo.ncl", " rest"))
        );
        assert_eq!(take_quoted_string("'other.ncl'"), Some(("other.ncl", "")));
        assert_eq!(take_quoted_string("gh"), None);
        assert_eq!(take_quoted_string(r#""unterminated"#), None);
    }

    #[test]
    fn ncl_static_import_paths_table() {
        let cases: &[(&str, &[&str])] = &[
            ("", &[]),
            ("{ keys = [] }", &[]),
            (r#"let K = import "keys.ncl" in K"#, &["keys.ncl"]),
            (
                r#"(import "../split_3x5+3/keymap.ncl")"#,
                &["../split_3x5+3/keymap.ncl"],
            ),
            (r#"let x = import 'other.ncl' in x"#, &["other.ncl"]),
            (r#"import"keys.ncl""#, &["keys.ncl"]),
            ("import\n  \"keys.ncl\"", &["keys.ncl"]),
            (r#"import "data.json" as 'Json"#, &["data.json"]),
            ("import gh", &[]),
            ("let imported = 1 in imported", &[]),
            (r#"let y = reimport "nope.ncl" in y"#, &[]),
            (
                r#"
                    let K = import "keys.ncl" in
                    (import "../split_3x5+3/keymap.ncl")
                    let x = import 'other.ncl' in
                    let imported = 1 in
                    let y = reimport "nope.ncl" in
                    import gh
                "#,
                &["keys.ncl", "../split_3x5+3/keymap.ncl", "other.ncl"],
            ),
        ];
        for (src, expected) in cases {
            assert_eq!(
                ncl_static_import_paths(src),
                expected.to_vec(),
                "src={src:?}"
            );
        }
    }

    #[test]
    fn relative_import_candidates_with_and_without_extension() {
        let dir = Path::new("/keymaps/ortho-4x12");
        assert_eq!(
            relative_import_candidates(dir, "../split_3x5+3/keymap.ncl"),
            vec![PathBuf::from(
                "/keymaps/ortho-4x12/../split_3x5+3/keymap.ncl"
            )]
        );
        assert_eq!(
            relative_import_candidates(dir, "keys"),
            vec![
                PathBuf::from("/keymaps/ortho-4x12/keys"),
                PathBuf::from("/keymaps/ortho-4x12/keys.ncl"),
            ]
        );
    }

    #[test]
    fn collect_relative_ncl_imports_follows_existing_files() {
        let dir = std::env::temp_dir().join(format!(
            "sk-ncl-imports-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let ortho = dir.join("ortho-4x12");
        let split = dir.join("split_3x5+3");
        std::fs::create_dir_all(&ortho).unwrap();
        std::fs::create_dir_all(&split).unwrap();
        std::fs::write(
            split.join("keymap.ncl"),
            r#"let K = import "keys.ncl" in { keys = [] }"#,
        )
        .unwrap();
        let root = ortho.join("keymap.ncl");
        std::fs::write(&root, r#"(import "../split_3x5+3/keymap.ncl")"#).unwrap();

        // Both keymap files; keys.ncl is not next to the importer so it is skipped.
        let paths = collect_relative_ncl_imports(&root);
        assert_eq!(paths.len(), 2);
        let as_str: Vec<String> = paths
            .iter()
            .map(|p| p.to_string_lossy().into_owned())
            .collect();
        assert!(
            as_str.iter().any(|p| p.ends_with("ortho-4x12/keymap.ncl")),
            "{as_str:?}"
        );
        assert!(
            as_str.iter().any(|p| p.ends_with("split_3x5+3/keymap.ncl")),
            "{as_str:?}"
        );

        let _ = std::fs::remove_dir_all(&dir);
    }
}
