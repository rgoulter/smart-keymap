{
  keymap
    | doc "The imported keymap.json."
    = import "keymap.json",

  simple
    | doc "Generates type declaration and value expression for smart_keymap::key::simple::Key."
    = {
      key_type = "crate::key::simple::Key",
      rust_expr = fun key => "crate::key::simple::Key(%{std.to_string key})",
    },

  keymap_len
    | doc "The keymap length."
    = std.array.length keymap,

  keys_id
    | doc "The identifier for the KeysN tuple struct used in the generated keymap.rs."
    = "Keys%{std.to_string keymap_len}",

  key_type_of_key
    | doc "The type declaration for the key."
    = match {
      { simple = key } => simple.key_type,
      _ => "unknown_key_type"
    },

  rust_expr_of_key
    | doc "The Rust expression for the key."
    = match {
      { simple = key } => simple.rust_expr key,
      _ => "unknown_key_type"
    },

  key_types
    | doc "The comma-separated list of key types, passed to the KeysN generics."
    =
      keymap
      |> std.array.map key_type_of_key
      |> std.string.join ",",

  key_exprs
    | doc "The comma-separated list of key expressions, passed to the KeysN constructor."
    =
      keymap
      |> std.array.map (fun key => "%{rust_expr_of_key key},")
      |> std.string.join "",

  keymap_rs
    | doc "Text contents of the keymap.rs generated from the keymap.json"
    = m%"
crate::tuples::define_keys!(%{std.to_string keymap_len});

type KeyDefinitionsType = %{keys_id}<
%{key_types}
>;

const KEY_DEFINITIONS: KeyDefinitionsType = %{keys_id}::new((
%{key_exprs}
));
"%
}
