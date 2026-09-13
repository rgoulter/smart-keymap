let viz = import "viz.ncl" in
let km =
  (import "../../../tests/ncl/keymap-36key-rgoulter/keymap.ncl")
  & (import "keymap-ncl-to-json.ncl")
in
let layout = viz.combinators.split_3x5_3 {} in
let layer_rows = km.layers_of_keys in
let base_keys = std.array.at 0 layer_rows in
{
  files =
    viz.files_for { stem = "36key-split-legends", result = viz.recipes.legends { points = layout, keys = base_keys, title = "36key-rgoulter" }, points = layout }
    @ viz.files_for { stem = "36key-split-dense", result = viz.recipes.dense { points = layout, layers = layer_rows, title = "36key-rgoulter dense" }, points = layout }
    @ viz.files_for { stem = "36key-split-chords", result = viz.recipes.chords { points = layout, keys = base_keys, chords = km.chords, title = "36key-rgoulter chords" }, points = layout },
}
