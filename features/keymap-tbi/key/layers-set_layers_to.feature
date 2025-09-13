Feature: Layers (Set Layers To)

  The `K.layer_mod.set_active_layers_to` key allows setting the active layers.

  For examples of this feature in other smart keyboard firmware, see e.g.:

  - [QMK's TO(layer)](https://docs.qmk.fm/feature_layers#switching-and-toggling-layers),

  - [ZMK's "to layer" behaviour](https://zmk.dev/docs/keymaps/behaviors/layers#to-layer)

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        layers = [
          [
            K.layer_mod.set_active_layers_to [1],
            K.A,
          ],
          [
            K.layer_mod.set_active_layers_to [0],
            K.B,
          ],
        ],
      }
      """

  Example: pressing the set active layers modifier key changes the active layers

    When the keymap registers the following input
      """
      [
        press (K.layer_mod.set_active_layers_to [1]),
        press (K.B),
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press (K.B),
      ]
      """
