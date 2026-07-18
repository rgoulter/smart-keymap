{
  validators,
  lib,
  key_data_and_refs,

  keyboard_modifiers,

  smart_keymap.consumer
    | doc "for key::consumer::Key."
    = {
      module = "smart_keymap::key::consumer",

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        json_validator =
          validators.record.validator {
            fields_validator =
              validators.all_of [
                validators.record.has_any_field_of ["consumer_code"],
                validators.record.has_only_fields ["consumer_code", "modifiers"],
              ],
            field_validators = {
              consumer_code = validators.is_number,
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

        data_and_ref = fun key_data cv @ { json, .. } =>
          json
          |> match {
            { consumer_code, modifiers = mods } if mods != 0 =>
              let { consumer = consumer_, ..other_data } = key_data & { consumer | default = [] } in
              let new_index = std.array.length consumer_ in
              let new_key = {
                json = {
                  usage_code = consumer_code,
                  modifiers = mods,
                },
                rust_expr = m%"
                %{module}::Key {
                  usage_code: %{std.to_string consumer_code},
                  modifiers: %{keyboard_modifiers.rust_expr mods},
                }
              "%,
              }
              in
              {
                key_data = other_data & { consumer = std.array.append new_key consumer_ },
                ref = {
                  include module,
                  variant = "Key",
                  json = { "%{variant}" = new_index },
                  rust_expr = "%{module}::Ref::%{variant}(%{std.to_string new_index})",
                },
              },
            { consumer_code, .. } =>
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

      system = {
        rust_expr =
          let consumer_data = (key_data_and_refs.key_data & { consumer | default = [] }).consumer in
          "%{module}::System::new(%{consumer_data |> lib.array_rust_expr})",
      },
    },
}
