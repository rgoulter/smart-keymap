Feature: Layered Keys

  "Layered Keys" are a lower-level key which implements
   layering functionality.

  See [Layers](#layers) for a friendlier way to declare layers
   in a keymap.ncl file.

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        keys = [
          K.layer_mod.hold 1,
          K.A & { layered = [ K.B ] },
        ],
      }
      """

  Example: acts as the base key when no layer is active

    If no layers are active, the key will be the key
     on the base layer.

    When the keymap registers the following input
      """
      [
        press (K.A & { layered = [ K.B ] }),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.A] }
      """

  Example: acts as the key on that layer when its layer modifier held

    When the keymap registers the following input
      """
      [
        press (K.layer_mod.hold 1),
        press (K.A & { layered = [ K.B ] }),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.B] }
      """
