let keymap = import "keymap.json" in

let simple = {
  key_type = "crate::key::simple::Key",
  rust_expr = fun key => "crate::key::simple::Key(%{std.to_string key})",
}
in

let keymap_len = std.array.length keymap in
let keys_id = "Keys%{std.to_string keymap_len}" in

let key_type_of_key = match {
  { simple = key } => simple.key_type,
  _ => "unknown_key_type"
}
in

let rust_expr_of_key = match {
  { simple = key } => simple.rust_expr key,
  _ => "unknown_key_type"
}
in

let key_types =
  keymap
  |> std.array.map key_type_of_key
  |> std.string.join ""
in

let key_exprs =
  keymap
  |> std.array.map (fun key => "%{rust_expr_of_key key},")
  |> std.string.join ""
in

m%"
crate::tuples::define_keys!(%{std.to_string keymap_len});

type KeyDefinitionsType = %{keys_id}<
%{key_types}
>;

const KEY_DEFINITIONS: KeyDefinitionsType = %{keys_id}::new((
%{key_exprs}
));
"%
