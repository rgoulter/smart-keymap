# Nested named layer_mod under composites that must fully lower + codegen:
#   chorded → layered (from named_layers) → tap_hold → layer_mod.hold "fn"
#
# Regression for resolve_named_layer_refs only walking one map_accum level
# (left Hold: ["fn", 0] in JSON and broke smart_key.Json / keymap.rs codegen).
let K = import "keys.ncl" in

{
  chords = [
    {
      indices = [0, 1],
      key = K.C,
    },
  ],

  layers = [
    [
      K.Escape & K.hold (K.layer_mod.hold "fn"),
      K.A,
    ],
  ],

  named_layers = {
    fn = [
      null,
      K.B,
    ],
  },
}
