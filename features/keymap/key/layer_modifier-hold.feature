Feature: Layer Modifier: Hold

  The `K.layer_mod.hold` key activates a layer when it is held.

  For examples of this feature in other smart keyboard firmware, see e.g.:

  - [QMK's MO(layer)](https://docs.qmk.fm/feature_layers#switching-and-toggling-layers),

  - [ZMK's "momentary layer" behaviour](https://zmk.dev/docs/keymaps/behaviors/layers#momentary-layer)

  Background:

    Layers can be used by setting using the `layers` field
     of a keymap.ncl.

    Here, a keymap.ncl file with 2 keys, and 2 layers (base layer + 1 layer).

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        layers = [
          [
            K.layer_mod.hold 1,
            K.A,
          ],
          [
            K.TTTT,
            K.B,
          ],
        ],
      }
      """

  Example: layers acts as the base when no hold modifier key is held

    If no layers are active, the key will be the key
     on the base layer.

    When the keymap registers the following input
      """
      [
        press (K.A),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.A] }
      """

  Example: layers acts as active layer when a hold modifier key is held

    When the keymap registers the following input
      """
      [
        press (K.layer_mod.hold 1),
        press (K.B),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.B] }
      """
