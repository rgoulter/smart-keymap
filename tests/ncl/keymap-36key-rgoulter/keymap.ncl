# Port of my layout.
#
# Implemented elsewhere, e.g.:
# - https://github.com/rgoulter/fak-config/blob/master/shared/lib/keymaps/split_3x5_3/rgoulter.ncl
# - https://github.com/rgoulter/qmk_userspace/tree/master/layouts/split_3x5_3/rgoulter

let NUM_L = 1 in
let SYM_L = 2 in
let FUN_L = 3 in

let NAV_R = 4 in
let MOU_R = 5 in
let MED_R = 6 in

{
  config.tap_hold.interrupt_response = "HoldOnKeyTap",
  custom_keys = fun K =>
    let HoldLayerMod = fun layer_index => K.hold (K.layer_mod.hold layer_index) in
    {
      SPC_NAVR = K.Space & HoldLayerMod NAV_R,
      TAB_MOUR = K.Tab & HoldLayerMod MOU_R,
      ESC_MEDR = K.Escape & HoldLayerMod MED_R,
      BKSP_NSL = K.Backspace & HoldLayerMod NUM_L,
      ENT_NSSL = K.Return & HoldLayerMod SYM_L,
      DEL_FUNL = K.Delete & HoldLayerMod FUN_L,

      A_A = K.A & K.H_LAlt,
      G_O = K.O & K.H_LGUI,
      C_E = K.E & K.H_LCtrl,
      S_U = K.U & K.H_LShift,

      S_H = K.H & K.H_RShift,
      C_T = K.T & K.H_LCtrl,
      G_N = K.N & K.H_RGUI,
      A_S = K.S & K.H_LAlt,
    },

  layers = [
    m%"
      '     ,     .    P        Y                           F        G        C    R    L
      A_A   G_O   C_E  S_U      I                           D        S_H      C_T  G_N  A_S
      ;     Q     J    K        X                           B        M        W    V    Z
                       TAB_MOUR ESC_MEDR SPC_NAVR  ENT_NSSL BKSP_NSL DEL_FUNL
    "%,

    # NSL (Numbers/Symbols)
    # Different from Miryoku: Number layer, LHS: GRV in middle & slash (rather than semicolon)
    m%"
      [    7    8    9   ]             TTTT TTTT TTTT TTTT TTTT
      `    4    5    6   =             TTTT TTTT TTTT TTTT TTTT
      /    1    2    3   \             TTTT TTTT TTTT TTTT TTTT
                     .   0  -     TTTT TTTT TTTT
    "%,

    # NSSL (Numbers/Symbols (Shifted))
    # Different from Miryoku: Number layer, LHS: TILD in middle & slash (rather than colon)
    m%"
      {    &    *    (   }             TTTT TTTT TTTT TTTT TTTT
      ~    $    %    ^   +             TTTT TTTT TTTT TTTT TTTT
      ?    !    @    #   |             TTTT TTTT TTTT TTTT TTTT
                     >   )  _     TTTT TTTT TTTT
    "%,

    # FunL (Function keys etc.)
    m%"
      F12  F7   F8   F9   PSCR            TTTT TTTT TTTT TTTT TTTT
      F11  F4   F5   F6   SCRL            TTTT TTTT TTTT TTTT TTTT
      F10  F1   F2   F3   PAUS            TTTT TTTT TTTT TTTT TTTT
                     TTTT TTTT TTTT  TTTT TTTT TTTT
    "%,

    # NavR (Navigation keys)
    ## TBI: "Desktop Keys": set-OS (Win, MacOS, Linux)
    ## TBI: Different from Miryoku: Nav, RHS, upper: TBI the convenience cut/copy/paste and undo/redo
    m%"
      TTTT TTTT TTTT TTTT TTTT            TTTT TTTT TTTT TTTT TTTT
      TTTT TTTT TTTT TTTT TTTT            LEFT DOWN UP   RGHT CAPS
      TTTT TTTT TTTT TTTT TTTT            HOME PGDN PGUP END  INS
                     TTTT TTTT TTTT  TTTT TTTT TTTT
    "%,

    # MouR (Mouse keys)
    # TBI: mouse keys
    m%"
      XXXX XXXX XXXX XXXX XXXX            XXXX XXXX XXXX XXXX XXXX
      XXXX XXXX XXXX XXXX XXXX            XXXX XXXX XXXX XXXX XXXX
      XXXX XXXX XXXX XXXX XXXX            XXXX XXXX XXXX XXXX XXXX
                     XXXX XXXX XXXX  XXXX XXXX XXXX
    "%,

    # MedR (Media keys)
    # TBI: consumer keys
    m%"
      XXXX XXXX XXXX XXXX XXXX            XXXX XXXX XXXX XXXX XXXX
      XXXX XXXX XXXX XXXX XXXX            XXXX XXXX XXXX XXXX XXXX
      XXXX XXXX XXXX XXXX XXXX            XXXX XXXX XXXX XXXX XXXX
                     XXXX XXXX XXXX  XXXX XXXX XXXX
    "%,
  ],
}
