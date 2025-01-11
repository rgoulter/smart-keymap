let K = import "keys.ncl" in

{
  layers = [
    [
      { layer_modifier = { hold = 0 } }, K.A,
    ],
    [
      null, K.B,
    ],
  ],
}
