let K = import "keys.ncl" in

{
  custom_keys = fun K =>
    {
      FN = K.layer_mod.hold "fn",
    },

  layers = [
    m%"
      FN   A
    "%,
  ],
  named_layers = {
    fn = m%"
      TTTT B
    "%,
  },
}
