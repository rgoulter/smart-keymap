Feature: Simple Key

  Example: Keymap with a simple Key
    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      { keys = [ K.A ] }
      """
    When the keymap registers the following input
      """
      [
        Press(keymap_index: 0)
      ]
      """
    Then the HID keyboard report should equal
      """
      let K = import "hid-usage-keyboard.ncl" in
      { key_codes = [K.A] }
      """
