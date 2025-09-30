{
  validators,

  keyboard_modifiers,

  checks.keyboard =
    let K = import "keys.ncl" in
    {
      keymap_example_key_code = {
        actual = K.A,
        expected = { key_code = 4 },
      },

      check_json_value_key_code = {
        actual = K.A |> keymap_ncl.key.to_json_value,
        expected = { key_code = 4 },
      },
    },

  keymap_ncl.keyboard
    | doc "for key::keyboard::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator =
        validators.record.validator {
          fields_validator =
            validators.all_of [
              validators.record.has_any_field_of ["key_code", "modifiers"],
              validators.record.has_only_fields ["key_code", "modifiers"],
            ],
          field_validators = {
            key_code = validators.is_number,
            modifiers = keyboard_modifiers.validator,
          },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = match {
        { modifiers = km, ..k } => k & { modifiers = keyboard_modifiers.to_json_value km },
        k => k,
      },

      map_accum = fun f acc k => f acc k,
    },
}
