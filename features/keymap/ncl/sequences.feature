Feature: Sequences

  Smart Keymap supports defining sequences of keys to resolve
   as a different key.

  This is similar to key chords,
   except is for keys pressed in sequence (pressed one after another)
   rather than as a chord (pressed all at once).

  This is a similar idea to Vim's "leader key sequences".

  Sequence `indices` may be a raw array (`[1, 2]`), or a layout string
  via `sequence.ncl`. Non-`_` markers are sorted so tap order can
  differ from scan order (unlike chord `X` marks):

      let Seq = import "sequence.ncl" in
      { indices = "1 0 _" |> Seq.indices, key = K.C }

  `"1 0 _"` is keys 0 then 1 in scan order, but the sequence is
  key 1 then key 0.

  For examples of this feature in other smart keyboard firmware, see e.g.:

  - [QMK's Leader Key](https://docs.qmk.fm/features/leader_key)

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      let Seq = import "sequence.ncl" in
      {
        sequences = [
          { indices = "_ 0 1 _" |> Seq.indices, key = K.C },
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
