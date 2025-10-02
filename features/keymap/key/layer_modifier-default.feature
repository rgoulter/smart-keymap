Feature: Layer Modifier: Set Default Layer

  The `K.layer_mod.set_default` key allows setting the Default layer.

  For examples of this feature in other smart keyboard firmware, see e.g.:

  - [QMK's DF(layer)](https://docs.qmk.fm/feature_layers#switching-and-toggling-layers),

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        layers = [
          [
            K.layer_mod.set_default 0,
            K.layer_mod.set_default 1,
            K.A,
          ],
          [
            K.TTTT,
            K.TTTT,
            K.B,
          ],
        ],
      }
      """

  Example: tapping the set default layer modifier key changes the default layer

    When the keymap registers the following input
      """
      [
        tap (K.layer_mod.set_default 1),
        press_keymap_index 2
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.B] }
      """
