{
  KeymapKey = Dyn,

  layers
    | Array (Array KeymapKey)
    | doc "The layers of the keymap that gets rendered to JSON",

  to_json_serialized_key
    | doc "Constructs a JSON-representable key value from the given value from keymap.ncl."
    = match {
      # Make key::tap_hold::Key from keys with a "hold" modifier.
      { hold = hold_kc, key_code = tap_kc } => { hold = hold_kc, tap = tap_kc },
      # Otherwise, keys with just a base key_code are key::simple keys.
      { key_code = kc } => kc,
      _ => std.fail_with "unsupported item in keymap.ncl",
    },

  serialized_json_keymap
    | doc "The keymap.json output value."
    =
      layers |> std.array.at 0 |> std.array.map to_json_serialized_key,
}
