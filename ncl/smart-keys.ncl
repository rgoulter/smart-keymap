{
  smart_keys = [
    "callback",
    "caps_word",
    "chorded",
    "custom",
    "keyboard",
    "layered",
    "sticky",
    "tap_dance",
    "tap_hold",
  ],
  merged_ncl_of = fun file_name =>
    smart_keys
    |> std.array.map (fun sk => import "smart_keys/%{sk}/%{file_name}")
    |> std.record.merge_all,
  keymap_codegen = merged_ncl_of "keymap-codegen.ncl",
  keymap_ncl_to_json = merged_ncl_of "keymap-ncl-to-json.ncl",
  key_extensions = merged_ncl_of "key-extensions.ncl",
}
