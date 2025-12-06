let K = import "keys.ncl" in

let RAISE = K.layer_mod.hold 1 in
let LOWER = K.layer_mod.hold 2 in
let ADJUST = K.layer_mod.hold 3 in

let BOOT = K.reset_to_bootloader in

let TTTTTT = K.TTTT in

{
  layers = [
    [
      K.TAB,  K.Q,    K.W,    K.E,   K.R,   K.T,   K.Y,   K.U,   K.I,    K.O,    K.P,    K.BSPC,
      K.ESC,  K.A,    K.S,    K.D,   K.F,   K.G,   K.H,   K.J,   K.K,    K.L,    K.SCLN, K.QUOT,
      K.LSFT, K.Z,    K.X,    K.C,   K.V,   K.B,   K.N,   K.M,   K.COMM, K.DOT,  K.SLSH, K.RSFT,
      K.LCTL, K.LGUI, K.LALT, K.TAB, LOWER, K.SPC, K.RET, RAISE, K.DEL,  K.RALT, K.RGUI, K.RCTL,
    ],
    # Raise
    [
      K.GRV,  K.N1,   K.N2,   K.N3,   K.N4,   K.N5,   K.N6,   K.N7,   K.N8,   K.N9,   K.N0,   K.BSLS,
      K.DEL,  K.F1,   K.F2,   K.F3,   K.F4,   K.F5,   K.F6,   K.MINS, K.EQLS, K.LBRK, K.RBRK, K.SLSH,
      TTTTTT, K.F7,   K.F8,   K.F9,   K.F10,  K.F11,  K.F12,  TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT,
      TTTTTT, TTTTTT, TTTTTT, TTTTTT, ADJUST, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT,
    ],
    # Lower
    [
      K.TILD, K.EXCL, K.AT,   K.HASH, K.DLR,  K.PERC, K.CARE, K.AMP,  K.ASTR, K.LPRN, K.RPRN, K.PIPE,
      K.INS,  K.F1,   K.F2,   K.F3,   K.F4,   K.F5,   K.F6,   K.UNDS, K.PLUS, K.LCBR, K.RCBR, K.QUES,
      TTTTTT, K.F7,   K.F8,   K.F9,   K.F10,  K.F11,  K.F12,  TTTTTT, K.LEFT, K.DOWN, K.UP,   K.RGHT,
      TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, ADJUST, K.HOME, K.PGDN, K.PGUP, K.END,
    ],
    # Adjust
    [
      TTTTTT, BOOT,   TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, K.PSCR, K.SCRL, K.PAUS, TTTTTT, TTTTTT,
      K.CAPS, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT,
      TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT,
      TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT, TTTTTT,
    ],
  ],
}
