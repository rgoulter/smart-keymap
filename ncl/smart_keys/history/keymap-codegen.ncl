{
  validators,

  lib,

  key_data_and_refs,

  json_keymap,

  key_output,

  smart_keymap.history
    | doc "for key::history::Key."
    = {
      module = "smart_keymap::key::history",

      alt_repeat_rule = {
        json_validator =
          validators.record.validator {
            fields_validator = validators.record.has_exact_fields ["prev", "emit"],
            field_validators = {
              prev = key_output.json_validator,
              emit = key_output.json_validator,
            },
          },

        rust_expr = fun { prev, emit } =>
          m%"%{module}::AltRepeatRule {
            prev: %{key_output.rust_expr prev},
            emit: %{key_output.rust_expr emit},
          }"%,
      },

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        json_validator = fun json =>
          json
          |> match {
            "Repeat" => 'Ok,
            "AltRepeat" => 'Ok,
            _ => 'Error { message = "Expected \"Repeat\" or \"AltRepeat\"" },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json =>
          {
            include json,
            include module,
            include key_type,
            rust_expr = "%{module}::Key::%{json}",
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

      config = {
        Json = {
          alt_repeat | optional | Array Dyn,
        },

        expr =
          if std.record.has_field "history" json_keymap.config then
            let c = json_keymap.config.history in
            (
              if std.record.has_field "alt_repeat" c then
                {
                  alt_repeat = m%"%{module}::alt_repeat_rules([
                   %{c.alt_repeat |> std.array.map alt_repeat_rule.rust_expr |> std.string.join ", "}
                ])"%,
                }
              else
                {}
            )
          else
            {},

        rust_expr = lib.config_rust_expr module expr,
      },
    },
}
