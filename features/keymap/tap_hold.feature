Feature: TapHold Key

  The TapHold key can behave differently depending on
   whether the key has been tapped or held.

  Quickly pressing and releasing a TapHold key results in a the 'tap' behaviour.
  Pressing the TapHold key for a long enough period results in the 'hold' behaviour.
  Pressing another key while the TapHold key is pressed "interrupts" the TapHold key,
   resulting in the 'hold' behaviour.

  Background:

    Let's use a keymap a tap-hold key, and a simple key.

    Given a keymap.ncl:
      """
      let K = import "keys.ncl" in
      { keys = [ K.A & K.hold K.LeftCtrl, K.B ] }
      """

  Example: acts as 'tap' when tapped
    When the keymap registers the following input
      """
      [
        Press(keymap_index: 0),
        Release(keymap_index: 0)
      ]
      """
    Then the HID keyboard report should equal
      """
      let K = import "hid-usage-keyboard.ncl" in
      { key_codes = [K.A] }
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
    Then the HID keyboard report from the next tick() should equal
      """
      let K = import "hid-usage-keyboard.ncl" in
      { modifiers = { left_ctrl = true } }
      """

  Example: acts as 'hold' when interrupted
    When the keymap registers the following input
      """
      [
        Press(keymap_index: 0),
        Press(keymap_index: 1)
      ]
      """
    Then the HID keyboard report from the next tick() should equal
      """
      let K = import "hid-usage-keyboard.ncl" in
      { modifiers = { left_ctrl = true }, key_codes = [K.B] }
      """
