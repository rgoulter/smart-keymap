let remap_8keys = { source_key_count = 8 } & (import "layouts/remap.ncl") in

let from_artseyio = fun
  {
    no_key = XXX,
    from_layout = [
      k00, k01, k02, k03,
      k04, k05, k06, k07,
    ]
  } =>
  [
    k00, k01, k02, k03, XXX,           XXX, XXX, XXX, XXX, XXX,
    k04, k05, k06, k07, XXX,           XXX, XXX, XXX, XXX, XXX,
    XXX, XXX, XXX, XXX, XXX, XXX, XXX, XXX, XXX, XXX, XXX, XXX,
    XXX, XXX, XXX, XXX, XXX, XXX, XXX, XXX, XXX, XXX, XXX, XXX,
  ]
in
(import "../keymap-8key-artseyio-rhs/keymap.ncl")
  |> remap_8keys.into_layout from_artseyio
