Feature: Simple Key

  Example: Deserialize a simple::Key
    Given a keymap, expressed as a RON string
      """
      [Simple(key: Key(0x04)), Simple(key: Key(0x05))]
      """
    When the keymap registers the following input
      """
      [
        Press(keymap_index: 0)
      ]
      """
    Then the HID keyboard report should be
      """
      [0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00]
      """
