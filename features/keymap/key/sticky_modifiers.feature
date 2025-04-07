Feature: Sticky Modifiers Key

  The "Sticky Modifiers" key is keymap implementation of the
   "sticky key" accessibility feature that many desktop environments have.

  If the sticky modifier key is tapped (without interruption),
   it modifies the next key press.

  If the sticky modifier key is interrupted by another key press,
   then it behaves as a regular modifier key.

  For examples of this key in other smart keyboard firmware, see e.g.:

  - [QMK's one shot keys](https://docs.qmk.fm/one_shot_keys),

  - [ZMK's sticky key behavior](https://zmk.dev/docs/keymaps/behaviors/sticky-key)

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        keys = [
          (K.sticky K.LeftShift),
          K.A,
          K.B,
        ]
      }
      """

  Example: tapping sticky modifier key modifies next key press
    When the keymap registers the following input
      """
      [
        press (K.sticky K.LeftShift),
        release (K.sticky K.LeftShift),
        press K.A,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press (K.LeftShift),
        press K.A,
      ]
      """
