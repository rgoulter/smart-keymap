# Scene → SVG string. Pure.
let theme = import "viz/theme.ncl" in
{
  include theme,

  escape = fun s =>
    s
    |> std.string.replace "&" "&amp;"
    |> std.string.replace "<" "&lt;"
    |> std.string.replace ">" "&gt;"
    |> std.string.replace "\"" "&quot;",

  n = fun x => std.string.from x,

  layer_swatch = fun th i =>
    let pal = th.layers in
    let n = std.array.length pal in
    let j =
      if i < 0 then
        0
      else if i >= n then
        i % n
      else
        i
    in
    std.array.at j pal,

  pair_of = fun th node =>
    node.look
    |> match {
      'Pressed => { side = th.pressed_side, top = th.pressed_top, op = "1" },
      'Muted => { side = th.muted_side, top = th.muted_top, op = th.muted_opacity },
      'Unused => { side = th.unused_side, top = th.unused_top, op = "1" },
      'Transparent => { side = th.ghost_side, top = th.ghost_top, op = th.ghost_opacity },
      _ =>
        if std.record.has_field "wash" node then
          let sw = layer_swatch th node.wash in
          { side = sw.side, top = sw.top, op = "1" }
        else
          { side = th.key_side, top = th.key_top, op = "1" },
    },

  fill_of = fun th role =>
    role
    |> match {
      'Legend => th.legend,
      'Hold => th.hold,
      'Overlay => th.overlay,
      'Overlay2 => th.overlay2,
      'Overlay3 => th.overlay3,
      'ChordOutput => th.chord_output,
      'Title => th.title,
      _ => th.legend,
    },

  rect_svg = fun node fill op stroke =>
    let body = "<rect x=\"%{n node.x}\" y=\"%{n node.y}\" width=\"%{n node.w}\" height=\"%{n node.h}\" rx=\"%{n node.rx}\" fill=\"%{fill}\" stroke=\"%{stroke}\" opacity=\"%{op}\"/>" in
    if node.rot == 0 then
      body
    else
      "<g transform=\"rotate(%{n node.rot} %{n node.cx} %{n node.cy})\">%{body}</g>",

  node_svg = fun th node =>
    node.kind
    |> match {
      'KeySide =>
        let pair = pair_of th node in
        rect_svg node pair.side pair.op th.key_stroke,
      'KeyTop =>
        let pair = pair_of th node in
        rect_svg node pair.top pair.op th.key_stroke,
      'Wash =>
        let sw = layer_swatch th node.wash in
        rect_svg node sw.top "0.55" "none",
      'Panel =>
        rect_svg node th.chord_panel "1" th.chord_panel_stroke,
      'Rect =>
        rect_svg node th.key_top "1" th.key_stroke,
      'Text =>
        "<text x=\"%{n node.x}\" y=\"%{n node.y}\" text-anchor=\"%{node.anchor}\" fill=\"%{fill_of th node.role}\" font-family=\"%{th.font}\" font-size=\"%{n node.size}\" font-weight=\"%{node.weight}\">%{escape node.text}</text>",
      'Title =>
        "<text x=\"%{n node.x}\" y=\"%{n node.y}\" fill=\"%{th.title}\" font-family=\"%{th.title_font}\" font-size=\"14\">%{escape node.text}</text>",
      'Caption =>
        "<text x=\"%{n node.x}\" y=\"%{n node.y}\" text-anchor=\"middle\" fill=\"%{th.chord_output}\" font-family=\"%{th.title_font}\" font-size=\"11\">%{escape node.text}</text>",
      _ => "",
    },

  of_scene = fun scene th =>
    let inner =
      scene.nodes
      |> std.array.map (node_svg th)
      |> std.string.join "\n"
    in
    "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"%{n scene.width}\" height=\"%{n scene.height}\" viewBox=\"0 0 %{n scene.width} %{n scene.height}\">\n<rect width=\"100%\" height=\"100%\" fill=\"%{th.background}\"/>\n%{inner}\n</svg>\n",
}
