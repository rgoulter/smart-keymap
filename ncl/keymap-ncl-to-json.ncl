{
  KeymapKey = Dyn,

  keys
    | optional,

  layers
    | default
    | Array (Array KeymapKey)
    | doc "The layers of the keymap that gets rendered to JSON"
    = [keys],

  checks.check_to_json_serialized_key =
    let K = import "keys.ncl" in
    {
      keyboard = {
        keymap_example_key_code = {
          actual = K.A,
          expected = { key_code = 0x04 },
        },
        check_serialized_key_code = {
          actual = K.A |> to_json_serialized_key,
          expected = { key_code = 0x04 },
        },
      },

      layered = {
        keymap_example_layered_keyboard = {
          actual = K.A & { layered = [null, K.C] },
          expected = {
            key_code = 0x04,
            layered = [
              null,
              { key_code = 0x06 },
            ],
          },
        },
        check_serialized_layered_keyboard = {
          actual = K.A & { layered = [null, K.C] } |> to_json_serialized_key,
          expected = {
            base = { key_code = 0x04 },
            layered = [
              null,
              { key_code = 0x06 },
            ],
          },
        },
      },

      layer_modifier = {
        keymap_example_hold = {
          actual = K.layer_mod.hold 0,
          expected = { layer_modifier.hold = 0 },
        },
        check_serialized_hold = {
          actual = K.layer_mod.hold 0 |> to_json_serialized_key,
          expected = { Hold = 0 },
        },
      },

      tap_hold = {
        keymap_example_tap_keyboard_hold_keyboard = {
          actual = K.A & { hold = K.LeftCtrl },
          expected = { key_code = 0x04, hold = { key_code = 0xE0 }  },
        },
        check_serialized_tap_keyboard_hold_keyboard = {
          actual = K.A & K.hold K.LeftCtrl |> to_json_serialized_key,
          expected = { tap = { key_code = 0x04 }, hold = { key_code = 0xE0 }  },
        },
      },
    },

  to_json_serialized_key
    | doc "Constructs a JSON-representable key value from the given value from keymap.ncl."
    = match {
      # Make key::layered::LayeredKey
      { key_code = base_key, layered = layered_keys } =>
        {
          base = { key_code = base_key },
          layered = std.array.map to_json_serialized_key layered_keys
        },
      # Make key::tap_hold::Key from keys with a "hold" modifier.
      { hold = hold_kc, key_code = tap_kc } =>
        {
          hold = { key_code = hold_kc },
          tap = { key_code = tap_kc },
        },
      # Otherwise, keys with just a base key_code are key::keyboard keys.
      { key_code = kc } => { key_code = kc },
      # Make key::layered::ModifierKey
      { layer_modifier = { hold = hold_layer } } => { Hold = hold_layer },
      # Null values (None) stay null
      null => null,
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
      # Otherwise, keys with just a base key_code are key::keyboard keys.
      k@{ key_code = kc } =>
        { Keyboard = { key = to_json_serialized_key k } },
      # Make key::layered::ModifierKey
      k@{ layer_modifier = { hold = hold_layer } } =>
        { LayerModifier = { key = to_json_serialized_key k } },
      # Null values (None) stay null
      null => null,
      _ => std.fail_with "unsupported item in keymap.ncl",
    },

  layered_keys
    | doc "Constructs array of key::layered::LayeredKey values from layers."
    =
    if std.array.length layers == 0 then
      std.fail_with "keys or layers must be provided"
    else
    if std.array.length layers <= 1 then
      std.array.at 0 layers
    else
      let base_keys = std.array.first layers in
      let layered_keys = std.array.drop_first layers in
      std.array.generate
        (fun idx =>
          let base_key = std.array.at idx base_keys in
          let layered_ = std.array.map (std.array.at idx) layered_keys in
          let no_layered = std.array.all (fun x => x == null) layered_ in
          if no_layered then
            base_key
          else
            base_key & { layered = layered_ })
        (std.array.length base_keys),

  serialized_json_keymap
    | default
    | doc "The keymap.json output value."
    =
      layered_keys |> std.array.map to_json_serialized_key,

  serialized_json_composite_keys
    | doc "The keymap.json output value."
    =
      layered_keys |> std.array.map to_composite_json_serialized_key,
}
