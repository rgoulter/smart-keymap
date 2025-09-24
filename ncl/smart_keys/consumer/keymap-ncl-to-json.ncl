{
  validators,

  keymap_ncl.consumer
    | doc "for key::consumer::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator =
        validators.record.validator {
          fields_validator =
            validators.all_of [
              validators.record.has_field "usage_code",
              validators.record.has_only_fields ["usage_code"],
            ],
          field_validators = {
            usage_code = validators.is_number,
          },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun k => k,
    },
}
