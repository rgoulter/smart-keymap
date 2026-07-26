let K = import "keys.ncl" in

# Classic tri-layer: lower (1) + raise (2) → adjust (3).
{
  conditional_layers = [
    { then_layer = 3, if_layers = [1, 2] },
  ],
  layers = [
    [
      K.layer_mod.hold 1,
      K.layer_mod.hold 2,
      K.A,
    ],
    [
      K.TTTT,
      K.TTTT,
      K.B,
    ],
    [
      K.TTTT,
      K.TTTT,
      K.C,
    ],
    [
      K.TTTT,
      K.TTTT,
      K.D,
    ],
  ],

  ceedling_fixture = {
    suite = "CONDITIONAL_LAYERS",
    LOWER_KEY = 0,
    RAISE_KEY = 1,
    LETTER_KEY = 2,
  },
}
