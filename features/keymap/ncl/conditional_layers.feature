Feature: Conditional layers

  Conditional layers activate a then-layer when all of a set of
   if-layers are active.
  This generalizes QMK-style "tri-layer" (lower + raise → adjust).

  For examples of this feature in other smart keyboard firmware, see e.g.:

  - [ZMK's conditional layers](https://zmk.dev/docs/keymaps/conditional-layers),

  - [FAK's conditional layers](https://github.com/semickolon/fak)

  Background:

    Three keys: lower (MO 1), raise (MO 2), and a letter key.
    Layer 3 ("adjust") is only activated
     by the conditional rule when both lower and raise are active.

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        conditional_layers = [
          { then_layer = 3, if_layers = [1, 2] },
        ],
        layers = [
          [
            K.layer_mod.hold 1,
            K.layer_mod.hold 2,
            K.A,
          ],
          [
            K.TTTT,
            K.TTTT,
            K.B,
          ],
          [
            K.TTTT,
            K.TTTT,
            K.C,
          ],
          [
            K.TTTT,
            K.TTTT,
            K.D,
          ],
        ],
      }
      """

  Example: lower and raise together activate adjust

    The letter key is pressed by keymap index: the Nickel input helper
    tracks hold/toggle layers but not conditional then-layers, so the
    key cannot be looked up as `K.D` while adjust is only firmware-active.

    When the keymap registers the following input
      """
      [
        press (K.layer_mod.hold 1),
        press (K.layer_mod.hold 2),
        press_keymap_index 2,
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.D] }
      """
