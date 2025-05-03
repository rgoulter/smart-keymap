# Port of seniply.
# https://stevep99.github.io/seniply/
# https://github.com/stevep99/seniply/blob/master/downloads/kmonad/colemak-dh-seniply-split.kbd

let BASE_0 = 0 in
let BASE_1 = 1 in
let EXTEND = 2 in
let FUNCTION = 3 in
let SYMBOLS = 4 in
let NUMBERS = 5 in

{
  custom_keys = fun K =>
    {
      L0 = K.layer_mod.set_default BASE_0,
      L1 = K.layer_mod.set_default BASE_1,

      Ext = K.layer_mod.hold EXTEND,
      Fun = K.layer_mod.hold FUNCTION,
      Sym = K.layer_mod.hold SYMBOLS,
      Num = K.layer_mod.hold NUMBERS,

      SAlt = K.sticky K.LeftAlt,
      SGui = K.sticky K.LeftGUI,
      SSft = K.sticky K.LeftShift,
      SCtl = K.sticky K.LeftCtrl,

      Bk = K.LeftAlt & K.Left,
      Fw = K.LeftAlt & K.Right,

      Fnd = K.LeftCtrl & K.F,

      Undo = K.LeftCtrl & K.Z,
      Cut  = K.LeftCtrl & K.X,
      Copy = K.LeftCtrl & K.C,
      Pste = K.LeftCtrl & K.V,

      CS_C = K.LeftCtrl & K.LeftShift & K.C,
      CS_V = K.LeftCtrl & K.LeftShift & K.V,
    },

  layers = [
    m%"
      Q    W    F    P    B       J    L    U    Y    '
      A    R    S    T    G       M    N    E    I    O
      Z    X    C    D    V       K    H    ,    .    /
                     Ext  LSFT    SPC  Sym
    "%,

    m%"
      Q    W    E    R    T       Y    U    I    O    P
      A    S    D    F    G       H    J    K    L    '
      Z    X    C    V    B       N    M    ,    .    /
                     Ext  LSFT    SPC  Sym
    "%,

    # Extend
    m%"
      ESC  Bk   Fnd  Fw   INS     PGUP HOME UP   END  CAPS
      SAlt SGui SSft SCtl RALT    PGDN LEFT DOWN RGHT DEL
      Undo Cut  Copy LGUI Pste    RET  BSPC TAB  APPN PSCR
                     XXXX TTTT    RET  Fun
    "%,

    # Function
    # TODO: Consumer Keys, etc.
    m%"
      XXXX XXXX XXXX XXXX XXXX    F12  F7   F8   F9   SLCK
      SAlt SGui SSft SCtl XXXX    F11  F4   F5   F6   L0
      XXXX XXXX CS_C XXXX CS_V    F10  F1   F2   F3   L1
                     XXXX TTTT    RET  XXXX
    "%,

    # Symbols
    m%"
      !    @    #    $    %       =    `    :    ;    +
      SAlt SGui SSft SCtl ^       *    (    {    [    -
      TTTT TTTT \    |    &       ~    )    }    ]    _
                     TTTT Num     TTTT XXXX
    "%,

    # Numbers
    m%"
      TTTT TTTT TTTT TTTT NPNL    =    7    8    9    +
      SAlt SGui SSft SCtl RALT    *    4    5    6    -
      TTTT APPN TAB  BSPC RET     0    1    2    3    /
                     TTTT XXXX     TTTT XXXX
    "%,
  ],
}
