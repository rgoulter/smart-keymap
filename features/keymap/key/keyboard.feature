Feature: HID Keyboard Key

  Example: Keymap with a keyboard Key
    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      { keys = [ K.A ] }
      """
    When the keymap registers the following input
      """
      [
        press K.A,
      ]
      """
    Then the HID keyboard report should equal
      """
      { key_codes = [K.A] }
      """

  Example: Keymap with a modified keyboard Key

    In keymap.ncl, modifiers can be merged with keys
     to form a modified key.

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      { keys = [ K.A & K.LeftCtrl ] }
      """
    When the keymap registers the following input
      """
      [
        press (K.A & K.LeftCtrl),
      ]
      """
    Then the HID keyboard report should equal
      """
      { modifiers = { left_ctrl = true }, key_codes = [K.A] }
      """
