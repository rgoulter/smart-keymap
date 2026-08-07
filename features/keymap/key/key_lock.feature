Feature: Key Lock

  The Key Lock key holds down the next key you press until that key is pressed
  again. You can think of it as Caps Lock, but for any key (including modifiers).

  For examples of this key in other smart keyboard firmware, see e.g.:

  - [QMK's Key Lock](https://docs.qmk.fm/features/key_lock)

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        keys = [
          K.key_lock,
          K.LeftShift,
          K.A,
          K.B,
        ]
      }
      """

  Example: key lock holds next key after release
    When the keymap registers the following input
      """
      [
        tap K.key_lock,
        tap K.LeftShift,
        press K.A,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press K.LeftShift,
        press K.A,
      ]
      """

  Example: locked key releases when pressed again
    When the keymap registers the following input
      """
      [
        tap K.key_lock,
        tap K.LeftShift,
        tap K.LeftShift,
        press K.A,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap K.LeftShift,
        press K.A,
      ]
      """

  Example: key lock watching cancels when toggled off
    When the keymap registers the following input
      """
      [
        tap K.key_lock,
        tap K.key_lock,
        tap K.A,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        tap K.A,
      ]
      """
