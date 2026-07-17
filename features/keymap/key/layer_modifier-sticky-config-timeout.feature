Feature: Layer Modifier: Sticky (configure sticky layer timeout)

  After a sticky layer is activated by tapping the sticky layer
   modifier, it normally stays active until another key is used.

  Setting `config.layered.sticky_timeout` (milliseconds) deactivates
   the sticky layer if no key is pressed before the timeout.

  This is similar to ZMK sticky layer `release-after-ms`.

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        config.layered.sticky_timeout = 100,
        layers = [
          [K.layer_mod.sticky 1, K.A, K.B],
          [K.TTTT, K.X, K.Y],
        ],
      }
      """

  Example: sticky layer deactivates after timeout without a key press

    When the keymap registers the following input
      """
      [
        tap (K.layer_mod.sticky 1),
        wait 110,
        tap_keymap_index 1,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap (K.A),
      ]
      """

  Example: sticky layer still applies if a key is pressed before timeout

    When the keymap registers the following input
      """
      [
        tap (K.layer_mod.sticky 1),
        wait 50,
        tap_keymap_index 1,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap (K.X),
      ]
      """
