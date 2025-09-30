(import "smart_keys/callback/keymap-ncl-to-json.ncl")
& (import "smart_keys/caps_word/keymap-ncl-to-json.ncl")
& (import "smart_keys/consumer/keymap-ncl-to-json.ncl")
& (import "smart_keys/chorded/keymap-ncl-to-json.ncl")
& (import "smart_keys/custom/keymap-ncl-to-json.ncl")
& (import "smart_keys/keyboard/keymap-ncl-to-json.ncl")
& (import "smart_keys/layered/keymap-ncl-to-json.ncl")
& (import "smart_keys/mouse/keymap-ncl-to-json.ncl")
& (import "smart_keys/sticky/keymap-ncl-to-json.ncl")
& (import "smart_keys/tap_dance/keymap-ncl-to-json.ncl")
& (import "smart_keys/tap_hold/keymap-ncl-to-json.ncl")
& {
  validators | default = import "validators.ncl",

  Chord = { indices | Array Number, key | KeymapKey },

  Config = {
    chorded | optional | keymap_ncl.chorded.Config,
    sticky | optional | keymap_ncl.sticky.Config,
    tap_dance | optional | keymap_ncl.tap_dance.Config,
    tap_hold | optional | keymap_ncl.tap_hold.Config,
  },

  KeymapKey = keymap_ncl.nullable_key.Key,

  KeymapLayerString = String,

  KeymapLayer = std.contract.any_of [KeymapLayerString, Array KeymapKey],

  config | Config = {},

  keys
    | optional,

  layers
    | default
    | Array KeymapLayer
    | doc "The layers of the keymap that gets rendered to JSON"
    = [keys],

  chords
    | Array Chord
    | default
    = [],

  custom_keys
    | default
    | doc "Key extension which gets applied when using whitespace-delimited string keymap layers"
    = fun K => {},

  keyboard_modifiers = {
    validator =
      validators.record.validator {
        fields_validator =
          validators.record.has_only_fields [
            "left_ctrl",
            "left_shift",
            "left_alt",
            "left_gui",
            "right_ctrl",
            "right_shift",
            "right_alt",
            "right_gui",
          ],
        field_validators = {
          left_ctrl = validators.is_bool,
          left_shift = validators.is_bool,
          left_alt = validators.is_bool,
          left_gui = validators.is_bool,
          right_ctrl = validators.is_bool,
          right_shift = validators.is_bool,
          right_alt = validators.is_bool,
          right_gui = validators.is_bool,
        },
      },

    # Must agree with consts used in key::KeyboardModifiers
    to_json_value = fun km =>
      let km =
        km
        & {
          left_ctrl | default = false,
          left_shift | default = false,
          left_alt | default = false,
          left_gui | default = false,
          right_ctrl | default = false,
          right_shift | default = false,
          right_alt | default = false,
          right_gui | default = false,
        }
      in
      0
      + (if km.left_ctrl then 1 else 0)
      + (if km.left_shift then 2 else 0)
      + (if km.left_alt then 4 else 0)
      + (if km.left_gui then 8 else 0)
      + (if km.right_ctrl then 16 else 0)
      + (if km.right_shift then 32 else 0)
      + (if km.right_alt then 64 else 0)
      + (if km.right_gui then 128 else 0),
  },

  keymap_ncl.null_ = {
    Key = std.contract.from_validator key_validator,

    key_validator = fun k =>
      k
      |> match {
        null => 'Ok,
        _ => 'Err "not null",
      },

    is_key = fun k => 'Ok == key_validator k,

    to_json_value = fun k => null,

    map_accum = fun f acc k => f acc k,
  },

  keymap_ncl.key = {
    key_modules = [
      keymap_ncl.callback,
      keymap_ncl.caps_word,
      keymap_ncl.chorded,
      keymap_ncl.chorded_aux,
      keymap_ncl.consumer,
      keymap_ncl.custom,
      keymap_ncl.keyboard,
      keymap_ncl.layer_modifier,
      keymap_ncl.layered,
      keymap_ncl.mouse,
      keymap_ncl.sticky,
      keymap_ncl.tap_dance,
      keymap_ncl.tap_hold,
    ],

    Key = std.contract.from_validator key_validator,

    key_validator = fun k =>
      k
      |> validators.any_of (key_modules |> std.array.map (fun km => km.key_validator)),

    is_key = fun k => 'Ok == key_validator k,

    key_module_for_value = fun k =>
      key_modules
      |> std.array.filter (fun km => km.is_key k)
      |> match {
        [km, ..] => km,
        [] => std.fail_with "unknown key type",
      },

    to_json_value = fun k =>
      let km = key_module_for_value k in
      km.to_json_value k,

    map_accum = fun f acc k =>
      let km = key_module_for_value k in
      km.map_accum f acc k,
  },

  keymap_ncl.nullable_key = {
    Key = std.contract.from_validator key_validator,

    key_validator = fun k =>
      k
      |> validators.any_of [
        keymap_ncl.key.key_validator,
        keymap_ncl.null_.key_validator,
      ],

    is_key = fun k => 'Ok == key_validator k,

    to_json_value = match {
      k if keymap_ncl.key.is_key k => keymap_ncl.key.to_json_value k,
      k if keymap_ncl.null_.is_key k => keymap_ncl.null_.to_json_value k,
      _ => std.fail_with "unsupported nullable_key item in keymap.ncl",
    },

    map_accum = fun f acc k =>
      match {
        k if keymap_ncl.key.is_key k => keymap_ncl.key.map_accum f acc k,
        k if keymap_ncl.null_.is_key k => keymap_ncl.null_.map_accum f acc k,
        _ => std.fail_with "unsupported nullable_key item in keymap.ncl",
      },
  },

  checks.check_words_from_whitespace_delimited_string = {
    check_words_from_whitespace_delimited_string = {
      actual = words_from_whitespace_delimited_string "   a b  c ",
      expected = ["a", "b", "c"],
    },
  },

  words_from_whitespace_delimited_string
    | doc "Return array of fields from whitespace-delimited string."
    = fun s =>
      s
      |> std.string.replace_regex "\\s+" " "
      |> std.string.split " "
      |> std.array.filter ((!=) ""),

  checks.check_map_fields_with_record = {
    check_map_fields_with_record = {
      actual = map_fields_with_record { a = 1, b = 2, c = 3 } ["a", "c"],
      expected = [1, 3],
    },
  },

  map_fields_with_record
    | doc "Maps each field in an array to the value of the given record."
    = fun record fields =>
      fields |> std.array.map (std.function.flip std.record.get record),

  checks.check_layer_as_array_of_keys = {
    check_layer_as_array_of_keys = {
      actual = layer_as_array_of_keys " A ",
      expected = [{ key_code = 4 }],
    },

    check_layer_as_array_of_keys_tttt = {
      actual = layer_as_array_of_keys " TTTT ",
      expected = [ null ],
    },
  },

  # Return array-of-keys representation of layer.
  #
  # A KeymapLayer may be either:
  #   - Array Key
  #   - String
  layer_as_array_of_keys = fun l =>
    if std.is_string l then
      let { extend_keys, .. } = import "key-extensions.ncl" in
      let K = extend_keys (import "keys.ncl") custom_keys in
      l
      |> words_from_whitespace_delimited_string
      |> map_fields_with_record K
    else
      l,

  layers_of_keys = layers |> std.array.map layer_as_array_of_keys,

  layered_keys
    | doc "Constructs array of key::layered::LayeredKey values from layers."
    =
      if std.array.length layers == 0 then
        std.fail_with "keys or layers must be provided"
      else if std.array.length layers <= 1 then
        std.array.at 0 layers |> layer_as_array_of_keys
      else
        let base_keys = std.array.first layers_of_keys in
        let layered_keys = std.array.drop_first layers_of_keys in
        std.array.generate
          (fun idx =>
            let base_key = std.array.at idx base_keys in
            let layered_ = std.array.map (std.array.at idx) layered_keys in
            let no_layered = std.array.all (fun x => x == null) layered_ in
            if no_layered then
              base_key
            else
              base_key & { layered = layered_ }
          )
          (std.array.length base_keys),

  chorded_keys =
    std.array.map_with_index
      (fun key_index k =>
        chords
        |> std.array.map_with_index (fun idx ch => ch & { chord_index = idx })
        |> std.array.filter (fun { indices = [chord_primary_key_index, ..], .. } => chord_primary_key_index == key_index)
        |> match {
          [] =>
            if std.array.any (fun { indices, .. } => std.array.elem key_index indices) chords then
              { passthrough = k }
            else
              k,
          chords_ =>
            let chords_ = chords_ |> std.array.map (fun { chord_index, key, .. } => [chord_index, key]) in
            { chords = chords_, passthrough = k },
        }
      )
      layered_keys,

  json_keymap
    | default
    | doc "The keymap.json output value."
    =
      let { chorded = km_config_chorded, ..km_config } = config & { chorded = {} } in
      let chord_indices = chords |> std.array.map (fun { key, indices, .. } => indices) in
      let config_json_value =
        km_config
        & {
          chorded =
            km_config_chorded
            & {
              chords = chord_indices,
            },
        }
      in
      {
        keys = chorded_keys |> std.array.map keymap_ncl.key.to_json_value,
        config = config_json_value,
      },
}
