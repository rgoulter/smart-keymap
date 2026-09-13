# Keymap / layout visualiser (FCIS).
#
# Physical combinators emit points; keymap.ncl keys zip by array order;
# recipes build panes; scene is role-typed; SVG is one projection.
let points_m = import "viz/points.ncl" in
let combinators_m = import "viz/combinators.ncl" in
let bind_m = import "viz/bind.ncl" in
let legends_m = import "viz/legends.ncl" in
let theme_m = import "viz/theme.ncl" in
let scene_m = import "viz/scene.ncl" in
let svg_m = import "viz/svg.ncl" in
let recipes_m = import "viz/recipes.ncl" in
{
  points = points_m,
  combinators = combinators_m,
  bind = bind_m,
  legends = legends_m,
  theme = theme_m,
  scene = scene_m,
  svg = svg_m,
  recipes = recipes_m,

  render = fun args =>
    let sc =
      scene_m.of_panes {
        points = args.points,
        y_up = points_m.get_or args "y_up" false,
        panes = args.panes,
      }
    in
    svg_m.of_scene sc args.theme,

  svg_of = fun result args =>
    result
    |> match {
      'Panes { panes = pane_list } =>
        render {
          points = args.points,
          y_up = points_m.get_or args "y_up" false,
          panes = pane_list,
          theme = args.theme,
        },
      { panes = pane_list } =>
        render {
          points = args.points,
          y_up = points_m.get_or args "y_up" false,
          panes = pane_list,
          theme = args.theme,
        },
      'NoLayerSugar => std.fail_with "viz: no layer sugar for this recipe",
      'NoChords => std.fail_with "viz: no chords for this recipe",
    },

  files_for = fun args =>
    let flip_y = points_m.get_or args "y_up" false in
    [
      {
        name = "%{args.stem}-light.svg",
        svg = svg_of args.result { points = args.points, y_up = flip_y, theme = theme_m.light },
      },
      {
        name = "%{args.stem}-dark.svg",
        svg = svg_of args.result { points = args.points, y_up = flip_y, theme = theme_m.dark },
      },
    ],

  checks =
    points_m.checks
    & combinators_m.checks
    & bind_m.checks
    & legends_m.checks
    & recipes_m.checks
    & scene_m.checks
    & {
      check_48key_dense_is_panes =
        let km =
          (import "../tests/ncl/keymap-48key-basic/keymap.ncl")
          & (import "keymap-ncl-to-json.ncl")
        in
        let pts = combinators_m.grid { rows = 4, cols = 12 } in
        recipes_m.dense { points = pts, layers = km.layers_of_keys }
        |> match {
          'Panes { panes } =>
            {
              actual = {
                n_panes = std.array.length panes,
                n_cells = std.array.length (std.array.at 0 panes).cells,
                tab = (std.array.at 0 (std.array.at 0 panes).cells).legends.center,
              },
              expected = { n_panes = 1, n_cells = 48, tab = "Tab" },
            },
          _ => { actual = 'NoLayerSugar, expected = 'Panes },
        },

      check_48key_scene_has_rects =
        let km =
          (import "../tests/ncl/keymap-48key-basic/keymap.ncl")
          & (import "keymap-ncl-to-json.ncl")
        in
        let pts = combinators_m.grid { rows = 4, cols = 12 } in
        let r = recipes_m.legends { points = pts, keys = std.array.at 0 km.layers_of_keys } in
        let sc = scene_m.of_panes { points = pts, panes = r.panes } in
        let top_count =
          sc.nodes
          |> std.array.filter (fun n => n.kind == 'KeyTop)
          |> std.array.length
        in
        let saw_tab = std.array.any (fun n => n.kind == 'Text && n.text == "Tab") sc.nodes in
        {
          actual = { n_top = top_count, has_tab = saw_tab },
          expected = { n_top = 48, has_tab = true },
        },
    },
}
