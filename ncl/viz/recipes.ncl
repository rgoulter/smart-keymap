# View recipes. Layer boards fail closed when there is no layer sugar.
let L = import "viz/legends.ncl" in
{
  normal = 'Normal,
  pressed = 'Pressed,
  muted = 'Muted,

  cell = fun key_legends key_emphasis => { legends = key_legends, emphasis = key_emphasis },

  # One pane: base-layer legends, all keys normal.
  legends = fun args =>
    let keys = args.keys in
    let heading =
      if std.record.has_field "title" args then args.title else "Base"
    in
    let pane_cells =
      keys
      |> std.array.map (fun k => cell (L.of_key k) 'Normal)
    in
    { panes = [{ title = heading, cells = pane_cells }] },

  # Stacked layer boards. 'NoLayerSugar when there is only a single layer row.
  stacked = fun args =>
    let layers = args.layers in
    let heading =
      if std.record.has_field "title" args then args.title else "Stacked"
    in
    if std.array.length layers <= 1 then
      'NoLayerSugar
    else
      let pane_list =
        layers
        |> std.array.map_with_index (fun i layer =>
          {
            title =
              if i == 0 then
                "%{heading} · layer %{i}"
              else
                "layer %{i}",
            cells = layer |> std.array.map (fun k => cell (L.of_layer_key k) 'Normal),
          }
        )
      in
      'Panes { panes = pane_list },

  # Dense corners from other numbered layers. Missing slots omitted (empty string).
  dense = fun args =>
    let layers = args.layers in
    let heading =
      if std.record.has_field "title" args then args.title else "Dense"
    in
    if std.array.length layers <= 1 then
      'NoLayerSugar
    else
      let n = std.array.length (std.array.at 0 layers) in
      let slot = fun li ki =>
        if li < std.array.length layers then
          let k = std.array.at ki (std.array.at li layers) in
          (L.of_layer_key k).center
        else
          ""
      in
      let pane_cells =
        std.array.generate
          (fun i =>
            let base = L.of_layer_key (std.array.at i (std.array.at 0 layers)) in
            cell
              {
                center = base.center,
                hold = base.hold,
                tl = slot 1 i,
                tr = slot 2 i,
                bl = slot 3 i,
              }
              'Normal
          )
          n
      in
      'Panes { panes = [{ title = heading, cells = pane_cells }] },

  # One pane per chord: non-members muted, members pressed, output at centroid.
  chords = fun args =>
    let keys = args.keys in
    let chords = args.chords in
    let heading =
      if std.record.has_field "title" args then args.title else "Chords"
    in
    if chords == [] then
      'NoChords
    else
      let n = std.array.length keys in
      let pane_list =
        chords
        |> std.array.map (fun ch =>
          let active = ch.indices in
          let pane_cells =
            std.array.generate
              (fun i =>
                if std.array.elem i active then
                  cell (L.of_layer_key (std.array.at i keys)) 'Pressed
                else
                  cell L.empty 'Muted
              )
              n
          in
          {
            title = heading,
            cells = pane_cells,
            output = { text = L.label ch.key, indices = ch.indices },
          }
        )
      in
      'Panes { panes = pane_list },

  checks =
    let K = import "keys.ncl" in
    {
      check_no_layer_sugar_stacked = {
        actual = stacked { points = [], layers = [[K.A, K.B]] },
        expected = 'NoLayerSugar,
      },
      check_stacked_two_layers =
        stacked { points = [], layers = [[K.A, K.B], [K.C, null ]] }
        |> match {
          'Panes { panes } =>
            {
              actual = {
                n = std.array.length panes,
                l1 = (std.array.at 0 (std.array.at 1 panes).cells).legends.center,
              },
              expected = { n = 2, l1 = "C" },
            },
          _ => { actual = false, expected = true },
        },
      check_no_chords = {
        actual = chords { points = [], keys = [K.A], chords = [] },
        expected = 'NoChords,
      },
    },
}
