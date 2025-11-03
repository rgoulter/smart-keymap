# Port of my layout.
#
# Implemented elsewhere, e.g.:
# - https://github.com/rgoulter/fak-config/blob/master/shared/lib/keymaps/split_3x5_3/rgoulter.ncl
# - https://github.com/rgoulter/qmk_userspace/tree/master/layouts/split_3x5_3/rgoulter

let BASE_ = 0 in  # Dvorak (with home row mods)
let TAP_ = 1 in # Dvorak
let EXTRA_ = 2 in # QWERTY (with home row mods)
let XTAP_ = 3 in # QWERTY (and retain thumbkey tap-hold)

let BASE_LAYER_COUNT = 4 in

let NUM_L = BASE_LAYER_COUNT + 0 in
let SYM_L = BASE_LAYER_COUNT + 1 in
let FUN_L = BASE_LAYER_COUNT + 2 in

let NAV_R = BASE_LAYER_COUNT + 3 in
let MOU_R = BASE_LAYER_COUNT + 4 in
let MED_R = BASE_LAYER_COUNT + 5 in

let HoldLayerMod =
  let K = import "keys.ncl" in
  fun layer_index => K.hold (K.layer_mod.hold layer_index) in
{
  chords =
    let K = import "keys.ncl" in
    let CH = import "chording.ncl" in
    [
        {
          indices = m%"
            _ _ _ _ _   _ _ _ _ _
            _ _ _ _ _   _ _ _ _ _
            _ _ X X _   _ _ _ _ _
                _ _ _   _ _ _
          "% |> CH.indices,
          key = {
            gnome = K.LeftGUI & K.PageUp,
            macos = K.LeftCtrl & K.Left,
          }.gnome,
        },
        {
          indices = m%"
            _ _ _ _ _   _ _ _ _ _
            _ _ _ _ _   _ _ _ _ _
            _ _ _ _ _   _ X X _ _
                _ _ _   _ _ _
          "% |> CH.indices,
          key = {
            gnome = K.LeftGUI & K.PageDown,
            macos = K.LeftCtrl & K.Right,
          }.gnome,
        },
        # TODO: "Alias"? (NCL feature?)
        {
          indices = m%"
            _ _ _ _ _   _ _ _ _ _
            _ _ _ _ _   _ _ _ _ _
            _ _ _ _ _   _ _ _ _ _
                _ X X   _ _ _
          "% |> CH.indices,
          key = K.Tab & HoldLayerMod MOU_R,
        },
        {
          indices = m%"
            _ _ _ _ _   _ _ _ _ _
            _ _ _ _ _   _ _ _ _ _
            _ _ _ _ _   _ _ _ _ _
                _ _ _   X X _
          "% |> CH.indices,
          key = K.Delete & HoldLayerMod FUN_L,
        },
    ],
  config.tap_hold.interrupt_response = "HoldOnKeyTap",
  # config.chorded.required_idle_time = 200,
  custom_keys = fun K =>
    {
      SPC_NAVR = K.Space & HoldLayerMod NAV_R,
      TAB_MOUR = K.Tab & HoldLayerMod MOU_R,
      ESC_MEDR = K.Escape & HoldLayerMod MED_R,
      BKSP_NSL = K.Backspace & HoldLayerMod NUM_L,
      ENT_NSSL = K.Return & HoldLayerMod SYM_L,
      DEL_FUNL = K.Delete & HoldLayerMod FUN_L,

      # Dvorak HRM
      A_A = K.A & K.H_LAlt,
      G_O = K.O & K.H_LGUI,
      C_E = K.E & K.H_LCtrl,
      S_U = K.U & K.H_LShift,

      S_H = K.H & K.H_RShift,
      C_T = K.T & K.H_LCtrl,
      G_N = K.N & K.H_RGUI,
      A_S = K.S & K.H_LAlt,

      # QWERTY HRM
      A_A = K.A & K.H_LAlt,
      G_S = K.S & K.H_LGUI,
      C_D = K.D & K.H_LCtrl,
      S_F = K.F & K.H_LShift,

      S_J = K.J & K.H_RShift,
      C_K = K.K & K.H_LCtrl,
      G_L = K.L & K.H_RGUI,
      A_SC = K.Semicolon & K.H_LAlt,

      # Sticky Modifiers
      SAlt = K.sticky K.LeftAlt,
      SGui = K.sticky K.LeftGUI,
      SSft = K.sticky K.LeftShift,
      SCtl = K.sticky K.LeftCtrl,

      # Additional Features
      AF_R = K.XXXX & { tap_dances = [K.reset_to_bootloader] },
      AF_T = K.XXXX & { tap_dances = [K.layer_mod.set_default XTAP_] },
      AF_E = K.XXXX & { tap_dances = [K.layer_mod.set_default EXTRA_] },
      AF_B = K.XXXX & { tap_dances = [K.layer_mod.set_default BASE_] },
      AF_A = K.XXXX & { tap_dances = [K.layer_mod.set_default TAP_] },

      # TODO: OPP, CURR layer lock
    },

  layers = [
    # Base0: Dvorak + tap-hold HRM
    m%"
      '     ,     .    P        Y                           F        G        C    R    L
      A_A   G_O   C_E  S_U      I                           D        S_H      C_T  G_N  A_S
      ;     Q     J    K        X                           B        M        W    V    Z
                       TAB_MOUR ESC_MEDR SPC_NAVR  ENT_NSSL BKSP_NSL DEL_FUNL
    "%,

    # Base1: Dvorak ("Tap", no tap-hold HRM)
    m%"
      '     ,     .    P        Y                           F        G        C    R    L
      A     O     E    U        I                           D        H        T    N    S
      ;     Q     J    K        X                           B        M        W    V    Z
                       TAB_MOUR ESC_MEDR SPC_NAVR  ENT_NSSL BKSP_NSL DEL_FUNL
    "%,

    # Base2: Extra: QWERTY + tap-hold HRM (with ' instead of /)
    m%"
      Q     W     E    R        T                           Y        U        I    O    P
      A_A   G_S   C_D  S_F      G                           H        S_J      C_K  G_L  A_SC
      Z     X     C    V        B                           N        M        ,    .    '
                       TAB_MOUR ESC_MEDR SPC_NAVR  ENT_NSSL BKSP_NSL DEL_FUNL
    "%,

    # Base3: Tap: QWERTY (with ' instead of /)
    m%"
      Q     W     E    R        T                           Y        U        I    O    P
      A     S     D    F        G                           H        J        K    L    ;
      Z     X     C    V        B                           N        M        ,    .    '
                       TAB_MOUR ESC_MEDR SPC_NAVR  ENT_NSSL BKSP_NSL DEL_FUNL
    "%,

    # NSL (Numbers/Symbols)
    # Different from Miryoku: Number layer, LHS: GRV in middle & slash (rather than semicolon)
    m%"
      [    7    8    9   ]             AF_A AF_B AF_E AF_T AF_R
      `    4    5    6   =             TTTT SSft SCtl SGui SAlt
      /    1    2    3   \             TTTT TTTT TTTT TTTT TTTT
                     .   0  -     XXXX TTTT TTTT
    "%,

    # NSSL (Numbers/Symbols (Shifted))
    # Different from Miryoku: Number layer, LHS: TILD in middle & slash (rather than colon)
    m%"
      {    &    *    (   }             AF_A AF_B AF_E AF_T AF_R
      ~    $    %    ^   +             TTTT SSft SCtl SGui SAlt
      ?    !    @    #   |             TTTT TTTT TTTT TTTT TTTT
                     >   )  _     TTTT XXXX TTTT
    "%,

    # FunL (Function keys etc.)
    m%"
      F12  F7   F8   F9   PSCR            AF_A AF_B AF_E AF_T AF_R
      F11  F4   F5   F6   SCRL            TTTT SSft SCtl SGui SAlt
      F10  F1   F2   F3   PAUS            TTTT TTTT TTTT TTTT TTTT
                     TTTT TTTT TTTT  TTTT TTTT XXXX
    "%,

    # NavR (Navigation keys)
    ## TBI: "Desktop Keys": set-OS (Win, MacOS, Linux)
    ## TBI: Different from Miryoku: Nav, RHS, upper: TBI the convenience cut/copy/paste and undo/redo
    m%"
      AF_R AF_T AF_E AF_B AF_A            TTTT TTTT TTTT TTTT TTTT
      SAlt SGui SCtl SSft TTTT            LEFT DOWN UP   RGHT CAPS
      TTTT TTTT TTTT TTTT TTTT            HOME PGDN PGUP END  INS
                     TTTT TTTT XXXX  TTTT TTTT TTTT
    "%,

    # MouR (Mouse keys)
    m%"
      AF_R AF_T AF_E AF_B AF_A            XXXX XXXX XXXX XXXX XXXX
      SAlt SGui SCtl SSft XXXX            MouseLeft MouseDown MouseUp MouseRight XXXX
      XXXX XXXX XXXX XXXX XXXX            MouseWheelLeft MouseWheelDown MouseWheelUp MouseWheelRight XXXX
                     XXXX TTTT TTTT  MouseBtn1 MouseBtn2 MouseBtn3
    "%,

    # MedR (Media keys)
    m%"
      AF_R AF_T AF_E AF_B AF_A            XXXX         XXXX    XXXX  XXXX     XXXX
      SAlt SGui SCtl SSft XXXX            ScanPrevious VolDown VolUp ScanNext XXXX
      XXXX XXXX XXXX XXXX XXXX            XXXX         XXXX    XXXX  XXXX     BOOT
                     TTTT XXXX TTTT  PlayPause Stop Mute
    "%,
  ],
}
