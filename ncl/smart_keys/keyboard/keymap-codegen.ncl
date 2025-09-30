{
  validators,

  lib,

  key_data_and_refs,

  keyboard_modifiers,

  checks.check_keyboard = {
    # Use a JSON value to check the "is" predicate.
    check_json_is =
      let json = { key_code = 4 } in
      smart_keymap.keyboard.key.is_json json,
    check_json_with_modifier_is =
      let json = { key_code = 4, modifiers = 1 } in
      smart_keymap.keyboard.key.is_json json,
    check_json_modifiers_is =
      let json = { modifiers = 1 } in
      smart_keymap.keyboard.key.is_json json,

    check_keyboard_codegen_values = {
      check_basic =
        let json = { key_code = 4 } in
        let cv = smart_keymap.keyboard.key.codegen_values json in
        {
          check_module_type = {
            actual = cv.key_type,
            expected = "smart_keymap::key::keyboard::Key",
          },
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "smart_keymap::key::keyboard::Key::new(4)",
          },
        },
    },
  },

  smart_keymap.keyboard
    | doc "for key::keyboard::Key."
    = {
      module = "smart_keymap::key::keyboard",

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        # c.f. doc_de_keyboard.md.
        json_validator =
          validators.record.validator {
            fields_validator =
              validators.all_of [
                validators.record.has_any_field_of ["key_code", "modifiers"],
                validators.record.has_only_fields ["key_code", "modifiers"],
              ],
            field_validators = {
              key_code = validators.is_number,
              modifiers = keyboard_modifiers.json_validator,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json =>
          {
            include json,
            include module,
            include key_type,
            rust_expr =
              json
              |> match {
                { key_code } =>
                  "smart_keymap::key::keyboard::Key::new(%{std.to_string key_code})",
                { modifiers } =>
                  "smart_keymap::key::keyboard::Key::from_modifiers(%{keyboard_modifiers.rust_expr modifiers})",
                { key_code, modifiers } =>
                  "smart_keymap::key::keyboard::Key::new_with_modifiers(%{std.to_string key_code}, %{keyboard_modifiers.rust_expr modifiers})",
              },
          },

        traverse = fun f acc cv => f acc cv,

        data_and_ref = fun key_data cv @ { json, .. } =>
          json
          |> match {
            { key_code } =>
              {
                include key_data,
                ref = {
                  include module,
                  variant = "KeyCode",
                  json = { "%{variant}" = key_code },
                  rust_expr = "%{module}::Ref::%{variant}(%{std.to_string key_code})",
                },
              },
            { modifiers } =>
              {
                include key_data,
                ref = {
                  include module,
                  variant = "Modifiers",
                  json = { "%{variant}" = modifiers },
                  rust_expr = "%{module}::Ref::%{variant}(%{std.to_string modifiers})",
                },
              },
            { key_code, modifiers } =>
              let { keyboard = keyboard_, ..other_data } = key_data & { keyboard | default = [] } in
              let new_index = std.array.length keyboard_ in
              let new_key = {
                json = {
                  include key_code,
                  include modifiers,
                },
                rust_expr = m%"
                %{module}::Key {
                  key_code: %{std.to_string key_code},
                  modifiers: %{keyboard_modifiers.rust_expr modifiers},
                }
              "%,
              }
              in
              {
                key_data = other_data & { keyboard = std.array.append new_key keyboard_ },
                ref = {
                  include module,
                  variant = "KeyCodeAndModifier",
                  json = { "%{variant}" = new_index },
                  rust_expr = "%{module}::Ref::%{variant}(%{std.to_string new_index})",
                },
              },
          },
      },

      system = {
        rust_expr =
          let keyboard_data = (key_data_and_refs.key_data & { keyboard | default = [] }).keyboard in
          "%{module}::System::new(%{keyboard_data |> lib.array_rust_expr})",
      },
    },
}
