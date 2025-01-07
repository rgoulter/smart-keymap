let K = import "hid-usage-keyboard.ncl" in

{
  layers = [
    [
      K.Grave, K.N1, K.N2, K.N3, K.N4, K.N5, K.N6, K.N7, K.N8, K.N9, K.N0, K.Delete,
      K.Tab,       K.Quote, K.Comma, K.Dot, K.P, K.Y,    K.F, K.G, K.C, K.R, K.L, K.Backspace,
      K.Escape,    K.A, K.O, K.E, K.U, K.I,              K.D, K.H, K.T, K.N, K.S, K.Return,
      K.LeftShift, K.Semicolon, K.Q, K.J, K.K, K.X,      K.B, K.M, K.W, K.V, K.Z, K.RightShift,
      K.LeftCtrl, K.LeftGUI, K.LeftAlt, K.Tab, K.Escape, K.Space,   K.Backspace, K.Return, K.Delete, K.RightAlt, K.RightGUI, K.RightCtrl,
    ]
  ],
}
