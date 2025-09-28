{
  validators,

  lib,

  key_data_and_refs,

  smart_keymap.custom
    | doc "for key::custom::Key."
    = {
      module = "smart_keymap::key::custom",

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        json_validator =
          validators.record.validator {
            fields_validator = validators.record.has_exact_fields ["custom"],
            field_validators = {
              custom = validators.is_number,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json @ { custom } =>
          {
            include json,
            include module,
            include key_type,
            rust_expr = "%{module}::Key::new(%{std.to_string custom})",
          },

        traverse = fun f acc cv => f acc cv,

        data_and_ref = fun key_data cv @ { json = { custom }, .. } =>
          {
            include key_data,
            ref = {
              include module,
              json = custom,
              rust_expr = "%{module}::Ref(%{custom |> std.to_string})",
            },
          },
      },
    },
}
