{
  validators,

  keymap_ncl.caps_word
    | doc "for key::caps_word::Key."
    = {
      Key = std.contract.from_validator key_validator,

      key_validator = fun k =>
        k
        |> match {
          { caps_word = "ToggleCapsWord" } => 'Ok,
          _ => 'Error { message = "Expected { caps_word = \"ToggleCapsWord\" }" },
        },

      is_key = fun k => 'Ok == key_validator k,

      to_json_value = fun key =>
        key
        |> match {
          { caps_word = json } => json,
          _ => std.fail_with "Expected { caps_word = \"ToggleCapsWord\" }",
        },

      # Leaves have no nested keys; map_accum maps children only.
      map_accum = fun f acc k => { include acc, include k },
    },

  checks.check_caps_word =
    let K = import "keys.ncl" in
    {
      check_toggle_ok =
        keymap_ncl.caps_word.key_validator K.caps_word.toggle == 'Ok,

      check_toggle_json = {
        actual = keymap_ncl.caps_word.to_json_value K.caps_word.toggle,
        expected = "ToggleCapsWord",
      },

      # Record form merges with layered authoring as a layered base.
      check_toggle_layered_base_ok =
        keymap_ncl.key.key_validator (K.caps_word.toggle & { layered = [K.A] }) == 'Ok,

      check_toggle_layered_base_json = {
        actual =
          keymap_ncl.key.to_json_value (K.caps_word.toggle & { layered = [K.A] }),
        expected = {
          base = "ToggleCapsWord",
          layered = [{ key_code = 4 }],
        },
      },
    },
}
