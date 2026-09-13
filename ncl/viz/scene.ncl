# Scene graph with semantic roles. Theme is applied at SVG time.
#
# Keycaps follow KLE: side/border + raised top, hold on the front lip.
let P = import "viz/points.ncl" in
{
  # KLE 54px 1u: top inset 6 left/right, 3 top, 9 bottom lip.
  # Insets are fractions of *key height* (1u), not width, so a 6.25u
  #  spacebar keeps the same side walls as a 1u key.
  top_ox = 0.111,
  top_oy = 0.056,
  top_bot = 0.167,

  keytop_box = fun args =>
    let u1 = args.h in
    let inset_x = u1 * top_ox in
    let inset_top = u1 * top_oy in
    let inset_bot = u1 * top_bot in
    {
      x = args.x + inset_x,
      y = args.y + inset_top,
      w = args.w - 2 * inset_x,
      h = args.h - inset_top - inset_bot,
    },

  show_text = fun em => em == 'Normal || em == 'Pressed,

  draw_keycap = fun args =>
    let pk = args.placed in
    let cell = args.cell in
    let y_off = args.y_off in
    let canv = args.canvas in
    let L = cell.legends in
    let em = cell.emphasis in
    let ox = pk.x in
    let oy = pk.y + y_off in
    let ow = pk.w in
    let oh = pk.h in
    let ocx = pk.cx in
    let ocy = pk.cy + y_off in
    let kt =
      keytop_box {
        x = ox,
        y = oy,
        w = ow,
        h = oh,
      }
    in
    let tx = kt.x in
    let ty = kt.y in
    let tw = kt.w in
    let th = kt.h in
    let tcx = tx + tw / 2 in
    let tcy = ty + th / 2 in
    let lip_y = ty + th + (oy + oh - (ty + th)) * 0.55 in
    let rx_side = canv.rx in
    let rx_top = 3 in
    let side = {
      kind = 'KeySide,
      look = em,
      x = ox,
      y = oy,
      w = ow,
      h = oh,
      rx = rx_side,
      cx = ocx,
      cy = ocy,
      rot = pk.svg_rot,
    }
    in
    let top = {
      kind = 'KeyTop,
      look = em,
      x = tx,
      y = ty,
      w = tw,
      h = th,
      rx = rx_top,
      cx = ocx,
      cy = ocy,
      rot = pk.svg_rot,
    }
    in
    let texts =
      if !show_text em then
        []
      else
        let center =
          if L.center == "" then
            []
          else
            [
              {
                kind = 'Text,
                x = tcx,
                y = tcy + 4,
                anchor = "middle",
                role = 'Legend,
                size = 11,
                weight = "600",
                text = L.center,
              }
            ]
        in
        let hold =
          if L.hold == "" then
            []
          else
            [
              {
                kind = 'Text,
                x = ocx,
                y = lip_y,
                anchor = "middle",
                role = 'Hold,
                size = 8,
                weight = "400",
                text = L.hold,
              }
            ]
        in
        let corner = fun txt pos_x pos_y text_anchor text_role =>
          if txt == "" then
            []
          else
            [
              {
                kind = 'Text,
                x = pos_x,
                y = pos_y,
                anchor = text_anchor,
                role = text_role,
                size = 7,
                weight = "400",
                text = txt,
              }
            ]
        in
        center
        @ hold
        @ corner L.tl (tx + 4) (ty + 10) "start" 'Overlay
        @ corner L.tr (tx + tw - 4) (ty + 10) "end" 'Overlay2
        @ corner L.bl (tx + 4) (ty + th - 5) "start" 'Overlay3
    in
    [side, top] @ texts,

  virtual_placed = fun args =>
    let vcx = args.cx in
    let vcy = args.cy in
    let size = args.size in
    {
      x = vcx - size / 2,
      y = vcy - size / 2,
      w = size,
      h = size,
      cx = vcx,
      cy = vcy,
      svg_rot = 0,
    },

  pane_nodes = fun args =>
    let placed = args.placed in
    let cells = args.cells in
    let title = P.get_or args "title" "" in
    let canv = P.get_or args "canvas" P.default_canvas in
    let y_shift = P.get_or args "y_off" 0 in
    let title_nodes =
      if title == "" then
        []
      else
        [
          {
            kind = 'Title,
            x = 16,
            y = y_shift + 20,
            text = title,
          }
        ]
    in
    let key_nodes =
      placed
      |> std.array.map_with_index (fun i pk =>
        draw_keycap {
          placed = pk,
          cell = std.array.at i cells,
          y_off = y_shift,
          canvas = canv,
        }
      )
      |> std.array.flatten
    in
    title_nodes @ key_nodes,

  of_panes = fun args =>
    let src_points = args.points in
    let y_up_flag = P.get_or args "y_up" false in
    let canv = P.get_or args "canvas" P.default_canvas in
    let panes = args.panes in
    let u_px = P.unit * canv.scale in
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
    let any_guide =
      panes
      |> std.array.any (fun pane => std.record.has_field "guide" pane)
    in
    let out_w = scene_w + (if any_guide then u_px + 24 else 0) in
    let stacked =
      std.array.generate (fun i => i) (std.array.length panes)
      |> std.array.fold_left
        (fun acc i =>
          let pane = std.array.at i panes in
          let sz = std.array.at i sizes in
          let has_footer = std.record.has_field "footer" pane in
          let has_guide = std.record.has_field "guide" pane in
          let extra =
            pane_nodes {
              placed = sz.placed,
              cells = pane.cells,
              title = pane.title,
              canvas = canv,
              y_off = acc.height,
            }
            @ (
              if has_guide then
                draw_keycap {
                  placed =
                    virtual_placed {
                      cx = sz.width + 8 + u_px / 2,
                      cy = acc.height + sz.title_h + canv.pad + u_px / 2,
                      size = u_px,
                    },
                  cell = {
                    legends = pane.guide,
                    emphasis = 'Normal,
                  },
                  y_off = 0,
                  canvas = canv,
                }
              else
                []
            )
            @ (
              if has_footer then
                draw_keycap {
                  placed =
                    virtual_placed {
                      cx = sz.width / 2,
                      cy = acc.height + sz.height - canv.pad + 10 + u_px / 2,
                      size = u_px,
                    },
                  cell = {
                    legends = pane.footer,
                    emphasis = 'Pressed,
                  },
                  y_off = 0,
                  canvas = canv,
                }
              else
                []
            )
          in
          {
            nodes = acc.nodes @ extra,
            height =
              acc.height
              + sz.height
              + canv.pane_gap
              + (if has_footer then u_px + 14 else 0),
          }
        )
        { nodes = [], height = 0 }
    in
    {
      width = out_w,
      height = stacked.height - canv.pane_gap + canv.pad,
      nodes = stacked.nodes,
    },

  checks = {
    check_wide_key_same_side_walls =
      let u1 = keytop_box { x = 0, y = 0, w = 50, h = 50 } in
      let wide = keytop_box { x = 0, y = 0, w = 300, h = 50 } in
      {
        actual = {
          same_left = u1.x == wide.x,
          extra_width = wide.w - u1.w,
        },
        expected = { same_left = true, extra_width = 250 },
      },
  },
}
