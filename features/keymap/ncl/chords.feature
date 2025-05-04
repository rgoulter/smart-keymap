Feature: Chords

  "Chords" (also sometimes called "combos")
   allow pressing multiple keys together at the same time
   to behave as another key.

  For examples of this feature in other smart keyboard firmware, see e.g.:

  - [QMK's combo feature](https://docs.qmk.fm/features/combo),

  - [ZMK's keymap combos](https://zmk.dev/docs/keymaps/combos)

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        chords = [
            { indices = [0, 1], key = K.C, },
        ],
        keys = [
            K.A, K.B,
        ],
      }
      """

  Example: chorded key behaves as usual when tapped individually

    When the keymap registers the following input
      """
      [
        press K.A,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press (K.A),
      ]
      """

  Example: chorded keys tapped together behaves as chord

    When the keymap registers the following input
      """
      [
        press K.A,
        press K.B,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press (K.C),
      ]
      """

  Example: chorded keys pressed together with delay behaves as separate keys

    When the keymap registers the following input
      """
      [
        press K.A,
      ]
      """
    And the keymap ticks 250 times
    And the keymap registers the following input
      """
      [
        press K.B,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press (K.A),
        press (K.B),
      ]
      """
