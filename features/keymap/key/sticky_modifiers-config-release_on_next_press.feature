Feature: Sticky Modifiers Key (configure release: OnNextKeyPress)

  The `config.sticky.release` can be set to `"OnNextKeyPress"`
   so that the sticky key releases when the next key is pressed
   after the modified key.

  This helps with rolling key presses.

  Background:

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        config.sticky.release = "OnNextKeyPress",
        keys = [
          (K.sticky K.LeftShift),
          (K.sticky K.LeftCtrl),
          K.A,
          K.B,
        ]
      }
      """

  Example: the sticky modifier releases when the next key is pressed
    When the keymap registers the following input
      """
      [
        tap (K.sticky K.LeftShift),
        press K.A,
        press K.B,
      ]
      """
    Then the output should be equivalent to output from
      """
      [
        press (K.LeftShift),
        press K.A,
        release (K.LeftShift),
        press K.B,
      ]
      """

