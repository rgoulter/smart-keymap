let validators = import "validators.ncl" in
{
  Chord = { indices | Array Number, key | KeymapKey },

  ChordedConfig = {
    timeout | optional | Number,
  },

  StickyActivation =
    std.contract.from_validator (
      validators.is_elem_of [
        "OnStickyKeyRelease"
      ]
    ),

  StickyReleaseJson =
    std.contract.from_validator (
      validators.is_elem_of [
        "OnModifiedKeyRelease",
        "OnNextKeyPress",
      ]
    ),

  StickyConfig = {
    activation
      | optional
      | StickyActivation,
    release
      | optional
      | StickyReleaseJson,
  },

  TapHoldInterruptResponse =
    std.contract.from_validator (
      validators.is_elem_of [
        "HoldOnKeyPress",
        "HoldOnKeyTap",
        "Ignore",
      ]
    ),

  TapDanceConfig = {
    timeout | optional | Number,
  },

  TapHoldConfig = {
    timeout | optional | Number,
    interrupt_response | optional | TapHoldInterruptResponse,
    required_idle_time | optional | Number,
  },

  Config = {
    chorded | optional | ChordedConfig,
    sticky | optional | StickyConfig,
    tap_dance | optional | TapDanceConfig,
    tap_hold | optional | TapHoldConfig,
  },

  KeymapKey =
    std.contract.any_of [
      keyboard.Key,
      layer_modifier.Key,
      keymap_callback.Key,
      caps_word.Key,
      sticky.Key,
      custom.Key,
      layered.Key,
      tap_dance.Key,
      tap_hold.Key,
      chorded.Key,
      chorded_aux.Key,
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

  checks.keyboard =
    let K = import "keys.ncl" in
    {
      keymap_example_key_code = {
        actual = K.A,
        expected = { key_code = 4 },
      },

      check_json_value_key_code = {
        actual = K.A |> key.to_json_value,
        expected = { key_code = 4 },
      },
    },

  keyboard
    | doc "for key::keyboard::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator =
        validators.record.validator {
          fields_validator =
            validators.all_of [
              validators.record.has_any_field_of ["key_code", "modifiers"],
              validators.record.has_only_fields ["key_code", "modifiers"],
            ],
          field_validators = {
            key_code = validators.is_number,
            modifiers = keyboard_modifiers.validator,
          },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = match {
        { modifiers = km, ..k } => k & { modifiers = keyboard_modifiers.to_json_value km },
        k => k,
      },
    },

  checks.layer_modifier =
    let K = import "keys.ncl" in
    {
      keymap_example_hold = {
        actual = K.layer_mod.hold 0,
        expected = { layer_modifier.hold = 0 },
      },

      check_json_value_hold = {
        actual = K.layer_mod.hold 0 |> key.to_json_value,
        expected = { Hold = 0 },
      },
    },

  layer_modifier
    | doc "for key::layered::ModifierKey."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { layer_modifier = { default_ } } => validators.is_number default_,
          { layer_modifier = { hold } } => validators.is_number hold,
          _ => 'Error { message = "expected { layer_modifier = { default_ } } or { layer_modifier = { hold } }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun layer_modifier_key =>
        layer_modifier_key
        |> match {
          { layer_modifier = { default_ = default_layer } } =>
            { Default = default_layer },
          { layer_modifier = { hold = hold_layer } } =>
            { Hold = hold_layer },
          _ => 'Error { message = "expected { layer_modifier = { default_ } } or { layer_modifier = { hold } }" },
        }
    },

  keymap_callback
    | doc "for key::callback::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { keymap_callback = { Custom = indices @ [i, j] } } =>
            validators.array.validator validators.is_number indices,
          { keymap_callback } => validators.is_string keymap_callback,
          _ => 'Error { message = "expected { keymap_callback = <KeymapCallback variant> }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun key => key,
    },

  caps_word
    | doc "for key::caps_word::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          "ToggleCapsWord" => 'Ok,
          _ => 'Error { message = "Expected \"ToggleCapsWord\"" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun key => key,
    },

  sticky
    | doc "for key::sticky::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator =
        validators.record.validator {
          fields_validator =
            validators.record.has_exact_fields ["sticky_modifiers"],
          field_validators = {
            sticky_modifiers = keyboard_modifiers.validator,
          },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun { sticky_modifiers = sm } => { sticky_modifiers = keyboard_modifiers.to_json_value sm },
    },

  custom
    | doc "for key::custom::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { custom } => validators.is_number custom,
          _ => 'Error { message = "Expected { custom }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun key => key,
    },

  checks.layered =
    let K = import "keys.ncl" in
    {
      keymap_example_layered_keyboard = {
        actual = K.A & { layered = [ null, K.C] },
        expected = {
          key_code = 4,
          layered = [
            null,
            { key_code = 6 },
          ],
        },
      },

      check_layered_base_keyboard_base_is_key =
        let k = K.A & { layered = [ null, K.C] } in
        let { layered, ..base_key } = k in
        key.is_key base_key,

      check_layered_base_keyboard_is_key =
        let key = K.A & { layered = [ null, K.C] } in
        layered.is_key key,

      check_json_value_layered_keyboard = {
        actual = K.A & { layered = [ null, K.C] } |> key.to_json_value,
        expected = {
          base = { key_code = 4 },
          layered = [
            null,
            { key_code = 6 },
          ],
        },
      },
    },

  layered
    | doc "for key::layered::LayeredKey."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { layered, ..base_key } =>
            let valid_layered =
              layered
              |> validators.array.validator (
                validators.any_of [validators.is_null, key.key_validator]
              )
            in
            let valid_base_key = key.key_validator base_key in
            [valid_layered, valid_base_key]
            |> match {
              ['Ok, 'Ok] => 'Ok,
              [err, _] => err,
              [_, err] => err,
            },
          _ => 'Error { message = "expected { layered = Array Key, ..base_key }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun { layered = layered_keys, ..base_key } =>
        {
          base = key.to_json_value base_key,
          layered =
            std.array.map
              (fun k => if k != null then key.to_json_value k else null)
              layered_keys,
        },
    },

  tap_dance
    | doc "for key::tap_dance::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { tap_dances, ..tap_key } =>
            let valid_tap_key = key.key_validator tap_key in
            let valid_tap_dances = validators.array.validator key.key_validator tap_dances in
            [valid_tap_dances, valid_tap_key]
            |> match {
              ['Ok, 'Ok] => 'Ok,
              [err, _] => err,
              [_, err] => err,
            },
          _ => 'Error { message = "expected { tap_dances = [Key, ..], ..tap_key }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun { tap_dances, ..tap_key } =>
        {
          definitions = [key.to_json_value tap_key] @ (std.array.map key.to_json_value tap_dances),
        },
    },

  checks.tap_hold =
    let K = import "keys.ncl" in
    {
      keymap_example_tap_keyboard_hold_keyboard = {
        actual = K.A & K.hold K.LeftCtrl,
        expected = { key_code = 4, hold = { modifiers = { left_ctrl = true } } },
      },

      check_tap_hold_tap_keyboard_is_key =
        let k = K.A & K.hold K.LeftCtrl in
        let { hold, ..tap_key } = k in
        key.is_key tap_key,

      check_tap_keyboard_hold_keyboard_is_key =
        let key = K.A & K.hold K.LeftCtrl in
        tap_hold.is_key key,

      check_tap_keyboard_hold_keyboard_to_json_json_value_keyboard = {
        actual = K.A & K.hold K.LeftCtrl |> tap_hold.to_json_value,
        expected = { tap = { key_code = 4 }, hold = { modifiers = 1 } },
      },

      check_json_value_tap_keyboard_hold_keyboard = {
        actual = K.A & K.hold K.LeftCtrl |> key.to_json_value,
        expected = { tap = { key_code = 4 }, hold = { modifiers = 1 } },
      },
    },

  tap_hold
    | doc "for key::tap_hold::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { hold = hold_key, ..tap_key } =>
            let valid_hold_key = key.key_validator hold_key in
            let valid_tap_key = key.key_validator tap_key in
            [valid_hold_key, valid_tap_key]
            |> match {
              ['Ok, 'Ok] => 'Ok,
              [err, _] => err,
              [_, err] => err,
            },
          _ => 'Error { message = "expected { hold = Key, ..tap_key }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun { hold = hold_key, ..tap_key } =>
        {
          hold = key.to_json_value hold_key,
          tap = key.to_json_value tap_key,
        },
    },

  chorded
    | doc "for key::chorded::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { chord = chord_key, passthrough = passthrough_key } =>
            let valid_chord_key = key.key_validator chord_key in
            let valid_passthrough_key = key.key_validator passthrough_key in
            [valid_chord_key, valid_passthrough_key]
            |> match {
              ['Ok, 'Ok] => 'Ok,
              [err, _] => err,
              [_, err] => err,
            },
          _ => 'Error { message = "expected { chord = Key, passthrough = Key }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun { chord = chord_key, passthrough = passthrough_key } =>
        {
          chord = key.to_json_value chord_key,
          passthrough = key.to_json_value passthrough_key,
        },
    },

  chorded_aux
    | doc "for key::chorded::AuxiliaryKey."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { passthrough = passthrough_key } => key.key_validator passthrough_key,
          _ => 'Error { message = "expected { passthrough = Key }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun { passthrough = passthrough_key } =>
        {
          passthrough = key.to_json_value passthrough_key,
        },
    },

  key = {
    Key = std.contract.from_validator key_validator,

    key_validator = fun k =>
      k
      |> validators.any_of [
        keyboard.key_validator,
        layer_modifier.key_validator,
        keymap_callback.key_validator,
        caps_word.key_validator,
        sticky.key_validator,
        custom.key_validator,
        layered.key_validator,
        tap_dance.key_validator,
        tap_hold.key_validator,
        chorded.key_validator,
        chorded_aux.key_validator,
      ],

    is_key = fun k => 'Ok == key_validator k,

    to_json_value = match {
      # Make key::chorded::Key
      k if chorded.is_key k => chorded.to_json_value k,
      # Make key::chorded::AuxiliaryKey
      k if chorded_aux.is_key k => chorded_aux.to_json_value k,
      # Make key::layered::LayeredKey
      k if layered.is_key k => layered.to_json_value k,
      # Make key::tap_dance::Key from keys with a "tap_dances" modifier.
      k if tap_dance.is_key k => tap_dance.to_json_value k,
      # Make key::tap_hold::Key from keys with a "hold" modifier.
      k if tap_hold.is_key k => tap_hold.to_json_value k,
      # Make key::callback::Key
      k if keymap_callback.is_key k => keymap_callback.to_json_value k,
      # Make key::caps_word::Key
      k if caps_word.is_key k => caps_word.to_json_value k,
      # Make key::sticky::Key
      k if sticky.is_key k => sticky.to_json_value k,
      # Make key::custom::Key
      k if custom.is_key k => custom.to_json_value k,
      # Otherwise, keys with just a base key_code are key::keyboard keys.
      k if keyboard.is_key k => keyboard.to_json_value k,
      # Make key::layered::ModifierKey
      k if layer_modifier.is_key k => layer_modifier.to_json_value k,
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
          [{ key, .. }] =>
            { chord = key, passthrough = k },
          [] =>
            if std.array.any (fun { indices, .. } => std.array.elem key_index indices) chords then
              { passthrough = k }
            else
              k,
          _ => std.fail_with "chorded_keys: multiple chords for key",
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
        keys = chorded_keys |> std.array.map key.to_json_value,
        config = config_json_value,
      },
}
