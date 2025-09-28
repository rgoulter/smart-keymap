{
  validators,

  lib,

  key_data_and_refs,

  smart_key,

  composite,

  checks.check_layered = {
    # Use a json value to check the "is" predicate.
    check_json_base_keyboard_layered_keyboard_is =
      let json = {
        base = { key_code = 4 },
        layered = [
          null,
          { key_code = 6 },
        ]
      }
      in
      smart_keymap.layered.layered_key.is_json json,

    check_layered_codegen_values = {
      # A layered::LayeredKey of keyboard::Key
      #  codegens to layered::LayeredKey<keyboard::Key>
      check_layered_keyboard =
        let json = {
          base = { key_code = 4 },
          layered = [
            null,
            { key_code = 6 },
          ]
        }
        in
        let cv = smart_keymap.layered.layered_key.codegen_values json in
        {
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "smart_keymap::key::layered::LayeredKey::new(smart_keymap::key::keyboard::Key::new(4), [None,Some(smart_keymap::key::keyboard::Key::new(6))])",
          },
        },

      check_layered_composite_taphold_keyboard =
        let json = {
          base = {
            hold = { key_code = 224 },
            tap = { key_code = 4 },
          },
          layered = [
            null,
            { key_code = 6 },
          ]
        }
        in
        let cv = smart_keymap.layered.layered_key.codegen_values json in
        {
          check_rust_expr = {
            actual = cv.rust_expr,
            expected = "smart_keymap::key::layered::LayeredKey::new(smart_keymap::key::tap_hold::Key::new(smart_keymap::key::keyboard::Key::new(4), smart_keymap::key::keyboard::Key::new(224)), [None,Some(smart_keymap::key::keyboard::Key::new(6))])",
          },
        },
    },
  },

  checks.layer_modifier_is = {
    # Use a json value to check the "is" predicate.
    check_json_is_hold =
      let json = { Hold = 0 } in
      smart_keymap.layered.modifier_key.is_json json,
    check_json_is_toggle =
      let json = { Toggle = 1 } in
      smart_keymap.layered.modifier_key.is_json json,
  },

  smart_keymap.layered
    | doc "for key::layered::LayeredKey."
    = {
      module = "smart_keymap::key::layered",

      modifier_key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::ModifierKey",

        # c.f. doc_de_layered.md.
        # JSON serialization of key::layered::ModifierKey has variants: Default(layer), Hold(layer), SetActiveLayers([layer, ...]).
        json_validator =
          validators.record.validator {
            fields_validator =
              validators.all_of [
                validators.record.has_any_field_of ["Default", "Hold", "SetActiveLayers", "Toggle"],
                validators.record.has_only_fields ["Default", "Hold", "SetActiveLayers", "Toggle"],
              ],
            field_validators = {
              Hold = validators.is_number,
              Toggle = validators.is_number,
              SetActiveLayers = validators.is_number,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json =>
          json
          |> match {
            { Default = layer_index } =>
              {
                include json,
                include module,
                include key_type,
                variant = "Default",
                rust_expr = "%{module}::ModifierKey::default(%{std.to_string layer_index})",
              },
            { Hold = layer_index } =>
              {
                include json,
                include module,
                include key_type,
                variant = "Hold",
                rust_expr = "%{module}::ModifierKey::hold(%{std.to_string layer_index})",
              },
            { Toggle = layer_index } =>
              {
                include json,
                include module,
                include key_type,
                variant = "Toggle",
                rust_expr = "%{module}::ModifierKey::toggle(%{std.to_string layer_index})",
              },
            { SetActiveLayers = active_layers_bitset } =>
              {
                include json,
                include module,
                include key_type,
                variant = "SetActiveLayers",
                rust_expr = "%{module}::ModifierKey::set_active_layers_from_bitset(%{active_layers_bitset |> std.to_string})",
              },
          },

        traverse = fun f acc cv => f acc cv,

        data_and_ref = fun key_data cv @ { json, .. } =>
          let { layer_modifiers = layer_modifiers_, ..other_data } = key_data & { layer_modifiers | default = [] } in
          let new_index = std.array.length layer_modifiers_ in
          let new_key = {
            include json,
            rust_expr = cv.rust_expr,
          }
          in
          {
            key_data = other_data & { layer_modifiers = std.array.append new_key layer_modifiers_ },
            ref = {
              include module,
              variant = "Modifier",
              json = { "%{variant}" = new_index },
              rust_expr = "%{module}::Ref::%{variant}(%{std.to_string new_index})",
            },
          },
      },

      layered_key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::LayeredKey",

        # c.f. doc_de_layered.md.
        # e.g.:
        # ```
        #   {
        #     "base": 4,
        #     "layered": [5, null, 7]
        #   }
        # ```
        # JSON serialization of key::layered::Layered has fields: { base, layered }
        json_validator =
          validators.record.validator {
            fields_validator = validators.record.has_exact_fields ["base", "layered"],
            field_validators = {
              base = smart_key.json_validator,
              layered =
                validators.array.validator (
                  validators.any_of [
                    validators.is_null,
                    smart_key.json_validator,
                  ]
                ),
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json @ { base, layered } =>
          let base_cv = base |> smart_key.codegen_values in
          let non_null_layered = std.array.filter ((!=) null) layered in
          let layered_cvs =
            std.array.map (fun json => if json != null then json |> smart_key.codegen_values else null) layered
          in
          let nn_layered_cvs =
            std.array.filter ((!=) null) layered_cvs
          in

          {
            nested = {
              base = base_cv,
              layered = layered_cvs,
            },
            include json,
            include module,
            include key_type,
            rust_expr =
              let base_expr = nested.base.rust_expr in
              let layered_exprs =
                nested.layered
                |> std.array.map (fun cv => if cv == null then "None" else "Some(%{cv.rust_expr})")
                |> std.string.join ","
              in
              "smart_keymap::key::layered::LayeredKey::new(%{base_expr}, [%{layered_exprs}])",
          },

        map_nested = fun f cv @ { nested, ..rest } =>
          rest
          & {
            nested = {
              base = f cv.nested.base,
              layered = std.array.map (fun cv => if cv != null then f cv else null) cv.nested.layered,
            },
          },

        # Traverse by visiting the layered key itself,
        #  then traversing the base, non-null nested layered keys.
        traverse = fun f acc cv =>
          let acc = f acc cv in
          let acc = smart_key.traverse f acc cv.nested.base in
          let nn_layered_cvs =
            std.array.filter ((!=) null) cv.nested.layered
          in
          std.array.fold_left
            (fun acc nested_cv => smart_key.traverse f acc nested_cv)
            acc
            nn_layered_cvs,

        data_and_ref = fun key_data cv @ { nested = { base = base_cv, layered = layered_cvs }, .. } =>
          let { key_data, ref = base_ref } = smart_key.data_and_ref key_data base_cv in
          let base_ref = base_ref |> composite.ref.wrap in
          let { key_data, layered_refs } =
            std.array.fold_left
              (fun { key_data, layered_refs = parts } cv =>
                if cv == null then
                  { include key_data, layered_refs = std.array.append null parts }
                else
                  let { key_data, ref = layer_ref } = smart_key.data_and_ref key_data cv in
                  let layer_ref = layer_ref |> composite.ref.wrap in
                  { include key_data, layered_refs = std.array.append layer_ref parts }
              )
              { include key_data, layered_refs = [] }
              layered_cvs
          in
          let { layered = layered_, ..other_data } = key_data & { layered | default = [] } in
          let new_index = std.array.length layered_ in
          let new_key = {
            json = {
              base = base_ref.json,
              layered = layered_refs |> std.array.map (fun layer_ref => if layer_ref == null then null else layer_ref.json),
            },
            rust_expr = m%"
            %{module}::LayeredKey::new(
              %{base_ref.rust_expr},
              [%{
                layered_refs
                |> std.array.map (fun layer_ref => if layer_ref == null then "None" else "Some(%{layer_ref.rust_expr})")
                |> std.string.join ", "
              }],
            )
          "%,
          }
          in
          {
            key_data = other_data & { layered = std.array.append new_key layered_ },
            ref = {
              include module,
              variant = "Layered",
              json = { "%{variant}" = new_index },
              rust_expr = "%{module}::Ref::%{variant}(%{std.to_string new_index})",
            },
          },
      },

      system = {
        rust_expr =
          let layer_modifiers_data = (key_data_and_refs.key_data & { layer_modifiers | default = [] }).layer_modifiers in
          let layered_data = (key_data_and_refs.key_data & { layered | default = [] }).layered in
          m%"
          smart_keymap::key::layered::System::new(
            %{layer_modifiers_data |> lib.array_rust_expr},
            %{layered_data |> lib.array_rust_expr},
          )
        "%,
      },
    },
}
