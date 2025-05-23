let K = import "keys.ncl" in

# Define tap_hold keys
# by merging a `K.hold` modifier
# with a key.
let A_A = K.A & K.hold K.LeftAlt in
let G_O = K.O & K.hold K.LeftGUI in
let C_E = K.E & K.hold K.LeftCtrl in
let S_U = K.U & K.hold K.LeftShift in
let S_H = K.H & K.hold K.RightShift in
let C_T = K.T & K.hold K.RightCtrl in
let G_N = K.N & K.hold K.RightGUI in
let A_S = K.S & K.hold K.RightAlt in

{
  keys = [
    K.Quote, K.Comma, K.Dot, K.P, K.Y,          K.F, K.G, K.C, K.R, K.L,
    A_A, G_O, C_E, S_U, K.I,                    K.D, S_H, C_T, G_N, A_S,
    K.Semicolon, K.Q, K.J, K.K, K.X,            K.B, K.M, K.W, K.V, K.Z,
    K.LeftCtrl, K.LeftGUI, K.LeftAlt, K.Tab, K.Escape, K.Space,   K.Backspace, K.Return, K.Delete, K.RightAlt, K.RightGUI, K.RightCtrl,
  ],
}
