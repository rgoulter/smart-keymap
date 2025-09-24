{
  validators,
  lib,
  key_data_and_refs,

  smart_keymap.consumer
    | doc "for key::consumer::Key."
    = {
      module = "smart_keymap::key::consumer",

      key = {
        Json = { usage_code | Number, .. },

        key_type = "%{module}::Key",

        json_validator =
          validators.record.validator {
            fields_validator =
              validators.record.has_exact_fields ["usage_code"],
            field_validators = {
              usage_code = validators.is_number,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json =>
          {
            json = json,
            include module,
            include key_type,
            rust_expr = "smart_keymap::key::consumer::Key { usage_code: %{std.to_string json.usage_code} }",
          },

        traverse = fun f acc cv => f acc cv,

        data_and_ref = fun key_data cv @ { json = { usage_code }, .. } =>
          {
            include key_data,
            ref = {
              include module,
              variant = "UsageCode",
              json = { "%{variant}" = usage_code },
              rust_expr = "%{module}::Ref::%{variant}(%{std.to_string usage_code})",
            },
          },
      },

      system = {
        rust_expr = "smart_keymap::key::consumer::System::new()",
      },
    },
}
