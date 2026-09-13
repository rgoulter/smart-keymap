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

  fill_of = fun th role =>
    role
    |> match {
      'KeyFill => th.key_fill,
      'KeyFillPressed => th.pressed_fill,
      'KeyFillMuted => th.muted_fill,
      'Legend => th.legend,
      'Hold => th.hold,
      'Overlay => th.overlay,
      'Overlay2 => th.overlay2,
      'Overlay3 => th.overlay3,
      'ChordOutput => th.chord_output,
      'Title => th.title,
      _ => th.legend,
    },

  opacity_of = fun th role =>
    role
    |> match {
      'KeyFillMuted => th.muted_opacity,
      _ => "1",
    },

  node_svg = fun th node =>
    node.kind
    |> match {
      'Rect =>
        let op = opacity_of th node.role in
        let body = "<rect x=\"%{n node.x}\" y=\"%{n node.y}\" width=\"%{n node.w}\" height=\"%{n node.h}\" rx=\"%{n node.rx}\" fill=\"%{fill_of th node.role}\" stroke=\"%{th.key_stroke}\" opacity=\"%{op}\"/>" in
        if node.rot == 0 then
          body
        else
          "<g transform=\"rotate(%{n node.rot} %{n node.cx} %{n node.cy})\">%{body}</g>",
      'Text =>
        "<text x=\"%{n node.x}\" y=\"%{n node.y}\" text-anchor=\"%{node.anchor}\" fill=\"%{fill_of th node.role}\" font-family=\"%{th.font}\" font-size=\"%{n node.size}\" font-weight=\"%{node.weight}\">%{escape node.text}</text>",
      'Title =>
        "<text x=\"%{n node.x}\" y=\"%{n node.y}\" fill=\"%{th.title}\" font-family=\"%{th.title_font}\" font-size=\"14\">%{escape node.text}</text>",
      'OutputBox =>
        "<rect x=\"%{n node.x}\" y=\"%{n node.y}\" width=\"%{n node.w}\" height=\"%{n node.h}\" rx=\"3\" fill=\"%{th.chord_output_fill}\" stroke=\"%{th.chord_output}\"/>",
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
