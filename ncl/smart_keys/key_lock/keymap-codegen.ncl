{
  validators,

  lib,

  key_data_and_refs,

  smart_keymap.key_lock
    | doc "for key::key_lock::Key."
    = {
      module = "smart_keymap::key::key_lock",

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        json_validator = fun json =>
          json
          |> match {
            "KeyLock" => 'Ok,
            _ => 'Error { message = "Expected \"KeyLock\"" },
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
