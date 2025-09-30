{
  validators,

  keymap_ncl.mouse
    | doc "for key::mouse::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator =
        validators.record.validator {
          fields_validator =
            validators.record.has_exact_fields ["mouse"],
          field_validators = {
            mouse =
              validators.any_of [
                validators.is_string,
                validators.record.validator {
                  fields_validator = validators.record.has_exact_fields ["Button"],
                  field_validators = { Button = validators.is_number },
                },
              ],
          },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun k => k,

      map_accum = fun f acc k => f acc k,
    },
}
