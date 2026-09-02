let remap_8keys = { source_key_count = 8 } & (import "layouts/remap.ncl") in

let from_artseyio = fun
  {
    no_key = XXX,
    from_layout = [
      a, r, t, s,
      y, e, i, o,
    ]
  } =>
  [
    s, t, r, a,
    o, i, e, y,
  ]
in
(import "../keymap-8key-artseyio-rhs/keymap.ncl")
  |> remap_8keys.into_layout from_artseyio
