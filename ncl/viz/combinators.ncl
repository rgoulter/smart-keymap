# Layout combinators. All emit arrays of points (centres, millimetres).
#
# Engine placement does not know about ortho / ANSI / split; those live here.
let P = import "viz/points.ncl" in
{
  # Row-major ortho grid. Row 0 is the top (y-down).
  grid = fun args =>
    let rows = args.rows in
    let cols = args.cols in
    let pitch = P.get_or args "pitch" P.unit in
    (
      std.array.generate
        (fun r =>
          std.array.generate
            (fun c =>
              {
                x = (c + 0.5) * pitch,
                y = (r + 0.5) * pitch,
                w = pitch,
                h = pitch,
                rot = 0,
              }
            )
            cols
        )
        rows
    )
    |> std.array.flatten,

  # ANSI-style rows: each inner array is key widths in u.
  rows = fun args =>
    let row_widths = args.row_widths in
    let pitch = P.get_or args "pitch" P.unit in
    row_widths
    |> std.array.map_with_index (fun r widths =>
      widths
      |> std.array.fold_left
        (fun acc w =>
          let ww = w * pitch in
          {
            x = acc.x + ww,
            keys =
              acc.keys
              @ [
                {
                  x = acc.x + ww / 2,
                  y = (r + 0.5) * pitch,
                  w = ww,
                  h = pitch,
                  rot = 0,
                }
              ],
          }
        )
        { x = 0, keys = [] }
      |> (fun acc => acc.keys)
    )
    |> std.array.flatten,

  # Column-staggered matrix, row-major. stagger[c] is added to y (y-down:
  #  negative stagger moves the column toward the top of the keyboard).
  columns = fun args =>
    let row_count = args.row_count in
    let stagger = args.stagger in
    let pitch = P.get_or args "pitch" P.unit in
    let col_count = std.array.length stagger in
    (
      std.array.generate
        (fun r =>
          std.array.generate
            (fun c =>
              {
                x = (c + 0.5) * pitch,
                y = (r + 0.5) * pitch + (std.array.at c stagger),
                w = pitch,
                h = pitch,
                rot = 0,
              }
            )
            col_count
        )
        row_count
    )
    |> std.array.flatten,

  shift = fun dx points =>
    points |> std.array.map (fun p => std.record.update "x" (p.x + dx) p),

  # Place `right` to the right of `left` with gap millimetres between bboxes.
  # Order is left ++ right (not keymap-weaved).
  split = fun { gap, left, right } =>
    let lb = P.bbox left in
    let rb = P.bbox right in
    let dx = (lb.max_x + gap) - rb.min_x in
    left @ (shift dx right),

  # Reflect points through the vertical midline of their bbox.
  mirror_x = fun pts =>
    let bb = P.bbox pts in
    let axis = bb.min_x + bb.max_x in
    pts
    |> std.array.map (fun p =>
      p
      |> std.record.update "x" (axis - p.x)
      |> std.record.update "rot" (0 - p.rot)
    ),

  # Reverse every `width`-sized row (mirrored left → inner-to-pinky).
  reverse_rows = fun width pts =>
    let rows = std.array.length pts / width in
    (
      std.array.generate
        (fun r =>
          std.array.slice (r * width) (r * width + width) pts
          |> std.array.reverse
        )
        rows
    )
    |> std.array.flatten,

  # 3x5+3 split in keymap.ncl row-major order:
  #  left row, right row, …, left thumbs, right thumbs.
  # RHS is a mirror of LHS (thumbs toward the gap), then row-reversed
  #  so the right hand is inner-to-pinky like the keymap.
  split_3x5_3 = fun args =>
    let split_gap = P.get_or args "gap" 28 in
    let u = P.get_or args "pitch" P.unit in
    let col_stagger = P.get_or args "stagger" [0, -2.5, -5, -2.5, 0] in
    let thumb_extra = P.get_or args "thumb_extra" 2 in
    let matrix_pts = columns { row_count = 3, stagger = col_stagger, pitch = u } in
    let thumb_pts =
      std.array.generate
        (fun i =>
          let c = i + 2 in
          {
            x = (c + 0.5) * u,
            y = (3 + 0.5) * u + thumb_extra,
            w = u,
            h = u,
            rot = 0,
          }
        )
        3
    in
    let mirrored = mirror_x (matrix_pts @ thumb_pts) in
    let right_matrix = reverse_rows 5 (std.array.slice 0 15 mirrored) in
    let right_thumbs = std.array.reverse (std.array.slice 15 18 mirrored) in
    let right_all =
      split {
        gap = split_gap,
        left = matrix_pts @ thumb_pts,
        right = right_matrix @ right_thumbs,
      }
    in
    let R_matrix = std.array.slice 18 33 right_all in
    let R_thumbs = std.array.slice 33 36 right_all in
    let woven =
      (
        std.array.generate
          (fun r =>
            std.array.slice (r * 5) (r * 5 + 5) matrix_pts
            @ std.array.slice (r * 5) (r * 5 + 5) R_matrix
          )
          3
      )
      |> std.array.flatten
    in
    woven @ thumb_pts @ R_thumbs,

  checks = {
    check_grid_2x2 =
      let ps = grid { rows = 2, cols = 2, pitch = 10 } in
      {
        actual = {
          n = std.array.length ps,
          first = { x = (std.array.at 0 ps).x, y = (std.array.at 0 ps).y },
          last = { x = (std.array.at 3 ps).x, y = (std.array.at 3 ps).y },
        },
        expected = {
          n = 4,
          first = { x = 5, y = 5 },
          last = { x = 15, y = 15 },
        },
      },

    check_rows_widths =
      let ps = rows { row_widths = [[1, 2]], pitch = 10 } in
      {
        actual = {
          n = std.array.length ps,
          w0 = (std.array.at 0 ps).w,
          w1 = (std.array.at 1 ps).w,
          x1 = (std.array.at 1 ps).x,
        },
        expected = { n = 2, w0 = 10, w1 = 20, x1 = 20 },
      },

    check_split_gap =
      let l = [{ x = 5, y = 5, w = 10, h = 10, rot = 0 }] in
      let r = [{ x = 5, y = 5, w = 10, h = 10, rot = 0 }] in
      let ps = split { gap = 4, left = l, right = r } in
      {
        actual = (std.array.at 1 ps).x,
        expected = 19,
      },

    check_split_3x5_3_count = {
      actual = std.array.length (split_3x5_3 {}),
      expected = 36,
    },

    check_split_3x5_3_rhs_is_mirror =
      let ps = split_3x5_3 {} in
      let px = fun i => (std.array.at i ps).x in
      {
        actual = {
          right_pinky_outer = px 9 > px 5,
          right_thumbs_toward_gap = px 33 < px 9,
          right_thumbs_not_outer = px 35 < px 9,
        },
        expected = {
          right_pinky_outer = true,
          right_thumbs_toward_gap = true,
          right_thumbs_not_outer = true,
        },
      },
  },
}
