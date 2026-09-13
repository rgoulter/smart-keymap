# Scene graph with semantic roles. Theme is applied at SVG time.
let P = import "viz/points.ncl" in
{
  # cells[i] aligns with points[i]:
  #   { legends = { center, hold, tl, tr, bl }, emphasis = 'Normal | 'Pressed | 'Muted }
  pane_nodes = fun args =>
    let placed = args.placed in
    let cells = args.cells in
    let title = P.get_or args "title" "" in
    let canvas = P.get_or args "canvas" P.default_canvas in
    let y_off = P.get_or args "y_off" 0 in
    let title_nodes =
      if title == "" then
        []
      else
        [
          {
            kind = 'Title,
            x = 16,
            y = y_off + 20,
            text = title,
          }
        ]
    in
    let key_nodes =
      placed
      |> std.array.map_with_index (fun i pk =>
        let cell = std.array.at i cells in
        let fill_role =
          cell.emphasis
          |> match {
            'Pressed => 'KeyFillPressed,
            'Muted => 'KeyFillMuted,
            _ => 'KeyFill,
          }
        in
        let show = cell.emphasis != 'Muted in
        let L = cell.legends in
        let rect = [
          {
            kind = 'Rect,
            x = pk.x,
            y = pk.y + y_off,
            w = pk.w,
            h = pk.h,
            rx = canvas.rx,
            cx = pk.cx,
            cy = pk.cy + y_off,
            rot = pk.svg_rot,
            role = fill_role,
          }
        ]
        in
        let center =
          if !show || L.center == "" then
            []
          else
            [
              {
                kind = 'Text,
                x = pk.cx,
                y = pk.cy + y_off + 4,
                anchor = "middle",
                role = 'Legend,
                size = 12,
                weight = "600",
                text = L.center,
              }
            ]
        in
        let hold =
          if !show || L.hold == "" then
            []
          else
            [
              {
                kind = 'Text,
                x = pk.cx,
                y = pk.y + y_off + pk.h - 6,
                anchor = "middle",
                role = 'Hold,
                size = 8,
                weight = "400",
                text = L.hold,
              }
            ]
        in
        let corner = fun txt pos_x pos_y text_anchor text_role =>
          if !show || txt == "" then
            []
          else
            [
              {
                kind = 'Text,
                x = pos_x,
                y = pos_y,
                anchor = text_anchor,
                role = text_role,
                size = 8,
                weight = "400",
                text = txt,
              }
            ]
        in
        rect
        @ center
        @ hold
        @ corner L.tl (pk.x + 6) (pk.y + y_off + 12) "start" 'Overlay
        @ corner L.tr (pk.x + pk.w - 6) (pk.y + y_off + 12) "end" 'Overlay2
        @ corner L.bl (pk.x + 6) (pk.y + y_off + pk.h - 6) "start" 'Overlay3
      )
      |> std.array.flatten
    in
    title_nodes @ key_nodes,

  output_nodes = fun args =>
    let placed = args.placed in
    let indices = args.indices in
    let label_text = args.text in
    let y_off = args.y_off in
    if label_text == "" || indices == [] then
      []
    else
      let pts =
        indices
        |> std.array.filter (fun i => i >= 0 && i < std.array.length placed)
        |> std.array.map (fun i => std.array.at i placed)
      in
      pts
      |> match {
        [] => [],
        pts =>
          let n = std.array.length pts in
          let cx = (pts |> std.array.map (fun p => p.cx) |> std.array.fold_left (+) 0) / n in
          let cy = (pts |> std.array.map (fun p => p.cy) |> P.min_of) - 14 + y_off in
          let box_w = 12 + 7 * std.string.length label_text in
          [
            {
              kind = 'OutputBox,
              x = cx - box_w / 2,
              y = cy - 10,
              w = box_w,
              h = 16,
            },
            {
              kind = 'Text,
              x = cx,
              y = cy + 2,
              anchor = "middle",
              role = 'ChordOutput,
              size = 9,
              weight = "600",
              text = label_text,
            },
          ],
      },

  of_panes = fun args =>
    let src_points = args.points in
    let y_up_flag = P.get_or args "y_up" false in
    let canv = P.get_or args "canvas" P.default_canvas in
    let panes = args.panes in
    let sizes =
      panes
      |> std.array.map (fun pane =>
        P.place {
          points = src_points,
          y_up = y_up_flag,
          canvas = canv,
          title = pane.title != "",
        }
      )
    in
    let scene_w =
      sizes
      |> std.array.map (fun s => s.width)
      |> P.max_of
    in
    let stacked =
      std.array.generate (fun i => i) (std.array.length panes)
      |> std.array.fold_left
        (fun acc i =>
          let pane = std.array.at i panes in
          let sz = std.array.at i sizes in
          let extra =
            pane_nodes {
              placed = sz.placed,
              cells = pane.cells,
              title = pane.title,
              canvas = canv,
              y_off = acc.height,
            }
            @ (
              if std.record.has_field "output" pane then
                output_nodes {
                  placed = sz.placed,
                  indices = pane.output.indices,
                  text = pane.output.text,
                  y_off = acc.height,
                }
              else
                []
            )
          in
          {
            nodes = acc.nodes @ extra,
            height = acc.height + sz.height + canv.pane_gap,
          }
        )
        { nodes = [], height = 0 }
    in
    {
      width = scene_w,
      height = stacked.height - canv.pane_gap + canv.pad,
      nodes = stacked.nodes,
    },
}
