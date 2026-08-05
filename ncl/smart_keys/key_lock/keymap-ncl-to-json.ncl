{
  validators,

  keymap_ncl.key_lock
    | doc "for key::key_lock::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          "KeyLock" => 'Ok,
          _ => 'Error { message = "Expected \"KeyLock\"" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun key => key,

      # Leaves have no nested keys; map_accum maps children only.
      map_accum = fun f acc k => { include acc, include k },
    },
}
