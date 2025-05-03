let remap = import "layouts/remap.ncl" in

let remap_34keys = remap & { source_key_count = 34 } in

let from_3x5_2 = fun
  {
    no_key = XXX,
    from_layout = [
      k00, k01, k02, k03, k04,           k05, k06, k07, k08, k09,
      k10, k11, k12, k13, k14,           k15, k16, k17, k18, k19,
      k20, k21, k22, k23, k24,           k25, k26, k27, k28, k29,
                          k30, k31, k32, k33,
    ]
  } =>
  [
    k00, k01, k02, k03, k04,           k05, k06, k07, k08, k09,
    k10, k11, k12, k13, k14,           k15, k16, k17, k18, k19,
    k20, k21, k22, k23, k24,           k25, k26, k27, k28, k29,
    XXX, XXX, XXX, XXX, k30, k31, k32, k33, XXX, XXX, XXX, XXX,
  ]
in
(import "../keymap-34key-seniply/keymap.ncl")
  |> remap_34keys.into_layout from_3x5_2
