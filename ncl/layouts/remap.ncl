# Remap a keymap from a source layout size into a target layout shape.
#
# `from_layout` is a function that takes
#   { no_key, from_layout = source_row }
# and returns a target-width row (using `no_key` for unmapped positions).
#
# Remaps:
#   - numbered `layers` (and `keys` via layers default in consumers)
#   - KM-level `named_layers` rows (same geometry as layers)
#   - chord `indices`
let into_layout_impl = fun source_key_count from_layout source_keymap =>
  let remapped_indices # Array { target_index : Number, source_index : Number }
  =
    let source_indices = std.array.range 0 source_key_count in
    let target_indices = from_layout { no_key = false, from_layout = source_indices } in
    target_indices
    |> std.array.map_with_index (fun ti si => { target_index = ti, source_index = si })
    |> std.array.filter (fun { target_index, .. } => target_index != false)
  in
  let remap_index = fun source_idx =>
    remapped_indices
    |> std.array.filter (fun { source_index, .. } => source_index == source_idx)
    |> match {
      [] => false,
      [{ target_index, .. }] => target_index,
      _ => std.fail_with "More than one remapped index found",
    }
  in
  let remap_chords = fun chords =>
    let remap_chord = fun { indices = source_indices, ..chord } =>
      let target_indices = source_indices |> std.array.map remap_index in
      { indices = target_indices, } & chord
    in
    chords |> std.array.map remap_chord
  in
  let km_aug = import "keymap-ncl-to-json.ncl" in
  let as_keys = (source_keymap & km_aug).layer_as_array_of_keys in
  let NO_KEY =
    let K = import "keys.ncl" in
    K.NO
  in
  let remap_layer_row = fun l =>
    from_layout { no_key = NO_KEY, from_layout = as_keys l }
  in
  let target_layers = source_keymap.layers |> std.array.map remap_layer_row in
  let target_named_layers =
    source_keymap
    |> match {
      { named_layers = nl, .. } =>
        std.record.map (fun _name layer => remap_layer_row layer) nl,
      _ => {},
    }
  in
  (
    source_keymap
    |> (match {
      { chords = source_chords, ..km } => { chords = remap_chords source_chords } & km,
      km => km,
    }
    )
    |> (match {
      { layers = _, ..km } => km,
      km => km,
    }
    )
    |> (match {
      { named_layers = _, ..km } => km,
      km => km,
    }
    )
  )
  & {
    layers = target_layers,
    named_layers = target_named_layers,
  }
in
{
  source_key_count | Number,
  into_layout = into_layout_impl source_key_count,

  checks.check_into_layout =
    let K = import "keys.ncl" in
    # 2-key source → 3-key target with a gap in the middle.
    let from_2_to_3 = fun {
      no_key = XXX,
      from_layout = [a, b],
    } =>
      [a, XXX, b]
    in
    let remapped =
      into_layout_impl
        2
        from_2_to_3
        {
          layers = [
            [K.A, K.B],
            [ null, K.C],
          ],
          named_layers = {
            fn = [K.D, K.E],
          },
          chords = [
            { indices = [0, 1], key = K.F },
          ],
        }
    in
    {
      check_layers = {
        actual = remapped.layers,
        expected = [
          [K.A, K.NO, K.B],
          [ null, K.NO, K.C],
        ],
      },

      check_named_layers = {
        actual = remapped.named_layers,
        expected = {
          fn = [K.D, K.NO, K.E],
        },
      },

      check_chords = {
        actual = remapped.chords,
        expected = [
          { indices = [0, 2], key = K.F },
        ],
      },

      # Full pipeline: remapped layers + named_layers fold / lower to target width.
      check_json_pipeline =
        let km = remapped & (import "keymap-ncl-to-json.ncl") in
        {
          check_key_count = {
            actual = std.array.length km.json_keymap.keys,
            expected = 3,
          },
          check_named_layer_names = {
            actual = km.named_layer_names,
            expected = ["fn"],
          },
          # Numbered layer (1) + named layer "fn" → dense layered length 2.
          check_max_layered_length = {
            actual = km.max_layered_length,
            expected = 2,
          },
        },
    },
}
