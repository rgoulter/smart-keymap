use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

use smart_keymap_nickel_helper::{
    codegen_rust_module, nickel_composite_full_vec_rs, nickel_keymap_rs_for_keymap_path, rustfmt,
    CodegenInputs, NickelError,
};

fn main() {
    let ncl_import_path = format!("{}/ncl", env!("CARGO_MANIFEST_DIR"));

    codegen_rust_module(CodegenInputs {
        env_var: "SMART_KEYMAP_CUSTOM_KEYMAP",
        cfg_name: "custom_keymap",
        module_basename: "keymap.rs",
        ncl_import_path: ncl_import_path.as_str(),
        nickel_eval_fn: nickel_keymap_rs_for_keymap_path,
    });

    // Full composite shell with Vec storage for std consumers (cucumber, etc.).
    // Firmware builds use --no-default-features and skip this.
    if env::var_os("CARGO_FEATURE_STD").is_some() {
        codegen_composite_full_vec(&ncl_import_path);
    }
}

fn codegen_composite_full_vec(ncl_import_path: &str) {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    println!("cargo:rerun-if-changed={manifest_dir}/ncl/composite-key-system.ncl");
    println!("cargo:rerun-if-changed={manifest_dir}/ncl/keymap-codegen.ncl");
    // Family modules can affect the registry merge / types referenced by emit.
    println!("cargo:rerun-if-changed={manifest_dir}/ncl/smart_keys");

    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("composite_full_vec.rs");

    match nickel_composite_full_vec_rs(ncl_import_path) {
        Ok(module_src) => {
            let mut file = fs::File::create(&dest_path).unwrap();
            let formatted = rustfmt(module_src);
            file.write_all(formatted.as_bytes()).unwrap();
        }
        Err(NickelError::NickelNotFound) => {
            panic!(
                "`nickel` not found in PATH (required to build smart-keymap with feature \"std\")"
            );
        }
        Err(NickelError::EvalError(e)) => {
            panic!("Nickel evaluation failed while emitting composite_full_vec:\n{e}");
        }
    }
}
