let K = import "keys.ncl" in

let RAISE_  = 2 in
let LOWER_  = RAISE_ + 1 in
let ADJUST_ = RAISE_ + 2 in

{
  chords =
    let K = import "keys.ncl" in
    [
        { indices = [48, 49], key = K.LeftGUI & K.PageUp, },
        { indices = [55, 56], key = K.LeftGUI & K.PageDown, },
    ],

  config.tap_hold.interrupt_response = "HoldOnKeyTap",

  custom_keys = fun K =>
    let HoldLayerMod = fun layer_index => K.hold (K.layer_mod.hold layer_index) in
    {
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
    },

  layers = [
    # Base: Dvorak
    m%"
      `    1    2    3   4     5   XXXX XXXX XXXX 6    7     8    9    0    DEL
      TAB  '    ,    .   P     Y   7    8    9    F    G     C    R    L    BSPC
      ESC  A_A  G_O  C_E S_U   I   4    5    6    D    S_H   C_T  G_N  A_S  /
      LSFT ;    Q    J   K     X   1    2    3    B    M     W    V    Z    RSFT
      LCTL LGUI LALT TAB ESC_L SPC SPC  0    BSPC BSPC ENT_R DEL  RALT RGUI RCTL
    "%,
    # Base: Gaming
    m%"
      `    1    2    3   4     5   XXXX XXXX XXXX 6    7     8    9    0    DEL
      TAB  Q    W    E   R     T   XXXX XXXX XXXX Y    U     I    O    P    BSPC
      ESC  A    S    D   F     G   XXXX XXXX XXXX H    J     K    L    ;    '
      LSFT Z    X    C   V     B   XXXX XXXX XXXX N    M     ,    .    /    RSFT
      LCTL LGUI LALT TAB ESC_L SPC XXXX XXXX XXXX BSPC ENT_R DEL  RALT RGUI RCTL
    "%,
    # Raise
    m%"
      TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT
      `    1    2    3    4    5    TTTT TTTT TTTT 6    7    8    9    0    \
      DEL  F1   F2   F3   F4   F5   TTTT TTTT TTTT F6   -    =    [    ]    /
      TTTT F7   F8   F9   F10  F11  TTTT TTTT TTTT F12  TTTT TTTT TTTT TTTT TTTT
      TTTT TTTT TTTT TTTT ADJ  TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT
    "%,
    # Lower
    m%"
      TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT
      ~    !    @    #    $    %    TTTT TTTT TTTT ^    &    *    (    )    |
      INS  F1   F2   F3   F4   F5   TTTT TTTT TTTT F6   _    +    {    }    ?
      TTTT F7   F8   F9   F10  F11  TTTT TTTT TTTT F12  TTTT HOME PGDN PGUP END
      TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT ADJ  LEFT DOWN UP   RGHT
    "%,
    # Adjust
    m%"
      TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT
      TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT PSCR SCRL PAUS TTTT TTTT
      CAPS TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT
      TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT
      TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT TTTT
    "%,
  ],

}
