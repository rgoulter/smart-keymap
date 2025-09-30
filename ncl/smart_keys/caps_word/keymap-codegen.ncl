{
  validators,

  lib,

  key_data_and_refs,

  smart_keymap.caps_word
    | doc "for key::caps_word::Key."
    = {
      module = "smart_keymap::key::caps_word",

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        json_validator = fun json =>
          json
          |> match {
            "ToggleCapsWord" => 'Ok,
            _ => 'Error { message = "Expected \"ToggleCapsWord\"" },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json =>
          {
            include json,
            include module,
            include key_type,
            rust_expr = "%{module}::Key::new()",
          },

        traverse = fun f acc cv => f acc cv,

        data_and_ref = fun key_data cv @ { json, .. } =>
          {
            include key_data,
            ref = {
              include module,
              include json,
              rust_expr = "%{module}::Ref(%{module}::Key::%{json})",
            },
          },
      },
    },
}
