Feature: Layers

  Background:

    Layers can be used by setting using the `layers` property
     of a keymap.ncl.

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        layers = [
          [
            { layer_modifier = { hold = 0 } }, K.A,
          ],
          [
            K.TTTT, K.B,
          ],
        ],
      }
      """

  Example: acts base key when no layer active
    When the keymap registers the following input
      """
      [
        Press(keymap_index: 1),
      ]
      """
    Then the HID keyboard report from the next tick() should equal
      """
      let K = import "hid-usage-keyboard.ncl" in
      { key_codes = [K.A] }
      """

  Example: acts the key on that layer when layer modifier held
    When the keymap registers the following input
      """
      [
        Press(keymap_index: 0),
        Press(keymap_index: 1),
      ]
      """
    Then the HID keyboard report from the next tick() should equal
      """
      let K = import "hid-usage-keyboard.ncl" in
      { key_codes = [K.B] }
      """
