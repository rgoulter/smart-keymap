Feature: Layer Modifier: Set Active Layers Amongst (Mask)

  The `K.layer_mod.set_active_layers_amongst` key sets active layers within a
  mask, leaving layers outside the mask unchanged.

  This supports orthogonal layer groups, e.g. switching "OS desktop" layers
  without affecting unrelated layers such as LOWER or RAISE.

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        layers = [
          [
            K.layer_mod.set_active_layers_to [1],
            K.layer_mod.set_active_layers_amongst {
              set_active_layers_to = [3],
              affected_layers = [3, 4],
            },
            K.layer_mod.set_active_layers_amongst {
              set_active_layers_to = [4],
              affected_layers = [3, 4],
            },
            K.A,
            K.G,
            K.D,
          ],
          [K.TTTT, K.TTTT, K.TTTT, K.TTTT, K.B, K.E],
          [K.TTTT, K.TTTT, K.TTTT, K.TTTT, K.TTTT, K.TTTT],
          [K.TTTT, K.TTTT, K.TTTT, K.TTTT, K.C, K.TTTT],
          [K.TTTT, K.TTTT, K.TTTT, K.TTTT, K.F, K.TTTT],
        ],
      }
      """

  Example: masked set active layers preserves layers outside the mask

    When the keymap registers the following input
      """
      [
        tap (K.layer_mod.set_active_layers_to [1]),
        tap_keymap_index 5,
        tap (K.layer_mod.set_active_layers_amongst {
          set_active_layers_to = [3],
          affected_layers = [3, 4],
        }),
        tap_keymap_index 4,
        tap_keymap_index 5,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap (K.E),
        tap (K.C),
        tap (K.E),
      ]
      """

  Example: masked set active layers switches within the mask

    When the keymap registers the following input
      """
      [
        tap (K.layer_mod.set_active_layers_amongst {
          set_active_layers_to = [3],
          affected_layers = [3, 4],
        }),
        tap_keymap_index 4,
        tap (K.layer_mod.set_active_layers_amongst {
          set_active_layers_to = [4],
          affected_layers = [3, 4],
        }),
        tap_keymap_index 4,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap (K.C),
        tap (K.F),
      ]
      """
