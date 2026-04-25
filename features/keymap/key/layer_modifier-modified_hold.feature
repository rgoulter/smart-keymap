Feature: Layer Modifier: Hold (Modified)

  `K.layer_mod.hold` keys support being modified with keyboard modifiers,
   just as keyboard keys can be modified.

  e.g. `K.layer_mod.hold 1 & K.LeftShift` key acts as both `Hold(1)` and
   `LeftShift` while held.

  Background:
    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        layers = [
          [
            K.layer_mod.hold 1 & K.LeftShift,
            K.A,
          ],
          [
            K.TTTT,
            K.B,
          ],
        ],
      }
      """

  Example: modified hold layer modifier key reports as modifier
    When the keymap registers the following input
      """
      [
        press (K.layer_mod.hold 1 & K.LeftShift),
      ]
      """
    Then the HID keyboard report should equal
      """
      { modifiers = { left_shift = true } }
      """

  Example: layers acts as active layer when a hold modifier key is held
    When the keymap registers the following input
      """
      [
        press (K.layer_mod.hold 1 & K.LeftShift),
        press (K.B),
      ]
      """
    Then the HID keyboard report should equal
      """
      { modifiers = { left_shift = true }, key_codes = [K.B] }
      """
