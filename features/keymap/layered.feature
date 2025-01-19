Feature: Layers

  Layers are a basic part of smart keyboard firmware.

  Layers are like the Fn key on laptop keyboards,
   where holding the Fn key allows alternate functionality
   for other keys on the keyboard.

  For examples of this key in other smart keyboard firmware, see e.g.:

  - [QMK's layers feature](https://docs.qmk.fm/feature_layers),

  - [ZMK's layers keymap behaviors](https://zmk.dev/docs/keymaps/behaviors/layers)

  Background:

    Layers can be used by setting using the `layers` property
     of a keymap.ncl.

    Here, a keymap.ncl file with 2 keys, and 2 layers (base layer + 1 layer).

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        layers = [
          [
            K.layer_mod.hold 0,
            K.A,
          ],
          [
            K.TTTT,
            K.B,
          ],
        ],
      }
      """

  Example: acts as the base key when no layer is active

    If no layers are active, the key will be the key
     on the base layer.

    When the keymap registers the following input
      """
      [
        Press(keymap_index: 1),
      ]
      """
    Then the HID keyboard report should equal
      """
      let K = import "hid-usage-keyboard.ncl" in
      { key_codes = [K.A] }
      """

  Example: acts as the key on that layer when its layer modifier held

    When the keymap registers the following input
      """
      [
        Press(keymap_index: 0),
        Press(keymap_index: 1),
      ]
      """
    Then the HID keyboard report should equal
      """
      let K = import "hid-usage-keyboard.ncl" in
      { key_codes = [K.B] }
      """
