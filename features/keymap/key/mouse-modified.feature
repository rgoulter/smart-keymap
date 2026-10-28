Feature: Mouse Keys (Modified)

  Mouse keys support being modified with keyboard modifiers,
   just as keyboard keys can be modified.

  e.g. `K.MouseBtn1 & K.LeftCtrl` sends mouse button 1 on the mouse report
   and LeftCtrl on the keyboard report while held.

  Background:
    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      {
        keys = [
          K.MouseBtn1 & K.LeftCtrl,
        ],
      }
      """

  Example: modified mouse key reports as keyboard modifier
    When the keymap registers the following input
      """
      [
        press (K.MouseBtn1 & K.LeftCtrl),
      ]
      """
    Then the HID keyboard report should equal
      """
      { modifiers = { left_ctrl = true } }
      """
