let validators = import "validators.ncl" in
{
  KeymapKey = Dyn,

  keys
    | optional,

  layers
    | default
    | Array (Array KeymapKey)
    | doc "The layers of the keymap that gets rendered to JSON"
    = [keys],

  checks.nestable = {
    check_nestable_is_key_keyboard =
      nestable.is_key { key_code = 0x04 },

    check_nestable_to_json_serialized_keyboard = {
      actual = { key_code = 0x04 } |> nestable.to_json_serialized,
      expected = { key_code = 0x04 },
    },
  },

  nestable
    | doc "for key::composite::NestableKey implementors."
    = {
        key_validator
          = keyboard.key_validator,

        is_key
          = keyboard.is_key,

        to_json_serialized
          = match {
            k if keyboard.is_key k => keyboard.to_json_serialized k,
          },
      },

  checks.keyboard
    = let K = import "keys.ncl" in
      {
        keymap_example_key_code = {
          actual = K.A,
          expected = { key_code = 0x04 },
        },

        check_serialized_key_code = {
          actual = K.A |> key.to_json_serialized,
          expected = { key_code = 0x04 },
        },
      },

  keyboard
    | doc "for key::keyboard::Key."
    = {
        key_validator
          = validators.record.validator {
              fields_validator = validators.all_of [
                validators.record.has_any_field_of ["key_code", "modifiers"],
                validators.record.has_only_fields ["key_code", "modifiers"],
              ],
              field_validators = {
                key_code = validators.is_number,
                modifiers = validators.record.validator {
                  fields_validator = validators.record.has_only_fields [
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
              },
            },

        is_key
          = fun k => 'Ok == key_validator k,

        to_json_serialized
          = fun key => key,
      },

  checks.layer_modifier
    = let K = import "keys.ncl" in {
        keymap_example_hold = {
          actual = K.layer_mod.hold 0,
          expected = { layer_modifier.hold = 0 },
        },

        check_serialized_hold = {
          actual = K.layer_mod.hold 0 |> key.to_json_serialized,
          expected = { Hold = 0 },
        },
      },

  layer_modifier
    | doc "for key::layered::ModifierKey."
    = {
        key_validator
          = fun k => k |>
              match {
                { layer_modifier = { hold } } => validators.is_number hold,
                _ => 'Error { message = "expected { layer_modifier = { hold } }" },
              },

        is_key
          = fun k => 'Ok == key_validator k,

        to_json_serialized
          = fun { layer_modifier = { hold = hold_layer } } =>
              { Hold = hold_layer },
      },

  checks.layered
    = let K = import "keys.ncl" in
      {
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

        check_layered_base_keyboard_base_is_nestable_key =
          let key = K.A & { layered = [null, K.C] } in
          let { layered, ..base_key } = key in
          nestable.is_key base_key,

        check_layered_base_keyboard_is_key =
          let key = K.A & { layered = [null, K.C] } in
          layered.is_key key,

        check_serialized_layered_keyboard = {
          actual = K.A & { layered = [null, K.C] } |> key.to_json_serialized,
          expected = {
            base = { key_code = 0x04 },
            layered = [
              null,
              { key_code = 0x06 },
            ],
          },
        },
      },

  layered
    | doc "for key::layered::LayeredKey."
    = {
        key_validator
          = fun k => k |> match {
              { layered, ..base_key } =>
                let valid_layered =
                  layered |>
                    validators.array.validator (
                      validators.any_of [validators.is_null, nestable.key_validator]
                    ) in
                let valid_base_key = nestable.key_validator base_key in
                [valid_layered, valid_base_key] |> match {
                  ['Ok, 'Ok] => 'Ok,
                  [err, _] => err,
                  [_, err] => err,
                },
              _ => 'Error { message = "expected { layered = Array NestableKey, ..base_key }" },
            },

        is_key
          = fun k => 'Ok == key_validator k,

        to_json_serialized
          = fun { layered = layered_keys, ..base_key } =>
              {
                base = nestable.to_json_serialized base_key,
                layered =
                  std.array.map
                    (fun k => if k != null then nestable.to_json_serialized k else null)
                    layered_keys,
              },
      },

  checks.tap_hold
    = let K = import "keys.ncl" in
      {
        keymap_example_tap_keyboard_hold_keyboard = {
          actual = K.A & K.hold K.LeftCtrl,
          expected = { key_code = 0x04, hold = { modifiers = { left_ctrl = true } } },
        },

        check_tap_hold_tap_keyboard_is_nestable_key =
          let key = K.A & K.hold K.LeftCtrl  in
          let { hold, ..tap_key } = key in
          nestable.is_key tap_key,

        check_tap_keyboard_hold_keyboard_is_key =
          let key = K.A & K.hold K.LeftCtrl  in
          tap_hold.is_key key,

        check_tap_keyboard_hold_keyboard_to_json_serialized_keyboard = {
          actual = K.A & K.hold K.LeftCtrl |> tap_hold.to_json_serialized,
          expected = { tap = { key_code = 0x04 }, hold = { modifiers = { left_ctrl = true } } },
        },

        check_serialized_tap_keyboard_hold_keyboard = {
          actual = K.A & K.hold K.LeftCtrl |> key.to_json_serialized,
          expected = { tap = { key_code = 0x04 }, hold = { modifiers = { left_ctrl = true } } },
        },
      },

  tap_hold
    | doc "for key::tap_hold::Key."
    = {
        key_validator
          = fun k => k |>
              match {
                { hold = hold_key, ..tap_key } =>
                  let valid_hold_key = nestable.key_validator hold_key in
                  let valid_tap_key = nestable.key_validator tap_key in
                  [valid_hold_key, valid_tap_key] |> match {
                    ['Ok, 'Ok] => 'Ok,
                    [err, _] => err,
                    [_, err] => err,
                  },
                _ => 'Error { message = "expected { hold = NestableKey, ..tap_key }" },
              },

        is_key
          = fun k => 'Ok == key_validator k,

        to_json_serialized
          = fun { hold = hold_key, ..tap_key } =>
              {
                hold = nestable.to_json_serialized hold_key,
                tap = nestable.to_json_serialized tap_key,
              },
      },

  key
    = {
        to_json_serialized
          = match {
              # Make key::layered::LayeredKey
              k if layered.is_key k => layered.to_json_serialized k,
              # Make key::tap_hold::Key from keys with a "hold" modifier.
              k if tap_hold.is_key k => tap_hold.to_json_serialized k,
              # Otherwise, keys with just a base key_code are key::keyboard keys.
              k if keyboard.is_key k => keyboard.to_json_serialized k,
              # Make key::layered::ModifierKey
              k if layer_modifier.is_key k => layer_modifier.to_json_serialized k,
              # Null values (None) stay null
              null => null,
              _ => std.fail_with "unsupported item in keymap.ncl",
            },
      },

  to_composite_json_serialized_key
    | doc "Constructs a JSON-representable composite::Key value from the given value from keymap.ncl."
    = match {
        # Make key::layered::LayeredKey
        k if layered.is_key k =>
          { Layered = key.to_json_serialized k },
        # Make key::tap_hold::Key from keys with a "hold" modifier.
        k if tap_hold.is_key k =>
          { TapHold = key.to_json_serialized k },
        # Otherwise, keys with just a base key_code are key::keyboard keys.
        k if keyboard.is_key k =>
          { Keyboard = key.to_json_serialized k },
        # Make key::layered::ModifierKey
        k if layer_modifier.is_key k =>
          { LayerModifier = key.to_json_serialized k },
        # Null values (None) stay null
        null => null,
        _ => std.fail_with "unsupported item in keymap.ncl",
      },

  layered_keys
    | doc "Constructs array of key::layered::LayeredKey values from layers."
    = if std.array.length layers == 0 then
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
    = layered_keys |> std.array.map key.to_json_serialized,

  serialized_json_composite_keys
    | doc "The keymap.json output value."
    = layered_keys |> std.array.map to_composite_json_serialized_key,
}
