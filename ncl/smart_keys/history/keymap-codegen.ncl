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
            json if std.is_record json =>
              validators.record.validator
                {
                  fields_validator =
                    validators.all_of [
                      validators.record.has_all_fields ["default"],
                      validators.record.has_only_fields ["default", "rules"],
                    ],
                  field_validators = {
                    default = key_output.json_validator,
                    rules = validators.array.validator alt_repeat_rule.json_validator,
                  },
                }
                json,
            _ => 'Error { message = "Expected \"Repeat\", \"AltRepeat\", or adaptive { default, rules }" },
          },

        is_json = fun json => 'Ok == json_validator json,

        key_output_rust_expr = fun json =>
          if json == {} then
            "smart_keymap::key::KeyOutput::NO_OUTPUT"
          else
            key_output.rust_expr json,

        adaptive_rust_expr = fun json =>
          let default_expr = key_output_rust_expr json.default in
          let rules =
            if std.record.has_field "rules" json then json.rules else []
          in
          let rules_expr =
            rules
            |> std.array.map alt_repeat_rule.rust_expr
            |> std.string.join ", "
          in
          m%"%{module}::AdaptiveKey {
            default: %{default_expr},
            rules: %{module}::adaptive_rules([%{rules_expr}]),
          }"%,

        codegen_values = fun json =>
          json
          |> match {
            "Repeat" =>
              {
                include json,
                include module,
                include key_type,
                rust_expr = "%{module}::Key::Repeat",
              },
            "AltRepeat" =>
              {
                include json,
                include module,
                include key_type,
                rust_expr = "%{module}::Key::AltRepeat",
              },
            _ =>
              {
                include json,
                include module,
                include key_type,
                rust_expr = adaptive_rust_expr json,
              },
          },

        traverse = fun f acc cv => f acc cv,

        data_and_ref = fun key_data cv @ { json, rust_expr, .. } =>
          json
          |> match {
            "Repeat" =>
              {
                include key_data,
                ref = {
                  include module,
                  include json,
                  rust_expr = "%{module}::Ref(%{module}::Key::Repeat)",
                },
              },
            "AltRepeat" =>
              {
                include key_data,
                ref = {
                  include module,
                  include json,
                  rust_expr = "%{module}::Ref(%{module}::Key::AltRepeat)",
                },
              },
            _ =>
              let { history = history_, ..other_data } = key_data & { history | default = [] } in
              let new_index = std.array.length history_ in
              let new_key = {
                include json,
                include rust_expr,
              }
              in
              {
                key_data = other_data & { history = std.array.append new_key history_ },
                ref = {
                  include module,
                  json = { Adaptive = new_index },
                  rust_expr = "%{module}::Ref(%{module}::Key::Adaptive(%{std.to_string new_index}))",
                },
              },
          },
      },

      system = {
        rust_expr =
          let history_data = (key_data_and_refs.key_data & { history | default = [] }).history in
          "%{module}::System::new(%{history_data |> lib.array_rust_expr})",
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
