{
  validators,

  lib,

  json_keymap,

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
      let json = { Hold = [0, 0] } in
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
        # JSON serialization of key::layered::ModifierKey has variants: Default(layer), Hold([layer, kb_mods]), SetActiveLayers({ layers, mask? }).
        json_validator =
          validators.record.validator {
            fields_validator =
              validators.all_of [
                validators.record.has_any_field_of ["Default", "Hold", "Sticky", "SetActiveLayers", "Toggle"],
                validators.record.has_only_fields ["Default", "Hold", "Sticky", "SetActiveLayers", "Toggle"],
              ],
            field_validators =
              let modifier_bitset_validator =
                validators.record.validator {
                  fields_validator =
                    validators.all_of [
                      validators.record.has_any_field_of ["layers"],
                      validators.record.has_only_fields ["layers", "mask"],
                    ],
                  field_validators = {
                    layers = validators.is_number,
                    mask = validators.is_number,
                  },
                }
              in
              {
                Hold = fun hold =>
                  hold
                  |> match {
                    [layer_index, kb_mods] =>
                      validators.all_ok [
                        validators.is_number layer_index,
                        validators.is_number kb_mods,
                      ],
                    _ => 'Error { message = "expected Hold to be [layer_index, kb_mods]" },
                  },
                Toggle = validators.is_number,
                Sticky = validators.is_number,
                SetActiveLayers = modifier_bitset_validator,
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
            { Hold = [layer_index, kb_mods] } =>
              {
                include json,
                include module,
                include key_type,
                variant = "Hold",
                rust_expr =
                  let layer_index_expr = layer_index |> std.to_string in
                  let key_expr = "%{module}::ModifierKey::hold(%{layer_index_expr})" in
                  if kb_mods == 0 then
                    key_expr
                  else
                    let kmod_expr = "smart_keymap::key::KeyboardModifiers::from_byte(%{kb_mods |> std.to_string})" in
                    "(%{key_expr}).with_keyboard_modifiers(%{kmod_expr})",
              },
            { Toggle = layer_index } =>
              {
                include json,
                include module,
                include key_type,
                variant = "Toggle",
                rust_expr = "%{module}::ModifierKey::toggle(%{std.to_string layer_index})",
              },
            { Sticky = layer_index } =>
              {
                include json,
                include module,
                include key_type,
                variant = "Sticky",
                rust_expr = "%{module}::ModifierKey::sticky(%{std.to_string layer_index})",
              },
            { SetActiveLayers = { layers = active_layers_bitset } } =>
              {
                include json,
                include module,
                include key_type,
                variant = "SetActiveLayers",
                rust_expr =
                  let layers_expr = "%{module}::LayerBitset::from_bits(%{active_layers_bitset |> std.to_string})" in
                  "%{module}::ModifierKey::set_active_layers_from_bitset(%{layers_expr})",
              },
            { SetActiveLayers = { layers = active_layers_bitset, mask = layer_mask } } =>
              {
                include json,
                include module,
                include key_type,
                variant = "SetActiveLayers",
                rust_expr =
                  let layers_expr = "%{module}::LayerBitset::from_bits(%{active_layers_bitset |> std.to_string})" in
                  let mask_expr = "%{module}::LayerBitset::from_bits(%{layer_mask |> std.to_string})" in
                  "%{module}::ModifierKey::set_active_layers_from_bitset_with_mask(%{layers_expr}, %{mask_expr})",
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
        # Layer cells: null | { TransparentLayerExit = null } | nested key JSON
        json_validator =
          validators.record.validator {
            fields_validator = validators.record.has_exact_fields ["base", "layered"],
            field_validators = {
              base = smart_key.json_validator,
              layered =
                validators.array.validator (
                  validators.any_of [
                    validators.is_null,
                    fun json =>
                      json
                      |> match {
                        { TransparentLayerExit = null } => 'Ok,
                        { TransparentLayerExit = _ } => 'Ok,
                        _ => 'Error { message = "not tlex" },
                      },
                    smart_key.json_validator,
                  ]
                ),
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        # Layer cell JSON: null | { TransparentLayerExit = null } | nested key JSON
        is_tlex_json = fun json =>
          json
          |> match {
            { TransparentLayerExit = null } => true,
            { TransparentLayerExit = _ } => true,
            _ => false,
          },

        codegen_values = fun json @ { base, layered } =>
          let base_cv = base |> smart_key.codegen_values in
          let layered_cvs =
            std.array.map
              (fun json =>
                if json == null then
                  null
                else if is_tlex_json json then
                  {
                    kind = 'tlex,
                    rust_expr = "%{module}::LayeredBinding::TransparentLayerExit",
                    json = { TransparentLayerExit = null },
                  }
                else
                  (json |> smart_key.codegen_values) & { kind = 'key }
              )
              layered
          in
          let has_tlex =
            std.array.any
              (fun cv => cv != null && std.record.has_field "kind" cv && cv.kind == 'tlex)
              layered_cvs
          in

          {
            nested = {
              base = base_cv,
              layered = layered_cvs,
            },
            include json,
            include module,
            include key_type,
            include has_tlex,
            rust_expr =
              let base_expr = nested.base.rust_expr in
              let layered_exprs =
                nested.layered
                |> std.array.map (fun cv =>
                  if cv == null then
                    "None"
                  else if has_tlex then
                    if cv.kind == 'tlex then
                      "Some(%{cv.rust_expr})"
                    else
                      "Some(%{module}::LayeredBinding::Key(%{cv.rust_expr}))"
                  else
                    "Some(%{cv.rust_expr})"
                )
                |> std.string.join ","
              in
              if has_tlex then
                "smart_keymap::key::layered::LayeredKey::with_bindings(%{base_expr}, [%{layered_exprs}])"
              else
                "smart_keymap::key::layered::LayeredKey::new(%{base_expr}, [%{layered_exprs}])",
          },

        map_nested = fun f cv @ { nested, ..rest } =>
          rest
          & {
            nested = {
              base = f cv.nested.base,
              layered =
                std.array.map
                  (fun cv =>
                    if cv == null then
                      null
                    else if std.record.has_field "kind" cv && cv.kind == 'tlex then
                      cv
                    else
                      f cv
                  )
                  cv.nested.layered,
            },
          },

        # Traverse by visiting the layered key itself,
        #  then traversing the base, non-null nested layered keys (not tlex markers).
        traverse = fun f acc cv =>
          let acc = f acc cv in
          let acc = smart_key.traverse f acc cv.nested.base in
          let nn_layered_cvs =
            cv.nested.layered
            |> std.array.filter (fun nested_cv =>
              nested_cv != null
              && !(std.record.has_field "kind" nested_cv && nested_cv.kind == 'tlex)
            )
          in
          std.array.fold_left
            (fun acc nested_cv => smart_key.traverse f acc nested_cv)
            acc
            nn_layered_cvs,

        data_and_ref = fun key_data cv @ { nested = { base = base_cv, layered = layered_cvs }, has_tlex, .. } =>
          let { key_data, ref = base_ref } = smart_key.data_and_ref key_data base_cv in
          let base_ref = base_ref |> composite.ref.wrap in
          let { key_data, layered_refs } =
            std.array.fold_left
              (fun { key_data, layered_refs = parts } cv =>
                if cv == null then
                  { include key_data, layered_refs = std.array.append null parts }
                else if std.record.has_field "kind" cv && cv.kind == 'tlex then
                  {
                    include key_data,
                    layered_refs =
                      std.array.append
                        {
                          kind = 'tlex,
                          json = { TransparentLayerExit = null },
                          rust_expr = "%{module}::LayeredBinding::TransparentLayerExit",
                        }
                        parts,
                  }
                else
                  let { key_data, ref = layer_ref } = smart_key.data_and_ref key_data cv in
                  let layer_ref = layer_ref |> composite.ref.wrap in
                  {
                    include key_data,
                    layered_refs =
                      std.array.append (layer_ref & { kind = 'key }) parts,
                  }
              )
              { include key_data, layered_refs = [] }
              layered_cvs
          in
          let { layered = layered_, ..other_data } = key_data & { layered | default = [] } in
          let new_index = std.array.length layered_ in
          let cell_json = fun layer_ref =>
            if layer_ref == null then
              null
            else if layer_ref.kind == 'tlex then
              layer_ref.json
            else
              layer_ref.json
          in
          let cell_rust = fun layer_ref =>
            if layer_ref == null then
              "None"
            else if has_tlex then
              if layer_ref.kind == 'tlex then
                "Some(%{layer_ref.rust_expr})"
              else
                "Some(%{module}::LayeredBinding::Key(%{layer_ref.rust_expr}))"
            else
              "Some(%{layer_ref.rust_expr})"
          in
          let ctor =
            if has_tlex then
              "%{module}::LayeredKey::with_bindings"
            else
              "%{module}::LayeredKey::new"
          in
          let new_key = {
            json = {
              base = base_ref.json,
              layered = layered_refs |> std.array.map cell_json,
            },
            rust_expr = m%"
            %{ctor}(
              %{base_ref.rust_expr},
              [%{
                layered_refs
                |> std.array.map cell_rust
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

      config = {
        # Intermediate JSON / keymap.json shape (bitset form for if_layers).
        ConditionalLayerJson = {
          then_layer | Number,
          if_layers | Number,
        },

        Json = {
          sticky_timeout | optional | Number,
          conditional_layers | optional | Array ConditionalLayerJson,
        },

        expr =
          if std.record.has_field "layered" json_keymap.config then
            let c = json_keymap.config.layered in
            (
              if std.record.has_field "sticky_timeout" c then
                {
                  sticky_timeout = "Some(%{std.to_string c.sticky_timeout})",
                }
              else
                {}
            )
            & (
              if std.record.has_field "conditional_layers" c then
                let rules_fragment =
                  c.conditional_layers
                  |> std.array.map (fun { then_layer, if_layers } =>
                    "%{module}::ConditionalLayer::new(%{std.to_string then_layer}, %{module}::LayerBitset::from_bits(%{std.to_string if_layers}))"
                  )
                  |> std.string.join ", "
                in
                {
                  conditional_layers = "smart_keymap::slice::Slice::from_slice(&[%{rules_fragment}])",
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
          let layer_modifiers_data = (key_data_and_refs.key_data & { layer_modifiers | default = [] }).layer_modifiers in
          let layered_data = (key_data_and_refs.key_data & { layered | default = [] }).layered in
          m%"
          %{module}::System::new(
            %{layer_modifiers_data |> lib.array_rust_expr},
            %{layered_data |> lib.array_rust_expr},
          )
        "%,
      },
    },
}
