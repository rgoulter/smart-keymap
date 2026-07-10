let K = import "keys.ncl" in

{
  layers = [
    [
      { layer_modifier = { hold = 1 } }, K.A, K.B,
    ],
    [
      null, K.C,
    ],
  ],
}
