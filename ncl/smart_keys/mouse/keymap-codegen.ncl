{
  validators,
  lib,
  key_data_and_refs,

  smart_keymap.mouse
    | doc "for key::mouse::Key."
    = {
      module = "smart_keymap::key::mouse",

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        json_validator =
          validators.record.validator {
            fields_validator =
              validators.record.has_exact_fields ["mouse"],
            field_validators = {
              mouse =
                validators.any_of [
                  validators.is_string,
                  validators.record.validator {
                    fields_validator = validators.record.has_exact_fields ["Button"],
                    field_validators = { Button = validators.is_number },
                  },
                ],
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

        data_and_ref = fun key_data cv @ { json = { mouse }, .. } =>
          let { variant, rust_expr } =
            mouse
            |> match {
              { Button = b } => { variant = "Button", rust_expr = "%{module}::Ref::%{variant}(%{std.to_string b})" },
              s if std.is_string s => { variant = s, rust_expr = "%{module}::Ref::%{variant}" },
              _ => std.fail_with "bad mouse: %{std.serialize 'Json mouse}",
            }
          in
          {
            include key_data,
            ref = {
              include module,
              include variant,
              include rust_expr,
              json = mouse,
            },
          },
      },
    },
}
