# Demo: remap a compact source keymap (with named_layers) into a wider layout.
#
# Source is 2 keys: layer-mod + layered key. Target is 3 keys with a gap
# (NO_KEY) in the middle — same idea as split 3x5+3 → ortho 4x12.
let remap = (import "layouts/remap.ncl") & { source_key_count = 2 } in
let K = import "keys.ncl" in

let from_2_to_3 = fun {
  no_key = XXX,
  from_layout = [a, b],
} =>
  [a, XXX, b]
in

{
  layers = [
    [
      K.layer_mod.hold "fn",
      K.A,
    ],
  ],

  named_layers = {
    fn = [
      null,
      K.B,
    ],
  },

  ceedling_fixture = {
    suite = "REMAP_NAMED_LAYERS",
    LAYER_MOD_KEY = 0,
    NO_KEY = 1,
    LAYERED_KEY = 2,
  },
}
|> remap.into_layout from_2_to_3
