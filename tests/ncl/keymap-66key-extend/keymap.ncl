# ANSI 66-key 60% with a 13-key bottom row
#  (1.25u×4 + 1u×5 + 1.25u×4, GH60-ish / ch32x-60-improved).
#
# Same QWERTY base and thumb cluster as keymap-66key-ansi-fn,
#  but the inner 1u next to Esc is Extend instead of Fn.
#
# Extend is a hold layer on a thumb, not Caps.
# Nav matches the Fn map (IJKL arrows, U/O Home/End, H/N PgUp/PgDn)
#  so the two maps share muscle memory.
# Left home row becomes modifiers (ASDF = Alt/Gui/Shift/Ctrl).
# ZXCV is Undo/Cut/Copy/Paste. Space is Enter.
# XXXX on the overlay marks the held layer-mod.

{
  custom_keys = fun K =>
    {
      EXT = K.layer_mod.hold "extend",

      Bk = K.LeftAlt & K.Left,
      Fw = K.LeftAlt & K.Right,

      Undo = K.LeftCtrl & K.Z,
      Cut = K.LeftCtrl & K.X,
      Copy = K.LeftCtrl & K.C,
      Pste = K.LeftCtrl & K.V,
    },

  layers = [
    m%"
      `    1    2    3    4    5    6    7    8    9    0    -    =      BSPC
        TAB  Q    W    E    R    T    Y    U    I    O    P    [    ]    \
          CAPS A    S    D    F    G    H    J    K    L    ;    '      ENT
            LSFT Z    X    C    V    B    N    M    ,    .    /        RSFT
      LCTL  LGUI  LALT  ESC   EXT  SPC  SPC  SPC  DEL   RALT  RGUI  APPN  RCTL
    "%,
  ],

  named_layers = {
    extend = m%"
      BOOT F1   F2   F3   F4   F5   F6   F7   F8   F9   F10  F11  F12     DEL
        TTTT ESC  TTTT Bk   Fw   TTTT TTTT HOME UP   END  INS  TTTT TTTT TTTT
          CWTG LALT LGUI LSFT LCTL TTTT PGUP LEFT DOWN RGHT BSPC DEL    TTTT
            TTTT Undo Cut  Copy Pste TTTT PGDN TTTT TTTT TTTT TTTT     TTTT
      TTTT  TTTT  TTTT  TTTT  XXXX ENT  ENT  ENT  TTTT  TTTT  TTTT  TTTT  TTTT
    "%,
  },
}
