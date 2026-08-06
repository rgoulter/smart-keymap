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
        retro_tap | optional | Bool,
      },

      Key = std.contract.from_validator key_validator,

      # Strip optional positional-hold metadata before validating the tap key.
      strip_hold_meta = fun r =>
        if std.record.has_field "hold_trigger_key_positions" r then
          std.record.remove "hold_trigger_key_positions" r
        else
          r,

      key_validator = fun k =>
        k
        |> match {
          { hold = hold_key, ..rest } =>
            let tap_key = strip_hold_meta rest in
            let positions_ok =
              if std.record.has_field "hold_trigger_key_positions" rest then
                if std.is_array rest.hold_trigger_key_positions then
                  'Ok
                else
                  'Error { message = "hold_trigger_key_positions must be an array" }
              else
                'Ok
            in
            validators.all_ok [
              keymap_ncl.key.key_validator hold_key,
              keymap_ncl.key.key_validator tap_key,
              positions_ok,
            ],
          _ => 'Error { message = "expected { hold = Key, ..tap_key }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun { hold = hold_key, ..rest } =>
        let hold_triggers =
          if std.record.has_field "hold_trigger_key_positions" rest then
            { hold_trigger_key_positions = rest.hold_trigger_key_positions }
          else
            {}
        in
        let tap_key = strip_hold_meta rest in
        {
          hold = keymap_ncl.key.to_json_value hold_key,
          tap = keymap_ncl.key.to_json_value tap_key,
        }
        & hold_triggers,

      map_accum = fun f acc { hold = hold_key, ..rest } =>
        let hold_triggers =
          if std.record.has_field "hold_trigger_key_positions" rest then
            { hold_trigger_key_positions = rest.hold_trigger_key_positions }
          else
            {}
        in
        let tap_key = strip_hold_meta rest in
        let { acc, k = tap_key } = f acc tap_key in
        let { acc, k = hold_key } = f acc hold_key in
        {
          include acc,
          k =
            {
              hold = hold_key,
            }
            & hold_triggers
            & tap_key,
        },
    },
}
