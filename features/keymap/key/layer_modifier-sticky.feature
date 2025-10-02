Feature: Layer Modifier: Sticky

  The `K.layer_mod.sticky` key activates a layer in a similar manner to sticky key modifiers:

  - tapping the sticky layer modifier activates the layer for the next key press.
  - holding the sticky layer modifier activates the layer, similar to `K.layer_mod.hold`.

  For examples of this feature in other smart keyboard firmware, see e.g.:

  - [QMK's OSL(layer)](https://docs.qmk.fm/feature_layers#switching-and-toggling-layers),

  - [ZMK's "sticky layer" behaviour](https://zmk.dev/docs/keymaps/behaviors/sticky-layer)

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
          layers = [
              [K.layer_mod.sticky 1, K.A, K.B],
              [K.TTTT, K.X, K.Y],
          ],
      }
      """

  Example: tapping sticky layer modifier activates the later for the next pressed key

    When the keymap registers the following input
      """
      [
        tap (K.layer_mod.sticky 1),
        tap_keymap_index 1,
        tap_keymap_index 2,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap (K.X),
        tap (K.B),
      ]
      """

  Example: tapping sticky layer modifier activates the layer only for the next pressed key

    When the keymap registers the following input
      """
      [
        tap (K.layer_mod.sticky 1),
        press_keymap_index 1,
        press_keymap_index 2,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press (K.X),
        press (K.B),
      ]
      """

  Example: pressing sticky layer modifier activates the layer

    When the keymap registers the following input
      """
      [
        press (K.layer_mod.sticky 1),
        tap_keymap_index 1,
        tap_keymap_index 2,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap (K.X),
        tap (K.Y),
      ]
      """
