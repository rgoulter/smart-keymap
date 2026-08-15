# Split 3x5+3 thumbs: six tap-hold layer-mods and the two inner-pair chords.
# Resembles thumb keys from rgoulter's split 3x5+3.
let K = import "keys.ncl" in
let HoldLayerMod = fun name => K.hold (K.layer_mod.hold name) in
{
  config.tap_hold.interrupt_response = "HoldOnKeyTap",
  config.tap_hold.timeout = 200,
  config.chorded.required_idle_time = 125,
  chords = [
    {
      indices = [2, 3],
      key = K.Tab & HoldLayerMod "mou_r",
    },
    {
      indices = [4, 5],
      key = K.Delete & HoldLayerMod "fun_l",
    },
  ],
  layers = [
    [
      K.A,
      K.Tab & HoldLayerMod "mou_r",
      K.Escape & HoldLayerMod "med_r",
      K.Space & HoldLayerMod "nav_r",
      K.Return & HoldLayerMod "sym_l",
      K.Backspace & HoldLayerMod "num_l",
      K.Delete & HoldLayerMod "fun_l",
    ],
  ],
  named_layers = {
    mou_r = [K.B, K.TTTT, K.TTTT, K.TTTT, K.TTTT, K.TTTT, K.TTTT],
    med_r = [K.C, K.TTTT, K.TTTT, K.TTTT, K.TTTT, K.TTTT, K.TTTT],
    nav_r = [K.Left, K.TTTT, K.TTTT, K.TTTT, K.TTTT, K.TTTT, K.TTTT],
    sym_l = [K.D, K.TTTT, K.TTTT, K.TTTT, K.TTTT, K.TTTT, K.TTTT],
    num_l = [K.N1, K.TTTT, K.TTTT, K.TTTT, K.TTTT, K.TTTT, K.TTTT],
    fun_l = [K.F1, K.TTTT, K.TTTT, K.TTTT, K.TTTT, K.TTTT, K.TTTT],
  },
  ceedling_fixture = {
    suite = "THUMBS",
    PROBE_KEY = 0,
    TAB_MOUR = 1,
    ESC_MEDR = 2,
    SPC_NAVR = 3,
    ENT_NSSL = 4,
    BKSP_NSL = 5,
    DEL_FUNL = 6,
  },
}
