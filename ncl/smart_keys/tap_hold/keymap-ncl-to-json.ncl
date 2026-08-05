{
  validators,

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
        keymap_ncl.key.is_key tap_key,

      check_tap_keyboard_hold_keyboard_is_key =
        let key = K.A & K.hold K.LeftCtrl in
        keymap_ncl.tap_hold.is_key key,

      check_tap_keyboard_hold_keyboard_to_json_json_value_keyboard = {
        actual = K.A & K.hold K.LeftCtrl |> keymap_ncl.tap_hold.to_json_value,
        expected = { tap = { key_code = 4 }, hold = { modifiers = 1 } },
      },

      check_json_value_tap_keyboard_hold_keyboard = {
        actual = K.A & K.hold K.LeftCtrl |> keymap_ncl.key.to_json_value,
        expected = { tap = { key_code = 4 }, hold = { modifiers = 1 } },
      },

      # Autoshift is tap-hold sugar: tap key / hold Shift+key.
      check_autoshift_expands_to_tap_hold = {
        actual = K.autoshift K.A,
        expected = K.A & K.hold (K.A & K.LeftShift),
      },

      check_autoshift_is_tap_hold_key =
        keymap_ncl.tap_hold.is_key (K.autoshift K.A),

      check_autoshift_to_json = {
        actual = K.autoshift K.A |> keymap_ncl.key.to_json_value,
        expected = {
          tap = { key_code = 4 },
          hold = { key_code = 4, modifiers = 2 },
        },
      },

      check_autoshift_as_expands_to_tap_hold = {
        actual = K.autoshift_as K.Comma K.Semicolon,
        expected = K.Comma & K.hold K.Semicolon,
      },

      check_autoshift_as_to_json = {
        actual = K.autoshift_as K.Comma K.Semicolon |> keymap_ncl.key.to_json_value,
        expected = {
          tap = { key_code = 54 },
          hold = { key_code = 51 },
        },
      },
    },

  keymap_ncl.tap_hold
    | doc "for key::tap_hold::Key."
    = {
      TapHoldInterruptResponse =
        std.contract.from_validator (
          validators.is_elem_of [
            "HoldOnKeyPress",
            "HoldOnKeyTap",
            "Ignore",
          ]
        ),

      Config = {
        timeout
          | optional
          | std.contract.from_validator (
            validators.any_of [
              validators.is_null,
              validators.is_number,
            ]
          ),
        interrupt_response | optional | TapHoldInterruptResponse,
        required_idle_time | optional | Number,
      },

      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { hold = hold_key, ..tap_key } =>
            validators.all_ok [
              keymap_ncl.key.key_validator hold_key,
              keymap_ncl.key.key_validator tap_key,
            ],
          _ => 'Error { message = "expected { hold = Key, ..tap_key }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun { hold = hold_key, ..tap_key } =>
        {
          hold = keymap_ncl.key.to_json_value hold_key,
          tap = keymap_ncl.key.to_json_value tap_key,
        },

      map_accum = fun f acc k @ { hold = hold_key, ..tap_key } =>
        let { acc, k = tap_key } = f acc tap_key in
        let { acc, k = hold_key } = f acc hold_key in
        {
          include acc,
          k =
            {
              hold = hold_key,
            }
            & tap_key,
        },
    },
}
