{
  validators,

  keyboard_modifiers,

  checks.mod_conditioned =
    let K = import "keys.ncl" in
    {
      check_record_is_key =
        let key = {
          base = K.Backspace,
          morphed = K.Delete,
          mods = { left_shift = true },
        }
        in
        keymap_ncl.mod_conditioned.is_key key,

      check_record_to_json = {
        actual =
          {
            base = K.Backspace,
            morphed = K.Delete,
            mods = { left_shift = true },
          }
          |> keymap_ncl.mod_conditioned.to_json_value,
        expected = {
          base = { key_code = 42 },
          morphed = { key_code = 76 },
          mods = 2,
        },
      },

      check_gresc_is_key = keymap_ncl.mod_conditioned.is_key K.gresc,

      check_bspc_del_to_json = {
        actual = K.bspc_del |> keymap_ncl.mod_conditioned.to_json_value,
        expected = {
          base = { key_code = 42 },
          morphed = { key_code = 76 },
          mods = 34,
          keep_mods = 32,
        },
      },
    },

  keymap_ncl.mod_conditioned
    | doc "for key::mod_conditioned::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator =
        validators.record.validator {
          fields_validator =
            validators.all_of [
              validators.record.has_all_fields ["base", "morphed", "mods"],
              validators.record.has_only_fields ["base", "morphed", "mods", "keep_mods"],
            ],
          field_validators = {
            base = keymap_ncl.key.key_validator,
            morphed = keymap_ncl.key.key_validator,
            mods = keyboard_modifiers.validator,
            keep_mods = keyboard_modifiers.validator,
          },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun k @ { base = base_key, morphed = morphed_key, mods = trigger_mods, ..rest } =>
        {
          base = keymap_ncl.key.to_json_value base_key,
          morphed = keymap_ncl.key.to_json_value morphed_key,
          mods = keyboard_modifiers.to_json_value trigger_mods,
        }
        & (
          if std.record.has_field "keep_mods" rest then
            { keep_mods = keyboard_modifiers.to_json_value rest.keep_mods }
          else
            {}
        ),

      map_accum = fun f acc k @ { base = base_key, morphed = morphed_key, mods = trigger_mods, ..rest } =>
        let keep =
          if std.record.has_field "keep_mods" rest then
            { keep_mods = rest.keep_mods }
          else
            {}
        in
        let { acc, k = base_key } = f acc base_key in
        let { acc, k = morphed_key } = f acc morphed_key in
        {
          include acc,
          k =
            {
              base = base_key,
              morphed = morphed_key,
              mods = trigger_mods,
            }
            & keep,
        },
    },
}
