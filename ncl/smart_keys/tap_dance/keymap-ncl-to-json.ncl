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

      map_accum = fun f acc k @ { tap_dances, ..tap_key } =>
        let { acc, tap_dances } =
          tap_dances
          |> std.array.fold_left
            (fun { acc, tap_dances } k =>
              let { acc, k } = f acc k in
              let tap_dances = tap_dances @ [k] in
              { include acc, include tap_dances }
            )
            { include acc, tap_dances = [] }
        in
        let { acc, k = tap_key } = f acc tap_key in
        {
          include acc,
          k =
            {
              include tap_dances,
            }
            & tap_key,
        },
    }
}
