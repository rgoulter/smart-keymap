let K = import "keys.ncl" in

let A_A = K.A & K.hold K.LeftAlt in
let G_O = K.O & K.hold K.LeftGUI in
let C_E = K.E & K.hold K.LeftCtrl in
let S_U = K.U & K.hold K.LeftShift in
let S_H = K.H & K.hold K.RightShift in
let C_T = K.T & K.hold K.LeftCtrl in
let G_N = K.N & K.hold K.RightGUI in
let A_S = K.S & K.hold K.LeftAlt in

let RAISE = K.layer_mod.hold 0 in
let LOWER = K.layer_mod.hold 1 in
let ADJUST = K.layer_mod.hold 2 in

let LW_ES = K.ESC & K.hold LOWER in
let RS_EN = K.ENT & K.hold RAISE in

let TTTTTT = K.TTTT in

{
  config.tap_hold.interrupt_response = "HoldOnKeyTap",
  config.tap_hold.timeout = 200 / 8,
  layers = [
    [
      K.GRV,  K.N1,   K.N2,   K.N3,  K.N4,  K.N5,  K.N6,   K.N7,  K.N8,   K.N9,   K.N0,   K.DEL,
      K.TAB,  K.QUOT, K.COMM, K.DOT, K.P,   K.Y,   K.F,    K.G,   K.C,    K.R,    K.L,    K.BSPC,
      K.ESC,  A_A,    G_O,    C_E,   S_U,   K.I,   K.D,    S_H,   C_T,    G_N,    A_S,    K.RET,
      K.LSFT, K.SCLN, K.Q,    K.J,   K.K,   K.X,   K.B,    K.M,   K.W,    K.V,    K.Z,    K.RSFT,
      K.LCTL, K.LGUI, K.LALT, K.TAB, LW_ES, K.SPC, K.BSPC, RS_EN, K.DEL,  K.RALT, K.RGUI, K.RCTL,
    ],
    # Raise
    [
      TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT,
      K.GRV,  K.N1,   K.N2,   K.N3,   K.N4,   K.N5,   K.N6,   K.N7,   K.N8,   K.N9,   K.N0,   K.BSLS,
      K.DEL,  K.F1,   K.F2,   K.F3,   K.F4,   K.F5,   K.F6,   K.MINS, K.EQLS, K.LBRK, K.RBRK, K.SLSH,
      TTTTTT, K.F7,   K.F8,   K.F9,   K.F10,  K.F11,  K.F12,  TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT,
      TTTTTT, TTTTTT, TTTTTT, TTTTTT, ADJUST, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT,
    ],
    # Lower
    [
      TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT,
      K.TILD, K.EXCL, K.AT,   K.HASH, K.DLR,  K.PERC, K.CARE, K.AMP,  K.ASTR, K.LPRN, K.RPRN, K.PIPE,
      K.INS,  K.F1,   K.F2,   K.F3,   K.F4,   K.F5,   K.F6,   K.UNDS, K.PLUS, K.LCBR, K.RCBR, K.QUES,
      TTTTTT, K.F7,   K.F8,   K.F9,   K.F10,  K.F11,  K.F12,  TTTTTT, K.HOME, K.PGDN, K.PGUP, K.END,
      TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, ADJUST, K.LEFT, K.DOWN, K.UP,   K.RGHT,
    ],
    # Adjust
    [
      TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT,
      TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, K.PSCR, K.SCRL, K.PAUS, TTTTTT, TTTTTT,
      K.CAPS, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT,
      TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT,
      TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT,
    ],
  ],
}
