{
  validators,

  keyboard_modifiers,

  checks.mouse =
    let K = import "keys.ncl" in
    {
      keymap_example_mouse = {
        actual = K.MouseBtn1,
        expected = { mouse = { Button = 1 } },
      },

      check_json_value_mouse = {
        actual = K.MouseBtn1 |> keymap_ncl.key.to_json_value,
        expected = { mouse = { Button = 1 } },
      },

      keymap_example_modified_mouse = {
        actual = K.MouseBtn1 & K.LeftCtrl,
        expected = {
          mouse = { Button = 1 },
          modifiers = { left_ctrl = true },
        },
      },

      check_json_value_modified_mouse = {
        actual = K.MouseBtn1 & K.LeftCtrl |> keymap_ncl.key.to_json_value,
        expected = { mouse = { Button = 1 }, modifiers = 1 },
      },
    },

  mouse_field_validator =
    validators.any_of [
      validators.is_string,
      validators.record.validator {
        fields_validator = validators.record.has_exact_fields ["Button"],
        field_validators = { Button = validators.is_number },
      },
    ],

  keymap_ncl.mouse
    | doc "for key::mouse::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { mouse, modifiers } =>
            let mouse_validation = mouse_field_validator mouse in
            let mods_validation = keyboard_modifiers.validator modifiers in
            [mouse_validation, mods_validation]
            |> match {
              ['Ok, 'Ok] => 'Ok,
              [err, 'Ok] => err,
              ['Ok, err] => err,
            },
          { mouse } => mouse_field_validator mouse,
          _ =>
            'Error {
              message = "expected { mouse } or { mouse, modifiers }",
            },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = match {
        { mouse, modifiers = km } =>
          {
            include mouse,
            modifiers = keyboard_modifiers.to_json_value km,
          },
        k => k,
      },

      map_accum = fun f acc k => f acc k,
    },
}
