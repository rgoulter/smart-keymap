use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

use smart_keymap_nickel_helper::{nickel_keymap_expr_for_keymap_ncl, NickelError};

#[proc_macro]
/// Constructs a [smart_keymap::keymap::Keymap] from a Nickel keymap definition.
pub fn keymap(input: TokenStream) -> TokenStream {
    let keymap_ncl_lit = parse_macro_input!(input as LitStr);
    let keymap_ncl_str = keymap_ncl_lit.value();

    let ncl_import_path = env!("SMART_KEYMAP_NCL_PATH");

    match nickel_keymap_expr_for_keymap_ncl(ncl_import_path, &keymap_ncl_str) {
        Ok(rust_code_str) => {
            let rust_code: proc_macro2::TokenStream =
                rust_code_str.parse().expect("Failed to parse rust code");
            (quote! { #rust_code }).into()
        }
        Err(NickelError::EvalError(error_message)) => {
            panic!("Nickel evaluation failed:\n{}", error_message);
        }
        Err(NickelError::NickelNotFound) => {
            panic!("`nickel` executable not found in PATH");
        }
    }
}
