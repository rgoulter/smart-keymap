Feature: History Repeat Key

  The Repeat key re-emits the last resolved key output while held.

  After typing a letter, tapping Repeat types that letter again.
  Holding Repeat keeps that output pressed until Repeat is released.

  For examples of this key in other smart keyboard firmware, see e.g.:

  - [QMK's Repeat Key](https://docs.qmk.fm/features/repeat_key),

  - [ZMK's key-repeat](https://zmk.dev/docs/behaviors/key-repeat)

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        keys = [
          K.A,
          K.B,
          K.history.repeat,
        ]
      }
      """

  Example: tapping repeat after A types A again
    When the keymap registers the following input
      """
      [
        tap K.A,
        tap K.history.repeat,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap K.A,
        tap K.A,
      ]
      """

  Example: holding repeat holds the previous key output
    When the keymap registers the following input
      """
      [
        tap K.A,
        press K.history.repeat,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap K.A,
        press K.A,
      ]
      """

  Example: repeat with no prior key does nothing
    When the keymap registers the following input
      """
      [
        tap K.history.repeat,
        tap K.A,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap K.A,
      ]
      """

  Example: repeat uses the most recent key, not an earlier one
    When the keymap registers the following input
      """
      [
        tap K.A,
        tap K.B,
        tap K.history.repeat,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap K.A,
        tap K.B,
        tap K.B,
      ]
      """
