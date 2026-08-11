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
      # Nested keys are validated/codegen'd via smart_key (keyboard, etc.).
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

      check_taphold_with_profile =
        let json = {
          hold = { key_code = 224 },
          tap = { key_code = 4 },
          profile = 1,
        }
        in
        let cv = smart_keymap.tap_hold.key.codegen_values json in
        {
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "smart_keymap::key::tap_hold::Key::with_profile(smart_keymap::key::keyboard::Key::new(4), smart_keymap::key::keyboard::Key::new(224), 1)",
          },
        },

      check_profile_with_hold_triggers =
        let rust_expr =
          smart_keymap.tap_hold.profile_rust_expr {
            interrupt_response = "HoldOnKeyPress",
            hold_trigger_key_positions = [2, 3],
          }
        in
        {
          check_rust_expr = {
            actual = rust_expr,
            expected = m%"smart_keymap::key::tap_hold::Profile {
            hold_trigger_positions: Some(smart_keymap::slice::Slice::from_slice(&[2, 3])),
            interrupt_response: smart_keymap::key::tap_hold::InterruptResponse::HoldOnKeyPress,
            ..smart_keymap::key::tap_hold::Profile::new()
          }"%,
          },
        },

      check_profile_with_retro_tap =
        let rust_expr =
          smart_keymap.tap_hold.profile_rust_expr {
            interrupt_response = "HoldOnKeyPress",
            retro_tap = true,
          }
        in
        {
          check_rust_expr = {
            actual = rust_expr,
            expected = m%"smart_keymap::key::tap_hold::Profile {
            interrupt_response: smart_keymap::key::tap_hold::InterruptResponse::HoldOnKeyPress,
            retro_tap: true,
            ..smart_keymap::key::tap_hold::Profile::new()
          }"%,
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

      hold_trigger_positions_expr = fun positions =>
        let idxs =
          positions
          |> std.array.map (fun i => "%{std.to_string i}")
          |> std.string.join ", "
        in
        "Some(smart_keymap::slice::Slice::from_slice(&[%{idxs}]))",

      profile_field_expr = fun c =>
        (
          if std.record.has_field "timeout" c then
            {
              timeout =
                if c.timeout == null then
                  "None"
                else
                  "Some(%{std.to_string c.timeout})",
            }
          else
            {}
        )
        & (
          if std.record.has_field "interrupt_response" c then
            {
              interrupt_response = "%{module}::InterruptResponse::%{c.interrupt_response}",
            }
          else
            {}
        )
        & (
          if std.record.has_field "required_idle_time" c then
            {
              required_idle_time = "Some(%{std.to_string c.required_idle_time})",
            }
          else
            {}
        )
        & (
          if std.record.has_field "hold_trigger_key_positions" c then
            {
              hold_trigger_positions = hold_trigger_positions_expr c.hold_trigger_key_positions,
            }
          else
            {}
        )
        & (
          if std.record.has_field "retro_tap" c then
            {
              retro_tap = if c.retro_tap then "true" else "false",
            }
          else
            {}
        ),

      profile_rust_expr = fun c =>
        let fields = profile_field_expr c in
        if fields == {} then
          "%{module}::Profile::new()"
        else
          let field_lines =
            fields
            |> std.record.to_array
            |> std.array.map (fun { field, value } => "%{field}: %{value},")
            |> std.string.join "\n"
          in
          m%"%{module}::Profile {
            %{field_lines}
            ..%{module}::Profile::new()
          }"%,

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        # JSON: { tap, hold, profile? }
        json_validator =
          validators.record.validator {
            fields_validator =
              validators.all_of [
                validators.record.has_all_fields ["tap", "hold"],
                validators.record.has_only_fields ["tap", "hold", "profile"],
              ],
            field_validators = {
              tap = smart_key.json_validator,
              hold = smart_key.json_validator,
              profile = validators.is_number,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json @ { hold, tap, ..rest } =>
          let hold_cv = hold |> smart_key.codegen_values in
          let tap_cv = tap |> smart_key.codegen_values in
          let profile =
            if std.record.has_field "profile" rest then
              rest.profile
            else
              0
          in

          {
            nested = {
              tap = tap_cv,
              hold = hold_cv,
            },
            include profile,
            include json,
            include module,
            include key_type,
            rust_expr =
              let hold_expr = nested.hold.rust_expr in
              let tap_expr = nested.tap.rust_expr in
              if profile == 0 then
                "%{module}::Key::new(%{tap_expr}, %{hold_expr})"
              else
                "%{module}::Key::with_profile(%{tap_expr}, %{hold_expr}, %{std.to_string profile})",
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

        data_and_ref = fun key_data cv @ {
          nested = { tap = tap_cv, hold = hold_cv },
          profile = profile_id,
          ..
        } =>
          let { key_data, ref = tap_ref } = smart_key.data_and_ref key_data tap_cv in
          let tap_ref = tap_ref |> composite.ref.wrap in
          let { key_data, ref = hold_ref } = smart_key.data_and_ref key_data hold_cv in
          let hold_ref = hold_ref |> composite.ref.wrap in
          let { tap_hold = tap_hold_, ..other_data } = key_data & { tap_hold | default = [] } in
          let new_index = std.array.length tap_hold_ in
          let profile_json =
            if profile_id == 0 then
              {}
            else
              { profile = profile_id }
          in
          let new_key = {
            json =
              {
                tap = tap_ref.json,
                hold = hold_ref.json,
              }
              & profile_json,
            rust_expr = m%"
            %{module}::Key {
              tap: %{tap_ref.rust_expr},
              hold: %{hold_ref.rust_expr},
              profile: %{std.to_string profile_id},
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
        ProfileJson = {
          timeout
            | optional
            | std.contract.from_validator (
              validators.any_of [
                validators.is_null,
                validators.is_number,
              ]
            ),
          interrupt_response | optional | TapHoldInterruptResponseJson,
          required_idle_time | optional | Number,
          hold_trigger_key_positions | optional | Array Number,
          retro_tap | optional | Bool,
        },

        # Lowered JSON form: nested default_profile + profiles array.
        Json = {
          default_profile | optional | ProfileJson,
          # Lowered form: array of extra profiles (index 1..).
          profiles | optional | Array ProfileJson,
        },

        expr =
          if std.record.has_field "tap_hold" json_keymap.config then
            let c = json_keymap.config.tap_hold in
            (
              if std.record.has_field "default_profile" c then
                { default_profile = profile_rust_expr c.default_profile }
              else
                {}
            )
            & (
              if std.record.has_field "profiles" c && c.profiles != [] then
                let profiles_fragment =
                  c.profiles
                  |> std.array.map (fun p => profile_rust_expr p)
                  |> std.string.join ", "
                in
                {
                  profiles = "smart_keymap::slice::Slice::from_slice(&[%{profiles_fragment}])",
                }
              else
                {}
            )
          else
            {},

        rust_expr = lib.config_rust_expr module expr,
      },

      system = {
        rust_expr =
          let tap_hold_data = (key_data_and_refs.key_data & { tap_hold | default = [] }).tap_hold in
          "%{module}::System::new(%{tap_hold_data |> lib.array_rust_expr})",
      },
    },
}
