# LegendIR export helpers (Nickel).
#
# Emits: legend-bundle/v0 JSON — { schema, source, base, layer_legends[], dense }
#   where each of base / layer_legends[i] / dense is legend/v0
#   { schema, cells: [{ legends:{center,hold,tl,tr,bl}, emphasis, wash? }] }.
#
# Consumed by: Rhombus (tools/keymap-viz-rhombus) via load_bundle_layers /
#   load_bundle_dense → Cell ADTs → Scene → SVG.
#
# Does NOT: produce SVG, PointsIR, or scene geometry. Legend text/emphasis/wash
# only; points come from Rhombus factories or KiCad switch JSON.
#
let L = import "legends.ncl" in
{
  emphasis_str = fun e =>
    e
    |> match {
      'Transparent => "transparent",
      'Unused => "unused",
      'Normal => "normal",
      'Pressed => "pressed",
      'Muted => "muted",
      _ => "normal",
    },

  # Optional wash dest layer when key is hold→layer N (layer_mod.hold)
  maybe_wash = fun k =>
    let idx = L.layer_hold_index k in
    if idx == null then
      {}
    else
      { wash = idx },

  cell_of = fun k =>
    {
      legends = L.of_layer_key k,
      emphasis = emphasis_str (L.emphasis_of k),
    }
    & (maybe_wash k),

  cell_of_base = fun k =>
    {
      legends = L.of_key k,
      emphasis = "normal",
    }
    & (maybe_wash k),

  bundle_of = fun src layer_rows =>
    let dense_cells =
      let n = std.array.length (std.array.at 0 layer_rows) in
      std.array.generate
        (fun i =>
          let base_k = std.array.at i (std.array.at 0 layer_rows) in
          let base = L.of_layer_key base_k in
          let slot = fun li =>
            if li < std.array.length layer_rows then
              (L.of_layer_key (std.array.at i (std.array.at li layer_rows))).center
            else
              ""
          in
          {
            legends = {
              center = base.center,
              hold = base.hold,
              tl = slot 1,
              tr = slot 2,
              bl = slot 3,
            },
            emphasis = "normal",
          }
          & (maybe_wash base_k)
        )
        n
    in
    {
      schema = "legend-bundle/v0",
      source = src,
      base = {
        schema = "legend/v0",
        cells = (std.array.at 0 layer_rows) |> std.array.map cell_of_base,
      },
      layer_legends =
        layer_rows
        |> std.array.map_with_index (fun i layer =>
          {
            schema = "legend/v0",
            name = "layer%{i}",
            cells = layer |> std.array.map cell_of,
          }
        ),
      dense = {
        schema = "legend/v0",
        cells = dense_cells,
      },
    },
}
