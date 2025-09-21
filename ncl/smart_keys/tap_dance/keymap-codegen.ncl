{
  validators,

  lib,

  json_keymap,

  key_data_and_refs,

  smart_key,

  composite,

  smart_keymap.tap_dance
    | doc "for key::tap_dance::Key."
    = {
      module = "smart_keymap::key::tap_dance",

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        # JSON serialization of key::tap_dance::Key is { definitions: [Option<json>] }
        #  and the codegen uses the from_definitions(&[json]) constructor.
        json_validator =
          validators.record.validator {
            fields_validator = validators.record.has_exact_fields ["definitions"],
            field_validators = {
              definitions = validators.array.validator smart_key.json_validator,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json @ { definitions } =>
          let definitions_cv = definitions |> std.array.map (fun json => json |> smart_key.codegen_values) in

          {
            nested = {
              definitions = definitions_cv,
            },
            include json,
            include module,
            include key_type,
            rust_expr =
              let definitions_expr = nested.definitions |> std.array.map (fun cv => cv.rust_expr) in
              let definitions_slice_expr = "&[%{definitions_expr |> std.string.join ", "}]" in
              "%{module}::Key::from_definitions(%{definitions_slice_expr})",
          },

        map_nested = fun f cv @ { nested, ..rest } =>
          rest
          & {
            nested = {
              definitions = std.array.map f cv.nested.definitions,
            },
          },

        # Traverse by visiting the tap_dance key,
        #  then traversing the definitions.
        traverse = fun f acc cv =>
          let acc = f acc cv in
          std.array.fold_left
            (fun acc nested_cv => smart_key.traverse f acc nested_cv)
            acc
            cv.nested.definitions,

        data_and_ref = fun key_data cv @ { nested = { definitions = definitions_cvs }, .. } =>
          let { key_data, definitions_refs } =
            std.array.fold_left
              (fun { key_data, definitions_refs = parts } cv =>
                let { key_data, ref = definition_ref } = smart_key.data_and_ref key_data cv in
                let definition_ref = definition_ref |> composite.ref.wrap in
                { include key_data, definitions_refs = std.array.append definition_ref parts }
              )
              { include key_data, definitions_refs = [] }
              definitions_cvs
          in
          let { tap_dance = tap_dance_, ..other_data } = key_data & { tap_dance | default = [] } in
          let new_index = std.array.length tap_dance_ in
          let new_key = {
            json = {
              definitions = definitions_refs |> std.array.map (fun { json, .. } => json),
            },
            rust_expr = m%"
            %{module}::Key::from_definitions(
              &[%{
                definitions_refs
                |> std.array.map (fun definition_ref => definition_ref.rust_expr)
                |> std.string.join ", "
              }],
            )
          "%,
          }
          in
          {
            key_data = other_data & { tap_dance = std.array.append new_key tap_dance_ },
            ref = {
              include module,
              variant = "TapDance",
              json = new_index,
              rust_expr = "%{module}::Ref(%{std.to_string new_index})",
            },
          },
      },

      config = {
        Json = {
          timeout | optional | Number,
        },

        rust_expr =
          if std.record.has_field "tap_dance" json_keymap.config then
            let tap_dance_config = json_keymap.config.tap_dance in
            let timeout_field_fragment =
              if std.record.has_field "timeout" tap_dance_config then
                "timeout: %{std.to_string tap_dance_config.timeout},"
              else
                ""
            in
            m%"
            smart_keymap::key::tap_dance::Config {
                %{timeout_field_fragment}
                ..smart_keymap::key::tap_dance::DEFAULT_CONFIG
            }
          "%
          else
            "smart_keymap::key::tap_dance::DEFAULT_CONFIG",
      },

      system = {
        rust_expr =
          let tap_dance_data = (key_data_and_refs.key_data & { tap_dance | default = [] }).tap_dance in
          "smart_keymap::key::tap_dance::System::new(%{tap_dance_data |> lib.array_rust_expr})",
      },
    },
}
