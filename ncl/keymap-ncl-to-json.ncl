{
  KeymapKey = Dyn,

  keys
    | optional,

  layers
    | default
    | Array (Array KeymapKey)
    | doc "The layers of the keymap that gets rendered to JSON"
    = [keys],

  to_json_serialized_key
    | doc "Constructs a JSON-representable key value from the given value from keymap.ncl."
    = match {
      # Make key::layered::LayeredKey
      { key_code = base_key, layered = layered_keys } =>
        {
          base = base_key,
          layered = std.array.map to_json_serialized_key layered_keys
        },
      # Make key::tap_hold::Key from keys with a "hold" modifier.
      { hold = hold_kc, key_code = tap_kc } => { hold = hold_kc, tap = tap_kc },
      # Otherwise, keys with just a base key_code are key::simple keys.
      { key_code = kc } => kc,
      # Make key::layered::ModifierKey
      { layer_modifier = { hold = hold_layer } } => { Hold = hold_layer },
      _ => std.fail_with "unsupported item in keymap.ncl",
    },

  to_composite_json_serialized_key
    | doc "Constructs a JSON-representable composite::Key value from the given value from keymap.ncl."
    = match {
      # Make key::layered::LayeredKey
      k@{ key_code = base_key, layered = layered_keys } =>
        { Layered = { key = to_json_serialized_key k } },
      # Make key::tap_hold::Key from keys with a "hold" modifier.
      k@{ hold = hold_kc, key_code = tap_kc } =>
        { TapHold = { key = to_json_serialized_key k } },
      # Otherwise, keys with just a base key_code are key::simple keys.
      k@{ key_code = kc } =>
        { Simple = { key = to_json_serialized_key k } },
      # Make key::layered::ModifierKey
      k@{ layer_modifier = { hold = hold_layer } } =>
        { LayerModifier = { key = to_json_serialized_key k } },
      _ => std.fail_with "unsupported item in keymap.ncl",
    },

  serialized_json_keymap
    | doc "The keymap.json output value."
    =
      layers |> std.array.at 0 |> std.array.map to_json_serialized_key,

  serialized_json_composite_keys
    | doc "The keymap.json output value."
    =
      layers |> std.array.at 0 |> std.array.map to_composite_json_serialized_key,
}
