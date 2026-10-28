{
  checks.sequence_indices = {
    trivial_empty = {
      expected = [],
      actual = "_" |> indices,
    },
    trivial_nonempty = {
      expected = [0],
      actual = "0" |> indices,
    },
    in_scan_order = {
      expected = [1, 3],
      actual = " _ 0 _ 1 _ " |> indices,
    },
    reverse_of_scan_order = {
      expected = [3, 1],
      actual = " _ 1 _ 0 _ " |> indices,
    },
    same_markers_keep_scan_order = {
      expected = [1, 3],
      actual = " _ X _ X _ " |> indices,
    },
    layout_string = {
      expected = [9, 35],
      actual =
        m%"
          _ _ _ _ _   _ _ _ _ 0
          _ _ _ _ _   _ _ _ _ _
          _ _ _ _ _   _ _ _ _ _
              _ _ _   _ _ 1
        "%
        |> indices,
    },
  },

  indices
    | doc "calculate keymap indices for sequence steps, ordered by the non-'_' markers."
    = fun s =>
      s
      |> std.string.replace_regex "\\s+" ""
      |> std.string.characters
      |> std.array.map_with_index (fun i c => if c == "_" then [] else [{ label = c, index = i }])
      |> std.array.flatten
      |> std.array.sort (fun a b =>
        std.string.compare a.label b.label
        |> match {
          'Equal => std.number.compare a.index b.index,
          cmp => cmp,
        }
      )
      |> std.array.map (fun { index, .. } => index),
}
