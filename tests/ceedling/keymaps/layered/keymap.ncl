let K = import "keys.ncl" in

{
  layers = [
    [
      { layer_modifier = { hold = 1 } },
      K.A,
    ],
    [
      null,
      K.B,
    ],
  ],

  ceedling_fixture = {
    suite = "LAYERED",
    LAYER_MOD_KEY = 0,
    LAYERED_KEY = 1,
  },
}
