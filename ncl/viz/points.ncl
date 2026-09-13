# Compiled physical points (millimetres) and canvas placement.
#
# A point is a key centre + size + rotation. Join to the keymap is by array
# order (or a remap beside the layout), not a keymap_index field on the point.
{
  unit = 19.05,

  get_or = fun r field fallback =>
    if std.record.has_field field r then
      std.record.get field r
    else
      fallback,

  Point = {
    x | Number,
    y | Number,
    rot | Number | default = 0,
    w | Number | default = 19.05,
    h | Number | default = 19.05,
  },

  min_of = fun xs =>
    xs
    |> match {
      [] => std.fail_with "min_of: empty",
      [x, ..rest] => std.array.fold_left (fun a b => if a < b then a else b) x rest,
    },

  max_of = fun xs =>
    xs
    |> match {
      [] => std.fail_with "max_of: empty",
      [x, ..rest] => std.array.fold_left (fun a b => if a > b then a else b) x rest,
    },

  bbox = fun points =>
    {
      min_x = points |> std.array.map (fun p => p.x - p.w / 2) |> min_of,
      max_x = points |> std.array.map (fun p => p.x + p.w / 2) |> max_of,
      min_y = points |> std.array.map (fun p => p.y - p.h / 2) |> min_of,
      max_y = points |> std.array.map (fun p => p.y + p.h / 2) |> max_of,
    },

  Canvas = {
    scale | Number | default = 2.8,
    pad | Number | default = 16,
    inset | Number | default = 0,
    title_h | Number | default = 28,
    pane_gap | Number | default = 24,
    rx | Number | default = 6,
  },

  default_canvas = {
    scale = 2.8,
    pad = 16,
    inset = 0,
    title_h = 28,
    pane_gap = 24,
    rx = 6,
  },

  # Place points into SVG space. y_up: larger source y is toward the top.
  # rot is KiCad-style (CCW); svg_rot is negated for SVG Y-down.
  place = fun args =>
    let points = args.points in
    let y_up = get_or args "y_up" false in
    let canvas = get_or args "canvas" default_canvas in
    let title = get_or args "title" false in
    let bb = bbox points in
    let header_px = if title then canvas.title_h else 0 in
    let y0 = canvas.pad + header_px in
    let placed_keys =
      points
      |> std.array.map (fun p =>
        let sx = canvas.pad + (p.x - bb.min_x) * canvas.scale in
        let sy =
          if y_up then
            y0 + (bb.max_y - p.y) * canvas.scale
          else
            y0 + (p.y - bb.min_y) * canvas.scale
        in
        let ww = (p.w - 2 * canvas.inset) * canvas.scale in
        let hh = (p.h - 2 * canvas.inset) * canvas.scale in
        {
          cx = sx,
          cy = sy,
          x = sx - ww / 2,
          y = sy - hh / 2,
          w = ww,
          h = hh,
          svg_rot = 0 - p.rot,
        }
      )
    in
    let scene_w = canvas.pad * 2 + (bb.max_x - bb.min_x) * canvas.scale in
    let scene_h = y0 + canvas.pad + (bb.max_y - bb.min_y) * canvas.scale in
    {
      placed = placed_keys,
      width = scene_w,
      height = scene_h,
      title_h = header_px,
    },

  checks = {
    check_min_max = {
      actual = { min = min_of [3, 1, 2], max = max_of [3, 1, 2] },
      expected = { min = 1, max = 3 },
    },

    check_y_up_flips =
      let keys = [
        { x = 0, y = 0, w = 19.05, h = 19.05, rot = 0 },
        { x = 0, y = 40, w = 19.05, h = 19.05, rot = 0 },
      ]
      in
      let test_canvas = {
        scale = 1,
        pad = 0,
        inset = 0,
        title_h = 0,
        pane_gap = 24,
        rx = 6,
      }
      in
      let up = place { points = keys, y_up = true, canvas = test_canvas, title = false } in
      let down = place { points = keys, y_up = false, canvas = test_canvas, title = false } in
      {
        actual = {
          up_high_is_top = (std.array.at 1 up.placed).cy < (std.array.at 0 up.placed).cy,
          down_high_is_bottom = (std.array.at 1 down.placed).cy > (std.array.at 0 down.placed).cy,
        },
        expected = { up_high_is_top = true, down_high_is_bottom = true },
      },

    check_packed_neighbours =
      let keys = [
        { x = 5, y = 5, w = 10, h = 10, rot = 0 },
        { x = 15, y = 5, w = 10, h = 10, rot = 0 },
      ]
      in
      let test_canvas = {
        scale = 1,
        pad = 0,
        inset = 0,
        title_h = 0,
        pane_gap = 24,
        rx = 6,
      }
      in
      let pl = place { points = keys, y_up = false, canvas = test_canvas, title = false } in
      let left = std.array.at 0 pl.placed in
      let right = std.array.at 1 pl.placed in
      {
        actual = left.x + left.w,
        expected = right.x,
      },

    check_bbox_grid_cell =
      let p = { x = 9.525, y = 9.525, w = 19.05, h = 19.05, rot = 0 } in
      {
        actual = bbox [p],
        expected = { min_x = 0, max_x = 19.05, min_y = 0, max_y = 19.05 },
      },
  },
}
