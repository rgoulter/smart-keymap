# Mirkoyu layout
# https://github.com/manna-harbour/miryoku/tree/master/docs/reference
#
# For simplicity,
#  - Miryoku_base = colemak mod-dh
#  - Miryoku_extra = qwerty
#  - Miryoku_tap = qwerty

let BASE_ = 0 in
let EXTRA_ = 1 in
let TAP_ = 2 in

let ALPHA_LAYERS_COUNT = 3 in

let NUM_L = ALPHA_LAYERS_COUNT + 0 in
let SYM_L = ALPHA_LAYERS_COUNT + 1 in
let FUN_L = ALPHA_LAYERS_COUNT + 2 in

let NAV_R = ALPHA_LAYERS_COUNT + 3 in
let MOU_R = ALPHA_LAYERS_COUNT + 4 in
let MED_R = ALPHA_LAYERS_COUNT + 5 in

{
  chords =
    let K = import "keys.ncl" in
    [
        { indices = [22, 23], key = K.LeftGUI & K.PageUp, },
        { indices = [26, 27], key = K.LeftGUI & K.PageDown, },
    ],
  config.tap_hold.interrupt_response = "HoldOnKeyTap",
  custom_keys = fun K =>
    let HoldLayerMod = fun layer_index => K.hold (K.layer_mod.hold layer_index) in
    {
      ESC_MEDR = K.Escape & HoldLayerMod MED_R,
      SPC_NAVR = K.Space & HoldLayerMod NAV_R,
      TAB_MOUR = K.Tab & HoldLayerMod MOU_R,
      ENT_NSSL = K.Return & HoldLayerMod SYM_L,
      BKSP_NSL = K.Backspace & HoldLayerMod NUM_L,
      DEL_FUNL = K.Delete & HoldLayerMod FUN_L,

      # Colemak HRM
      G_A = K.A & K.H_LGUI,
      A_R = K.R & K.H_LAlt,
      C_S = K.S & K.H_LCtrl,
      S_T = K.T & K.H_LShift,

      S_N = K.N & K.H_RShift,
      C_E = K.E & K.H_LCtrl,
      A_I = K.I & K.H_RAlt,
      G_O = K.O & K.H_LGUI,

      # QWERTY HRM
      G_A = K.A & K.H_LGUI,
      A_S = K.S & K.H_LAlt,
      C_D = K.D & K.H_LCtrl,
      S_F = K.F & K.H_LShift,

      S_J = K.J & K.H_RShift,
      C_K = K.K & K.H_LCtrl,
      A_L = K.L & K.H_RAlt,
      G_QT = K.QUOT & K.H_LGUI,

      # Additional Features
      AF_R = K.XXXX & { tap_dances = [K.reset_to_bootloader] },
      AF_T = K.XXXX & { tap_dances = [K.layer_mod.set_default TAP_] },
      AF_E = K.XXXX & { tap_dances = [K.layer_mod.set_default EXTRA_] },
      AF_B = K.XXXX & { tap_dances = [K.layer_mod.set_default BASE_] },

      # TODO: OPP, CURR layer lock

      Undo = K.LeftCtrl & K.Z,
      Redo = K.LeftCtrl & K.Y,

      Cut  = K.LeftCtrl & K.X,
      Copy = K.LeftCtrl & K.C,
      Pste = K.LeftCtrl & K.V,
    },

  layers = [
    # Base: Colemak mod-DH
    m%"
      Q    W    F    P    B       J    L    U    Y    '
      G_A  A_R  C_S  S_T  G       M    S_N  C_E  A_I  G_O
      Z    X    C    D    V       K    H    ,    .    /
      ESC_MEDR SPC_NAVR TAB_MOUR  ENT_NSSL BKSP_NSL DEL_FUNL
    "%,

    # Extra: QWERTY
    m%"
      Q    W    E    R    T       Y    U    I    O    P
      G_A  A_S  C_D  S_F  G       H    S_J  C_K  A_L  G_QT
      Z    X    C    V    B       N    M    ,    .    /
      ESC_MEDR SPC_NAVR TAB_MOUR  ENT_NSSL BKSP_NSL DEL_FUNL
    "%,

    # Tap: QWERTY
    m%"
      Q    W    E    R    T       Y    U    I    O    P
      A    S    D    F    G       H    J    K    L    '
      Z    X    C    V    B       N    M    ,    .    /
                ESC  SPC  TAB     ENT  BSPC DEL
    "%,

    # NSL (Numbers/Symbols)
    m%"
      [    7    8    9   ]             TTTT AF_B AF_E AF_T AF_R
      ;    4    5    6   =             TTTT LSFT LCTL LALT LGUI
      `    1    2    3   \             TTTT TTTT TTTT TTTT TTTT
                     .   0  -     TTTT TTTT TTTT
    "%,

    # NSSL (Numbers/Symbols (Shifted))
    m%"
      {    &    *    (   }             TTTT AF_B AF_E AF_T AF_R
      :    $    %    ^   +             TTTT LSFT LCTL LALT LGUI
      ~    !    @    #   |             TTTT TTTT TTTT TTTT TTTT
                     (   )  _     TTTT TTTT TTTT
    "%,

    # FunL (Function keys etc.)
    m%"
      F12  F7   F8   F9   PSCR            TTTT AF_B AF_E AF_T AF_R
      F11  F4   F5   F6   SCRL            TTTT LSFT LCTL LALT LGUI
      F10  F1   F2   F3   PAUS            TTTT TTTT TTTT TTTT TTTT
                     TTTT TTTT TTTT  TTTT TTTT TTTT
    "%,

    # NavR (Navigation keys)
    m%"
      AF_R AF_T AF_E AF_B TTTT            Redo Pste Copy Cut  Undo
      LGUI LALT LCTL LSFT TTTT            CAPS LEFT DOWN UP   RGHT
      TTTT TTTT TTTT TTTT TTTT            INS  HOME PGDN PGUP END
                     TTTT TTTT TTTT  TTTT TTTT TTTT
    "%,

    # MouR (Mouse keys)
    # TODO: mouse keys
    m%"
      AF_R AF_T AF_E AF_B XXXX            Redo Pste Copy Cut  Undo
      LGUI LALT LCTL LSFT XXXX            XXXX XXXX XXXX XXXX XXXX
      XXXX XXXX XXXX XXXX XXXX            XXXX XXXX XXXX XXXX XXXX
                     XXXX XXXX XXXX  XXXX XXXX XXXX
    "%,

    # MedR (Media keys)
    # TODO: consumer keys
    # TODO: Bluetooth keys (custom external callbacks)
    # TODO: RGB keys (custom external callbacks)
    m%"
      AF_R AF_T AF_E AF_B XXXX            XXXX XXXX XXXX XXXX XXXX
      LGUI LALT LCTL LSFT XXXX            XXXX XXXX XXXX XXXX XXXX
      XXXX XXXX XXXX XXXX XXXX            XXXX XXXX XXXX XXXX BOOT
                     XXXX XXXX XXXX  XXXX XXXX XXXX
    "%,

    # TODO: Button layer
  ],
}
