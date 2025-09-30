{
  validators,

  lib,

  json_keymap,

  key_data_and_refs,

  smart_key,

  composite,

  checks.check_tap_hold = {
    # Use a json value to check the "is" predicate.
    check_json_tap_keyboard_hold_keyboard_is =
      let json = {
        hold = { key_code = 224 },
        tap = { key_code = 4 },
      }
      in
      smart_keymap.tap_hold.key.is_json json,

    check_taphold_codegen_values = {
      # Current impl: tap_hold::Key<keyboard::Key> is still th::Key<composite::Base>
      check_taphold_keyboard =
        let json = {
          hold = { key_code = 224 },
          tap = { key_code = 4 },
        }
        in
        let cv = smart_keymap.tap_hold.key.codegen_values json in
        {
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "smart_keymap::key::tap_hold::Key::new(smart_keymap::key::keyboard::Key::new(4), smart_keymap::key::keyboard::Key::new(224))",
          },
        },
    },
  },

  smart_keymap.tap_hold
    | doc "for key::tap_hold::Key."
    = {
      TapHoldInterruptResponseJson =
        std.contract.from_validator (
          validators.is_elem_of [
            "HoldOnKeyPress",
            "HoldOnKeyTap",
            "Ignore",
          ]
        ),

      module = "smart_keymap::key::tap_hold",

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        # c.f. doc_de_tap_hold.md.
        # JSON serialization of key::tap_hold::Key is { tap: key, hold: key }
        json_validator =
          validators.record.validator {
            fields_validator = validators.record.has_exact_fields ["tap", "hold"],
            field_validators = {
              tap = smart_key.json_validator,
              hold = smart_key.json_validator,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json @ { hold, tap } =>
          let hold_cv = hold |> smart_key.codegen_values in
          let tap_cv = tap |> smart_key.codegen_values in

          {
            nested = {
              tap = tap_cv,
              hold = hold_cv,
            },
            include json,
            include module,
            include key_type,
            rust_expr =
              let hold_expr = nested.hold.rust_expr in
              let tap_expr = nested.tap.rust_expr in
              "%{module}::Key::new(%{tap_expr}, %{hold_expr})",
          },

        map_nested = fun f cv @ { nested, ..rest } =>
          rest
          & {
            nested = {
              tap = f cv.nested.tap,
              hold = f cv.nested.hold,
            },
          },

        # Traverse by visiting the tap_hold key,
        #  then traversing the tap, hold keys.
        traverse = fun f acc cv =>
          let acc = f acc cv in
          let acc = smart_key.traverse f acc cv.nested.tap in
          smart_key.traverse f acc cv.nested.hold,

        data_and_ref = fun key_data cv @ { nested = { tap = tap_cv, hold = hold_cv }, .. } =>
          let { key_data, ref = tap_ref } = smart_key.data_and_ref key_data tap_cv in
          let tap_ref = tap_ref |> composite.ref.wrap in
          let { key_data, ref = hold_ref } = smart_key.data_and_ref key_data hold_cv in
          let hold_ref = hold_ref |> composite.ref.wrap in
          let { tap_hold = tap_hold_, ..other_data } = key_data & { tap_hold | default = [] } in
          let new_index = std.array.length tap_hold_ in
          let new_key = {
            json = {
              tap = tap_ref.json,
              hold = hold_ref.json,
            },
            rust_expr = m%"
            %{module}::Key {
              tap: %{tap_ref.rust_expr},
              hold: %{hold_ref.rust_expr},
            }
          "%,
          }
          in
          {
            key_data = other_data & { tap_hold = std.array.append new_key tap_hold_ },
            ref = {
              include module,
              json = new_index,
              rust_expr = "%{module}::Ref(%{std.to_string new_index})",
            },
          },
      },

      config = {
        Json = {
          timeout | optional | Number,
          interrupt_response | optional | TapHoldInterruptResponseJson,
          required_idle_time | optional | Number,
        },

        rust_expr =
          if std.record.has_field "tap_hold" json_keymap.config then
            let tap_hold_config = json_keymap.config.tap_hold in
            let timeout_field_fragment =
              if std.record.has_field "timeout" tap_hold_config then
                "timeout: %{std.to_string tap_hold_config.timeout},"
              else
                ""
            in
            let interrupt_response_field_fragment =
              if std.record.has_field "interrupt_response" tap_hold_config then
                "interrupt_response: %{module}::InterruptResponse::%{tap_hold_config.interrupt_response},"
              else
                ""
            in
            let required_idle_time_field_fragment =
              if std.record.has_field "required_idle_time" tap_hold_config then
                "required_idle_time: Some(%{std.to_string tap_hold_config.required_idle_time}),"
              else
                ""
            in
            m%"
            %{module}::Config {
                %{timeout_field_fragment}
                %{interrupt_response_field_fragment}
                %{required_idle_time_field_fragment}
                ..%{module}::Config::new()
            }
          "%
          else
            "%{module}::Config::new()",
      },

      system = {
        rust_expr =
          let tap_hold_data = (key_data_and_refs.key_data & { tap_hold | default = [] }).tap_hold in
          "%{module}::System::new(%{tap_hold_data |> lib.array_rust_expr})",
      },
    },
}
