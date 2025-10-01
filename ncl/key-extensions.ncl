(import "smart_keys/automation/key-extensions.ncl")
& (import "smart_keys/callback/key-extensions.ncl")
& (import "smart_keys/caps_word/key-extensions.ncl")
& (import "smart_keys/consumer/key-extensions.ncl")
& (import "smart_keys/custom/key-extensions.ncl")
& (import "smart_keys/keyboard/key-extensions.ncl")
& (import "smart_keys/layered/key-extensions.ncl")
& (import "smart_keys/mouse/key-extensions.ncl")
& (import "smart_keys/sticky/key-extensions.ncl")
& (import "smart_keys/tap_hold/key-extensions.ncl")
& {
  extend_keys = fun K key_extension =>
    std.typeof key_extension
    |> match {
      'Record => K & key_extension,
      'Function => K & key_extension K,
    },
}
