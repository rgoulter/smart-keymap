{
  validators,

  keymap_ncl.caps_word
    | doc "for key::caps_word::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          "ToggleCapsWord" => 'Ok,
          _ => 'Error { message = "Expected \"ToggleCapsWord\"" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun key => key,
    },
}
