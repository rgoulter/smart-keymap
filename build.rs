use std::env;

use smart_keymap_nickel_helper::{
    codegen_rust_module, nickel_keymap_rs_for_keymap_path, CodegenInputs,
};

fn main() {
    codegen_rust_module(CodegenInputs {
        env_var: "SMART_KEYMAP_CUSTOM_KEYMAP",
        cfg_name: "custom_keymap",
        module_basename: "keymap.rs",
        ncl_import_path: format!("{}/ncl", env!("CARGO_MANIFEST_DIR")).as_str(),
        nickel_eval_fn: nickel_keymap_rs_for_keymap_path,
    });
}
