{
  validators,

  keyboard_modifiers,

  keymap_ncl.tri_state
    | doc "for key::tri_state::Key."
    = {
      Key = std.contract.from_validator key_validator,

      spec_validator =
        validators.record.validator {
          fields_validator = validators.record.has_exact_fields ["hold", "tap"],
          field_validators = {
            hold = keymap_ncl.keyboard.key_validator,
            tap = keymap_ncl.keyboard.key_validator,
          },
        },

      key_validator = fun k =>
        k
        |> match {
          { tri_state = spec } => spec_validator spec,
          _ => 'Error { message = "Expected { tri_state = { hold, tap } }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      # Keyboard key (author form) → KeyOutput JSON used by codegen / serde.
      keyboard_key_to_key_output = fun k =>
        k
        |> match {
          { key_code = kc, modifiers = km } =>
            {
              key_code = { Keyboard = kc },
              key_modifiers = keyboard_modifiers.to_json_value km,
            },
          { key_code = kc } => { key_code = { Keyboard = kc } },
          { modifiers = km } =>
            {
              key_modifiers = keyboard_modifiers.to_json_value km,
            },
          {} => {},
        },

      to_json_value = fun { tri_state = spec } =>
        {
          hold = keyboard_key_to_key_output spec.hold,
          tap = keyboard_key_to_key_output spec.tap,
        },

      # Leaves have no nested keys; map_accum maps children only.
      map_accum = fun f acc k => { include acc, include k },
    },

  checks.check_tri_state =
    let K = import "keys.ncl" in
    {
      check_custom_ok =
        keymap_ncl.tri_state.key_validator (
          K.tri_state.custom {
            hold = K.LeftAlt,
            tap = K.Tab,
          }
        ) == 'Ok,

      check_alt_tab_ok =
        keymap_ncl.tri_state.key_validator K.tri_state.alt_tab == 'Ok,

      check_alt_tab_json = {
        actual = keymap_ncl.tri_state.to_json_value K.tri_state.alt_tab,
        expected = {
          hold = { key_modifiers = 4 },
          tap = { key_code = { Keyboard = 43 } },
        },
      },
    },
}
