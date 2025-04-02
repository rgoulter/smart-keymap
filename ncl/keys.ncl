let key_extensions = import "key-extensions.ncl" in

std.array.fold_left
  key_extensions.extend_keys
  {}
  [
    key_extensions.keyboard,
    key_extensions.tap_hold,
    key_extensions.layered,
    key_extensions.shifted,
    key_extensions.aliases,
    key_extensions.keymap_callbacks,
    key_extensions.caps_word,
    key_extensions.abbreviations,
    key_extensions.literals,
  ]
