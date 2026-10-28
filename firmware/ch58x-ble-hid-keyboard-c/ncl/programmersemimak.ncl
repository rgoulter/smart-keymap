let K = import "keys.ncl" in

let SEMIMAK_  = 0 in
let SHIFT_    = 1 in
let FUNCTION_ = 2 in
let QWERTY_   = 3 in
let PCMODE_   = 4 in
let GAMEMODE_ = 5 in
{
  config.tap_hold.interrupt_response = "HoldOnKeyTap",

  custom_keys = fun K => {
      SEMI = K.layer_mod.set_default SEMIMAK_,
      QWER = K.layer_mod.set_default QWERTY_,
      PMOD = K.layer_mod.toggle PCMODE_,
      GMOD = K.layer_mod.toggle GAMEMODE_,
      
      LSF_O = K.LeftShift & K.layer_mod.hold SHIFT_,
      RSF_O = K.RightShift & K.layer_mod.hold SHIFT_,
      ESC_F = K.Escape & K.hold (K.layer_mod.hold FUNCTION_),

      # Emoji picker + right Option
      EMOJ  = K.RightCtrl & K.RightGUI & K.Space & K.hold (K.RightAlt),
      # FPS counter overlay for PC games
      FPSC  = K.RightCtrl & K.RightShift & K.O,
  },

  layers = [
    # Programmer Semimak
    m%"
      `     0    1    2    3    4                 {    (    -     >    <    NPEQ
      TAB   F    L    H    V    Z                 '    W    U     O    Y    NPPL
      ESC_F S    R    N    T    K                 C    D    E     A    I    BSPC
      BSLS  X    J    B    M    Q                 P    G    ,     .    /    SCLN
      LCTL  LBRK RBRK LALT LGUI LSF_O             SPC  RET  RSF_O RGUI EMOJ DEL
    "%,
    # Programmer Semimak (shift overrides)
    # Holding Shift or Esc adds 5 to each number. This works out of the box on MacOS. On Linux "Numeric keypad always enters digits (as in MacOS)" must be enabled in Tweaks or System Settings. Windows still has no such option, so you can only use Esc.
    m%"
      TTTT  NP5  NP6  NP7  NP8  NP9               }    )    TTTT  @    ^    #
      TTTT  TTTT TTTT TTTT TTTT TTTT              TTTT TTTT TTTT  TTTT TTTT %
      TTTT  TTTT TTTT TTTT TTTT TTTT              TTTT TTTT TTTT  TTTT TTTT TTTT
      TTTT  TTTT TTTT TTTT TTTT TTTT              TTTT TTTT &     !    TTTT TTTT
      TTTT  $    *    TTTT TTTT TTTT              TTTT TTTT TTTT  TTTT TTTT TTTT
    "%,
    # Programmer Semimak (function overrides)
    # LRHN (Left, Right, Higher, Netherwards) turn into arrow keys, and -><= turn into PageDown, End, Home, PageUp.
    # Remapping Vi/Helix to use LRHN as movement keys requires little reshuffling as they already use H and L by default. Also, End and Home coincidentally line up nicely with A and I which have the same functions in Vi/Helix.
    m%"
      NUSB  5          6        7    8    9                 }    )    PGDN END  HOME PGUP
      TTTT  TTTT       LEFT     UP   TTTT TTTT              F1   F2   F3   F4   F5   NumLock
      TTTT  TTTT       RGHT     DOWN TTTT TTTT              F6   F7   F8   F9   F10  TTTT
      TTTT  Mute       TTTT     GMOD SEMI QWER              F11  F12  SCRL PAUS INS  TTTT
      PMOD  VolumeDown VolumeUp TTTT TTTT TTTT              TTTT TTTT FPSC TTTT PSCR TTTT
    "%,
    # QWERTY / Native
    m%"
      `     -    1    2    3    4                 5    6    7     8    9    0
      TAB   Q    W    E    R    T                 Y    U    I     O    P    EQLS
      ESC_F A    S    D    F    G                 H    J    K     L    SCLN BSPC
      BSLS  Z    X    C    V    B                 N    M    ,     .    /    '
      LCTL  LBRK RBRK LALT LGUI LSFT              SPC  RET  RSFT  RGUI EMOJ DEL
    "%,
    # PC mode
    m%"
      TTTT  TTTT TTTT TTTT TTTT TTTT              TTTT TTTT TTTT  TTTT TTTT EQLS 
      TTTT  TTTT TTTT TTTT TTTT TTTT              TTTT TTTT TTTT  TTTT TTTT TTTT
      TTTT  TTTT TTTT TTTT TTTT TTTT              TTTT TTTT TTTT  TTTT TTTT TTTT
      NUSB  TTTT TTTT TTTT TTTT TTTT              TTTT TTTT TTTT  TTTT TTTT TTTT
      TTTT  TTTT TTTT TTTT RCTL TTTT              TTTT TTTT TTTT  TTTT TTTT TTTT
    "%,
    # Game compatibility mode
    # For games that cannot be remapped. LRHN/WESD becomes WASD. Right ⌘ exits this mode.
    m%"
      ESC   -          1        2    3    4                 5    6    7     8    9    0
      TAB   Q          A        W    E    R                 Y    U    I     UP   P    EQLS
      LCTL  T          D        S    F    G                 H    J    LEFT  DOWN RGHT BSPC
      INS   Z          X        C    V    B                 N    M    ,     .    /    SCLN
      `     VolumeDown VolumeUp LALT SPC  LSFT              SPC  RET  FPSC  GMOD EMOJ DEL
    "%,
  ],
}