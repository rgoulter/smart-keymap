{
  into_layout = fun from_split_3x5_3 split_3x5_3_keymap =>
    let remapped_indices # Array { target_index : Number, source_index : Number }
    =
      let split_3x5_3_indices = std.array.range 0 36 in
      let target_indices = from_split_3x5_3 { no_key = false, from_layout = split_3x5_3_indices } in
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
    let split_3x5_3_layers =
      let km_aug = import "keymap-ncl-to-json.ncl" in
      split_3x5_3_keymap.layers
      |> std.array.map (split_3x5_3_keymap & km_aug).layer_as_array_of_keys
    in
    (
      split_3x5_3_keymap
      |> (match {
        { chords = source_chords, ..km } => { chords = remap_chords source_chords } & km,
        km => km,
      }
      )
      |> std.record.remove "layers"
    )
    & {
      layers =
        let NO_KEY =
          let K = import "keys.ncl" in
          K.NO
        in
        std.array.map (fun l => from_split_3x5_3 { no_key = NO_KEY, from_layout = l }) split_3x5_3_layers
    },
}
