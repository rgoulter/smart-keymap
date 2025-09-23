{
  validators,

  keymap_ncl.callback
    | doc "for key::callback::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { keymap_callback = { Custom = indices @ [i, j] } } =>
            validators.array.validator validators.is_number indices,
          { keymap_callback } => validators.is_string keymap_callback,
          _ => 'Error { message = "expected { keymap_callback = <KeymapCallback variant> }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun key => key,
    },
}
