{
  KeymapKey = Dyn,

  layers
    | Array (Array KeymapKey)
    | doc "The layers of the keymap that gets rendered to JSON",

  make_key
    | doc "Constructs a JSON-representable key value from the given value from keymap.ncl."
    = match {
      # Make tap_hold from keys with a "hold" modifier.
      { hold = hold_kc, key_code = tap_kc } => { "tap_hold" = { hold = hold_kc, tap = tap_kc } },
      { key_code = kc } => { "simple" = kc },
      _ => std.fail_with "unsupported item in keymap.ncl",
    },

  serialized_json_keymap
    | doc "The keymap.json output value."
    =
      layers |> std.array.at 0 |> std.array.map make_key,
}
