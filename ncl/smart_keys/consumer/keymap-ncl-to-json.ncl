{
  validators,

  keyboard_modifiers,

  checks.consumer =
    let K = import "keys.ncl" in
    {
      keymap_example_consumer = {
        actual = K.VolumeUp,
        expected = { consumer_code = 233 },
      },

      check_json_value_consumer = {
        actual = K.VolumeUp |> keymap_ncl.key.to_json_value,
        expected = { consumer_code = 233 },
      },

      keymap_example_modified_consumer = {
        actual = K.VolumeUp & K.LeftShift,
        expected = {
          consumer_code = 233,
          modifiers = { left_shift = true },
        },
      },

      check_json_value_modified_consumer = {
        actual = K.VolumeUp & K.LeftShift |> keymap_ncl.key.to_json_value,
        expected = { consumer_code = 233, modifiers = 2 },
      },
    },

  keymap_ncl.consumer
    | doc "for key::consumer::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { consumer_code, modifiers } =>
            let code_validation = validators.is_number consumer_code in
            let mods_validation = keyboard_modifiers.validator modifiers in
            [code_validation, mods_validation]
            |> match {
              ['Ok, 'Ok] => 'Ok,
              [err, 'Ok] => err,
              ['Ok, err] => err,
            },
          { consumer_code } => validators.is_number consumer_code,
          _ =>
            'Error {
              message = "expected { consumer_code } or { consumer_code, modifiers }",
            },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = match {
        { consumer_code, modifiers = km } =>
          {
            include consumer_code,
            modifiers = keyboard_modifiers.to_json_value km,
          },
        k => k,
      },

      map_accum = fun f acc k => f acc k,
    },
}
