{
  validators,

  keymap_ncl.custom
    | doc "for key::custom::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { custom } => validators.is_number custom,
          _ => 'Error { message = "Expected { custom }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun key => key,

      map_accum = fun f acc k => f acc k,
    },
}
