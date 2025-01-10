let K = import "keys.ncl" in

{
  layers = [
    [
      K.N1, K.Q,  K.W,  K.E,  K.R,  K.T,      K.Y,  K.U,  K.I,     K.O,   K.P,         K.N5,
      K.N2, K.A,  K.S,  K.D,  K.F,  K.G,      K.H,  K.J,  K.K,     K.L,   K.Semicolon, K.N6,
      K.N3, K.Z,  K.X,  K.C,  K.V,  K.B,      K.N,  K.M,  K.Comma, K.Dot, K.Slash,     K.N7,
      K.N4, K.N1, K.N2, K.N3, K.N4, K.N5,     K.N6, K.N7, K.N8,    K.N9,  K.N0,        K.N8
    ]
  ],
}
