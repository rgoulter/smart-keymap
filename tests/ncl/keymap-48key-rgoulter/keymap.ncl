# Port of my layout.
#
# Implemented elsewhere, e.g.:
# - https://github.com/rgoulter/fak-config/blob/master/shared/lib/keymaps/split_3x5_3/rgoulter.ncl
# - https://github.com/rgoulter/qmk_userspace/tree/master/layouts/split_3x5_3/rgoulter

let K = import "keys.ncl" in

let NUM_L = 0 in
let SYM_L = 1 in
let FUN_L = 2 in

let NAV_R = 3 in
let MOU_R = 4 in
let MED_R = 5 in

let LT = fun layer_index key => key & K.hold { layer_modifier = { hold = layer_index } } in
# let LT = fun layer_index key => key in

let SPC_NAVR = LT NAV_R K.Space in
let TAB_MOUR = LT MOU_R K.Tab in
let ESC_MEDR = LT MED_R K.Escape in
let BKSP_NSL = LT NUM_L K.Backspace in
let ENT_NSSL = LT SYM_L K.Return in
let DEL_FUNL = LT FUN_L K.Delete in

let A_A = K.A & K.H_LAlt in
let G_O = K.O & K.H_LGUI in
let C_E = K.E & K.H_LCtrl in
let S_U = K.U & K.H_LShift in

let S_H = K.H & K.H_RShift in
let C_T = K.T & K.H_RCtrl in
let G_N = K.N & K.H_RGUI in
let A_S = K.S & K.H_LAlt in

let XXXX = { key_code = 0x00 } in
let TTTT = K.TTTT in

let S = K.LeftShift in

{
  layers = [
    [
      K.Quote,     K.Comma,   K.Dot, K.P,   K.Y,   XXXX, XXXX,    K.F,         K.G,   K.C,      K.R,        K.L,
      A_A,         G_O,       C_E,   S_U,   K.I,   XXXX, XXXX,    K.D,         S_H,   C_T,      G_N,        A_S,
      K.Semicolon, K.Q,       K.J,   K.K,   K.X,   XXXX, XXXX,    K.B,         K.M,   K.W,      K.V,        K.Z,
      K.LeftCtrl,  K.LeftGUI,   K.LeftAlt, TAB_MOUR, ESC_MEDR, SPC_NAVR,    ENT_NSSL, BKSP_NSL, DEL_FUNL, K.RightAlt, K.RightGUI, K.RightCtrl,
    ],

    # NSL (Numbers/Symbols)
    # Different from Miryoku: Number layer, LHS: GRV in middle & slash (rather than semicolon)
    [
      K.LeftBracket, K.N7, K.N8, K.N9,  K.RightBracket,  XXXX,        XXXX, TTTT, TTTT, TTTT, TTTT, TTTT,
      K.Grave,       K.N4, K.N5, K.N6,  K.Equals,        XXXX,        XXXX, TTTT, TTTT, TTTT, TTTT, TTTT,
      K.Slash,       K.N1, K.N2, K.N3,  K.Backslash,     XXXX,        XXXX, TTTT, TTTT, TTTT, TTTT, TTTT,
      TTTT,          TTTT, TTTT, K.Dot, K.N0,            K.Minus,     TTTT, TTTT, TTTT, TTTT, TTTT, TTTT,
    ],

    # NSSL (Numbers/Symbols (Shifted))
    # Different from Miryoku: Number layer, LHS: TILD in middle & slash (rather than colon)
    [
      K.LeftBracket, K.N7, K.N8, K.N9,  K.RightBracket,  XXXX,        XXXX, TTTT, TTTT, TTTT, TTTT, TTTT,
      K.Grave,       K.N4, K.N5, K.N6,  K.Equals,        XXXX,        XXXX, TTTT, TTTT, TTTT, TTTT, TTTT,
      K.Slash,       K.N1, K.N2, K.N3,  K.Backslash,     XXXX,        XXXX, TTTT, TTTT, TTTT, TTTT, TTTT,
      TTTT,          TTTT, TTTT, K.Dot, K.N0,            K.Minus,     TTTT, TTTT, TTTT, TTTT, TTTT, TTTT,
    ] |> std.array.map (fun key => if key != null then key & S else null),

    # FunL (Function keys etc.)
    [
      K.F12, K.F7, K.F8, K.F9, K.PrintScreen, XXXX,       XXXX, TTTT, TTTT, TTTT, TTTT, TTTT,
      K.F11, K.F4, K.F5, K.F6, K.ScrollLock,  XXXX,       XXXX, TTTT, TTTT, TTTT, TTTT, TTTT,
      K.F10, K.F1, K.F2, K.F3, K.Pause,       XXXX,       XXXX, TTTT, TTTT, TTTT, TTTT, TTTT,
      TTTT,  TTTT, TTTT, TTTT, TTTT,          TTTT,       TTTT, TTTT, TTTT, TTTT, TTTT, TTTT,
    ],

    # NavR (Navigation keys)
    ## TBI: "Desktop Keys": set-OS (Win, MacOS, Linux)
    ## TBI: Different from Miryoku: Nav, RHS, upper: TBI the convenience cut/copy/paste and undo/redo
    [
      TTTT, TTTT, TTTT, TTTT, TTTT, TTTT,      TTTT,  TTTT,    TTTT,    TTTT,    TTTT,    TTTT,
      TTTT, TTTT, TTTT, TTTT, TTTT, TTTT,      TTTT,  K.Left,  K.Down,  K.Up,    K.Right, K.CapsLock,
      TTTT, TTTT, TTTT, TTTT, TTTT, TTTT,      TTTT,  K.Home,  K.PageDown, K.PageUp, K.End,  K.Insert,
      TTTT, TTTT, TTTT, TTTT, TTTT, TTTT,      TTTT,  TTTT,  TTTT, TTTT, TTTT, TTTT,
    ],

    # MouR (Mouse keys)
    # TBI: mouse keys
    [
      XXXX, XXXX, XXXX, XXXX, XXXX, XXXX,     XXXX, XXXX, XXXX, XXXX, XXXX, XXXX,
      XXXX, XXXX, XXXX, XXXX, XXXX, XXXX,     XXXX, XXXX, XXXX, XXXX, XXXX, XXXX,
      XXXX, XXXX, XXXX, XXXX, XXXX, XXXX,     XXXX, XXXX, XXXX, XXXX, XXXX, XXXX,
      XXXX, XXXX, XXXX, XXXX, XXXX, XXXX,     XXXX, XXXX, XXXX, XXXX, XXXX, XXXX,
    ],

    # MedR (Media keys)
    # TBI: consumer keys
    [
      XXXX, XXXX, XXXX, XXXX, XXXX, XXXX,     XXXX, XXXX, XXXX, XXXX, XXXX, XXXX,
      XXXX, XXXX, XXXX, XXXX, XXXX, XXXX,     XXXX, XXXX, XXXX, XXXX, XXXX, XXXX,
      XXXX, XXXX, XXXX, XXXX, XXXX, XXXX,     XXXX, XXXX, XXXX, XXXX, XXXX, XXXX,
      XXXX, XXXX, XXXX, XXXX, XXXX, XXXX,     XXXX, XXXX, XXXX, XXXX, XXXX, XXXX,
    ],
  ],

}
