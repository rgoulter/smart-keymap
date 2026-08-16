{
  validators,

  lib,

  key_data_and_refs,

  key_output,

  smart_keymap.tri_state
    | doc "for key::tri_state::Key."
    = {
      module = "smart_keymap::key::tri_state",

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        json_validator =
          validators.record.validator {
            fields_validator = validators.record.has_exact_fields ["hold", "tap"],
            field_validators = {
              hold = key_output.json_validator,
              tap = key_output.json_validator,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json @ { hold, tap } =>
          {
            include json,
            include module,
            include key_type,
            rust_expr = "%{module}::Key::new(%{key_output.rust_expr hold}, %{key_output.rust_expr tap})",
          },

        traverse = fun f acc cv => f acc cv,

        data_and_ref = fun key_data cv =>
          let { tri_state = tri_state_, ..other_data } = key_data & { tri_state | default = [] } in
          let new_index = std.array.length tri_state_ in
          let new_key = {
            json = cv.json,
            rust_expr = cv.rust_expr,
          }
          in
          {
            key_data = other_data & { tri_state = std.array.append new_key tri_state_ },
            ref = {
              include module,
              json = new_index,
              rust_expr = "%{module}::Ref(%{std.to_string new_index})",
            },
          },
      },

      system = {
        rust_expr =
          let tri_state_data = (key_data_and_refs.key_data & { tri_state | default = [] }).tri_state in
          "%{module}::System::new(%{tri_state_data |> lib.array_rust_expr})",
      },
    },
}
