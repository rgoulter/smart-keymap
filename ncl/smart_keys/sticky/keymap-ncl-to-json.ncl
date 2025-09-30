{
  validators,

  keyboard_modifiers,

  keymap_ncl.sticky
    | doc "for key::sticky::Key."
    = {
      StickyActivation =
        std.contract.from_validator (
          validators.is_elem_of [
            "OnStickyKeyRelease"
          ]
        ),

      StickyReleaseJson =
        std.contract.from_validator (
          validators.is_elem_of [
            "OnModifiedKeyRelease",
            "OnNextKeyPress",
          ]
        ),

      Config = {
        activation
          | optional
          | StickyActivation,
        release
          | optional
          | StickyReleaseJson,
      },

      Key = std.contract.from_validator key_validator,

      key_validator =
        validators.record.validator {
          fields_validator =
            validators.record.has_exact_fields ["sticky_modifiers"],
          field_validators = {
            sticky_modifiers = keyboard_modifiers.validator,
          },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun { sticky_modifiers = sm } => { sticky_modifiers = keyboard_modifiers.to_json_value sm },

      map_accum = fun f acc k => f acc k,
    },
}
