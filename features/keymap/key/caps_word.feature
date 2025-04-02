Feature: Caps Word Key

  The "Caps Word" key can be thought of as "Caps Lock, for a single word".

  Where Caps Lock shifts all keys until it is disabled,
   Caps Word shifts while alphabetical keys (and underscore) are typed.

  A motivating use case is typing out `CONSTANTS_LIKE_THIS`,
   automatically leaving the caps word mode when space is hit.

  For examples of this key in other smart keyboard firmware, see e.g.:

  - [QMK's caps word feature](https://docs.qmk.fm/#/feature_caps_word),

  - [ZMK's caps word keycode](https://docs.qmk.fm/keycodes#mod-tap-keys)

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        keys = [
          K.caps_word.toggle,
          K.A,
          K.B,
          K.Space,
        ]
      }
      """

  Example: acts as 'tap' when tapped
    When the keymap registers the following input
      """
      [
        press K.caps_word.toggle,
        release K.caps_word.toggle,
        press K.A,
        release K.A,
        press K.Space,
        release K.Space,
        press K.A,
        release K.A,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press (K.LeftShift),
        press K.A,
        release K.A,
        release (K.LeftShift),
        press K.Space,
        release K.Space,
        press K.A,
        release K.A,
      ]
      """
