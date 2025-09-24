{
  validators,
  lib,
  key_data_and_refs,

  smart_keymap.consumer
    | doc "for key::consumer::Key."
    = {
      module = "smart_keymap::key::consumer",

      key = {
        Json = { consumer_code | Number, .. },

        key_type = "%{module}::Key",

        json_validator =
          validators.record.validator {
            fields_validator =
              validators.record.has_exact_fields ["consumer_code"],
            field_validators = {
              consumer_code = validators.is_number,
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

        data_and_ref = fun key_data cv @ { json = { consumer_code }, .. } =>
          {
            include key_data,
            ref = {
              include module,
              variant = "UsageCode",
              json = { "%{variant}" = consumer_code },
              rust_expr = "%{module}::Ref::%{variant}(%{std.to_string consumer_code})",
            },
          },
      },
    },
}
