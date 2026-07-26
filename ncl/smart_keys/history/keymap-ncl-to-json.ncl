{
  validators,

  keymap_ncl.history
    | doc "for key::history::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          "Repeat" => 'Ok,
          _ => 'Error { message = "Expected \"Repeat\"" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun key => key,

      # Leaves have no nested keys; map_accum maps children only.
      map_accum = fun f acc k => { include acc, include k },
    },
}
