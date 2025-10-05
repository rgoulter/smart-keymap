{
  validators,

  keymap_ncl.chorded
    | doc "for key::chorded::Key."
    = {
      Config = {
        required_idle_time | optional | Number,
        timeout | optional | Number,
      },

      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { chords = chords_, passthrough = passthrough_key } =>
            let valid_chords = chords_ |> validators.array.validator (validators.tuple.validator [validators.is_number, keymap_ncl.key.key_validator]) in
            let valid_passthrough_key = keymap_ncl.key.key_validator passthrough_key in
            [valid_chords, valid_passthrough_key]
            |> match {
              ['Ok, 'Ok] => 'Ok,
              [err, _] => err,
              [_, err] => err,
            },
          _ => 'Error { message = "expected { chords = [[index, Key]], passthrough = Key }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun { chords = chords_, passthrough = passthrough_key } =>
        {
          chords = chords_ |> std.array.map (fun [i, chord_key] => [i, keymap_ncl.key.to_json_value chord_key]),
          passthrough = keymap_ncl.key.to_json_value passthrough_key,
        },

      map_accum = fun f acc k @ { chords, passthrough } =>
        let { acc, chords } =
          chords
          |> std.array.fold_left
            (fun { acc, chords } [i, k] =>
              let { acc, k } = f acc k in
              let chords = chords @ [[i, k]] in
              { include acc, include chords }
            )
            { include acc, chords = [] }
        in
        let { acc, k = passthrough } = f acc passthrough in
        { include acc, k = { include chords, include passthrough, }, },
    },

  keymap_ncl.chorded_aux
    | doc "for key::chorded::AuxiliaryKey."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { passthrough = passthrough_key } => keymap_ncl.key.key_validator passthrough_key,
          _ => 'Error { message = "expected { passthrough = Key }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun { passthrough = passthrough_key } =>
        {
          passthrough = keymap_ncl.key.to_json_value passthrough_key,
        },

      map_accum = fun f acc k @ { passthrough } =>
        let { acc, k = passthrough } = f acc passthrough in
        { include acc, k = { include passthrough }, },
    },
}
