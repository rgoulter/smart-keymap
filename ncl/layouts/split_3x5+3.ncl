{
  into_layout = fun from_split_3x5_3 split_3x5_3_keymap =>
    let split_3x5_3_layers =
      let km_aug = import "keymap-ncl-to-json.ncl" in
      split_3x5_3_keymap.layers
      |> std.array.map (split_3x5_3_keymap & km_aug).layer_as_array_of_keys
    in
    (std.record.remove "layers" split_3x5_3_keymap)
    & {
      layers = std.array.map from_split_3x5_3 split_3x5_3_layers
    },
}
