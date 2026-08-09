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

      check_profile_name_to_json = {
        actual =
          K.A
          & K.hold K.LeftCtrl
          & { tap_hold_profile = 1 }
          |> keymap_ncl.tap_hold.to_json_value,
        expected = {
          tap = { key_code = 4 },
          hold = { modifiers = 1 },
          profile = 1,
        },
      },

      check_default_profile_omitted_from_json = {
        actual =
          K.A
          & K.hold K.LeftCtrl
          & { tap_hold_profile = 0 }
          |> keymap_ncl.tap_hold.to_json_value,
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

      # One behavior profile. Profile 0 is Config's default (flat on config.tap_hold);
      # extras live under config.tap_hold.profiles.
      Profile = {
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

      # Authoring keeps default-profile knobs flat on config.tap_hold;
      # lowering nests them as default_profile for JSON / Rust Config.
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
        # Authoring: name → profile record. Lowered to a JSON array (indices 1..).
        profiles | optional | { _ | Profile },
      },

      Key = std.contract.from_validator key_validator,

      # Optional profile selector metadata on the key (not part of the tap leaf).
      # After name→index resolution: number. Before: profile name string or number.
      strip_profile_meta = fun r =>
        if std.record.has_field "tap_hold_profile" r then
          std.record.remove "tap_hold_profile" r
        else
          r,

      key_validator = fun k =>
        k
        |> match {
          { hold = hold_key, ..rest } =>
            let tap_key = strip_profile_meta rest in
            let profile_ok =
              if std.record.has_field "tap_hold_profile" rest then
                let p = rest.tap_hold_profile in
                if std.is_number p || std.is_string p then
                  'Ok
                else
                  'Error {
                    message = "tap_hold_profile must be a profile name (string) or index (number)",
                  }
              else
                'Ok
            in
            validators.all_ok [
              keymap_ncl.key.key_validator hold_key,
              keymap_ncl.key.key_validator tap_key,
              profile_ok,
            ],
          _ => 'Error { message = "expected { hold = Key, ..tap_key }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun { hold = hold_key, ..rest } =>
        let tap_key = strip_profile_meta rest in
        let profile_json =
          if std.record.has_field "tap_hold_profile" rest then
            let p = rest.tap_hold_profile in
            if std.is_number p && p != 0 then
              { profile = p }
            else if std.is_string p then
              std.fail_with "tap_hold_profile name \"%{p}\" must be resolved to an index before to_json_value"
            else
              {}
          else
            {}
        in
        {
          hold = keymap_ncl.key.to_json_value hold_key,
          tap = keymap_ncl.key.to_json_value tap_key,
        }
        & profile_json,

      map_accum = fun f acc { hold = hold_key, ..rest } =>
        let profile_meta =
          if std.record.has_field "tap_hold_profile" rest then
            { tap_hold_profile = rest.tap_hold_profile }
          else
            {}
        in
        let tap_key = strip_profile_meta rest in
        let { acc, k = tap_key } = f acc tap_key in
        let { acc, k = hold_key } = f acc hold_key in
        {
          include acc,
          k =
            {
              hold = hold_key,
            }
            & profile_meta
            & tap_key,
        },
    },
}
