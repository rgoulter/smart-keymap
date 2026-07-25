use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

use smart_keymap_nickel_helper::{nickel_composite_full_vec_rs, rustfmt, NickelError};

fn main() {
    // Workspace root is the parent of this package.
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let workspace_root = Path::new(manifest_dir)
        .parent()
        .expect("package should live under workspace root");
    let ncl_import_path = workspace_root.join("ncl");
    let ncl_import_path = ncl_import_path
        .to_str()
        .expect("ncl import path should be UTF-8");

    println!("cargo:rerun-if-changed={}/key_system", ncl_import_path);
    println!(
        "cargo:rerun-if-changed={}/keymap-codegen.ncl",
        ncl_import_path
    );
    // Family modules can affect the registry merge / types referenced by emit.
    println!("cargo:rerun-if-changed={}/smart_keys", ncl_import_path);

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("composite_full_vec.rs");

    match nickel_composite_full_vec_rs(ncl_import_path) {
        Ok(module_src) => {
            // Nested under `init` in lib.rs; size consts use `super::…`.
            // Engine paths use the external `smart_keymap` dependency by name.
            let mut file = fs::File::create(&dest_path).unwrap();
            let formatted = rustfmt(module_src);
            file.write_all(formatted.as_bytes()).unwrap();
        }
        Err(NickelError::NickelNotFound) => {
            panic!("`nickel` not found in PATH (required to build smart-keymap-full-system-std)");
        }
        Err(NickelError::EvalError(e)) => {
            panic!("Nickel evaluation failed while emitting composite_full_vec:\n{e}");
        }
    }
}
