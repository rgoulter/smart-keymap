# ANSI 66-key 60% with a 13-key bottom row
#  (1.25u×4 + 1u×5 + 1.25u×4, GH60-ish / ch32x-60-improved).
#
# Base is QWERTY. The extra thumbs are Esc, Fn, three Space keys, and Del.
# Three Space keys cover left-thumb and right-thumb spacers.
# Fn is a dedicated hold layer (not dual-role Space).
#
# Fn overlay (SpaceFN-style):
#  number row = F1-F12, IJKL = arrows, U/O = Home/End, H/N = PgUp/PgDn.
# XXXX on the overlay marks the held layer-mod.

{
  custom_keys = fun K =>
    {
      FN = K.layer_mod.hold "fn",
    },

  layers = [
    m%"
      `    1    2    3    4    5    6    7    8    9    0    -    =      BSPC
        TAB  Q    W    E    R    T    Y    U    I    O    P    [    ]    \
          CAPS A    S    D    F    G    H    J    K    L    ;    '      ENT
            LSFT Z    X    C    V    B    N    M    ,    .    /        RSFT
      LCTL  LGUI  LALT  ESC   FN   SPC  SPC  SPC  DEL   RALT  RGUI  APPN  RCTL
    "%,
  ],

  named_layers = {
    fn = m%"
      BOOT F1   F2   F3   F4   F5   F6   F7   F8   F9   F10  F11  F12     DEL
        TTTT TTTT TTTT ESC  TTTT TTTT TTTT HOME UP   END  PSCR SCRL PAUS INS
          CWTG TTTT TTTT TTTT TTTT TTTT PGUP LEFT DOWN RGHT TTTT TTTT   TTTT
            TTTT MUTE VOLD VOLU MPLY TTTT PGDN TTTT TTTT TTTT TTTT TTTT
      TTTT  TTTT  TTTT  TTTT  XXXX TTTT TTTT TTTT TTTT  TTTT  TTTT  TTTT  TTTT
    "%,
  },
}
