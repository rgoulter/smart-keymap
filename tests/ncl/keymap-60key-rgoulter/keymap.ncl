let K = import "keys.ncl" in

let DVORAK_  = 0 in
let GAMING_  = 1 in
let RAISE_  = 2 in
let LOWER_  = RAISE_ + 1 in
let ADJUST_ = RAISE_ + 2 in

{
  chords =
    let K = import "keys.ncl" in
    [
        { indices = [39, 40], key = K.LeftGUI & K.PageUp, },
        { indices = [43, 44], key = K.LeftGUI & K.PageDown, },
    ],

  config.tap_hold.interrupt_response = "HoldOnKeyTap",

  custom_keys = fun K =>
    let HoldLayerMod = fun layer_index => K.hold (K.layer_mod.hold layer_index) in
    {
      DVOR = K.layer_mod.set_default DVORAK_,
      GAME = K.layer_mod.set_default GAMING_,

      ENT_R = K.Return & HoldLayerMod RAISE_,
      ESC_L = K.Escape & HoldLayerMod LOWER_,
      ADJ   = K.layer_mod.hold ADJUST_,

      A_A = K.A & K.H_LAlt,
      G_O = K.O & K.H_LGUI,
      C_E = K.E & K.H_LCtrl,
      S_U = K.U & K.H_LShift,

      S_H = K.H & K.H_RShift,
      C_T = K.T & K.H_LCtrl,
      G_N = K.N & K.H_RGUI,
      A_S = K.S & K.H_LAlt,

      SK_S = K.sticky K.LeftShift,
      SK_C = K.sticky K.LeftCtrl,
      SK_G = K.sticky K.LeftGUI,
      SK_A = K.sticky K.LeftAlt,

      BTCL = K.bluetooth_profile.clear,
      BTCA = K.bluetooth_profile.clear_all,
      BTDC = K.bluetooth_profile.disconnect,
      BTPN = K.bluetooth_profile.next,
      BTPP = K.bluetooth_profile.previous,
      BTP0 = K.bluetooth_profile.select 0,
      BTP1 = K.bluetooth_profile.select 1,
      BTP2 = K.bluetooth_profile.select 2,
    },

  layers = [
    # Base: Dvorak
    m%"
      `    1    2    3   4     5                  6    7     8    9    0    DEL
      TAB  '    ,    .   P     Y                  F    G     C    R    L    BSPC
      ESC  A_A  G_O  C_E S_U   I                  D    S_H   C_T  G_N  A_S  /
      LSFT ;    Q    J   K     X                  B    M     W    V    Z    RSFT
      LCTL LGUI LALT TAB ESC_L SPC                BSPC ENT_R DEL  RALT RGUI RCTL
    "%,
    # Base: Gaming
    m%"
      `    1    2    3   4     5                  6    7     8    9    0    DEL
      TAB  Q    W    E   R     T                  Y    U     I    O    P    BSPC
      ESC  A    S    D   F     G                  H    J     K    L    ;    '
      LSFT Z    X    C   V     B                  N    M     ,    .    /    RSFT
      LCTL LGUI LALT TAB ESC_L SPC                BSPC ENT_R DEL  RALT RGUI RCTL
    "%,
    # Raise
    m%"
      TTTT TTTT TTTT TTTT TTTT TTTT                TTTT TTTT TTTT TTTT TTTT TTTT
      `    1    2    3    4    5                   6    7    8    9    0    \
      DEL  F1   F2   F3   F4   F5                  F6   -    =    [    ]    /
      TTTT F7   F8   F9   F10  F11                 F12  TTTT TTTT TTTT TTTT TTTT
      TTTT TTTT TTTT TTTT ADJ  TTTT                TTTT TTTT TTTT TTTT TTTT TTTT
    "%,
    # Lower
    m%"
      TTTT TTTT TTTT TTTT TTTT TTTT                TTTT TTTT TTTT TTTT TTTT TTTT
      ~    !    @    #    $    %                   ^    &    *    (    )    |
      INS  F1   F2   F3   F4   F5                  F6   _    +    {    }    ?
      TTTT F7   F8   F9   F10  F11                 F12  TTTT HOME PGDN PGUP END
      TTTT TTTT TTTT TTTT TTTT TTTT                TTTT ADJ  LEFT DOWN UP   RGHT
    "%,
    # Adjust
    m%"
      DVOR GAME TTTT TTTT TTTT TTTT                BTCL BTP0 BTP1 BTP2 TTTT BTCA
      TTTT BOOT TTTT TTTT TTTT TTTT                TTTT PSCR SCRL PAUS TTTT TTTT
      CAPS SK_A SK_G SK_C SK_S TTTT                TTTT SK_S SK_C SK_G SK_A CWTG
      TTTT TTTT TTTT TTTT TTTT TTTT                TTTT TTTT TTTT TTTT TTTT TTTT
      TTTT TTTT TTTT TTTT TTTT TTTT                TTTT TTTT TTTT TTTT TTTT TTTT
    "%,
  ],
}
