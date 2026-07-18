# Pass: fold multi-layer authoring → per-key base & { layered }
#
# Inputs (from driver merge):
#   layers, named_layers, layer_as_array_of_keys, keymap_ncl, validators
#
# Outputs:
#   layers_of_keys, layered_keys, LayeredKeyElement
#
# Numbered `layers` fold first (base + overlay arrays → `{ layered = [...] }`).
# KM-level `named_layers` (name → full row) then zip into per-key `named_layers`
# records; later prepare_keys_named_layers lowers those to dense arrays.
{
  layers,
  layer_as_array_of_keys,
  keymap_ncl,
  validators,

  # Keymap-level named layer rows (optional; default empty).
  named_layers | default = {},

  layered_key_element_validator = fun k =>
    k
    |> match {
      { layered, .. } => keymap_ncl.layered.key_validator k,
      { named_layers, .. } => keymap_ncl.layered.key_validator k,
      _ => keymap_ncl.nullable_key.key_validator k,
    },

  LayeredKeyElement = std.contract.from_validator layered_key_element_validator,

  layers_of_keys = layers |> std.array.map layer_as_array_of_keys,

  # Zip each KM-level named layer row into per-key `named_layers`.
  # Per-key entries for the same name win (right merge). Row width must match
  # the base key count. Empty `km_named` is a no-op.
  inject_km_named_layers = fun km_named keys =>
    std.record.fields km_named
    |> std.array.fold_left
      (fun keys name =>
        let row = layer_as_array_of_keys (std.record.get name km_named) in
        let keys_len = std.array.length keys in
        let row_len = std.array.length row in
        if row_len != keys_len then
          std.fail_with "named_layers.%{name}: expected %{keys_len} keys, got %{row_len}"
        else
          std.array.map_with_index
            (fun i k =>
              let cell = std.array.at i row in
              let existing =
                k
                |> match {
                  { named_layers = nl, .. } => nl,
                  _ => {},
                }
              in
              # Per-key entry for the same name wins; otherwise take the KM cell.
              let named =
                if std.record.has_field name existing then
                  existing
                else
                  existing & { "%{name}" = cell }
              in
              k & { named_layers = named }
            )
            keys
      )
      keys,

  layered_keys_from_numbered_layers =
    if std.array.length layers == 0 then
      std.fail_with "keys or layers must be provided"
    else if std.array.length layers <= 1 then
      std.array.at 0 layers |> layer_as_array_of_keys
    else
      let base_keys = std.array.first layers_of_keys in
      let overlay_layers = std.array.drop_first layers_of_keys in
      std.array.generate
        (fun idx =>
          let base_key = std.array.at idx base_keys in
          let layered_ = std.array.map (std.array.at idx) overlay_layers in
          let no_layered = std.array.all (fun x => x == null) layered_ in
          if no_layered then
            base_key
          else
            base_key & { layered = layered_ }
        )
        (std.array.length base_keys),

  layered_keys
    | Array LayeredKeyElement
    | doc "Constructs array of key::layered::LayeredKey values from layers (+ optional named_layers)."
    = inject_km_named_layers named_layers layered_keys_from_numbered_layers,

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

  checks.check_inject_km_named_layers =
    let K = import "keys.ncl" in
    {
      check_inject_simple = {
        actual =
          inject_km_named_layers
            { fn = [ null, K.B] }
            [K.layer_mod.hold "fn", K.A],
        expected = [
          { layer_modifier = { hold = "fn" }, named_layers = { fn = null } },
          K.A & { named_layers = { fn = K.B } },
        ],
      },

      check_inject_empty_is_noop = {
        actual = inject_km_named_layers {} [K.A, K.B],
        expected = [K.A, K.B],
      },

      # Per-key named_layers wins over KM-level for the same name.
      check_per_key_wins_on_name_conflict = {
        actual =
          inject_km_named_layers
            { fn = [K.B, K.C] }
            [
              K.A & { named_layers = { fn = K.D } },
              K.E,
            ],
        expected = [
          K.A & { named_layers = { fn = K.D } },
          K.E & { named_layers = { fn = K.C } },
        ],
      },

      # KM-level merges with other per-key named layer names.
      check_merge_other_named_names = {
        actual =
          inject_km_named_layers
            { fn = [K.B] }
            [K.A & { named_layers = { nav = K.C } }],
        expected = [
          K.A & { named_layers = { fn = K.B, nav = K.C } },
        ],
      },

      # Multiple named rows (alphabetical order is for later lowering, not inject).
      check_inject_multiple_names = {
        actual =
          inject_km_named_layers
            {
              nav = [ null, K.Left],
              fn = [ null, K.F1],
            }
            [K.A, K.B],
        expected = [
          K.A & { named_layers = { fn = null, nav = null } },
          K.B & { named_layers = { fn = K.F1, nav = K.Left } },
        ],
      },

      # String KeymapLayer rows (via layer_as_array_of_keys).
      check_inject_string_row = {
        actual =
          inject_km_named_layers
            { fn = " TTTT B " }
            [K.A, K.C],
        expected = [
          K.A & { named_layers = { fn = null } },
          K.C & { named_layers = { fn = K.B } },
        ],
      },
    },
}
