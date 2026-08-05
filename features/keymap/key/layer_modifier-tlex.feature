Feature: Transparent Layer Exit

  `K.tlex` deactivates the layer that owns the binding (when that layer was
   activated non-persistently — hold, toggle, or sticky), then registers the
   key as if that layer were off (continue-eval to the next lower defined
   active layer, or base).

  This is an alternative to "smart layers": exit by pressing a key on the
   layer rather than only by releasing the layer modifier.

  For examples of this feature in other smart keyboard firmware, see e.g.:

  - [Fak's transparent layer exit (`tap.tlex` / `hold.tlex`)](https://github.com/semickolon/fak#transparent-layer-exit)

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        layers = [
          [
            K.layer_mod.toggle 1,
            K.A,
            K.C,
          ],
          [
            K.TTTT,
            K.tlex,
            K.B,
          ],
        ],
      }
      """

  Example: tlex types the base key and leaves the layer off

    When the keymap registers the following input
      """
      [
        tap (K.layer_mod.toggle 1),
        press (K.tlex),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.A] }
      """

  Example: after tlex, other keys on that layer use base bindings

    When the keymap registers the following input
      """
      [
        tap (K.layer_mod.toggle 1),
        tap (K.tlex),
        press (K.C),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.C] }
      """

  Example: without tlex, the layer key would still be active

    When the keymap registers the following input
      """
      [
        tap (K.layer_mod.toggle 1),
        press (K.B),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.B] }
      """
