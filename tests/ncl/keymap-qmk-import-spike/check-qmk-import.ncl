# Spike test: QMK Configurator `keymap.json` → smart-keymap Nickel layers.
#
# `qmk-keymap.json` uses a 4-key QMK `LAYOUT` key order (top-left, top-right,
#  bottom-left, bottom-right). This board's smart-keymap firmware instead
#  scans column-major (top-left, bottom-left, top-right, bottom-right), so
#  `board_index_order` gives the QMK layout index for each smart-keymap
#  position.
let K = import "keys.ncl" in
let { to_smart_keymap, .. } = import "import-keymap-json.ncl" in

let board_index_order = [0, 2, 1, 3] in

let qmk_keymap = import "qmk-keymap.json" in

let result = to_smart_keymap board_index_order qmk_keymap in

{
  checks = {
    check_layers = {
      actual = result.layers,
      expected = [
        [K.A, K.layer_mod.hold 1, K.B, K.NO],
        [K.ESC, K.TTTT, K.TTTT, K.BOOT],
      ],
    },

    check_warnings = {
      actual = result.warnings,
      expected = [
        "dropped unsupported top-level feature: combos",
        "dropped unsupported tap-hold keycode: LSFT_T(KC_ENT)",
      ],
    },
  },
}
