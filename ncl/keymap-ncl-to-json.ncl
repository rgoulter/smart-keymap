(import "smart_keys/callback/keymap-ncl-to-json.ncl")
& (import "smart_keys/caps_word/keymap-ncl-to-json.ncl")
& (import "smart_keys/chorded/keymap-ncl-to-json.ncl")
& (import "smart_keys/custom/keymap-ncl-to-json.ncl")
& (import "smart_keys/keyboard/keymap-ncl-to-json.ncl")
& (import "smart_keys/layered/keymap-ncl-to-json.ncl")
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

  KeymapKey =
    std.contract.any_of [
      keymap_ncl.keyboard.Key,
      keymap_ncl.layer_modifier.Key,
      keymap_ncl.callback.Key,
      keymap_ncl.caps_word.Key,
      keymap_ncl.sticky.Key,
      keymap_ncl.custom.Key,
      keymap_ncl.layered.Key,
      keymap_ncl.tap_dance.Key,
      keymap_ncl.tap_hold.Key,
      keymap_ncl.chorded.Key,
      keymap_ncl.chorded_aux.Key,
      std.contract.from_validator validators.is_null,
    ],

  KeymapLayerString = String,

  KeymapLayer = std.contract.any_of [ Array KeymapKey, KeymapLayerString],

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

  keymap_ncl.key = {
    Key = std.contract.from_validator key_validator,

    key_validator = fun k =>
      k
      |> validators.any_of [
        keymap_ncl.keyboard.key_validator,
        keymap_ncl.layer_modifier.key_validator,
        keymap_ncl.callback.key_validator,
        keymap_ncl.caps_word.key_validator,
        keymap_ncl.sticky.key_validator,
        keymap_ncl.custom.key_validator,
        keymap_ncl.layered.key_validator,
        keymap_ncl.tap_dance.key_validator,
        keymap_ncl.tap_hold.key_validator,
        keymap_ncl.chorded.key_validator,
        keymap_ncl.chorded_aux.key_validator,
      ],

    is_key = fun k => 'Ok == key_validator k,

    to_json_value = match {
      # Make key::callback::Key
      k if keymap_ncl.callback.is_key k => keymap_ncl.callback.to_json_value k,
      # Make key::caps_word::Key
      k if keymap_ncl.caps_word.is_key k => keymap_ncl.caps_word.to_json_value k,
      # Make key::chorded::Key
      k if keymap_ncl.chorded.is_key k => keymap_ncl.chorded.to_json_value k,
      # Make key::chorded::AuxiliaryKey
      k if keymap_ncl.chorded_aux.is_key k => keymap_ncl.chorded_aux.to_json_value k,
      # Make key::custom::Key
      k if keymap_ncl.custom.is_key k => keymap_ncl.custom.to_json_value k,
      # Otherwise, keys with just a base key_code are key::keyboard keys.
      k if keymap_ncl.keyboard.is_key k => keymap_ncl.keyboard.to_json_value k,
      # Make key::layered::ModifierKey
      k if keymap_ncl.layer_modifier.is_key k => keymap_ncl.layer_modifier.to_json_value k,
      # Make key::layered::LayeredKey
      k if keymap_ncl.layered.is_key k => keymap_ncl.layered.to_json_value k,
      # Make key::sticky::Key
      k if keymap_ncl.sticky.is_key k => keymap_ncl.sticky.to_json_value k,
      # Make key::tap_dance::Key from keys with a "tap_dances" modifier.
      k if keymap_ncl.tap_dance.is_key k => keymap_ncl.tap_dance.to_json_value k,
      # Make key::tap_hold::Key from keys with a "hold" modifier.
      k if keymap_ncl.tap_hold.is_key k => keymap_ncl.tap_hold.to_json_value k,
      # Null values (None) stay null
      null => null,
      _ => std.fail_with "unsupported item in keymap.ncl",
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
