let K = import "keys.ncl" in

{
  layers = [
    [
      K.layer_mod.hold "fn", K.A,
    ],
  ],
  named_layers = {
    fn = [
      K.TTTT, K.B,
    ],
  },
}
