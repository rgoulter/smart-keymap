Feature: Consumer Keys (Modified)

  Consumer keys support being modified with keyboard modifiers,
   just as keyboard keys can be modified.

  e.g. `K.VolumeUp & K.LeftShift` sends VolumeUp on the consumer report
   and LeftShift on the keyboard report while held.

  Background:
    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        keys = [
          K.VolumeUp & K.LeftShift,
        ],
      }
      """

  Example: modified consumer key reports as keyboard modifier
    When the keymap registers the following input
      """
      [
        press (K.VolumeUp & K.LeftShift),
      ]
      """
    Then the HID keyboard report should equal
      """
      { modifiers = { left_shift = true } }
      """
