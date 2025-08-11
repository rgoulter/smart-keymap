{
  checks.indices = {
    trivial_empty = {
      expected = [],
      actual = "_" |> indices,
    },
    trivial_nonempty = {
      expected = [0],
      actual = "X" |> indices,
    },
    simple_odd = {
      expected = [1, 3],
      actual = " _ X _ X _ " |> indices,
    },
  },

  indices
    | doc "calculate the indices of a given string for non-'_' characters."
    = fun s => s
    |> std.string.replace_regex "\\s+" ""
    |> std.string.characters
    |> std.array.map_with_index (fun i c => if c == "_" then [] else [i])
    |> std.array.flatten
}
