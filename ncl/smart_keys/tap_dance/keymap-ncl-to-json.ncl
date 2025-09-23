{
  validators,

  keymap_ncl.tap_dance
    | doc "for key::tap_dance::Key."
    = {
      Config = {
        timeout | optional | Number,
      },

      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { tap_dances, ..tap_key } =>
            let valid_tap_key = keymap_ncl.key.key_validator tap_key in
            let valid_tap_dances = validators.array.validator keymap_ncl.key.key_validator tap_dances in
            [valid_tap_dances, valid_tap_key]
            |> match {
              ['Ok, 'Ok] => 'Ok,
              [err, _] => err,
              [_, err] => err,
            },
          _ => 'Error { message = "expected { tap_dances = [Key, ..], ..tap_key }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun { tap_dances, ..tap_key } =>
        {
          definitions = [keymap_ncl.key.to_json_value tap_key] @ (std.array.map keymap_ncl.key.to_json_value tap_dances),
        },
    },
}
