Feature: Layer Modifier: Set Default Layer

  The `K.layer_mod.set_default` key allows setting the Default layer.

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
        press (K.layer_mod.set_default 1),
        release (K.layer_mod.set_default 1),
        press (K.B),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.B] }
      """
