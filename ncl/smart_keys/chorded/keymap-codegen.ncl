{
  validators,

  lib,

  json_keymap,

  key_data_and_refs,

  smart_key,

  composite,

  smart_keymap.chorded
    | doc "for key::chorded::Key."
    = {
      module = "smart_keymap::key::chorded",

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        # JSON serialization of key::chorded::Key is { chords: [[id, key]], passthrough: key }
        json_validator =
          validators.record.validator {
            fields_validator = validators.record.has_exact_fields ["chords", "passthrough"],
            field_validators = {
              chords =
                validators.array.validator (validators.tuple.validator [validators.is_number, smart_key.json_validator]),
              passthrough = smart_key.json_validator,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        map_chords = fun f chords => chords |> std.array.map (fun [id, ch_json] => [id, ch_json |> f]),

        codegen_values = fun json @ { chords, passthrough } =>
          let chords_cv = chords |> map_chords (fun ch_json => ch_json |> smart_key.codegen_values) in
          let passthrough_cv = passthrough |> smart_key.codegen_values in

          {
            nested = {
              chords = chords_cv,
              passthrough = passthrough_cv,
            },
            include json,
            include module,
            include key_type,
            rust_expr =
              let chords_expr = m%"&[%{
              nested.chords
              |> std.array.map (fun [i, ch_cv] => "(%{i |> std.to_string}, %{ch_cv.rust_expr})")
              |> std.string.join ", "
            }]"%
              in
              let passthrough_expr = nested.passthrough.rust_expr in
              "smart_keymap::key::chorded::Key::new(%{chords_expr}, %{passthrough_expr})",
          },

        map_nested = fun f cv @ { nested, ..rest } =>
          rest
          & {
            nested = {
              chords = cv.nested.chords |> map_chords f,
              passthrough = f cv.nested.passthrough,
            },
          },

        # Traverse by visiting the chorded key,
        #  then traversing the chord keys, passthrough keys.
        traverse = fun f acc cv =>
          let acc = f acc cv in
          let chords_cvs = cv.nested.chords |> std.array.map (fun [_, ch_cv] => ch_cv) in
          let acc = std.array.fold_left (fun acc ch_cv => smart_key.traverse f acc ch_cv) acc chords_cvs in
          smart_key.traverse f acc cv.nested.passthrough,

        data_and_ref = fun key_data cv @ { nested = { chords = chords_cvs, passthrough = passthrough_cv }, .. } =>
          let { key_data, chord_indices_and_refs } =
            std.array.fold_left
              (fun { key_data, chord_indices_and_refs = parts } [chord_index, cv] =>
                let { key_data, ref = chord_ref } = smart_key.data_and_ref key_data cv in
                let chord_ref = chord_ref |> composite.ref.wrap in
                { include key_data, chord_indices_and_refs = std.array.append [chord_index, chord_ref] parts }
              )
              { include key_data, chord_indices_and_refs = [] }
              chords_cvs
          in
          let { key_data, ref = passthrough_ref } = smart_key.data_and_ref key_data passthrough_cv in
          let passthrough_ref = passthrough_ref |> composite.ref.wrap in
          let { chorded = chorded_, ..other_data } = key_data & { chorded | default = [] } in
          let new_index = std.array.length chorded_ in
          let new_key = {
            json = {
              chords = chord_indices_and_refs |> map_chords (fun chord_ref => chord_ref.json),
              passthrough = passthrough_ref.json,
            },
            rust_expr = m%"
            %{module}::Key::new(
              &[%{
                chord_indices_and_refs
                |> std.array.map (fun [i, chord_ref] => "(%{std.to_string i}, %{chord_ref.rust_expr})")
                |> std.string.join ", "
              }],
              %{passthrough_ref.rust_expr},
            )
          "%,
          }
          in
          {
            key_data = other_data & { chorded = std.array.append new_key chorded_ },
            ref = {
              include module,
              variant = "Chorded",
              json = { "%{variant}" = new_index },
              rust_expr = "%{module}::Ref::%{variant}(%{std.to_string new_index})",
            },
          },
      },

      auxiliary_key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::AuxiliaryKey",

        # JSON serialization of key::chorded::AuxiliaryKey is { passthrough: key }
        json_validator =
          validators.record.validator {
            fields_validator = validators.record.has_exact_fields ["passthrough"],
            field_validators = {
              passthrough = smart_key.json_validator,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json @ { passthrough } =>
          let passthrough_cv = passthrough |> smart_key.codegen_values in

          {
            nested = {
              passthrough = passthrough_cv,
            },
            include json,
            include module,
            include key_type,
            rust_expr =
              let passthrough_expr = nested.passthrough.rust_expr in
              "smart_keymap::key::chorded::AuxiliaryKey::new(%{passthrough_expr})",
          },

        map_nested = fun f cv @ { nested, ..rest } =>
          rest
          & {
            nested = {
              passthrough = f cv.nested.passthrough,
            },
          },

        # Traverse by visiting the chorded aux key,
        #  then traversing the passthrough key.
        traverse = fun f acc cv =>
          let acc = f acc cv in
          smart_key.traverse f acc cv.nested.passthrough,

        data_and_ref = fun key_data cv @ { nested = { passthrough = passthrough_cv }, .. } =>
          let { key_data, ref = passthrough_ref } = smart_key.data_and_ref key_data passthrough_cv in
          let passthrough_ref = passthrough_ref |> composite.ref.wrap in
          let { chorded_auxiliary = chorded_auxiliary_, ..other_data } = key_data & { chorded_auxiliary | default = [] } in
          let new_index = std.array.length chorded_auxiliary_ in
          let new_key = {
            json = {
              passthrough = passthrough_ref.json,
            },
            rust_expr = m%"
            %{module}::AuxiliaryKey::new(
              %{passthrough_ref.rust_expr},
            )
          "%,
          }
          in
          {
            key_data = other_data & { chorded_auxiliary = std.array.append new_key chorded_auxiliary_ },
            ref = {
              include module,
              variant = "Auxiliary",
              json = { "%{variant}" = new_index },
              rust_expr = "%{module}::Ref::%{variant}(%{std.to_string new_index})",
            },
          },
      },

      config = {
        ChordIndicesJson = Array Number,

        Json = {
          chords | optional | Array ChordIndicesJson,
          timeout | optional | Number,
        },

        rust_expr =
          if std.record.has_field "chorded" json_keymap.config then
            let chorded_config = json_keymap.config.chorded in
            let timeout_field_fragment =
              if std.record.has_field "timeout" chorded_config then
                "timeout: %{std.to_string chorded_config.timeout},"
              else
                ""
            in
            let chords_field_fragment =
              if std.record.has_field "chords" chorded_config then
                let chords_fragment =
                  chorded_config.chords
                  |> std.array.map (fun indices =>
                    "smart_keymap::key::chorded::ChordIndices::from_slice(&[%{indices |> std.array.map std.to_string |> std.string.join ", "}])"
                  )
                  |> std.string.join ","
                in
                "chords: smart_keymap::slice::Slice::from_slice(&[%{chords_fragment}]),"
              else
                ""
            in
            m%"
          smart_keymap::key::chorded::Config {
              %{timeout_field_fragment}
              %{chords_field_fragment}
              ..smart_keymap::key::chorded::DEFAULT_CONFIG
          }
        "%
          else
            "smart_keymap::key::chorded::DEFAULT_CONFIG",
      },

      system = {
        rust_expr =
          let chorded_key_data = (key_data_and_refs.key_data & { chorded | default = [] }).chorded in
          let auxiliary_key_data = (key_data_and_refs.key_data & { chorded_auxiliary | default = [] }).chorded_auxiliary in
          m%"
        smart_keymap::key::chorded::System::new(
            %{chorded_key_data |> lib.array_rust_expr},
            %{auxiliary_key_data |> lib.array_rust_expr},
        )
      "%,
      },
    },
}
