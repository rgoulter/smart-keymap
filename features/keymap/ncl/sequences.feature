Feature: Sequences

  "Sequences" (also called "leader keys" in QMK) allow arming a mode,
  then tapping keys in order to behave as another key.

  After tapping the sequence start key, subsequent presses of the
  sequence members (in order) emit the bound key. Members act as
  their usual keys when sequence mode is not armed.

  For examples of this feature in other smart keyboard firmware, see e.g.:

  - [QMK's Leader Key](https://docs.qmk.fm/features/leader_key)

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        sequences = [
          { indices = [1, 2], key = K.C },
        ],
        config.sequence.timeout = 500,
        keys = [
          K.sequence_start,
          K.A,
          K.B,
          K.X,
        ],
      }
      """

  Example: sequence start then two steps emits bound key

    When the keymap registers the following input
      """
      [
        tap K.sequence_start,
        tap K.A,
        tap K.B,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap K.C,
      ]
      """

  Example: sequence member acts as usual when mode is inactive

    When the keymap registers the following input
      """
      [
        tap K.A,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap K.A,
      ]
      """

  Example: unknown key while armed aborts without sequence output

    Sequence mode ends without emitting the bound key. The interrupting
    key still acts as itself.

    When the keymap registers the following input
      """
      [
        tap K.sequence_start,
        tap K.A,
        tap K.X,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap K.X,
      ]
      """

  Example: timeout aborts incomplete sequence without output

    When the keymap registers the following input
      """
      [
        tap K.sequence_start,
        tap K.A,
        wait 600,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
      ]
      """

  Example: re-tapping start restarts the sequence buffer

    When the keymap registers the following input
      """
      [
        tap K.sequence_start,
        tap K.A,
        tap K.sequence_start,
        tap K.A,
        tap K.B,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap K.C,
      ]
      """
