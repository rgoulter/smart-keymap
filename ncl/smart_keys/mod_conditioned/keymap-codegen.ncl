{
  validators,

  lib,

  json_keymap,

  key_data_and_refs,

  smart_key,

  composite,

  keyboard_modifiers,

  smart_keymap.mod_conditioned
    | doc "for key::mod_conditioned::Key."
    = {
      module = "smart_keymap::key::mod_conditioned",

      key = {
        Json = std.contract.from_validator json_validator,

        key_type = "%{module}::Key",

        # JSON: { base, morphed, mods, keep_mods? }
        json_validator =
          validators.record.validator {
            fields_validator =
              validators.all_of [
                validators.record.has_all_fields ["base", "morphed", "mods"],
                validators.record.has_only_fields ["base", "morphed", "mods", "keep_mods"],
              ],
            field_validators = {
              base = smart_key.json_validator,
              morphed = smart_key.json_validator,
              mods = keyboard_modifiers.json_validator,
              keep_mods = keyboard_modifiers.json_validator,
            },
          },

        is_json = fun json => 'Ok == json_validator json,

        codegen_values = fun json @ { base, morphed, mods, .. } =>
          let keep_mods = if std.record.has_field "keep_mods" json then json.keep_mods else 0 in
          let base_cv = base |> smart_key.codegen_values in
          let morphed_cv = morphed |> smart_key.codegen_values in
          {
            nested = {
              base = base_cv,
              morphed = morphed_cv,
            },
            include json,
            include module,
            include key_type,
            keep_mods_byte = keep_mods,
            rust_expr =
              let base_expr = nested.base.rust_expr in
              let morphed_expr = nested.morphed.rust_expr in
              let mods_expr = keyboard_modifiers.rust_expr mods in
              let keep_expr = keyboard_modifiers.rust_expr keep_mods in
              "%{module}::Key::new(%{base_expr}, %{morphed_expr}, %{mods_expr}, %{keep_expr})",
          },

        map_nested = fun f cv @ { nested, ..rest } =>
          rest
          & {
            nested = {
              base = f cv.nested.base,
              morphed = f cv.nested.morphed,
            },
          },

        traverse = fun f acc cv =>
          let acc = f acc cv in
          let acc = smart_key.traverse f acc cv.nested.base in
          smart_key.traverse f acc cv.nested.morphed,

        data_and_ref = fun key_data cv @ { nested = { base = base_cv, morphed = morphed_cv }, json = key_json, .. } =>
          let { key_data, ref = base_ref } = smart_key.data_and_ref key_data base_cv in
          let base_ref = base_ref |> composite.ref.wrap in
          let { key_data, ref = morphed_ref } = smart_key.data_and_ref key_data morphed_cv in
          let morphed_ref = morphed_ref |> composite.ref.wrap in
          let { mod_conditioned = mc_, ..other_data } = key_data & { mod_conditioned | default = [] } in
          let new_index = std.array.length mc_ in
          let trigger_mods_byte = key_json.mods in
          let keep_mods_byte =
            if std.record.has_field "keep_mods" key_json then
              key_json.keep_mods
            else
              0
          in
          let new_key = {
            json = {
              base = base_ref.json,
              morphed = morphed_ref.json,
              mods = trigger_mods_byte,
              keep_mods = keep_mods_byte,
            },
            rust_expr = m%"
            %{module}::Key {
              base: %{base_ref.rust_expr},
              morphed: %{morphed_ref.rust_expr},
              trigger_mods: %{keyboard_modifiers.rust_expr trigger_mods_byte},
              keep_mods: %{keyboard_modifiers.rust_expr keep_mods_byte},
            }
          "%,
          }
          in
          {
            key_data = other_data & { mod_conditioned = std.array.append new_key mc_ },
            ref = {
              include module,
              json = new_index,
              rust_expr = "%{module}::Ref(%{std.to_string new_index})",
            },
          },
      },

      system = {
        rust_expr =
          let mc_data =
            (key_data_and_refs.key_data & { mod_conditioned | default = [] }).mod_conditioned
          in
          "%{module}::System::new(%{mc_data |> lib.array_rust_expr})",
      },
    },
}
