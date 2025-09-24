let { key_extensions, extend_keys, .. } = import "key-extensions.ncl" in

let rec keys =
  std.array.fold_left
    extend_keys
    { include keys }
    [
      key_extensions.abbreviations,
      key_extensions.aliases,
      key_extensions.callback,
      key_extensions.caps_word,
      key_extensions.consumer,
      key_extensions.custom,
      key_extensions.keyboard,
      key_extensions.layered,
      key_extensions.literals,
      key_extensions.shifted,
      key_extensions.sticky,
      key_extensions.tap_hold,
    ]
in keys |> std.record.remove "keys"
