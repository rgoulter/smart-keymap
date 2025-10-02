Feature: TapDance Key

  The TapDance key can behave differently depending on
   how many times it is tapped.

  For examples of this key in other smart keyboard firmware, see e.g.:

  - [QMK's tap dance keys](https://docs.qmk.fm/features/tap_dance),

  - [ZMK's tap dance keymap behaviors](https://zmk.dev/docs/keymaps/behaviors/tap-dance)

  Background:

    Let's use a keymap with a tap-hold key, and a keyboard key.

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        keys = [
          K.A & { tap_dances = [K.B, K.C] },
        ]
      }
      """

  Example: tap dance key acts as its first definition when tapped once
    When the keymap registers the following input
      """
      [
        press (K.A & { tap_dances = [K.B, K.C] }),
        release (K.A & { tap_dances = [K.B, K.C] }),
      ]
      """
    And the keymap ticks 250 times
    Then the output should be equivalent to output from
      """
      [
        press K.A,
        release K.A,
      ]
      """

  Example: tap dance key acts as its first definition when pressed once and held

    When the keymap registers the following input
      """
      [
        press (K.A & { tap_dances = [K.B, K.C] })
      ]
      """
    And the keymap ticks 250 times
    Then the output should be equivalent to output from
      """
      [
        press K.A,
      ]
      """

  Example: tap dance key acts as its third definition when tapped three times
    When the keymap registers the following input
      """
      [
        tap (K.A & { tap_dances = [K.B, K.C] }),
        tap (K.A & { tap_dances = [K.B, K.C] }),
        tap (K.A & { tap_dances = [K.B, K.C] }),
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap K.C,
      ]
      """
