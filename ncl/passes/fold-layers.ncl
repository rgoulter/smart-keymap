# Pass: fold multi-layer authoring → per-key base & { layered }
#
# Inputs (from driver merge):
#   layers, layer_as_array_of_keys, keymap_ncl, validators
#
# Outputs:
#   layers_of_keys, layered_keys, LayeredKeyElement
{
  layers,
  layer_as_array_of_keys,
  keymap_ncl,
  validators,

  layered_key_element_validator = fun k =>
    k
    |> match {
      { layered, .. } => keymap_ncl.layered.key_validator k,
      { named_layers, .. } => keymap_ncl.layered.key_validator k,
      _ => keymap_ncl.nullable_key.key_validator k,
    },

  LayeredKeyElement = std.contract.from_validator layered_key_element_validator,

  layers_of_keys = layers |> std.array.map layer_as_array_of_keys,

  layered_keys
    | Array LayeredKeyElement
    | doc "Constructs array of key::layered::LayeredKey values from layers."
    =
      if std.array.length layers == 0 then
        std.fail_with "keys or layers must be provided"
      else if std.array.length layers <= 1 then
        std.array.at 0 layers |> layer_as_array_of_keys
      else
        let base_keys = std.array.first layers_of_keys in
        let layered_keys = std.array.drop_first layers_of_keys in
        std.array.generate
          (fun idx =>
            let base_key = std.array.at idx base_keys in
            let layered_ = std.array.map (std.array.at idx) layered_keys in
            let no_layered = std.array.all (fun x => x == null) layered_ in
            if no_layered then
              base_key
            else
              base_key & { layered = layered_ }
          )
          (std.array.length base_keys),

  checks.check_layered_key_element =
    let K = import "keys.ncl" in
    {
      check_plain_key_ok =
        layered_key_element_validator K.A == 'Ok,

      check_null_ok =
        layered_key_element_validator null == 'Ok,

      check_string_token_ok =
        layered_key_element_validator K.caps_word.toggle == 'Ok,

      check_layered_composite_ok =
        layered_key_element_validator (K.A & { layered = [ null, K.B] }) == 'Ok,

      check_invalid_is_error =
        layered_key_element_validator { not_a_key = true } |> validators.check_is_error,
    },

  checks.check_layer_as_array_of_keys = {
    check_layer_as_array_of_keys = {
      actual = layer_as_array_of_keys " A ",
      expected = [{ key_code = 4 }],
    },

    check_layer_as_array_of_keys_tttt = {
      actual = layer_as_array_of_keys " TTTT ",
      expected = [ null ],
    },
  },
}
