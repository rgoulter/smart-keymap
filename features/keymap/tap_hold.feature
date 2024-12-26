Feature: TapHold Key

  The TapHold key can behave differently depending on
   whether the key has been tapped or held.

  Quickly pressing and releasing a TapHold key results in a the 'tap' behaviour.
  Pressing the TapHold key for a long enough period results in the 'hold' behaviour.
  Pressing another key while the TapHold key is pressed "interrupts" the TapHold key,
   resulting in the 'hold' behaviour.

  Background:

    Let's use a keymap with A=0x04, B=0x05, and Ctrl=0xE0.

    Given a keymap, expressed as a RON string
      """
      [TapHold(Key(tap: 0x04, hold: 0xE0)), Simple(Key(0x05))]
      """

  Example: acts as 'tap' when tapped
    When the keymap registers the following input
      """
      [
        Press(keymap_index: 0),
        Release(keymap_index: 0)
      ]
      """
    Then the HID keyboard report from the next tick() should be
      """
      [0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00]
      """

  Example: acts as 'hold' when held
    (Recall, holding down Ctrl=0xE0 shows up as 0x01 in the HID report.)

    When the keymap registers the following input
      """
      [
        Press(keymap_index: 0)
      ]
      """
    And the keymap ticks 500 times
    Then the HID keyboard report from the next tick() should be
      """
      [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
      """

  Example: acts as 'hold' when interrupted
    When the keymap registers the following input
      """
      [
        Press(keymap_index: 0),
        Press(keymap_index: 1)
      ]
      """
    Then the HID keyboard report from the next tick() should be
      """
      [0x01, 0x00, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00]
      """
