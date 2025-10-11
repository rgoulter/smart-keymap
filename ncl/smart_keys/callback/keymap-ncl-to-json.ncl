{
  validators,

  keymap_ncl.callback
    | doc "for key::callback::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { keymap_callback = { Bluetooth = "Clear" } } => 'Ok,
          { keymap_callback = { Bluetooth = "ClearAll" } } => 'Ok,
          { keymap_callback = { Bluetooth = "Disconnect" } } => 'Ok,
          { keymap_callback = { Bluetooth = "Next" } } => 'Ok,
          { keymap_callback = { Bluetooth = "Previous" } } => 'Ok,
          { keymap_callback = { Bluetooth = { Select } } } =>
            validators.is_number Select,
          { keymap_callback = { Custom = indices @ [i, j] } } =>
            validators.array.validator validators.is_number indices,
          { keymap_callback } => validators.is_string keymap_callback,
          _ => 'Error { message = "expected { keymap_callback = <KeymapCallback variant> }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun key => key,

      map_accum = fun f acc k => f acc k,
    },
}
