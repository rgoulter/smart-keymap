Feature: Layer Modifier: Toggle

  The `K.layer_mod.toggle` key toggles whether a layer is active when it is pressed.

  For examples of this feature in other smart keyboard firmware, see e.g.:

  - [QMK's TG(layer)](https://docs.qmk.fm/feature_layers#switching-and-toggling-layers),

  - [ZMK's "toggle layer" behaviour](https://zmk.dev/docs/keymaps/behaviors/layers#toggle-layer)

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        layers = [
          [
            K.layer_mod.toggle 1,
            K.A,
          ],
          [
            K.TTTT,
            K.B,
          ],
        ],
      }
      """

  Example: pressing the toggle layer modifier key activates the layer
    When the keymap registers the following input
      """
      [
        press (K.layer_mod.toggle 1),
        press (K.B),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.B] }
      """

  Example: pressing toggle layer modifier key a second time deactivates the layer
    When the keymap registers the following input
      """
      [
        press (K.layer_mod.toggle 1),
        release (K.layer_mod.toggle 1),
        press (K.layer_mod.toggle 1),
        press (K.A),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.A] }
      """
