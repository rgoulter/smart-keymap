{
  validators,

  lib,

  json_keymap,

  key_data_and_refs,

  smart_key,

  composite,

  smart_keymap.sequence
    | doc "for key::sequence::Key."
    = {
      module = "smart_keymap::key::sequence",

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        # JSON: { sequences: [[id, key]], passthrough: key }
        json_validator =
          validators.record.validator {
            fields_validator = validators.record.has_exact_fields ["sequences", "passthrough"],
            field_validators = {
              sequences =
                validators.array.validator (validators.tuple.validator [validators.is_number, smart_key.json_validator]),
              passthrough = smart_key.json_validator,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        map_sequences = fun f sequences => sequences |> std.array.map (fun [id, ch_json] => [id, ch_json |> f]),

        codegen_values = fun json @ { sequences, passthrough } =>
          let sequences_cv = sequences |> map_sequences (fun ch_json => ch_json |> smart_key.codegen_values) in
          let passthrough_cv = passthrough |> smart_key.codegen_values in

          {
            nested = {
              sequences = sequences_cv,
              passthrough = passthrough_cv,
            },
            include json,
            include module,
            include key_type,
            rust_expr =
              let sequences_expr = m%"&[%{
              nested.sequences
              |> std.array.map (fun [i, ch_cv] => "(%{i |> std.to_string}, %{ch_cv.rust_expr})")
              |> std.string.join ", "
            }]"%
              in
              let passthrough_expr = nested.passthrough.rust_expr in
              "%{module}::Key::new(%{sequences_expr}, %{passthrough_expr})",
          },

        map_nested = fun f cv @ { nested, ..rest } =>
          rest
          & {
            nested = {
              sequences = cv.nested.sequences |> map_sequences f,
              passthrough = f cv.nested.passthrough,
            },
          },

        traverse = fun f acc cv =>
          let acc = f acc cv in
          let sequences_cvs = cv.nested.sequences |> std.array.map (fun [_, ch_cv] => ch_cv) in
          let acc = std.array.fold_left (fun acc ch_cv => smart_key.traverse f acc ch_cv) acc sequences_cvs in
          smart_key.traverse f acc cv.nested.passthrough,

        data_and_ref = fun key_data cv @ { nested = { sequences = sequences_cvs, passthrough = passthrough_cv }, .. } =>
          let { key_data, sequence_indices_and_refs } =
            std.array.fold_left
              (fun { key_data, sequence_indices_and_refs = parts } [sequence_index, cv] =>
                let { key_data, ref = sequence_ref } = smart_key.data_and_ref key_data cv in
                let sequence_ref = sequence_ref |> composite.ref.wrap in
                { include key_data, sequence_indices_and_refs = std.array.append [sequence_index, sequence_ref] parts }
              )
              { include key_data, sequence_indices_and_refs = [] }
              sequences_cvs
          in
          let { key_data, ref = passthrough_ref } = smart_key.data_and_ref key_data passthrough_cv in
          let passthrough_ref = passthrough_ref |> composite.ref.wrap in
          let { sequence = sequence_, ..other_data } = key_data & { sequence | default = [] } in
          let new_index = std.array.length sequence_ in
          let new_key = {
            json = {
              sequences = sequence_indices_and_refs |> map_sequences (fun sequence_ref => sequence_ref.json),
              passthrough = passthrough_ref.json,
            },
            rust_expr = m%"
            %{module}::Key::new(
              &[%{
                sequence_indices_and_refs
                |> std.array.map (fun [i, sequence_ref] => "(%{std.to_string i}, %{sequence_ref.rust_expr})")
                |> std.string.join ", "
              }],
              %{passthrough_ref.rust_expr},
            )
          "%,
          }
          in
          {
            key_data = other_data & { sequence = std.array.append new_key sequence_ },
            ref = {
              include module,
              variant = "Sequence",
              json = { "%{variant}" = new_index },
              rust_expr = "%{module}::Ref::%{variant}(%{std.to_string new_index})",
            },
          },
      },

      auxiliary_key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::AuxiliaryKey",

        # Distinct from chorded aux `{ passthrough }` so is_json dispatch is unambiguous.
        json_validator =
          validators.record.validator {
            fields_validator = validators.record.has_exact_fields ["sequence_passthrough"],
            field_validators = {
              sequence_passthrough = smart_key.json_validator,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json @ { sequence_passthrough } =>
          let passthrough_cv = sequence_passthrough |> smart_key.codegen_values in

          {
            nested = {
              passthrough = passthrough_cv,
            },
            include json,
            include module,
            include key_type,
            rust_expr =
              let passthrough_expr = nested.passthrough.rust_expr in
              "%{module}::AuxiliaryKey::new(%{passthrough_expr})",
          },

        map_nested = fun f cv @ { nested, ..rest } =>
          rest
          & {
            nested = {
              passthrough = f cv.nested.passthrough,
            },
          },

        traverse = fun f acc cv =>
          let acc = f acc cv in
          smart_key.traverse f acc cv.nested.passthrough,

        data_and_ref = fun key_data cv @ { nested = { passthrough = passthrough_cv }, .. } =>
          let { key_data, ref = passthrough_ref } = smart_key.data_and_ref key_data passthrough_cv in
          let passthrough_ref = passthrough_ref |> composite.ref.wrap in
          let { sequence_auxiliary = sequence_auxiliary_, ..other_data } = key_data & { sequence_auxiliary | default = [] } in
          let new_index = std.array.length sequence_auxiliary_ in
          # key_data JSON uses `passthrough` so it deserializes as
          # sequence::AuxiliaryKey (serde). The keymap keys array still
          # uses sequence_passthrough for is_json discrimination vs chorded.
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
            key_data = other_data & { sequence_auxiliary = std.array.append new_key sequence_auxiliary_ },
            ref = {
              include module,
              variant = "Auxiliary",
              json = { "%{variant}" = new_index },
              rust_expr = "%{module}::Ref::%{variant}(%{std.to_string new_index})",
            },
          },
      },

      start_key = {
        Json = std.contract.from_validator json_validator,

        # Distinct from sequence.key.key_type so codegen_module_for_key_type is unique.
        key_type = "%{module}::SequenceStart",

        json_validator = fun json =>
          json
          |> match {
            "SequenceStart" => 'Ok,
            _ => 'Error { message = "Expected \"SequenceStart\"" },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json =>
          {
            include json,
            include module,
            include key_type,
            rust_expr = "%{module}::Ref::SequenceStart",
          },

        traverse = fun f acc cv => f acc cv,

        data_and_ref = fun key_data cv @ { json, .. } =>
          {
            include key_data,
            ref = {
              include module,
              include json,
              rust_expr = "%{module}::Ref::SequenceStart",
            },
          },
      },

      config = {
        SequenceIndicesJson = Array Number,

        Json = {
          sequences | optional | Array SequenceIndicesJson,
          required_idle_time | optional | Number,
          timeout | optional | Number,
        },

        expr =
          if std.record.has_field "sequence" json_keymap.config then
            let c = json_keymap.config.sequence in
            (
              if std.record.has_field "required_idle_time" c then
                {
                  required_idle_time = "Some(%{std.to_string c.required_idle_time})",
                }
              else
                {}
            )
            & (
              if std.record.has_field "timeout" c then
                {
                  timeout = "%{std.to_string c.timeout}",
                }
              else
                {}
            )
            & (
              if std.record.has_field "sequences" c then
                let sequences_fragment =
                  c.sequences
                  |> std.array.map (fun indices =>
                    "%{module}::SequenceIndices::from_slice(&[%{indices |> std.array.map std.to_string |> std.string.join ", "}])"
                  )
                  |> std.string.join ","
                in
                {
                  sequences = "smart_keymap::slice::Slice::from_slice(&[%{sequences_fragment}])",
                }
              else
                {}
            )
          else
            {},

        rust_expr = lib.config_rust_expr module expr,
      },

      system = {
        rust_expr =
          let sequence_key_data = (key_data_and_refs.key_data & { sequence | default = [] }).sequence in
          let auxiliary_key_data = (key_data_and_refs.key_data & { sequence_auxiliary | default = [] }).sequence_auxiliary in
          m%"
        %{module}::System::new(
            %{sequence_key_data |> lib.array_rust_expr},
            %{auxiliary_key_data |> lib.array_rust_expr},
        )
      "%,
      },
    },
}
