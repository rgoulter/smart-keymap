Feature: Named Layers

  Named layers are like `layers`, but whole overlay rows are declared by
   name rather than by numeric index.

  Use keymap-level `named_layers` for Fn-style full rows, and refer to
   those names from `layer_mod` (e.g. `K.layer_mod.hold "fn"`). Names
   are lowered to dense numbered slots after any numbered `layers`.
   Default order among names is alphabetical; set `named_layer_order`
   (array of name strings) to override which used names get which indices
   (listed names first, remaining used names append alphabetically).

  Per-key `{ named_layers = { … } }` still works and wins when the same
   name is set both at keymap scope and on a key.

  Background:

    Here, a keymap.ncl file with 2 keys: a hold layer-mod named `"fn"`,
     and a base `A` that becomes `B` on the `fn` named layer.

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        layers = [
          [
            K.layer_mod.hold "fn",
            K.A,
          ],
        ],
        named_layers = {
          fn = [
            K.TTTT,
            K.B,
          ],
        },
      }
      """

  Example: named layers act as the base when no layer is active

    If no named layers are active, the key will be the key
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

  Example: named layers act as active layer when its named layer modifier is held

    When the keymap registers the following input
      """
      [
        press (K.layer_mod.hold "fn"),
        press (K.B),
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.B] }
      """
