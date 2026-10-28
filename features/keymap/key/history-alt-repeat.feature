Feature: History Alternate Repeat Key

  The Alternate Repeat key emits a configured alternate of the last
  resolved key output while held (QMK-style alt-repeat for single keys).

  Its rules are defined in Nickel under `config.history.alt_repeat` as
  `{ prev, emit }` pairs using ordinary keyboard keys.

  Unmapped previous keys (and empty history) contribute no output.

  For examples of this key in other smart keyboard firmware, see e.g.:

  - [QMK's Alternate Repeat Key](https://docs.qmk.fm/features/repeat_key)

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        config.history.alt_repeat = [
          { prev = K.Left, emit = K.Right },
          { prev = K.Right, emit = K.Left },
          { prev = K.Up, emit = K.Down },
          { prev = K.Down, emit = K.Up },
        ],
        keys = [
          K.Left,
          K.Right,
          K.Up,
          K.Down,
          K.A,
          K.history.alt_repeat,
        ]
      }
      """

  Example: alt-repeat after Left types Right
    When the keymap registers the following input
      """
      [
        tap K.Left,
        tap K.history.alt_repeat,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap K.Left,
        tap K.Right,
      ]
      """

  Example: alt-repeat after Right types Left
    When the keymap registers the following input
      """
      [
        tap K.Right,
        tap K.history.alt_repeat,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap K.Right,
        tap K.Left,
      ]
      """

  Example: holding alt-repeat holds the mapped output
    When the keymap registers the following input
      """
      [
        tap K.Left,
        press K.history.alt_repeat,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap K.Left,
        press K.Right,
      ]
      """

  Example: alt-repeat with unmapped prior key does nothing
    When the keymap registers the following input
      """
      [
        tap K.A,
        tap K.history.alt_repeat,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap K.A,
      ]
      """

  Example: alt-repeat with no prior key does nothing
    When the keymap registers the following input
      """
      [
        tap K.history.alt_repeat,
        tap K.A,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap K.A,
      ]
      """
