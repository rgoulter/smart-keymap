{
  validators,
  lib,
  key_data_and_refs,

  keyboard_modifiers,

  smart_keymap.mouse
    | doc "for key::mouse::Key."
    = {
      module = "smart_keymap::key::mouse",

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        mouse_field_validator =
          validators.any_of [
            validators.is_string,
            validators.record.validator {
              fields_validator = validators.record.has_exact_fields ["Button"],
              field_validators = { Button = validators.is_number },
            },
          ],

        json_validator =
          validators.record.validator {
            fields_validator =
              validators.all_of [
                validators.record.has_any_field_of ["mouse"],
                validators.record.has_only_fields ["mouse", "modifiers"],
              ],
            field_validators = {
              mouse = mouse_field_validator,
              modifiers = keyboard_modifiers.json_validator,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json =>
          {
            include json,
            include module,
            include key_type,
          },

        traverse = fun f acc cv => f acc cv,

        action_expr = fun mouse =>
          mouse
          |> match {
            { Button = b } => "%{module}::Action::Button(%{std.to_string b})",
            s if std.is_string s => "%{module}::Action::%{s}",
            _ => std.fail_with "bad mouse: %{std.serialize 'Json mouse}",
          },

        simple_ref = fun mouse =>
          mouse
          |> match {
            { Button = b } =>
              {
                variant = "Button",
                rust_expr = "%{module}::Ref::Button(%{std.to_string b})",
                json = { Button = b },
              },
            s if std.is_string s =>
              {
                variant = s,
                rust_expr = "%{module}::Ref::%{s}",
                json = s,
              },
            _ => std.fail_with "bad mouse: %{std.serialize 'Json mouse}",
          },

        data_and_ref = fun key_data cv @ { json, .. } =>
          json
          |> match {
            { mouse, modifiers = mods } if mods != 0 =>
              let { mouse = mouse_, ..other_data } = key_data & { mouse | default = [] } in
              let new_index = std.array.length mouse_ in
              let action = action_expr mouse in
              let new_key = {
                json = {
                  action = mouse,
                  modifiers = mods,
                },
                rust_expr = m%"
                %{module}::Key {
                  action: %{action},
                  modifiers: %{keyboard_modifiers.rust_expr mods},
                }
              "%,
              }
              in
              {
                key_data = other_data & { mouse = std.array.append new_key mouse_ },
                ref = {
                  include module,
                  variant = "Key",
                  json = { "%{variant}" = new_index },
                  rust_expr = "%{module}::Ref::%{variant}(%{std.to_string new_index})",
                },
              },
            { mouse, .. } =>
              let { variant, rust_expr, json = ref_json } = simple_ref mouse in
              {
                include key_data,
                ref = {
                  include module,
                  include variant,
                  include rust_expr,
                  json = ref_json,
                },
              },
          },
      },

      system = {
        rust_expr =
          let mouse_data = (key_data_and_refs.key_data & { mouse | default = [] }).mouse in
          "%{module}::System::new(%{mouse_data |> lib.array_rust_expr})",
      },
    },
}
