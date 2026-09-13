let viz = import "viz.ncl" in
let km =
  (import "../../../tests/ncl/keymap-48key-basic/keymap.ncl")
  & (import "keymap-ncl-to-json.ncl")
in
let layout = viz.combinators.grid { rows = 4, cols = 12 } in
let layer_rows = km.layers_of_keys in
let base_keys = std.array.at 0 layer_rows in
{
  files =
    viz.files_for { stem = "48key-basic-legends", result = viz.recipes.legends { points = layout, keys = base_keys, title = "48key-basic" }, points = layout }
    @ viz.files_for { stem = "48key-basic-stacked", result = viz.recipes.stacked { points = layout, layers = layer_rows, title = "48key-basic" }, points = layout }
    @ viz.files_for { stem = "48key-basic-dense", result = viz.recipes.dense { points = layout, layers = layer_rows, title = "48key-basic dense" }, points = layout },
}
