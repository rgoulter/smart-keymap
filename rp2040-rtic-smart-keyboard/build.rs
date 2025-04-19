use std::env;

use smart_keymap_nickel_helper::{
    codegen_rust_module, nickel_board_rs_for_board_path, CodegenInputs,
};

fn main() {
    codegen_rust_module(CodegenInputs {
        env_var: "SMART_KEYBOARD_CUSTOM_BOARD",
        cfg_name: "custom_board",
        module_basename: "board.rs",
        ncl_import_path: format!("{}/ncl", env!("CARGO_MANIFEST_DIR")).as_str(),
        nickel_eval_fn: nickel_board_rs_for_board_path,
    });
}
