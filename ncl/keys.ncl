let { key_extensions, extend_keys, .. } = import "key-extensions.ncl" in

std.array.fold_left
  extend_keys
  {}
  [
    key_extensions.automation,
    key_extensions.callback,
    key_extensions.caps_word,
    key_extensions.consumer,
    key_extensions.consumer_aliases,
    key_extensions.custom,
    key_extensions.keyboard,
    key_extensions.keyboard_shifted,
    key_extensions.keyboard_aliases,
    key_extensions.keyboard_abbreviations,
    key_extensions.layered,
    key_extensions.mouse,
    key_extensions.mouse_aliases,
    key_extensions.literals,
    key_extensions.sticky,
    key_extensions.tap_hold,
  ]
